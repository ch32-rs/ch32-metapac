use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Systick",
        extends: None,
        description: Some("Systick registers, 32-bit downcounter for QingKeV2."),
        items: &[
            BlockItem {
                name: "ctlr",
                description: Some("System count control register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ctlr"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("System count status register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "cnt",
                description: Some("System counter register, 32-bit."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cntl",
                description: Some("System counter register, 32-bit."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cmp",
                description: Some("System count compare register, 32-bit."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cmpl",
                description: Some("System count compare register, 32-bit."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some("System count control register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ste",
                    description: Some("Counter enable control bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stie",
                    description: Some("Counter interrupt enable control bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stclk",
                    description: Some("Counter system clock sourse selection bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Stclk"),
                },
                Field {
                    name: "stre",
                    description: Some("Auto reload count enable bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swie",
                    description: Some("Software interrupt enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("System count status register."),
            bit_size: 32,
            fields: &[Field {
                name: "cntif",
                description: Some("Count value compare flag."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[Enum {
        name: "Stclk",
        description: None,
        bit_size: 1,
        variants: &[
            EnumVariant {
                name: "HCLK",
                description: Some("HCLK."),
                value: 1,
            },
            EnumVariant {
                name: "HCLK_DIV8",
                description: Some("HCLK/8."),
                value: 0,
            },
        ],
    }],
};
