use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Iwdg",
        extends: None,
        description: Some("Independent watchdog."),
        items: &[
            BlockItem {
                name: "ctlr",
                description: Some("control register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: Some("Ctlr"),
                }),
            },
            BlockItem {
                name: "pscr",
                description: Some("prescaler register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: Some("Pscr"),
                }),
            },
            BlockItem {
                name: "rldr",
                description: Some("reload register."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: Some("Rldr"),
                }),
            },
            BlockItem {
                name: "statr",
                description: Some("status register."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 16,
                    fieldset: Some("Statr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some("control register."),
            bit_size: 32,
            fields: &[Field {
                name: "key",
                description: Some(
                    "Key value (0x5555 to unlock PSCR/RLDR, 0xAAAA to reload, 0xCCCC to start).",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Pscr",
            extends: None,
            description: Some("prescaler register."),
            bit_size: 32,
            fields: &[Field {
                name: "pr",
                description: Some("Prescaler divider."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 3,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Rldr",
            extends: None,
            description: Some("reload register."),
            bit_size: 32,
            fields: &[Field {
                name: "rl",
                description: Some("Watchdog counter reload value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 12,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Statr",
            extends: None,
            description: Some("status register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvu",
                    description: Some("Watchdog prescaler value update."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rvu",
                    description: Some("Watchdog counter reload value update."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
