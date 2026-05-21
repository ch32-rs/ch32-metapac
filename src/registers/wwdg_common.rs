use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Wwdg",
        extends: None,
        description: Some("Window watchdog."),
        items: &[
            BlockItem {
                name: "ctlr",
                description: Some("Control register (WWDG_CR)."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ctlr"),
                }),
            },
            BlockItem {
                name: "cfgr",
                description: Some("Configuration register (WWDG_CFGR)."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfgr"),
                }),
            },
            BlockItem {
                name: "statr",
                description: Some("Status register (WWDG_SR)."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Statr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some("Configuration register (WWDG_CFGR)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "w",
                    description: Some("7-bit window value."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgtb",
                    description: Some("Timer Base."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ewi",
                    description: Some("Early Wakeup Interrupt."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some("Control register (WWDG_CR)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some("7-bit counter (MSB to LSB)."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdga",
                    description: Some("Activation bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Statr",
            extends: None,
            description: Some("Status register (WWDG_SR)."),
            bit_size: 32,
            fields: &[Field {
                name: "ewif",
                description: Some("Early Wakeup Interrupt Flag."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
