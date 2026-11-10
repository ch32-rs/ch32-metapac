use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Gpio",
        extends: None,
        description: Some("General purpose I/O. 8 lines per port."),
        items: &[
            BlockItem {
                name: "cfglr",
                description: Some("Port configuration register low (GPIOn_CFGLR)."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfglr"),
                }),
            },
            BlockItem {
                name: "indr",
                description: Some("Port input data register (GPIOn_INDR)."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Indr"),
                }),
            },
            BlockItem {
                name: "outdr",
                description: Some("Port output data register (GPIOn_OUTDR)."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Outdr"),
                }),
            },
            BlockItem {
                name: "bshr",
                description: Some("Port bit set/reset register (GPIOn_BSHR)."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Bshr"),
                }),
            },
            BlockItem {
                name: "bcr",
                description: Some("Port bit reset register (GPIOn_BCR)."),
                array: None,
                byte_offset: 0x14,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Bcr"),
                }),
            },
            BlockItem {
                name: "lckr",
                description: Some("Port configuration lock register."),
                array: None,
                byte_offset: 0x18,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lckr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Bcr",
            extends: None,
            description: Some("Port bit reset register (GPIOn_BCR)."),
            bit_size: 32,
            fields: &[Field {
                name: "br",
                description: Some("Reset bit 0."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 8, stride: 1 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Bshr",
            extends: None,
            description: Some("Port bit set/reset register (GPIOn_BSHR)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bs",
                    description: Some("Set bit 0."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "br",
                    description: Some("Reset bit 0."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 1 })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfglr",
            extends: None,
            description: Some("Port configuration register low (GPIOn_CFGLR)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some("Port n.0 mode bits."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                    enumm: Some("Mode"),
                },
                Field {
                    name: "cnf",
                    description: Some("Port n.0 configuration bits."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 2,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                    enumm: Some("Cnf"),
                },
            ],
        },
        FieldSet {
            name: "Indr",
            extends: None,
            description: Some("Port input data register (GPIOn_INDR)."),
            bit_size: 32,
            fields: &[Field {
                name: "idr",
                description: Some("Port input data."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 8, stride: 1 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Lckr",
            extends: None,
            description: Some("Port configuration lock register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lck",
                    description: Some("Port A Lock bit 0."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "lckk",
                    description: Some("Lock key."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Outdr",
            extends: None,
            description: Some("Port output data register (GPIOn_OUTDR)."),
            bit_size: 32,
            fields: &[Field {
                name: "odr",
                description: Some("Port output data."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 8, stride: 1 })),
                enumm: None,
            }],
        },
    ],
    enums: &[
        Enum {
            name: "Cnf",
            description: Some(
                "port x configuration selection, configure the corresponding port by these bits.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "AF_OPEN_DRAIN_OUT",
                    description: Some("Alternate function output push-pull"),
                    value: 3,
                },
                EnumVariant {
                    name: "ANALOG_IN__PUSH_PULL_OUT",
                    description: Some("Analog input / push-pull output."),
                    value: 0,
                },
                EnumVariant {
                    name: "FLOATING_IN__OPEN_DRAIN_OUT",
                    description: Some("Floating input / open-drain output."),
                    value: 1,
                },
                EnumVariant {
                    name: "PULL_IN__AF_PUSH_PULL_OUT",
                    description: Some("Input with pull-up / AF pull-down."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INPUT",
                    description: Some("Input mode."),
                    value: 0,
                },
                EnumVariant {
                    name: "OUTPUT_10MHZ",
                    description: Some("Output mode."),
                    value: 1,
                },
                EnumVariant {
                    name: "OUTPUT_2MHZ",
                    description: Some("Output mode."),
                    value: 2,
                },
                EnumVariant {
                    name: "OUTPUT_50MHZ",
                    description: Some("Output mode."),
                    value: 3,
                },
            ],
        },
    ],
};
