// Runtime manager - Routes requests to appropriate RuntimeAdapter implementations

use super::{RuntimeAdapter, RuntimeId, RuntimeSessionParams, SessionHandle, UsageData, FileAttachment};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Manages multiple runtime instances and routes requests
pub struct RuntimeManager {
    runtimes: Arc<RwLock<HashMap<RuntimeId, Arc<dyn RuntimeAdapter>>>>,
    // Map GUI session_id -> runtime_id for routing
    session_runtime_map: Arc<RwLock<HashMap<String, RuntimeId>>>,
}

impl RuntimeManager {
    pub fn new() -> Self {
        Self {
            runtimes: Arc::new(RwLock::new(HashMap::new())),
            session_runtime_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a runtime implementation
    pub async fn register_runtime(
        &self,
        runtime_id: RuntimeId,
        runtime: Arc<dyn RuntimeAdapter>,
    ) {
        let mut runtimes = self.runtimes.write().await;
        runtimes.insert(runtime_id, runtime);
    }

    /// Get available runtime IDs
    pub async fn list_runtimes(&self) -> Vec<RuntimeId> {
        let runtimes = self.runtimes.read().await;
        runtimes.keys().cloned().collect()
    }

    /// Check if a runtime is available
    pub async fn detect_runtime(&self, runtime_id: &RuntimeId) -> bool {
        let runtimes = self.runtimes.read().await;
        if let Some(runtime) = runtimes.get(runtime_id) {
            runtime.detect().await
        } else {
            false
        }
    }

    /// Get runtime version
    pub async fn get_runtime_version(&self, runtime_id: &RuntimeId) -> Result<String, String> {
        let runtimes = self.runtimes.read().await;
        let runtime = runtimes
            .get(runtime_id)
            .ok_or_else(|| format!("Runtime {:?} not registered", runtime_id))?;
        runtime.get_version().await
    }

    /// Start a session with the specified runtime
    pub async fn start_session(
        &self,
        runtime_id: RuntimeId,
        params: RuntimeSessionParams,
    ) -> Result<SessionHandle, String> {
        let runtimes = self.runtimes.read().await;
        let runtime = runtimes
            .get(&runtime_id)
            .ok_or_else(|| format!("Runtime {:?} not registered", runtime_id))?;

        let session_id = params.session_id.clone();
        let handle = runtime.start_session(params).await?;

        // Register session -> runtime mapping
        let mut map = self.session_runtime_map.write().await;
        map.insert(session_id, runtime_id);

        Ok(handle)
    }

    /// Send message to a session
    pub async fn send_message(
        &self,
        session_id: &str,
        text: &str,
        attachments: Vec<FileAttachment>,
    ) -> Result<(), String> {
        let runtime_id = self.get_session_runtime(session_id).await?;
        let runtimes = self.runtimes.read().await;
        let runtime = runtimes
            .get(&runtime_id)
            .ok_or_else(|| format!("Runtime {:?} not found", runtime_id))?;

        runtime.send_message(session_id, text, attachments).await
    }

    /// Stop a session
    pub async fn stop_session(&self, session_id: &str) -> Result<(), String> {
        let runtime_id = self.get_session_runtime(session_id).await?;
        let runtimes = self.runtimes.read().await;
        let runtime = runtimes
            .get(&runtime_id)
            .ok_or_else(|| format!("Runtime {:?} not found", runtime_id))?;

        runtime.stop_session(session_id).await?;

        // Remove session mapping
        let mut map = self.session_runtime_map.write().await;
        map.remove(session_id);

        Ok(())
    }

    /// Get usage data for a session
    pub async fn get_usage(&self, session_id: &str) -> Result<Option<UsageData>, String> {
        let runtime_id = self.get_session_runtime(session_id).await?;
        let runtimes = self.runtimes.read().await;
        let runtime = runtimes
            .get(&runtime_id)
            .ok_or_else(|| format!("Runtime {:?} not found", runtime_id))?;

        Ok(runtime.get_usage(session_id).await)
    }

    /// Get which runtime is handling a session
    async fn get_session_runtime(&self, session_id: &str) -> Result<RuntimeId, String> {
        let map = self.session_runtime_map.read().await;
        map.get(session_id)
            .cloned()
            .ok_or_else(|| format!("Session {} not found", session_id))
    }

    /// Get runtime instance by ID
    pub async fn get_runtime(&self, runtime_id: &RuntimeId) -> Option<Arc<dyn RuntimeAdapter>> {
        let runtimes = self.runtimes.read().await;
        runtimes.get(runtime_id).cloned()
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new()
    }
}
