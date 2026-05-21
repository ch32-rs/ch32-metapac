use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Iwdg",
        extends: None,
        description: Some("Independent watchdog."),
        items: &[
            BlockItem {
                name: "ctlr",
                description: Some("Key register (IWDG_CTLR)."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ctlr"),
                }),
            },
            BlockItem {
                name: "pscr",
                description: Some("Prescaler register (IWDG_PSCR)."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pscr"),
                }),
            },
            BlockItem {
                name: "rldr",
                description: Some("Reload register (IWDG_RLDR)."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Rldr"),
                }),
            },
            BlockItem {
                name: "statr",
                description: Some("Status register (IWDG_SR)."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Statr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some("Key register (IWDG_CTLR)."),
            bit_size: 32,
            fields: &[Field {
                name: "key",
                description: Some("Key value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Pscr",
            extends: None,
            description: Some("Prescaler register (IWDG_PSCR)."),
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
            description: Some("Reload register (IWDG_RLDR)."),
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
            description: Some("Status register (IWDG_SR)."),
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
