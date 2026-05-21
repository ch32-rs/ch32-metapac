use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Crc",
        extends: None,
        description: Some("CRC calculation unit."),
        items: &[
            BlockItem {
                name: "datar",
                description: Some("Data register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Datar"),
                }),
            },
            BlockItem {
                name: "idatar",
                description: Some("Independent Data register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 8,
                    fieldset: Some("Idatar"),
                }),
            },
            BlockItem {
                name: "ctlr",
                description: Some("Control register."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ctlr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some("Control register."),
            bit_size: 32,
            fields: &[Field {
                name: "reset",
                description: Some("Reset bit."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Datar",
            extends: None,
            description: Some("Data register."),
            bit_size: 32,
            fields: &[Field {
                name: "dr",
                description: Some("Data Register."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Idatar",
            extends: None,
            description: Some("Independent Data register."),
            bit_size: 8,
            fields: &[Field {
                name: "idr",
                description: Some("Independent Data register."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
