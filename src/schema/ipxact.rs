use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::attr;

// Simple IP-XACT 2014 implementation

const IEEE1685_2014_NS: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2014";
const XSI_NS: &str = "http://www.w3.org/2001/XMLSchema-instance";
const SCHEMA_LOCATION: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2014 http://www.accellera.org/XMLSchema/IPXACT/1685-2014/index.xsd";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ipxact:component")]
pub struct Component {
    #[serde(rename = "@xmlns:ipxact")]
    xmlns_ipxact: String,
    #[serde(rename = "@xmlns:xsi")]
    xmlns_xsi: String,
    #[serde(rename = "@xsi:schemaLocation")]
    schema_location: String,
    #[serde(rename = "ipxact:vendor")]
    vendor: String,
    #[serde(rename = "ipxact:library")]
    library: String,
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:version")]
    version: String,
    #[serde(rename = "ipxact:memoryMaps")]
    memory_maps: MemoryMaps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMaps {
    #[serde(rename = "ipxact:memoryMap")]
    memory_map: Vec<MemoryMap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMap {
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:addressBlock")]
    address_block: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "ipxact:baseAddress")]
    base_address: String,
    #[serde(rename = "ipxact:range")]
    range: String,
    #[serde(rename = "ipxact:width")]
    width: String,
    #[serde(rename = "ipxact:register")]
    register: Vec<Register>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "ipxact:addressOffset")]
    address_offset: String,
    #[serde(rename = "ipxact:size")]
    size: String,
    #[serde(rename = "ipxact:field")]
    field: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "ipxact:bitOffset")]
    bit_offset: String,
    #[serde(rename = "ipxact:bitWidth")]
    bit_width: String,
    #[serde(rename = "ipxact:access")]
    access: String,
    #[serde(
        rename = "ipxact:modifiedWriteValue",
        skip_serializing_if = "Option::is_none"
    )]
    modified_write_value: Option<String>,
    #[serde(rename = "ipxact:readAction", skip_serializing_if = "Option::is_none")]
    read_action: Option<String>,
    #[serde(rename = "ipxact:resets")]
    resets: Resets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resets {
    #[serde(rename = "ipxact:reset")]
    reset: Vec<Reset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reset {
    #[serde(rename = "ipxact:value")]
    value: String,
}

impl Component {
    pub fn new() -> Self {
        Self {
            xmlns_ipxact: IEEE1685_2014_NS.to_owned(),
            xmlns_xsi: XSI_NS.to_owned(),
            schema_location: SCHEMA_LOCATION.to_owned(),
            vendor: String::new(),
            library: String::new(),
            name: String::new(),
            version: String::new(),
            memory_maps: MemoryMaps::new(),
        }
    }

    pub fn set_vendor(mut self, vendor: String) -> anyhow::Result<Self, Error> {
        self.vendor = vendor;
        Ok(self)
    }

    pub fn set_library(mut self, library: String) -> anyhow::Result<Self, Error> {
        self.library = library;
        Ok(self)
    }

    pub fn set_name(mut self, name: String) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }

    pub fn set_version(mut self, version: String) -> anyhow::Result<Self, Error> {
        self.version = version;
        Ok(self)
    }

    pub fn set_memory_maps(mut self, memory_maps: MemoryMaps) -> anyhow::Result<Self, Error> {
        self.memory_maps = memory_maps;
        Ok(self)
    }
}

impl MemoryMaps {
    pub fn new() -> Self {
        Self {
            memory_map: Vec::new(),
        }
    }

    pub fn set_memory_map(mut self, memory_map: Vec<MemoryMap>) -> anyhow::Result<Self, Error> {
        self.memory_map = memory_map;
        Ok(self)
    }
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            address_block: Vec::new(),
        }
    }

    pub fn set_name(mut self, name: String) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }

    pub fn set_address_block(mut self, address_block: Vec<Block>) -> anyhow::Result<Self, Error> {
        self.address_block = address_block;
        Ok(self)
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: None,
            base_address: String::new(),
            range: String::new(),
            width: String::new(),
            register: Vec::new(),
        }
    }

    pub fn set_name(mut self, name: String) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }

    pub fn set_description(mut self, description: String) -> anyhow::Result<Self, Error> {
        self.description = description.into();
        Ok(self)
    }

    pub fn set_base_address(mut self, base_address: String) -> anyhow::Result<Self, Error> {
        self.base_address = base_address;
        Ok(self)
    }

    pub fn set_range(mut self, range: String) -> anyhow::Result<Self, Error> {
        self.range = range;
        Ok(self)
    }

    pub fn set_width(mut self, width: String) -> anyhow::Result<Self, Error> {
        self.width = width;
        Ok(self)
    }

    pub fn set_register(mut self, register: Vec<Register>) -> anyhow::Result<Self, Error> {
        self.register = register;
        Ok(self)
    }
}

impl Register {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: None,
            address_offset: String::new(),
            size: String::new(),
            field: Vec::new(),
        }
    }

    pub fn set_name(mut self, name: String) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }
    pub fn set_description(mut self, description: String) -> anyhow::Result<Self, Error> {
        self.description = description.into();
        Ok(self)
    }
    pub fn set_address_offset(mut self, address_offset: String) -> anyhow::Result<Self, Error> {
        self.address_offset = address_offset;
        Ok(self)
    }
    pub fn set_size(mut self, size: String) -> anyhow::Result<Self, Error> {
        self.size = size;
        Ok(self)
    }
    pub fn set_field(mut self, field: Vec<Field>) -> anyhow::Result<Self, Error> {
        self.field = field;
        Ok(self)
    }
}

impl Field {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: None,
            bit_offset: String::new(),
            bit_width: String::new(),
            access: String::new(),
            modified_write_value: None,
            read_action: None,
            resets: Resets::new(),
        }
    }

    pub fn set_name(mut self, name: String) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }

    pub fn set_description(mut self, description: String) -> anyhow::Result<Self, Error> {
        self.description = description.into();
        Ok(self)
    }

    pub fn set_bit_offset(mut self, bit_offset: String) -> anyhow::Result<Self, Error> {
        self.bit_offset = bit_offset;
        Ok(self)
    }

    pub fn set_bit_width(mut self, bit_width: String) -> anyhow::Result<Self, Error> {
        self.bit_width = bit_width;
        Ok(self)
    }

    pub fn set_access(mut self, attr: String) -> anyhow::Result<Self, Error> {
        self.access = attr::extract_access_value(&attr)?;
        Ok(self)
    }

    pub fn set_modified_write_value(mut self, attr: String) -> anyhow::Result<Self, Error> {
        self.modified_write_value = attr::extract_modified_write_value(&attr)?;
        Ok(self)
    }

    pub fn set_read_action(mut self, attr: String) -> anyhow::Result<Self, Error> {
        self.read_action = attr::extract_read_action_value(&attr)?;
        Ok(self)
    }

    pub fn set_resets(mut self, resets: Resets) -> anyhow::Result<Self, Error> {
        self.resets = resets;
        Ok(self)
    }
}

impl Resets {
    pub fn new() -> Self {
        Self { reset: Vec::new() }
    }

    pub fn set_reset(mut self, reset: Vec<Reset>) -> anyhow::Result<Self, Error> {
        self.reset = reset;
        Ok(self)
    }
}

impl Reset {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn set_value(mut self, value: String) -> anyhow::Result<Self, Error> {
        self.value = value;
        Ok(self)
    }
}
