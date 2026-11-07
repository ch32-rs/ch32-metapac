use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Systick",
            extends: None,
            description: Some(
                "Systick registers for the CH32H4 dual-core SysTick. Two independent 32-bit counters, indexed by hart, and one status register shared by both.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr",
                    description: Some(
                        "System count control register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "System counter interrupt status register. Shared by both counters.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "System counter count register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "cmp",
                    description: Some(
                        "System count compare register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some(
                "System count control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ste",
                    description: Some(
                        "Counter enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stie",
                    description: Some(
                        "Counter interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stclk",
                    description: Some(
                        "Counter clock source. Both counters are clocked from the V3F HCLK domain.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stclk",
                    ),
                },
                Field {
                    name: "stre",
                    description: Some(
                        "Auto reload enable. Leave clear for a free-running counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mode",
                    description: Some(
                        "Count mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "cid",
                    description: Some(
                        "Hart the counter interrupt is routed to. Updated to the operating hart when STE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "System counter interrupt status register (shared).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cntif0",
                    description: Some(
                        "Counter 0 compare flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cntif1",
                    description: Some(
                        "Counter 1 compare flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Mode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UPCOUNT",
                    description: Some(
                        "Upcount.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DOWNCOUNT",
                    description: Some(
                        "Downcount.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Stclk",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HCLK_DIV8",
                    description: Some(
                        "V3F HCLK/8.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HCLK",
                    description: Some(
                        "V3F HCLK.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
