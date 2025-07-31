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
    #[serde(rename = "ipxact:vendor", skip_serializing_if = "Option::is_none")]
    vendor: Option<String>,
    #[serde(rename = "ipxact:library", skip_serializing_if = "Option::is_none")]
    library: Option<String>,
    #[serde(rename = "ipxact:name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "ipxact:version", skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(rename = "ipxact:memoryMaps", skip_serializing_if = "Option::is_none")]
    memory_maps: Option<MemoryMaps>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMaps {
    #[serde(rename = "ipxact:memoryMap", skip_serializing_if = "Option::is_none")]
    memory_map: Option<Vec<MemoryMap>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMap {
    #[serde(rename = "ipxact:name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(
        rename = "ipxact:addressBlock",
        skip_serializing_if = "Option::is_none"
    )]
    address_block: Option<Vec<Block>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "ipxact:name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "ipxact:description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "ipxact:baseAddress", skip_serializing_if = "Option::is_none")]
    base_address: Option<String>,
    #[serde(rename = "ipxact:range", skip_serializing_if = "Option::is_none")]
    range: Option<String>,
    #[serde(rename = "ipxact:width", skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    #[serde(rename = "ipxact:register", skip_serializing_if = "Option::is_none")]
    register: Option<Vec<Register>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    #[serde(rename = "ipxact:name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "ipxact:description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(
        rename = "ipxact:addressOffset",
        skip_serializing_if = "Option::is_none"
    )]
    address_offset: Option<String>,
    #[serde(rename = "ipxact:size", skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    #[serde(rename = "ipxact:field", skip_serializing_if = "Option::is_none")]
    field: Option<Vec<Field>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename = "ipxact:name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "ipxact:description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "ipxact:bitOffset", skip_serializing_if = "Option::is_none")]
    bit_offset: Option<String>,
    #[serde(rename = "ipxact:bitWidth", skip_serializing_if = "Option::is_none")]
    bit_width: Option<String>,
    #[serde(rename = "ipxact:access", skip_serializing_if = "Option::is_none")]
    access: Option<String>,
    #[serde(
        rename = "ipxact:modifiedWriteValue",
        skip_serializing_if = "Option::is_none"
    )]
    modified_write_value: Option<String>,
    #[serde(rename = "ipxact:readAction", skip_serializing_if = "Option::is_none")]
    read_action: Option<String>,
    #[serde(rename = "ipxact:resets", skip_serializing_if = "Option::is_none")]
    resets: Option<Resets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resets {
    #[serde(rename = "ipxact:reset", skip_serializing_if = "Option::is_none")]
    reset: Option<Vec<Reset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reset {
    #[serde(rename = "ipxact:value", skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(rename = "ipxact:mask", skip_serializing_if = "Option::is_none")]
    mask: Option<String>,
}

impl Component {
    pub fn new() -> Self {
        Self {
            xmlns_ipxact: IEEE1685_2014_NS.to_owned(),
            xmlns_xsi: XSI_NS.to_owned(),
            schema_location: SCHEMA_LOCATION.to_owned(),
            vendor: None,
            library: None,
            name: None,
            version: None,
            memory_maps: None,
        }
    }

    pub fn set_vendor(mut self, vendor: Option<String>) -> anyhow::Result<Self, Error> {
        self.vendor = vendor;
        Ok(self)
    }

    pub fn set_library(mut self, library: Option<String>) -> anyhow::Result<Self, Error> {
        self.library = library;
        Ok(self)
    }

    pub fn set_name(mut self, name: Option<String>) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }

    pub fn set_version(mut self, version: Option<String>) -> anyhow::Result<Self, Error> {
        self.version = version;
        Ok(self)
    }

    pub fn set_memory_maps(
        mut self,
        memory_maps: Option<MemoryMaps>,
    ) -> anyhow::Result<Self, Error> {
        self.memory_maps = memory_maps;
        Ok(self)
    }
}

impl MemoryMaps {
    pub fn new() -> Self {
        Self { memory_map: None }
    }

