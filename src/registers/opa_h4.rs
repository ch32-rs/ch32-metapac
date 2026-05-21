use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Opa",
            extends: None,
            description: Some(
                "OPA configuration.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "OPA Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr2",
                    description: Some(
                        "OPA Control Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr3",
                    description: Some(
                        "OPA Control Register 3.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp_ctlr",
                    description: Some(
                        "CMP Control Registers.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmpCtlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp_statr",
                    description: Some(
                        "CMP Status Registers.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmpStatr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "CmpCtlr",
            extends: None,
            description: Some(
                "CMP Control Registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "psel",
                    description: Some(
                        "CMP Positive Channel Selection Bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nsel",
                    description: Some(
                        "CMP Negative End Channel Selection Bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mode",
                    description: Some(
                        "CMP Output Channel Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "en",
                    description: Some(
                        "CMP enables.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hypsel",
                    description: Some(
                        "CMP hysteresis voltage selector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vref",
                    description: Some(
                        "CMP Internal Bias Voltage Selector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "filt_en",
                    description: Some(
                        "CMP Digital Filtering Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "filt_cfg",
                    description: Some(
                        "CMP filter sampling interval configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "filt_base",
                    description: Some(
                        "CMP filtering sampling time base configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CmpStatr",
            extends: None,
            description: Some(
                "CMP Status Registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out_filt",
                    description: Some(
                        "CMP Output.",
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
            ],
        },
        FieldSet {
            name: "Ctlr1",
            extends: None,
            description: Some(
                "OPA Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en1",
                    description: Some(
                        "OPA1 enable.",
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
                    name: "mode1",
                    description: Some(
                        "OPA1 Output Channel Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psel1",
                    description: Some(
                        "OPA1 Positive Channel Selection.",
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
                    name: "nsel1",
                    description: Some(
                        "OPA1 Negative Channel Selection vs. PGA Gain Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fb_en1",
                    description: Some(
                        "OPA1's PGA mode feedback enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgadif1",
                    description: Some(
                        "OPA1 is used with NSEL1 as PGA and the N-terminal is connected to OPA1_CHN1 (PA7).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hs1",
                    description: Some(
                        "OPA1 high-speed mode enables.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlr2",
            extends: None,
            description: Some(
                "OPA Control Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en2",
                    description: Some(
                        "OPA2 enable.",
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
                    name: "mode2",
                    description: Some(
                        "OPA2 Output Channel Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psel2",
                    description: Some(
                        "OPA2 Positive Channel Selection.",
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
                    name: "nsel2",
                    description: Some(
                        "OPA2 Negative Channel Selection vs. PGA Gain Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fb_en2",
                    description: Some(
                        "OPA2's PGA mode feedback enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgadif2",
                    description: Some(
                        "OPA2 is used with NSEL2 as PGA and the N-terminal is connected to OPA2_CHN1 (PF12).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hs2",
                    description: Some(
                        "OPA2 high-speed mode enables.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlr3",
            extends: None,
            description: Some(
                "OPA Control Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en3",
                    description: Some(
                        "OPA3 enable.",
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
                    name: "mode3",
                    description: Some(
                        "OPA3 Output Channel Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psel3",
                    description: Some(
                        "OPA3 Positive Channel Selection.",
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
                    name: "nsel3",
                    description: Some(
                        "OPA3 Negative Channel Selection vs. PGA Gain Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fb_en3",
                    description: Some(
                        "OPA3's PGA mode feedback enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgadif3",
                    description: Some(
                        "OPA3 is used with NSEL3 as PGA and the N-terminal is connected to OPA3_CHN1 (PA3).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hs3",
                    description: Some(
                        "OPA3 high-speed mode enables.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
