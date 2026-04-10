use alloy::primitives::{Address, U256};
use alloy_signer::Signature;
use serde::{
    de::DeserializeOwned,
    ser::{Error, SerializeMap, SerializeStruct},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{
    actions::{
        core::{SignedAction, SignedActionKind},
        traits::Action,
        ActionKind,
    },
    SigningChain,
};

/// Compute the L1 action hash (MessagePack + metadata)
pub(crate) struct L1ActionWrapper<'a, T: Action + Serialize> {
    pub action: &'a T,
}

impl<'a, T: Action + Serialize> Serialize for L1ActionWrapper<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // For nested payloads, we serialize the action directly to avoid
        // serde_json::Value intermediary which produces different msgpack output
        if T::PAYLOAD_KEY != T::ACTION_TYPE {
            // Nested payload: {"type": "X", "payloadKey": {action fields}}
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("type", T::ACTION_TYPE)?;
            map.serialize_entry(T::PAYLOAD_KEY, self.action)?;
            return map.end();
        }

        // Flattened payload: action produces full wire format {type, ...fields} with canonical key order
        self.action.serialize(serializer)
    }
}

struct SigSer<'a>(&'a Signature);

impl<'a> Serialize for SigSer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_sig(self.0, serializer)
    }
}

fn action_type_from_obj<E: serde::de::Error>(
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Result<&str, E> {
    obj.get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| E::custom("missing action type"))
}

fn extract_action_payload_for<T: Action, E: serde::de::Error>(
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Result<serde_json::Value, E> {
    let payload = if T::PAYLOAD_KEY == T::ACTION_TYPE {
        let mut payload = obj.clone();
        payload.remove("type");
        if T::is_user_signed()
            && T::uses_time()
            && payload.contains_key("time")
            && !payload.contains_key("nonce")
        {
            if let Some(time_value) = payload.get("time").cloned() {
                payload.insert("nonce".to_string(), time_value);
            }
        }
        serde_json::Value::Object(payload)
    } else {
        let payload = obj
            .get(T::PAYLOAD_KEY)
            .ok_or_else(|| E::custom("missing action payload"))?;
        if T::is_user_signed() {
            let mut payload = payload
                .as_object()
                .cloned()
                .ok_or_else(|| E::custom("action payload must be object"))?;
            if T::uses_time() && payload.contains_key("time") && !payload.contains_key("nonce") {
                if let Some(time_value) = payload.get("time").cloned() {
                    payload.insert("nonce".to_string(), time_value);
                }
            }
            serde_json::Value::Object(payload)
        } else {
            payload.clone()
        }
    };
    Ok(payload)
}

/// Parse an action from a JSON object, extracting and normalizing the payload.
///
/// This is `pub(crate)` so the `dispatch_action_kind` macro-generated code in
/// `mod.rs` can call it.
pub(crate) fn parse_action_from_obj<T, E>(
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Result<T, E>
where
    T: Action + DeserializeOwned,
    E: serde::de::Error,
{
    let payload = extract_action_payload_for::<T, E>(obj)?;
    serde_json::from_value(payload).map_err(E::custom)
}

fn deserialize_action_kind<'de, D>(deserializer: D) -> Result<ActionKind, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    let obj = value
        .as_object()
        .ok_or_else(|| serde::de::Error::custom("action must be object"))?;

    // Calls the macro-generated dispatcher in mod.rs
    super::dispatch_action_kind::<D::Error>(obj)
}

fn deserialize_action<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Action + DeserializeOwned,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    let obj = value
        .as_object()
        .ok_or_else(|| serde::de::Error::custom("action must be object"))?;

    let action_type = action_type_from_obj::<D::Error>(obj)?;
    if action_type != T::ACTION_TYPE {
        return Err(serde::de::Error::custom(format!(
            "invalid action type: {}",
            action_type
        )));
    }

    let action_payload = extract_action_payload_for::<T, D::Error>(obj)?;
    serde_json::from_value(action_payload).map_err(serde::de::Error::custom)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "T: Action + DeserializeOwned"))]
