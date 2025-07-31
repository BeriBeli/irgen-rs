pub mod attr;
pub mod base;
pub mod ipxact;
pub mod regvue;

use crate::error::Error;

impl ipxact::Component {
    pub fn from(base: &base::Component) -> anyhow::Result<Self, Error> {
        // Convert blocks from base format to IP-XACT address blocks
        let address_blocks = base.blocks().as_ref().map(|blocks| {
            blocks.iter().map(|block| -> anyhow::Result<ipxact::Block, Error> {
                Ok(ipxact::Block::new()
                    .set_name(block.name().clone())?
                    .set_base_address(block.offset().clone())?
                    .set_range(block.range().clone())?
                    .set_width(block.size().clone())?
                    .set_register(block.registers().as_ref().map(|registers| {
                        registers.iter().map(|register| -> anyhow::Result<ipxact::Register, Error> {
                            Ok(ipxact::Register::new()
                                .set_name(register.name().clone())?
                                .set_address_offset(register.offset().clone())?
                                .set_size(register.size().clone())?
                                .set_field(register.fields().as_ref().map(|fields| {
                                    fields.iter().map(|field| -> anyhow::Result<ipxact::Field, Error> {
                                        Ok(ipxact::Field::new()
                                            .set_name(field.name().clone())?
                                            .set_bit_offset(field.offset().clone())?
                                            .set_bit_width(field.width().clone())?
                                            .set_access(field.attribute().clone())?
                                            .set_modified_write_value(field.attribute().clone())?
                                            .set_read_action(field.attribute().clone())?
                                            .set_resets(
                                                Some(ipxact::Resets::new()
                                                    .set_reset(Some(vec![
                                                        ipxact::Reset::new()
                                                            .set_value(field.default().clone())?
                                                    ]))?)
                                            )?
                                        )
                                    }).collect::<Result<Vec<_>, _>>().unwrap()
                                }))?
                            )
                        }).collect::<Result<Vec<_>, _>>().unwrap()
                    }))?
                )
            }).collect::<Result<Vec<_>, _>>().unwrap()
        });

        // Create memory maps
        let memory_maps = address_blocks.map(|address_block| {
            ipxact::MemoryMaps::new()
                .set_memory_map(Some(vec![
                    ipxact::MemoryMap::new()
                        .set_name(Some("default_memory_map".to_string()))
                        .unwrap()
                        .set_address_block(Some(address_block))
                        .unwrap(),
                ]))
                .unwrap()
        });

        Ok(ipxact::Component::new()
            .set_vendor(base.vendor().clone())?
            .set_library(base.library().clone())?
            .set_name(base.name().clone())?
            .set_version(base.version().clone())?
            .set_memory_maps(memory_maps)?)
    }
}

// impl regvue::Document {
//     pub fn from(base: &(base::Component)) -> Self {
//         todo!()
//     }
// }
