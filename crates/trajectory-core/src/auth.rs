use anyhow::{bail, Context, Result};
use data_encoding::BASE32_NOPAD;
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub const ACCESS_KEY_PREFIX: &str = "traj1";
pub const ACCESS_KEY_SECRET_LEN: usize = 32;
pub const AUTH_TAG_LEN: usize = 12;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientAccessKey {
    pub client_id: u32,
    pub secret: [u8; ACCESS_KEY_SECRET_LEN],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredClientKey {
    pub id: u32,
    pub label: String,
    pub secret_base32: String,
    pub created_unix: u64,
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StoredClientRegistry {
    pub keys: Vec<StoredClientKey>,
}

impl ClientAccessKey {
    pub fn generate() -> Self {
        let mut rng = thread_rng();
        let mut secret = [0u8; ACCESS_KEY_SECRET_LEN];
        rng.fill_bytes(&mut secret);
        Self {
            client_id: rng.next_u32(),
            secret,
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        let trimmed = value.trim();
        let mut parts = trimmed.split('_');
        let prefix = parts.next().context("missing key prefix")?;
        if !prefix.eq_ignore_ascii_case(ACCESS_KEY_PREFIX) {
            bail!("access key must start with {ACCESS_KEY_PREFIX}_");
        }
        let client_id_hex = parts.next().context("missing client id")?;
        let secret_base32 = parts.next().context("missing secret payload")?;
        if parts.next().is_some() {
            bail!("access key has too many sections");
        }

        let client_id = u32::from_str_radix(client_id_hex, 16).context("invalid access key id")?;
        let secret_bytes = BASE32_NOPAD
            .decode(secret_base32.to_ascii_uppercase().as_bytes())
            .map_err(|_| anyhow::anyhow!("invalid access key secret"))?;
        let secret: [u8; ACCESS_KEY_SECRET_LEN] = secret_bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("access key secret must be 32 bytes"))?;
        Ok(Self { client_id, secret })
    }

    pub fn to_display_string(&self) -> String {
        format!(
            "{ACCESS_KEY_PREFIX}_{:08x}_{}",
            self.client_id,
            BASE32_NOPAD.encode(&self.secret)
        )
    }
}

impl StoredClientKey {
    pub fn generate(label: impl Into<String>) -> Self {
        let key = ClientAccessKey::generate();
        Self::from_access_key(label, &key)
    }

    pub fn from_access_key(label: impl Into<String>, key: &ClientAccessKey) -> Self {
        Self {
            id: key.client_id,
            label: label.into(),
            secret_base32: BASE32_NOPAD.encode(&key.secret),
            created_unix: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            enabled: true,
        }
    }

    pub fn access_key(&self) -> Result<ClientAccessKey> {
        let secret_bytes = BASE32_NOPAD
            .decode(self.secret_base32.to_ascii_uppercase().as_bytes())
            .map_err(|_| anyhow::anyhow!("invalid stored client key secret"))?;
        let secret: [u8; ACCESS_KEY_SECRET_LEN] = secret_bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("stored client key secret must be 32 bytes"))?;
        Ok(ClientAccessKey {
            client_id: self.id,
            secret,
        })
    }

    pub fn access_key_string(&self) -> Result<String> {
        Ok(self.access_key()?.to_display_string())
    }
}

impl StoredClientRegistry {
    pub fn active_keys(&self) -> Result<HashMap<u32, ClientAccessKey>> {
        let mut out = HashMap::with_capacity(self.keys.len());
        for entry in self.keys.iter().filter(|entry| entry.enabled) {
            let key = entry.access_key()?;
            out.insert(key.client_id, key);
        }
        Ok(out)
    }

    pub fn create_key(&mut self, label: impl Into<String>) -> Result<StoredClientKey> {
        let label = label.into();
        loop {
            let record = StoredClientKey::generate(label.clone());
            if self.keys.iter().any(|existing| existing.id == record.id) {
                // Extremely unlikely, but keep the persisted registry unique.
                continue;
            }
            self.keys.push(record.clone());
            return Ok(record);
        }
    }

    pub fn remove_key_at(&mut self, index: usize) -> Option<StoredClientKey> {
        (index < self.keys.len()).then(|| self.keys.remove(index))
    }

    pub fn remove_key_by_id(&mut self, id: u32) -> Option<StoredClientKey> {
        let index = self.keys.iter().position(|entry| entry.id == id)?;
        Some(self.keys.remove(index))
    }
}

pub fn compute_auth_tag(
    secret: &[u8; ACCESS_KEY_SECRET_LEN],
    message: &[u8],
) -> [u8; AUTH_TAG_LEN] {
    let hash = blake3::keyed_hash(secret, message);
    let mut tag = [0u8; AUTH_TAG_LEN];
    tag.copy_from_slice(&hash.as_bytes()[..AUTH_TAG_LEN]);
    tag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_key_roundtrip() {
        let key = ClientAccessKey::generate();
        let encoded = key.to_display_string();
        let decoded = ClientAccessKey::parse(&encoded).unwrap();
        assert_eq!(decoded, key);
    }

    #[test]
    fn registry_returns_only_enabled_keys() {
        let mut registry = StoredClientRegistry::default();
        let first = registry.create_key("Alice").unwrap();
        let mut second = registry.create_key("Bob").unwrap();
        second.enabled = false;
        registry.keys[1] = second;
        let active = registry.active_keys().unwrap();
        assert_eq!(active.len(), 1);
        assert!(active.contains_key(&first.id));
    }

    #[test]
    fn registry_can_remove_keys() {
        let mut registry = StoredClientRegistry::default();
        let first = registry.create_key("Alice").unwrap();
        let second = registry.create_key("Bob").unwrap();
        let removed = registry.remove_key_by_id(first.id).unwrap();
        assert_eq!(removed.id, first.id);
        assert_eq!(registry.keys.len(), 1);
        assert_eq!(registry.keys[0].id, second.id);
    }
}