struct SignedActionHelper<T: Action> {
    #[serde(deserialize_with = "deserialize_action")]
    action: T,
    nonce: u64,
    #[serde(deserialize_with = "deserialize_sig")]
    signature: Signature,
    vault_address: Option<Address>,
    expires_after: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignedActionKindHelper {
    #[serde(deserialize_with = "deserialize_action_kind")]
    action: ActionKind,
    nonce: u64,
    #[serde(deserialize_with = "deserialize_sig")]
    signature: Signature,
    vault_address: Option<Address>,
    expires_after: Option<u64>,
}

impl<T: Action + Serialize> Serialize for SignedAction<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let action_value =
            build_action_value(&self.action, self.signing_chain.as_ref()).map_err(Error::custom)?;
        let field_count = 3
            + self.vault_address.is_some() as usize
            + self.expires_after.is_some() as usize;
        let mut state = serializer.serialize_struct("SignedAction", field_count)?;
        state.serialize_field("action", &action_value)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("signature", &SigSer(&self.signature))?;
        if let Some(vault_address) = &self.vault_address {
            state.serialize_field("vaultAddress", vault_address)?;
        }
        if let Some(expires_after) = &self.expires_after {
            state.serialize_field("expiresAfter", expires_after)?;
        }
        state.end()
    }
}

impl<'de, T: Action + DeserializeOwned> Deserialize<'de> for SignedAction<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = SignedActionHelper::deserialize(deserializer)?;
        Ok(SignedAction {
            action: helper.action,
            nonce: helper.nonce,
            signature: helper.signature,
            vault_address: helper.vault_address,
            expires_after: helper.expires_after,
            signing_chain: None,
        })
    }
}

impl<'de> Deserialize<'de> for SignedActionKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = SignedActionKindHelper::deserialize(deserializer)?;
        Ok(SignedActionKind {
            action: helper.action,
            nonce: helper.nonce,
            signature: helper.signature,
            vault_address: helper.vault_address,
            expires_after: helper.expires_after,
            signing_chain: None,
        })
    }
}

fn serialize_sig<S>(sig: &Signature, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut state = serializer.serialize_struct("Signature", 3)?;
    state.serialize_field("r", &format!("0x{:064x}", sig.r()))?;
    state.serialize_field("s", &format!("0x{:064x}", sig.s()))?;
    state.serialize_field("v", &(27 + sig.v() as u64))?;
    state.end()
}

fn deserialize_sig<'de, D>(deserializer: D) -> Result<Signature, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    let obj = value
        .as_object()
        .ok_or_else(|| serde::de::Error::custom("signature must be object"))?;

    let r = obj
        .get("r")
        .and_then(|v| v.as_str())
        .ok_or_else(|| serde::de::Error::custom("missing signature.r"))?;
    let s = obj
        .get("s")
        .and_then(|v| v.as_str())
        .ok_or_else(|| serde::de::Error::custom("missing signature.s"))?;

    let v_value = obj
        .get("v")
        .ok_or_else(|| serde::de::Error::custom("missing signature.v"))?;
    let v = if let Some(v) = v_value.as_u64() {
        v
    } else if let Some(v) = v_value.as_str() {
        v.parse::<u64>()
            .map_err(|e| serde::de::Error::custom(e.to_string()))?
    } else {
        return Err(serde::de::Error::custom("invalid signature.v"));
    };

    let r = r.strip_prefix("0x").unwrap_or(r);
    let s = s.strip_prefix("0x").unwrap_or(s);

    let r = U256::from_str_radix(r, 16).map_err(|e| serde::de::Error::custom(e.to_string()))?;
    let s = U256::from_str_radix(s, 16).map_err(|e| serde::de::Error::custom(e.to_string()))?;
    let v = v
        .checked_sub(27)
        .ok_or_else(|| serde::de::Error::custom("invalid v value"))?;

    Ok(Signature::new(r, s, v != 0))
}

pub(crate) fn ser_lowercase<S>(address: &Address, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&address.to_string().to_lowercase())
}

impl<T: Action + DeserializeOwned> SignedAction<T> {
    /// Deserialize from the exchange API format
    pub fn from_json(json: &str) -> Result<Self, crate::Error> {
        serde_json::from_str(json).map_err(|e| crate::Error::JsonParse(e.to_string()))
    }
}

