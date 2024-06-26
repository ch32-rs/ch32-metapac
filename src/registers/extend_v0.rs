use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Extend",
        extends: None,
        description: Some("Extend configuration."),
        items: &[
            BlockItem {
                name: "ctr",
                description: Some("Configure the extended control register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ctr"),
                }),
            },
            BlockItem {
                name: "keyr",
                description: Some("Configure the extended key register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Keyr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ctr",
            extends: None,
            description: Some("Configure the extended control register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pll_cfg",
                    description: Some("Configure the PLL clock delay time."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lockup_en",
                    description: Some("LOCKUP_Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lockup_reset",
                    description: Some("LOCKUP RESET."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ldo_trim",
                    description: Some("LDO_TRIM."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flash_clk_trim",
                    description: Some("FLASH clock trimming."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wr_en",
                    description: Some("Control Register write enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wr_lock",
                    description: Some("Control Register write lock."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opa_en",
                    description: Some("OPA Enalbe."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opa_nsel",
                    description: Some("OPA negative end channel selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opa_psel",
                    description: Some("OPA positive end channel selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Keyr",
            extends: None,
            description: Some("Configure the extended key register."),
            bit_size: 32,
            fields: &[Field {
                name: "key",
                description: Some("Write key value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
