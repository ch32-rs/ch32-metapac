use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Pwr",
        extends: None,
        description: Some("Power control."),
        items: &[
            BlockItem {
                name: "ctlr",
                description: Some("Power control register (PWR_CTRL)."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ctlr"),
                }),
            },
            BlockItem {
                name: "awucsr",
                description: Some("Automatic wake-up control state register (PWR_AWUCSR)."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Awucsr"),
                }),
            },
            BlockItem {
                name: "awuwr",
                description: Some("Automatic wake window comparison value register (PWR_AWUWR)."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Awuwr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Awucsr",
            extends: None,
            description: Some("Automatic wake-up control state register (PWR_AWUCSR)."),
            bit_size: 32,
            fields: &[Field {
                name: "awuen",
                description: Some("Automatic wake-up enable."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Awuwr",
            extends: None,
            description: Some("Automatic wake window comparison value register (PWR_AWUWR)."),
            bit_size: 32,
            fields: &[Field {
                name: "awuwr",
                description: Some("AWU window value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some("Power control register (PWR_CTRL)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdds",
                    description: Some("Power Down Deep Sleep."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "isinken",
                    description: Some("ISINK Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sel_io_vhv",
                    description: Some("PB4 function Selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pmuproen",
                    description: Some("PMU over-temperature protection Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
