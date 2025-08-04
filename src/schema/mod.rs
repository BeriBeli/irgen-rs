pub mod attr;
pub mod base;
pub mod ipxact;
pub mod regvue;

use std::collections::HashMap;

use crate::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    #[error("Regvue Schema error: {0}")]
    RegvueSchema(#[from] regvue::SchemaBuilderError),

    #[error("Regvue Link error: {0}")]
    RegvueLink(#[from] regvue::LinkBuilderError),

    #[error("Regvue Root error: {0}")]
    RegvueRoot(#[from] regvue::RootBuilderError),

    #[error("Regvue EnumValue error: {0}")]
    RegvueEnumValue(#[from] regvue::EnumValueBuilderError),

    #[error("Regvue Reset error: {0}")]
    RegvueReset(#[from] regvue::ResetBuilderError),

    #[error("Regvue Field error: {0}")]
    RegvueField(#[from] regvue::FieldBuilderError),

    #[error("Regvue Element error: {0}")]
    RegvueElement(#[from] regvue::ElementBuilderError),

    #[error("Regvue Document error: {0}")]
    RegvueDocument(#[from] regvue::DocumentBuilderError),
}

impl ipxact::Component {
    pub fn from(base: &base::Component) -> anyhow::Result<Self, Error> {
        let memory_maps = ipxact::MemoryMaps::new()
            .set_memory_map(vec![
                ipxact::MemoryMap::new()
                    .set_name(base.name().to_owned())?
                    .set_address_block(base.blks().iter().map(|blk| -> anyhow::Result<ipxact::Block, Error> {
                        Ok(ipxact::Block::new()
                            .set_name(blk.name().to_owned())?
                            .set_base_address(blk.offset().to_owned())?
                            .set_range(blk.range().to_owned())?
                            .set_width(blk.size().to_owned())?
                            .set_register(blk.regs().iter().map(|reg| -> anyhow::Result<ipxact::Register, Error> {
                                    Ok(ipxact::Register::new()
                                        .set_name(reg.name().to_owned())?
                                        .set_address_offset(reg.offset().to_owned())?
                                        .set_size(reg.size().to_owned())?
                                        .set_field(reg.fields().iter().map(|field| -> anyhow::Result<ipxact::Field, Error> {
                                                Ok(ipxact::Field::new()
                                                    .set_name(field.name().to_owned())?
                                                    .set_bit_offset(field.offset().to_owned())?
                                                    .set_bit_width(field.width().to_owned())?
                                                    .set_access(field.attr().to_owned())?
                                                    .set_modified_write_value(field.attr().to_owned())?
                                                    .set_read_action(field.attr().to_owned())?
                                                    .set_resets(
                                                        ipxact::Resets::new()
                                                            .set_reset(vec![
                                                                ipxact::Reset::new()
                                                                    .set_value(field.reset().to_owned())?
                                                            ])?
                                                    )?
                                                )
                                            }).collect::<Result<Vec<_>, _>>()?
                                        )?
                                    )
                                }).collect::<Result<Vec<_>, _>>()?
                            )?
                        )
                    }).collect::<Result<Vec<_>, _>>()?
                )?
            ])?;

        Ok(ipxact::Component::new()
            .set_vendor(base.vendor().to_owned())?
            .set_library(base.library().to_owned())?
            .set_name(base.name().to_owned())?
            .set_version(base.version().to_owned())?
            .set_memory_maps(memory_maps)?)
    }
}

impl regvue::Document {
    pub fn from(base: &base::Component) -> anyhow::Result<Self, SchemaError> {
        Ok(
            regvue::DocumentBuilder::default()
                .schema(
                    regvue::SchemaBuilder::default()
                        .name("register-description-format")
                        .version(format!("v{}", base.version().to_owned()))
                        .build()?
                )
                .root(
                    regvue::RootBuilder::default()
                        .desc(base.name())
                        .version(format!("v{}", base.version().to_owned()))
                        .children(
                            base.blks().iter().map(|blk| {
                                blk.name().to_owned()
                            }).collect::<Vec<_>>()
                        )
                        .default_reset("RS".to_owned())
                        .build()?
                )
                .elements(
                    {
                        let mut elements = HashMap::new();
                        for blk in base.blks() {
                            let blk_name = blk.name().to_owned();
                            
                            elements.insert(
                                blk_name.clone(),
                                regvue::ElementBuilder::default()
                                    .r#type("blk")
                                    .id(blk_name.clone())
                                    .name(blk_name.clone())
                                    .children(
                                        blk.regs().iter().map(|reg| {
                                            format!("{}.{}", blk_name.clone(), reg.name())
                                        }).collect::<Vec<_>>()
                                    )
                                    .build()?
                            );

                            for reg in blk.regs() {
                                let reg_name = reg.name().to_owned();
                                let block_reg_name = format!("{}.{}", blk_name, reg.name().to_owned());
                                elements.insert(
                                    block_reg_name.clone(),
                                    regvue::ElementBuilder::default()
                                        .r#type("reg")
                                        .id(block_reg_name)
                                        .name(reg_name)
                                        .offset(reg.offset().to_owned())
                                        .fields(
                                            {
                                                let mut fields = Vec::new();
                                                for field in reg.fields() {
                                                    fields.push(
                                                        regvue::FieldBuilder::default()
                                                            .name(field.name())
                                                            .lsb(field.offset().parse::<i32>().unwrap())
                                                            .nbits(field.width().parse::<i32>().unwrap())
                                                            .access(field.attr().to_ascii_lowercase())
                                                            .reset(field.reset().to_owned())
                                                            .build()?
                                                    );
                                                }
                                                fields
                                            }
                                        )
                                        .build()?
                                );
                            }
                        }
                        elements
                    }
                )
                .build()?
        )
    }
}
