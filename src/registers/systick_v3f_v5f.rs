use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Systick",
            extends: None,
            description: Some(
                "Systick registers for V3F + V5F dual-core (currently CH32H417). Two independent 32-bit up/down counters with per-counter core ID (CID) routing for interrupts.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr_0",
                    description: Some(
                        "System count control register for counter 0.",
                    ),
                    array: None,
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
                    name: "isr",
                    description: Some(
                        "System counter interrupt status register (shared by both counters).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_0",
                    description: Some(
                        "System counter 0 count register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp_0",
                    description: Some(
                        "System counter 0 compare register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr_1",
                    description: Some(
                        "System count control register for counter 1.",
                    ),
                    array: None,
                    byte_offset: 0x80,
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
                    name: "cnt_1",
                    description: Some(
                        "System counter 1 count register.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp_1",
                    description: Some(
                        "System counter 1 compare register.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmp",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cmp",
            extends: None,
            description: Some(
                "System count compare register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "32-bit compare value (used as reload value when counting down).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cnt",
            extends: None,
            description: Some(
                "System counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "32-bit Systick counter value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some(
                "System count control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
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
                    name: "ie",
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
                    name: "no_rtc",
                    description: Some(
                        "Counter clock source selection.",
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
                    name: "auto_reload",
                    description: Some(
                        "Auto reload enable.",
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
                    name: "down_mode",
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
                        "Core ID for routing the counter interrupt to a specific hart. Auto-updated when EN is set to the operating hart's core ID.",
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
            name: "Isr",
            extends: None,
            description: Some(
                "System counter interrupt status register (shared).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "isr0",
                    description: Some(
                        "Systick0 interrupt flag.",
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
                    name: "isr1",
                    description: Some(
                        "Systick1 interrupt flag.",
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
                        "HCLK/8.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HCLK",
                    description: Some(
                        "HCLK.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
