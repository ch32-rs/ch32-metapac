use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Esig",
        extends: None,
        description: Some("ESIG configuration."),
        items: &[
            BlockItem {
                name: "flacap",
                description: Some("Flash capacity register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: Some("Flacap"),
                }),
            },
            BlockItem {
                name: "uniid1",
                description: Some("UID register."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Uniid1"),
                }),
            },
            BlockItem {
                name: "uniid2",
                description: Some("UID register."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Uniid2"),
                }),
            },
            BlockItem {
                name: "uniid3",
                description: Some("Div register."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Uniid3"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Flacap",
            extends: None,
            description: Some("Flash capacity register."),
            bit_size: 16,
            fields: &[Field {
                name: "f_size",
                description: Some("F_SIZE/kByte."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Uniid1",
            extends: None,
            description: Some("UID register."),
            bit_size: 32,
            fields: &[Field {
                name: "u_id",
                description: Some("U_ID value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Uniid2",
            extends: None,
            description: Some("UID register."),
            bit_size: 32,
            fields: &[Field {
                name: "u_id",
                description: Some("U_ID value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Uniid3",
            extends: None,
            description: Some("Div register."),
            bit_size: 32,
            fields: &[Field {
                name: "u_id",
                description: Some("U_ID value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
