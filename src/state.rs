use crate::pipewire::PipeWireDump;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default, Clone)]
pub struct SharedMutableState {
    pub pw_dump: Arc<RwLock<PipeWireDump>>,
}
