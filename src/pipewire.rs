use serde::{Deserialize, Serialize};
use serde_json::Value;

// pub const PIPE_WIRE_INTERFACE_CLIENT: &str = "PipeWire:Interface:Client";
// pub const PIPE_WIRE_INTERFACE_CORE: &str = "PipeWire:Interface:Core";
pub const PIPE_WIRE_INTERFACE_DEVICE: &str = "PipeWire:Interface:Device";
// pub const PIPE_WIRE_INTERFACE_FACTORY: &str = "PipeWire:Interface:Factory";
// pub const PIPE_WIRE_INTERFACE_LINK: &str = "PipeWire:Interface:Link";
// pub const PIPE_WIRE_INTERFACE_METADATA: &str = "PipeWire:Interface:Metadata";
// pub const PIPE_WIRE_INTERFACE_MODULE: &str = "PipeWire:Interface:Module";
pub const PIPE_WIRE_INTERFACE_NODE: &str = "PipeWire:Interface:Node";
// pub const PIPE_WIRE_INTERFACE_PORT: &str = "PipeWire:Interface:Port";
// pub const PIPE_WIRE_INTERFACE_PROFILER: &str = "PipeWire:Interface:Profiler";
// pub const PIPE_WIRE_INTERFACE_SECURITY_CONTEXT: &str = "PipeWire:Interface:SecurityContext";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PipeWireObject {
    pub id: u32,
    pub version: u32,
    #[serde(rename = "type")]
    pub dtype: String,
    pub info: Value,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PipeWireDump {
    pub objects: Vec<PipeWireObject>,
}

impl PipeWireDump {
    pub fn find_nodes(&self) -> Vec<PipeWireObject> {
        self.objects.iter().filter(|a| a.dtype == PIPE_WIRE_INTERFACE_NODE).cloned().collect()
    }

    pub fn find_devices(&self) -> Vec<PipeWireObject> {
        self.objects.iter().filter(|a| a.dtype == PIPE_WIRE_INTERFACE_DEVICE).cloned().collect()
    }
}
