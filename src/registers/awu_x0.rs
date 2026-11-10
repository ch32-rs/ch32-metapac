use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Awu",
        extends: None,
        description: Some("AWU configuration."),
        items: &[
            BlockItem {
                name: "csr",
                description: Some("Status Control register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "wr",
                description: Some("AWU Window register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: Some("Wr"),
                }),
            },
            BlockItem {
                name: "psc",
                description: Some("PSC."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: Some("Psc"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("Status Control register."),
            bit_size: 16,
            fields: &[Field {
                name: "en",
                description: Some("AWU Enable."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Psc",
            extends: None,
            description: Some("PSC."),
            bit_size: 16,
            fields: &[Field {
                name: "tbr",
                description: Some("AWU_TBR value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: None,
                enumm: Some("Prescaler"),
            }],
        },
        FieldSet {
            name: "Wr",
            extends: None,
            description: Some("AWU Window register."),
            bit_size: 16,
            fields: &[Field {
                name: "apr",
                description: Some("AWU_APR value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 6,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[Enum {
        name: "Prescaler",
        description: None,
        bit_size: 4,
        variants: &[
            EnumVariant {
                name: "DIV1",
                description: Some("DIV1."),
                value: 0,
            },
            EnumVariant {
                name: "DIV1024",
                description: Some("DIV1024."),
                value: 11,
            },
            EnumVariant {
                name: "DIV10240",
                description: Some("DIV10240."),
                value: 14,
            },
            EnumVariant {
                name: "DIV128",
                description: Some("DIV128."),
                value: 8,
            },
            EnumVariant {
                name: "DIV16",
                description: Some("DIV16."),
                value: 5,
            },
            EnumVariant {
                name: "DIV2",
                description: Some("DIV2."),
                value: 2,
            },
            EnumVariant {
                name: "DIV2048",
                description: Some("DIV2048."),
                value: 12,
            },
            EnumVariant {
                name: "DIV256",
                description: Some("DIV256."),
                value: 9,
            },
            EnumVariant {
                name: "DIV32",
                description: Some("DIV32."),
                value: 6,
            },
            EnumVariant {
                name: "DIV4",
                description: Some("DIV4."),
                value: 3,
            },
            EnumVariant {
                name: "DIV4096",
                description: Some("DIV4096."),
                value: 13,
            },
            EnumVariant {
                name: "DIV512",
                description: Some("DIV512."),
                value: 10,
            },
            EnumVariant {
                name: "DIV61440",
                description: Some("DIV61440."),
                value: 15,
            },
            EnumVariant {
                name: "DIV64",
                description: Some("DIV64."),
                value: 7,
            },
            EnumVariant {
                name: "DIV8",
                description: Some("DIV8."),
                value: 4,
            },
        ],
    }],
};
