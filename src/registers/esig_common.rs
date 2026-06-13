use crate::metadata::ir::*;
pub(crate) static DESCRIPTOR: IR = IR {
    blocks: &[Block {
        name: "ESIG",
        extends: None,
        description: Some(
            "Factory-programmed chip identity — flash size and 96-bit unique device ID.",
        ),
        items: &[
            BlockItem {
                name: "FLACAP",
                description: Some("Total flash size, in KiB."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 16,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "UNIID1",
                description: Some("Unique device ID — bits 0-31 of the 96-bit identifier."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "UNIID2",
                description: Some("Unique device ID — bits 32-63 of the 96-bit identifier."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "UNIID3",
                description: Some("Unique device ID — bits 64-95 of the 96-bit identifier."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[],
    enums: &[],
};
