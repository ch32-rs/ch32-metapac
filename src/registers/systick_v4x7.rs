use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Systick",
            extends: None,
            description: Some(
                "CH32V407/CH32V467 system counter with 32-bit and 64-bit access views.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr",
                    description: Some(
                        "System count control register.",
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
                    name: "sr",
                    description: Some(
                        "System count status register.",
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
                        "64-bit system counter view (project register extension).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "cntl",
                    description: Some(
                        "32-bit system counter value (STK_CNTL).",
                    ),
                    array: None,
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
                    name: "cnth",
                    description: Some(
                        "System counter high word (project register extension).",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                        "64-bit compare value view (project register extension).",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "cmpl",
                    description: Some(
                        "32-bit compare value (STK_CMPLR).",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "cmph",
                    description: Some(
                        "Compare value high word (project register extension).",
                    ),
                    array: None,
                    byte_offset: 0x14,
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
                        "Counter enable control bit.",
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
                        "Counter interrupt enable control bit.",
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
                        "System counter clock source.",
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
                        "Auto reload count enable bit.",
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
                        "Counter mode.",
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
                    name: "init",
                    description: Some(
                        "Counter initial value update.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
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
                "System count status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cntif",
                    description: Some(
                        "Compare flag. Write 0 to clear; writing 1 has no effect.",
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
                    name: "swie",
                    description: Some(
                        "Software interrupt request. Set to trigger; clear in the software interrupt handler.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
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
