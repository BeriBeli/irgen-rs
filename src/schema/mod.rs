pub mod attr;
pub mod base;
pub mod ipxact;
pub mod regvue;

use crate::error::Error;

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

// impl regvue::Document {
//     pub fn from(base: &(base::Component)) -> Self {
//         todo!()
//     }
// }