fn build_action_value<T: Action + Serialize>(
    action: &T,
    signing_chain: Option<&SigningChain>,
) -> Result<serde_json::Value, String> {
    let payload_value = serde_json::to_value(action).map_err(|e| e.to_string())?;
    let mut payload = payload_value;

    if T::is_user_signed() {
        let signing_chain = signing_chain
            .ok_or_else(|| "signing_chain must be set for user-signed actions".to_string())?;
        let obj = payload
            .as_object_mut()
            .ok_or_else(|| "action payload must be object".to_string())?;
        obj.insert(
            "signatureChainId".to_string(),
            serde_json::Value::String(format!("0x{:x}", signing_chain.get_signature_chain_id())),
        );
        obj.insert(
            "hyperliquidChain".to_string(),
            serde_json::Value::String(signing_chain.get_hyperliquid_chain()),
        );
        if T::uses_time() && obj.contains_key("nonce") && !obj.contains_key("time") {
            if let Some(nonce_value) = obj.remove("nonce") {
                obj.insert("time".to_string(), nonce_value);
            }
        }
    }

    let mut action_obj = serde_json::Map::new();
    action_obj.insert(
        "type".to_string(),
        serde_json::Value::String(T::ACTION_TYPE.to_string()),
    );

    // If the payload key is the same as the action type, we can just use the payload directly
    // This will effectively flatten the payload
    if T::PAYLOAD_KEY == T::ACTION_TYPE {
        let payload = payload
            .as_object()
            .cloned()
            .ok_or_else(|| "action payload must be object".to_string())?;
        for (key, value) in payload {
            action_obj.insert(key, value);
        }
    } else {
        // If the payload key is different from the action type, we need to insert the payload key and value
        action_obj.insert(T::PAYLOAD_KEY.to_string(), payload);
    }

    Ok(serde_json::Value::Object(action_obj))
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{Address, U256};
    use alloy_signer::Signature;
    use rust_decimal_macros::dec;
    use serde_json::json;

    use super::*;
    use crate::actions::{
        ActionKind, PreparedAction, SetOpenInterestCaps, SignedActionKind, ToggleBigBlocks, UsdSend,
    };
    use crate::SigningChain;

    #[test]
    fn test_enable_big_blocks_serialization() {
        let action = ToggleBigBlocks::enable();
        let signing_chain = SigningChain::Mainnet;

        let prepared = PreparedAction::new(action, &signing_chain, None, None).unwrap();

        // Create a dummy signature for testing
        let sig = Signature::new(U256::from(1), U256::from(2), false);
        let signed = prepared.with_signature(sig);

        let json = serde_json::to_string_pretty(&signed).unwrap();
        println!("{}", json);

        // Verify structure
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.get("action").is_some());
        assert!(parsed.get("nonce").is_some());
        assert!(parsed.get("signature").is_some());

        let action_obj = parsed.get("action").unwrap();
        assert_eq!(action_obj.get("type").unwrap(), "evmUserModify");
        assert!(action_obj.get("usingBigBlocks").is_some());
    }

    #[test]
    fn test_usd_send_serialization() {
        let action = UsdSend::new(Address::ZERO, dec!(100.0));
        let signing_chain = SigningChain::Testnet;

        let prepared = PreparedAction::new(action, &signing_chain, None, None).unwrap();

        let sig = Signature::new(U256::from(1), U256::from(2), false);
        let signed = prepared.with_signature(sig);

        let json = serde_json::to_string_pretty(&signed).unwrap();
        println!("{}", json);

        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let action_obj = parsed.get("action").unwrap();
        assert_eq!(action_obj.get("type").unwrap(), "usdSend");
        assert_eq!(action_obj.get("hyperliquidChain").unwrap(), "Testnet");
        assert_eq!(action_obj.get("signatureChainId").unwrap(), "0x66eee");
        assert!(action_obj.get("destination").is_some());
        assert!(action_obj.get("amount").is_some());
        assert!(action_obj.get("time").is_some());
    }

    #[test]
    fn test_perp_deploy_v2_set_open_interest_caps_serialization_matches_docs() {
        let action = SetOpenInterestCaps {
            caps: vec![("BTC".to_string(), 1_000_000), ("ETH".to_string(), 500_000)],
            nonce: None,
        };
        let signing_chain = SigningChain::Testnet;
        let prepared = PreparedAction::new(action, &signing_chain, None, None).unwrap();
        let sig = Signature::new(U256::from(3), U256::from(4), true);
        let signed = prepared.with_signature(sig);

        let parsed = serde_json::to_value(&signed).unwrap();
        let action_obj = parsed.get("action").unwrap();
        assert_eq!(
            action_obj,
            &json!({
                "type": "perpDeploy",
                "setOpenInterestCaps": [
                    ["BTC", 1_000_000],
                    ["ETH", 500_000]
                ]
            })
        );
    }

    #[test]
    fn test_extract_action_kind_preserves_values() {
        // Test UsdSend
        let dest = Address::repeat_byte(0x42);
        let amount = dec!(123.456);
        let nonce = 9876543210u64;
        let usd_send = UsdSend {
            destination: dest,
            amount,
            nonce: Some(nonce),
        };

        let signing_chain = SigningChain::Testnet;
        let prepared = PreparedAction::new(usd_send, &signing_chain, None, None).unwrap();
        let sig = Signature::new(U256::from(1), U256::from(2), false);
        let signed = prepared.with_signature(sig);

        // Extract ActionKind and verify values are preserved
        let kind = signed.extract_action_kind();
        match kind {
            ActionKind::UsdSend(extracted) => {
                assert_eq!(extracted.destination, dest);
                assert_eq!(extracted.amount, amount);
                assert_eq!(extracted.nonce, Some(nonce));
            }
            _ => panic!("Expected ActionKind::UsdSend"),
        }

        // Test EnableBigBlocks
        let enable_blocks = ToggleBigBlocks::enable();
        let prepared = PreparedAction::new(enable_blocks, &signing_chain, None, None).unwrap();
        let sig = Signature::new(U256::from(3), U256::from(4), true);
        let signed = prepared.with_signature(sig);

        let kind = signed.extract_action_kind();
        match kind {
            ActionKind::ToggleBigBlocks(extracted) => {
                assert!(extracted.using_big_blocks);
            }
            _ => panic!("Expected ActionKind::EnableBigBlocks"),
        }

        // Also test disable variant
        let disable_blocks = ToggleBigBlocks::disable();
        let prepared = PreparedAction::new(disable_blocks, &signing_chain, None, None).unwrap();
        let sig = Signature::new(U256::from(5), U256::from(6), false);
        let signed = prepared.with_signature(sig);

        let kind = signed.extract_action_kind();
        match kind {
            ActionKind::ToggleBigBlocks(extracted) => {
                assert!(!extracted.using_big_blocks);
            }
            _ => panic!("Expected ActionKind::EnableBigBlocks"),
        }
    }

    #[test]
    fn test_signed_action_serialization_roundtrip() {
        // Test UsdSend roundtrip
        let dest = Address::repeat_byte(0xAB);
        let amount = dec!(999.123);
        let nonce = 1234567890u64;
        let usd_send = UsdSend {
            destination: dest,
            amount,
            nonce: Some(nonce),
        };

        let signing_chain = SigningChain::Testnet;
        let prepared = PreparedAction::new(usd_send, &signing_chain, None, None).unwrap();
        let nonce = prepared.nonce;

        let r = U256::from_str_radix("abcd1234", 16).unwrap();
        let s = U256::from_str_radix("5678ef90", 16).unwrap();
        let sig = Signature::new(r, s, true);
        let signed = prepared.with_signature(sig);

        // Serialize to JSON
        let json = serde_json::to_string(&signed).unwrap();
        println!("Serialized UsdSend: {}", json);

        // Deserialize back
        let deserialized: SignedAction<UsdSend> = SignedAction::from_json(&json).unwrap();

        // Verify all fields match
        assert_eq!(deserialized.action.destination, dest);
        assert_eq!(deserialized.action.amount, amount);
        assert_eq!(deserialized.action.nonce, Some(nonce));
        assert_eq!(deserialized.nonce, nonce);
        assert_eq!(deserialized.signature.r(), signed.signature.r());
        assert_eq!(deserialized.signature.s(), signed.signature.s());
        assert_eq!(deserialized.signature.v(), signed.signature.v());
        assert_eq!(deserialized.vault_address, None);
        assert_eq!(deserialized.expires_after, None);

        // Test EnableBigBlocks roundtrip
        let enable_blocks = ToggleBigBlocks::enable();
        let prepared = PreparedAction::new(enable_blocks, &signing_chain, None, None).unwrap();
        let nonce = prepared.nonce;

        let sig = Signature::new(U256::from(111), U256::from(222), false);
        let signed = prepared.with_signature(sig);

        let json = serde_json::to_string(&signed).unwrap();
        println!("Serialized EnableBigBlocks: {}", json);

        let deserialized: SignedAction<ToggleBigBlocks> = SignedAction::from_json(&json).unwrap();

        assert!(deserialized.action.using_big_blocks);
        assert_eq!(deserialized.nonce, nonce);
        assert_eq!(deserialized.signature.r(), signed.signature.r());
        assert_eq!(deserialized.signature.s(), signed.signature.s());

        // Test with optional fields (vault_address)
        let vault = Address::repeat_byte(0x99);
        let usd_send = UsdSend {
            destination: dest,
            amount: dec!(50.0),
            nonce: Some(111222333),
        };
        let prepared = PreparedAction::new(usd_send, &signing_chain, Some(vault), None).unwrap();
        let sig = Signature::new(U256::from(333), U256::from(444), true);
        let signed = prepared.with_signature(sig);

        let json = serde_json::to_string(&signed).unwrap();
        println!("Serialized with vault: {}", json);

        let deserialized: SignedAction<UsdSend> = SignedAction::from_json(&json).unwrap();
        assert_eq!(deserialized.vault_address, Some(vault));
    }

    #[test]
    fn test_dec_ser() {
        assert_eq!("10000.5".to_string(), dec!(10_000.5).to_string());
    }

    #[test]
    fn test_signed_action_kind_deserializes_user_signed_action() {
        let dest = Address::repeat_byte(0xAB);
        let amount = dec!(1.23);
        let signing_chain = SigningChain::Testnet;

        let prepared =
            PreparedAction::new(UsdSend::new(dest, amount), &signing_chain, None, None).unwrap();
        let nonce = prepared.nonce;

        let sig = Signature::new(U256::from(1), U256::from(2), true);
        let signed = prepared.with_signature(sig);

        let json = serde_json::to_string(&signed).unwrap();
        let deserialized = SignedActionKind::from_json(&json).unwrap();

        assert_eq!(deserialized.nonce, nonce);
        assert_eq!(deserialized.signature.r(), signed.signature.r());
        assert_eq!(deserialized.signature.s(), signed.signature.s());
        assert_eq!(deserialized.signature.v(), signed.signature.v());

        match deserialized.action {
            ActionKind::UsdSend(action) => {
                assert_eq!(action.destination, dest);
                assert_eq!(action.amount, amount);
                assert_eq!(action.nonce, Some(nonce));
            }
            other => panic!("expected ActionKind::UsdSend, got: {other:?}"),
        }
    }

    #[test]
    fn test_signed_action_kind_deserializes_perp_deploy_by_payload_key() {
        let action = SetOpenInterestCaps {
            caps: vec![("BTC".to_string(), 1_000_000), ("ETH".to_string(), 500_000)],
            nonce: None,
        };
        let signing_chain = SigningChain::Testnet;

        let prepared = PreparedAction::new(action, &signing_chain, None, None).unwrap();
        let sig = Signature::new(U256::from(3), U256::from(4), true);
        let signed = prepared.with_signature(sig);

        let json = serde_json::to_string(&signed).unwrap();
        let deserialized = SignedActionKind::from_json(&json).unwrap();

        match deserialized.action {
            ActionKind::SetOpenInterestCaps(action) => {
                assert_eq!(
                    action.caps,
                    vec![("BTC".to_string(), 1_000_000), ("ETH".to_string(), 500_000)]
                );
            }
            other => panic!("expected ActionKind::SetOpenInterestCaps, got: {other:?}"),
        }
    }

    #[test]
    fn test_signed_action_kind_unknown_preserves_raw_action() {
        let envelope = json!({
            "action": {
                "type": "someFutureAction",
                "foo": 1,
                "bar": { "baz": true }
            },
            "nonce": 123,
            "signature": {
                "r": "0x01",
                "s": "0x02",
                "v": 27
            }
        });

        let json = serde_json::to_string(&envelope).unwrap();
        let deserialized = SignedActionKind::from_json(&json).unwrap();

        match deserialized.action {
            ActionKind::Unknown { action_type, raw } => {
                assert_eq!(action_type, "someFutureAction");
                assert_eq!(raw, envelope.get("action").cloned().unwrap());
            }
            other => panic!("expected ActionKind::Unknown, got: {other:?}"),
        }
    }
}
