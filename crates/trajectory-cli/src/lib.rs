#![recursion_limit = "256"]

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use trajectory_core::auth::StoredClientRegistry;

pub mod runtime;

pub fn load_client_registry(path: &Path) -> Result<StoredClientRegistry> {
    if !path.exists() {
        return Ok(StoredClientRegistry::default());
    }
    let bytes = fs::read(path).with_context(|| format!("read registry {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse registry {}", path.display()))
}

pub fn save_client_registry(path: &Path, registry: &StoredClientRegistry) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create registry directory {}", parent.display()))?;
    }
    let json = serde_json::to_vec_pretty(registry)?;
    fs::write(path, json).with_context(|| format!("write registry {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use trajectory_core::auth::StoredClientRegistry;

    #[test]
    fn registry_round_trip_persists_enabled_and_deleted_keys() {
        let temp_dir = std::env::temp_dir().join(format!(
            "trajectory-client-registry-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).unwrap();
        let path = temp_dir.join("registry.json");

        let mut registry = StoredClientRegistry::default();
        let first = registry.create_key("Alpha").unwrap();
        let second = registry.create_key("Beta").unwrap();
        registry.keys[1].enabled = false;
        let removed = registry.remove_key_by_id(second.id).unwrap();
        assert_eq!(removed.id, second.id);
        save_client_registry(&path, &registry).unwrap();

        let loaded = load_client_registry(&path).unwrap();
        assert_eq!(loaded.keys.len(), 1);
        assert_eq!(loaded.keys[0].id, first.id);
        assert!(loaded.keys[0].enabled);

        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
