// Provider snapshot mechanism
//
// Captures immutable provider configuration at session start time.
// API keys are encrypted using platform-specific secure storage.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provider snapshot saved at session start (immutable during session lifetime)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSnapshot {
    pub provider_id: String,
    pub provider_name: String,
    pub api_format: String,  // "anthropic" or "openai"
    pub base_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_mappings: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    pub snapshot_at: i64,
    // API key is stored separately in secure storage
    pub api_key_ref: String,  // Reference to keychain entry
}

/// Provider configuration from frontend (before snapshot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub api_format: String,
    pub base_url: String,
    pub api_key_ref: String,  // May be keychain ref or plaintext (to be migrated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_mappings: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    pub enabled: bool,
}

impl ProviderSnapshot {
    /// Create a snapshot from provider config
    pub fn from_config(config: ProviderConfig) -> Self {
        Self {
            provider_id: config.id,
            provider_name: config.name,
            api_format: config.api_format,
            base_url: config.base_url,
            model_mappings: config.model_mappings,
            extra_env: config.extra_env,
            proxy_url: config.proxy_url,
            snapshot_at: chrono::Utc::now().timestamp(),
            api_key_ref: config.api_key_ref,
        }
    }

    /// Get environment variables to inject into CLI process
    pub fn to_env_vars(&self, api_key: &str) -> HashMap<String, String> {
        let mut env = HashMap::new();

        // Add API key based on format
        match self.api_format.as_str() {
            "anthropic" => {
                env.insert("ANTHROPIC_API_KEY".to_string(), api_key.to_string());
                if !self.base_url.is_empty() && self.base_url != "https://api.anthropic.com" {
                    env.insert("ANTHROPIC_BASE_URL".to_string(), self.base_url.clone());
                }
            }
            "openai" => {
                env.insert("OPENAI_API_KEY".to_string(), api_key.to_string());
                if !self.base_url.is_empty() && self.base_url != "https://api.openai.com" {
                    env.insert("OPENAI_BASE_URL".to_string(), self.base_url.clone());
                }
            }
            _ => {}
        }

        // Add proxy if configured
        if let Some(proxy) = &self.proxy_url {
            if !proxy.is_empty() {
                env.insert("https_proxy".to_string(), proxy.clone());
                env.insert("HTTPS_PROXY".to_string(), proxy.clone());
            }
        }

        // Add extra env vars
        if let Some(extra) = &self.extra_env {
            for (k, v) in extra {
                env.insert(k.clone(), v.clone());
            }
        }

        env
    }
}

// Keychain integration module (platform-specific)
pub mod keychain {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    // For Phase 1, we use an in-memory cache as a placeholder
    // In Phase 1.4, we'll integrate with actual platform keychains:
    // - Windows: Credential Manager via `winapi` or `keyring` crate
    // - macOS: Keychain via `security-framework` crate
    // - Linux: Secret Service API via `secret-service` crate

    fn key_cache() -> &'static Mutex<HashMap<String, String>> {
        static KEY_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
        KEY_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// Store API key securely (placeholder implementation)
    pub fn store_key(key_ref: &str, api_key: &str) -> Result<(), String> {
        let mut cache = key_cache().lock().unwrap();
        cache.insert(key_ref.to_string(), api_key.to_string());
        Ok(())
    }

    /// Retrieve API key from secure storage (placeholder implementation)
    pub fn get_key(key_ref: &str) -> Result<String, String> {
        let cache = key_cache().lock().unwrap();
        cache
            .get(key_ref)
            .cloned()
            .ok_or_else(|| format!("Key not found: {}", key_ref))
    }

    /// Delete API key from secure storage
    pub fn delete_key(key_ref: &str) -> Result<(), String> {
        let mut cache = key_cache().lock().unwrap();
        cache.remove(key_ref);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_env_vars_anthropic() {
        let snapshot = ProviderSnapshot {
            provider_id: "test".to_string(),
            provider_name: "Test Provider".to_string(),
            api_format: "anthropic".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
            model_mappings: None,
            extra_env: None,
            proxy_url: None,
            snapshot_at: 0,
            api_key_ref: "test_key".to_string(),
        };

        let env = snapshot.to_env_vars("sk-test-key");
        assert_eq!(env.get("ANTHROPIC_API_KEY"), Some(&"sk-test-key".to_string()));
    }

    #[test]
    fn test_snapshot_env_vars_openai_with_custom_base() {
        let snapshot = ProviderSnapshot {
            provider_id: "deepseek".to_string(),
            provider_name: "DeepSeek".to_string(),
            api_format: "openai".to_string(),
            base_url: "https://api.deepseek.com".to_string(),
            model_mappings: None,
            extra_env: Some([("CUSTOM_VAR".to_string(), "value".to_string())].into()),
            proxy_url: Some("http://127.0.0.1:7890".to_string()),
            snapshot_at: 0,
            api_key_ref: "deepseek_key".to_string(),
        };

        let env = snapshot.to_env_vars("sk-deepseek");
        assert_eq!(env.get("OPENAI_API_KEY"), Some(&"sk-deepseek".to_string()));
        assert_eq!(env.get("OPENAI_BASE_URL"), Some(&"https://api.deepseek.com".to_string()));
        assert_eq!(env.get("https_proxy"), Some(&"http://127.0.0.1:7890".to_string()));
        assert_eq!(env.get("CUSTOM_VAR"), Some(&"value".to_string()));
    }
}
