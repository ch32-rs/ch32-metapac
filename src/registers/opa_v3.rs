use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Opa",
            extends: None,
            description: Some(
                "OPA configuration register block. 4 independent operational amplifiers (OPA1..OPA4) on CH32F20x/CH32V20x/CH32V30x/CH32V31x.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr",
                    description: Some(
                        "OPA control register.",
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
                    name: "ctr2",
                    description: Some(
                        "OPA control register 2 (Configuration Extended Control Register 2 / EXTEN_CTR2). Per-OPA high-speed mode enable.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctr2",
                            ),
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
                "OPA control register. Each OPA has 4 control bits (EN, MODE, NSEL, PSEL) at a 4-bit stride.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en1",
                    description: Some(
                        "OPA1 enable. 0=disable, 1=enable.",
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
                        "OPA1 output channel selection. 0=OPA1_OUT0, 1=OPA1_OUT1.",
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
                    name: "nsel1",
                    description: Some(
                        "OPA1 negative input selection. 0=CHN0, 1=CHN1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psel1",
                    description: Some(
                        "OPA1 positive input selection. 0=CHP0, 1=CHP1.",
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
                    name: "en2",
                    description: Some(
                        "OPA2 enable. 0=disable, 1=enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mode2",
                    description: Some(
                        "OPA2 output channel selection. 0=OPA2_OUT0, 1=OPA2_OUT1.",
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
                Field {
                    name: "nsel2",
                    description: Some(
                        "OPA2 negative input selection. 0=CHN0, 1=CHN1.",
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
                Field {
                    name: "psel2",
                    description: Some(
                        "OPA2 positive input selection. 0=CHP0, 1=CHP1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "en3",
                    description: Some(
                        "OPA3 enable. Only present on D8/D8C variants. 0=disable, 1=enable.",
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
                    name: "mode3",
                    description: Some(
                        "OPA3 output channel selection. Only present on D8/D8C variants. 0=OPA3_OUT0, 1=OPA3_OUT1.",
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
                    name: "nsel3",
                    description: Some(
                        "OPA3 negative input selection. Only present on D8/D8C variants. 0=CHN0, 1=CHN1.",
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
                Field {
                    name: "psel3",
                    description: Some(
                        "OPA3 positive input selection. Only present on D8/D8C variants. 0=CHP0, 1=CHP1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "en4",
                    description: Some(
                        "OPA4 enable. Only present on D8/D8C variants. 0=disable, 1=enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mode4",
                    description: Some(
                        "OPA4 output channel selection. Only present on D8/D8C variants. 0=OPA4_OUT0, 1=OPA4_OUT1.",
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
                    name: "nsel4",
                    description: Some(
                        "OPA4 negative input selection. Only present on D8/D8C variants. 0=CHN0, 1=CHN1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psel4",
                    description: Some(
                        "OPA4 positive input selection. Only present on D8/D8C variants. 0=CHP0, 1=CHP1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctr2",
            extends: None,
            description: Some(
                "OPA control register 2 (Configuration Extended Control Register 2 / EXTEN_CTR2). Each OPA has a 1-bit high-speed mode enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsmd1",
                    description: Some(
                        "OPA1 high-speed mode enable. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate).",
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
                    name: "hsmd2",
                    description: Some(
                        "OPA2 high-speed mode enable. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate).",
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
                    name: "hsmd3",
                    description: Some(
                        "OPA3 high-speed mode enable. Only present on D8/D8C variants. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsmd4",
                    description: Some(
                        "OPA4 high-speed mode enable. Only present on D8/D8C variants. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate).",
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
            ],
        },
    ],
    enums: &[],
};
