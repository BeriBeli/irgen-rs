use serde::{Deserialize, Serialize};

// Simple IP-XACT 2014 implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    vendor: Option<String>,
    library: Option<String>,
    name: Option<String>,
    version: Option<String>,
    memory_maps: Option<MemoryMaps>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMaps {
    memory_map: Option<Vec<MemoryMap>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMap {
    name: Option<String>,
    address_block: Option<Vec<Block>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    name: Option<String>,
    description: Option<String>,
    base_address: Option<String>,
    range: Option<String>,
    width: Option<String>,
    register: Option<Vec<Register>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    name: Option<String>,
    description: Option<String>,
    address_offset: Option<String>,
    size: Option<String>,
    field: Option<Vec<Field>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    name: Option<String>,
    description: Option<String>,
    bit_offset: Option<String>,
    bit_width: Option<String>,
    access: Option<String>,
    modified_write_value: Option<String>,
    read_action: Option<String>,
    resets: Option<Resets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resets {
    reset: Option<Vec<Reset>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reset {
    value: Option<String>,
    mask: Option<String>,
}