// Claude Code CLI Runtime implementation
//
// Phase 1 architecture compatibility mode:
// This is a thin adapter for detection and version checking.
// The actual session management is still done by the existing Tauri commands in lib.rs.
// ClaudeRuntime provides the RuntimeAdapter interface without duplicating ProcessManager.

use super::{
    FileAttachment, RuntimeAdapter, RuntimeId, RuntimeSessionParams,
    SessionHandle, UsageData,
};
use async_trait::async_trait;

/// Claude Code CLI Runtime (Phase 1 adapter)
pub struct ClaudeRuntime {
    // No fields needed for Phase 1 - the actual session management
    // is still done by the existing Tauri commands in lib.rs
    // ClaudeRuntime is a thin adapter for detection and version
}

impl ClaudeRuntime {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ClaudeRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RuntimeAdapter for ClaudeRuntime {
    async fn detect(&self) -> bool {
        // Use existing CLI resolver
        crate::commands::cli_resolver::find_binary().is_some()
    }

    async fn get_version(&self) -> Result<String, String> {
        // Try to run `claude --version`
        let cli_path = crate::commands::cli_resolver::find_binary()
            .ok_or_else(|| "Claude CLI not found".to_string())?;

        match std::process::Command::new(&cli_path)
            .arg("--version")
            .output()
        {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                Ok(version.trim().to_string())
            }
            _ => Err("Failed to get Claude CLI version".to_string()),
        }
    }

    async fn start_session(
        &self,
        params: RuntimeSessionParams,
    ) -> Result<SessionHandle, String> {
        // For Phase 1, we return a handle but the actual process spawning
        // is still done by the existing tauri command handler in lib.rs.
        // This maintains backward compatibility while preparing for Phase 1.11 refactor.

        Ok(SessionHandle {
            session_id: params.session_id.clone(),
            pid: None, // Will be set by existing process manager
            cli_session_id: None,
            started_at: chrono::Utc::now().timestamp(),
        })
    }

    async fn send_message(
        &self,
        _session_id: &str,
        _text: &str,
        _attachments: Vec<FileAttachment>,
    ) -> Result<(), String> {
        // For Phase 1, the actual message sending is handled by
        // the existing stdin_manager in lib.rs commands.
        // This just returns Ok to satisfy the interface.
        Ok(())
    }

    async fn stop_session(&self, _session_id: &str) -> Result<(), String> {
        // For Phase 1, the actual process stopping is handled by
        // the existing process_manager in lib.rs commands.
        // This just returns Ok to satisfy the interface.
        Ok(())
    }

    async fn get_usage(&self, _session_id: &str) -> Option<UsageData> {
        // Claude Code CLI provides token usage via stream events
        // For Phase 1, we return None here and let the existing
        // usage tracking in the stream processor handle it.
        // In Phase 1.7 (usage enhancement), we'll cache usage data here.
        None
    }

    fn runtime_id(&self) -> RuntimeId {
        RuntimeId::Claude
    }

    fn runtime_name(&self) -> &'static str {
        "Claude Code CLI"
    }
}
