// Runtime abstraction layer for multi-CLI support
//
// This module defines the RuntimeAdapter trait that abstracts different
// CLI tools (Claude Code, Codex, Gemini, etc.) behind a unified interface.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod claude;
// pub mod codex;  // Phase 2
// pub mod gemini;  // Phase 4
// pub mod opencode;  // Phase 4
// pub mod custom;  // Phase 4

pub mod manager;

// Re-export ProviderSnapshot from the canonical location
pub use crate::provider::snapshot::ProviderSnapshot;

/// Unique identifier for a runtime implementation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuntimeId {
    #[serde(rename = "claude")]
    Claude,
    #[serde(rename = "codex")]
    Codex,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "opencode")]
    OpenCode,
    #[serde(rename = "custom")]
    Custom(String), // Custom CLI with user-defined name
}

impl RuntimeId {
    pub fn as_str(&self) -> &str {
        match self {
            RuntimeId::Claude => "claude",
            RuntimeId::Codex => "codex",
            RuntimeId::Gemini => "gemini",
            RuntimeId::OpenCode => "opencode",
            RuntimeId::Custom(name) => name.as_str(),
        }
    }
}

/// File attachment for sending with messages
#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAttachment {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,  // Small text files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,   // Images
}

/// Parameters for starting a new runtime session
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct RuntimeSessionParams {
    pub session_id: String,           // GUI-generated session ID
    pub prompt: String,               // Initial user message
    pub cwd: String,                  // Working directory
    pub provider_snapshot: ProviderSnapshot,
    pub model: Option<String>,
    pub permission_mode: String,      // "ask" | "plan" | "auto" | "bypass"
    pub context_window: Option<u32>,
    pub attachments: Vec<FileAttachment>,
    pub extra_args: Vec<String>,      // Additional CLI arguments
}

/// Handle returned after starting a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionHandle {
    pub session_id: String,            // GUI session ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,              // Process ID (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_session_id: Option<String>, // CLI-internal session ID (e.g., Codex thread ID)
    pub started_at: i64,               // Unix timestamp
}

/// Token usage data returned by the runtime
#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_tokens: Option<u64>,
    pub message_count: u32,
    pub stdout_bytes: u64,
    pub duration_ms: u64,
    pub is_estimated: bool,  // true if CLI doesn't provide token stats
}

/// Abstraction for different CLI runtimes (Claude, Codex, Gemini, etc.)
#[async_trait]
pub trait RuntimeAdapter: Send + Sync {
    /// Check if this CLI is installed and available
    async fn detect(&self) -> bool;

    /// Get the CLI version string
    async fn get_version(&self) -> Result<String, String>;

    /// Start a new session and return a handle
    async fn start_session(
        &self,
        params: RuntimeSessionParams,
    ) -> Result<SessionHandle, String>;

    /// Send a message to an active session
    async fn send_message(
        &self,
        session_id: &str,
        text: &str,
        attachments: Vec<FileAttachment>,
    ) -> Result<(), String>;

    /// Stop a running session
    async fn stop_session(&self, session_id: &str) -> Result<(), String>;

    /// Resume an existing session (if supported by the CLI)
    async fn resume_session(&self, _session_id: &str) -> Result<SessionHandle, String> {
        Err(format!(
            "Resume not supported by {} runtime",
            self.runtime_name()
        ))
    }

    /// Get usage statistics for a session (returns None if CLI doesn't provide them)
    async fn get_usage(&self, session_id: &str) -> Option<UsageData>;

    /// Get session logs
    async fn get_logs(&self, _session_id: &str) -> Vec<String> {
        vec![]
    }

    /// Runtime unique identifier
    fn runtime_id(&self) -> RuntimeId;

    /// Human-readable runtime name
    fn runtime_name(&self) -> &'static str;
}
