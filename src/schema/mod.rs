pub mod attr;
pub mod base;
pub mod ipxact;
pub mod regvue;

use std::collections::HashMap;

use regex::Regex;

use crate::error::Error;
use crate::schema::attr::{
    extract_access_value, extract_modified_write_value, extract_read_action_value,
};

impl ipxact::Component {
    pub fn from(base: &base::Component) -> anyhow::Result<Self, Error> {
        // use regex to remove rsvd\d* or reserved\d* in field
        let re = Regex::new(r"^(rsvd|reserved)\d*$")?;

        let memory_maps = ipxact::MemoryMapsBuilder::default()
            .memory_map(vec![
                ipxact::MemoryMapBuilder::default()
                    .name(base.name())
                    .address_block(base.blks().iter().map(|blk| -> anyhow::Result<ipxact::Block, Error> {
                        Ok(ipxact::BlockBuilder::default()
                            .name(blk.name())
                            .base_address(blk.offset())
                            .range(blk.range())
                            .width(blk.size())
                            // use iterator to get the array of registers
                            .register(blk.regs().iter().map(|reg| -> anyhow::Result<ipxact::Register, Error> {
                                    Ok(ipxact::RegisterBuilder::default()
                                        .name(reg.name())
                                        .address_offset(reg.offset())
                                        .size(reg.size())
                                        // use iterator to get the array of fields
                                        .field(reg.fields().iter().filter(|field| {
                                            !re.is_match(field.name())
                                        }).map(|field| -> anyhow::Result<ipxact::Field, Error> {
                                                Ok(ipxact::FieldBuilder::default()
                                                    .name(field.name())
                                                    .bit_offset(field.offset())
                                                    .bit_width(field.width())
                                                    // convert attribute to access
                                                    .access(extract_access_value(field.attr())?)
                                                    // convert attribute to modified_write_value
                                                    .modified_write_value(extract_modified_write_value(field.attr())?)
                                                    // convert attribute to read_action
                                                    .read_action(extract_read_action_value(field.attr())?)
                                                    .resets(
                                                        ipxact::ResetsBuilder::default()
                                                            .reset(vec![
                                                                ipxact::ResetBuilder::default()
                                                                    .value(field.reset())
                                                                    .build()?
                                                            ])
                                                            .build()?
                                                    )
                                                    .description(field.desc().to_owned())
                                                    .build()?
                                                )
                                            }).collect::<Result<Vec<_>, _>>()?
                                        )
                                        .build()?
                                    )
                                }).collect::<Result<Vec<_>, _>>()?
                            )
                            .build()?
                        )
                    }).collect::<Result<Vec<_>, _>>()?
                )
                .build()?
            ])
            .build()?;

        Ok(ipxact::ComponentBuilder::default()
            .vendor(base.vendor())
            .library(base.library())
            .name(base.name())
            .version(base.version())
            .memory_maps(memory_maps)
            .build()?)
    }
}

impl regvue::Document {
    pub fn from(base: &base::Component) -> anyhow::Result<Self, Error> {
        Ok(regvue::DocumentBuilder::default()
            .schema(
                regvue::SchemaBuilder::default()
                    .name("register-description-format")
                    .version(format!("v{}", base.version().to_owned()))
                    .build()?,
            )
            .root(
                regvue::RootBuilder::default()
                    .desc(base.name())
                    .version(format!("v{}", base.version().to_owned()))
                    .children(
                        base.blks()
                            .iter()
                            .map(|blk| blk.name().into())
                            .collect::<Vec<_>>(),
                    )
                    .default_reset(String::from("RS"))
                    .build()?,
            )
            .elements({
                let mut elements = HashMap::new();
                for blk in base.blks() {
                    let blk_name = blk.name();

                    elements.insert(
                        blk_name.into(),
                        regvue::ElementBuilder::default()
                            .r#type("blk")
                            .id(blk_name)
                            .name(blk_name)
                            .children(
                                blk.regs()
                                    .iter()
                                    .map(|reg| format!("{}.{}", blk_name, reg.name()))
                                    .collect::<Vec<_>>(),
                            )
                            .build()?,
                    );

                    for reg in blk.regs() {
                        let reg_name = reg.name();
                        let block_reg_name = format!("{}.{}", blk_name, reg.name());
                        elements.insert(
                            block_reg_name.clone(),
                            regvue::ElementBuilder::default()
                                .r#type("reg")
                                .id(block_reg_name)
                                .name(reg_name)
                                .offset(reg.offset().to_owned())
                                .fields({
                                    let mut fields = Vec::new();
                                    for field in reg.fields() {
                                        fields.push(
                                            regvue::FieldBuilder::default()
                                                .name(field.name())
                                                .lsb(field.offset().parse::<i32>()?)
                                                .nbits(field.width().parse::<i32>()?)
                                                .access(field.attr().to_ascii_lowercase())
                                                .reset(field.reset().to_owned())
                                                .doc(field.desc().to_owned())
                                                .build()?,
                                        );
                                    }
                                    fields
                                })
                                .build()?,
                        );
                    }
                }
                elements
            })
            .build()?)
    }
}
