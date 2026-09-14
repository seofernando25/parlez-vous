use super::{manifest, paths::ManagedAiPaths};
use std::{process::{Child, Command, Stdio}, sync::Mutex};

#[derive(Default)]
pub struct ManagedAiRuntime { child: Mutex<Option<Child>> }

impl ManagedAiRuntime {
    pub fn start(&self, paths: &ManagedAiPaths) -> Result<(), String> {
        let mut child = self.child.lock().map_err(|_| "Managed AI runtime lock failed")?;
        if child.as_mut().and_then(|proc| proc.try_wait().ok()).flatten().is_none() && child.is_some() { return Ok(()); }
        if !paths.server.exists() || !paths.presets.exists() { return Err("Local AI is not installed yet".into()); }
        let proc = Command::new(&paths.server)
            .args(["--models-preset", &paths.presets.to_string_lossy(), "--models-max", "2", "--host", "127.0.0.1", "--port", &manifest::MANAGED_PORT.to_string()])
            .stdout(Stdio::null()).stderr(Stdio::null()).spawn().map_err(|e| e.to_string())?;
        *child = Some(proc);
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        let Ok(mut child) = self.child.lock() else { return false; };
        child.as_mut().map(|proc| proc.try_wait().ok().flatten().is_none()).unwrap_or(false)
    }

    pub fn stop(&self) {
        if let Ok(mut child) = self.child.lock() { if let Some(mut proc) = child.take() { let _ = proc.kill(); } }
    }
}

impl Drop for ManagedAiRuntime { fn drop(&mut self) { self.stop(); } }