    pub fn set_memory_map(
        mut self,
        memory_map: Option<Vec<MemoryMap>>,
    ) -> anyhow::Result<Self, Error> {
        self.memory_map = memory_map;
        Ok(self)
    }
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            name: None,
            address_block: None,
        }
    }

    pub fn set_name(mut self, name: Option<String>) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }

    pub fn set_address_block(
        mut self,
        address_block: Option<Vec<Block>>,
    ) -> anyhow::Result<Self, Error> {
        self.address_block = address_block;
        Ok(self)
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            base_address: None,
            range: None,
            width: None,
            register: None,
        }
    }

    pub fn set_name(mut self, name: Option<String>) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }

    pub fn set_description(mut self, description: Option<String>) -> anyhow::Result<Self, Error> {
        self.description = description;
        Ok(self)
    }

    pub fn set_base_address(mut self, base_address: Option<String>) -> anyhow::Result<Self, Error> {
        self.base_address = base_address;
        Ok(self)
    }

    pub fn set_range(mut self, range: Option<String>) -> anyhow::Result<Self, Error> {
        self.range = range;
        Ok(self)
    }

    pub fn set_width(mut self, width: Option<String>) -> anyhow::Result<Self, Error> {
        self.width = width;
        Ok(self)
    }

    pub fn set_register(mut self, register: Option<Vec<Register>>) -> anyhow::Result<Self, Error> {
        self.register = register;
        Ok(self)
    }
}

impl Register {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            address_offset: None,
            size: None,
            field: None,
        }
    }

    pub fn set_name(mut self, name: Option<String>) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }
    pub fn set_description(mut self, description: Option<String>) -> anyhow::Result<Self, Error> {
        self.description = description;
        Ok(self)
    }
    pub fn set_address_offset(
        mut self,
        address_offset: Option<String>,
    ) -> anyhow::Result<Self, Error> {
        self.address_offset = address_offset;
        Ok(self)
    }
    pub fn set_size(mut self, size: Option<String>) -> anyhow::Result<Self, Error> {
        self.size = size;
        Ok(self)
    }
    pub fn set_field(mut self, field: Option<Vec<Field>>) -> anyhow::Result<Self, Error> {
        self.field = field;
        Ok(self)
    }
}

impl Field {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            bit_offset: None,
            bit_width: None,
            access: None,
            modified_write_value: None,
            read_action: None,
            resets: None,
        }
    }

    pub fn set_name(mut self, name: Option<String>) -> anyhow::Result<Self, Error> {
        self.name = name;
        Ok(self)
    }

    pub fn set_description(mut self, description: Option<String>) -> anyhow::Result<Self, Error> {
        self.description = description;
        Ok(self)
    }

    pub fn set_bit_offset(mut self, bit_offset: Option<String>) -> anyhow::Result<Self, Error> {
        self.bit_offset = bit_offset;
        Ok(self)
    }

    pub fn set_bit_width(mut self, bit_width: Option<String>) -> anyhow::Result<Self, Error> {
        self.bit_width = bit_width;
        Ok(self)
    }

    pub fn set_access(mut self, attr: Option<String>) -> anyhow::Result<Self, Error> {
        self.access = attr::extract_access_value(&attr.unwrap())?;
        Ok(self)
    }

    pub fn set_modified_write_value(mut self, attr: Option<String>) -> anyhow::Result<Self, Error> {
        self.modified_write_value = attr::extract_modified_write_value(&attr.unwrap())?;
        Ok(self)
    }

    pub fn set_read_action(mut self, attr: Option<String>) -> anyhow::Result<Self, Error> {
        self.read_action = attr::extract_read_action_value(&attr.unwrap())?;
        Ok(self)
    }

    pub fn set_resets(mut self, resets: Option<Resets>) -> anyhow::Result<Self, Error> {
        self.resets = resets;
        Ok(self)
    }
}

impl Resets {
    pub fn new() -> Self {
        Self { reset: None }
    }

    pub fn set_reset(mut self, reset: Option<Vec<Reset>>) -> anyhow::Result<Self, Error> {
        self.reset = reset;
        Ok(self)
    }
}

impl Reset {
    pub fn new() -> Self {
        Self {
            value: None,
            mask: None,
        }
    }

    pub fn set_value(mut self, value: Option<String>) -> anyhow::Result<Self, Error> {
        self.value = value;
        Ok(self)
    }

    pub fn set_mask(mut self, mask: Option<String>) -> anyhow::Result<Self, Error> {
        self.mask = mask;
        Ok(self)
    }
}
