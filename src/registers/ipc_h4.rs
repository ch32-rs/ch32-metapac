use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ipc",
            extends: None,
            description: Some(
                "Inter-Processor Communication. Four bidirectional channels, each carrying 8 status bits with independent enable/mask/set/clear, plus four 32-bit shared message slots for short messages.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr",
                    description: Some(
                        "IPC control register. Per-channel TX/RX core ID routing, interrupt enables, auto-update mode and configuration lock.",
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
                        "Per-channel interrupt status (read-only summary; bits set when channel state matches the configured trigger).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Channels",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ism",
                    description: Some(
                        "Per-channel interrupt mask status (read-only; bit set means channel is currently raising an interrupt request to its routed core).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Channels",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ena",
                    description: Some(
                        "Per-channel status-bit interrupt enable. Each enabled bit produces TX or RX interrupts based on the value in STS.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Channels",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sts",
                    description: Some(
                        "Per-channel status register. Bit value 1 may raise an RX interrupt; bit value 0 may raise a TX interrupt (subject to ENA / IER configuration).",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Channels",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "Per-channel status set register. Writing 1 sets the corresponding bit in STS (and in ENA when AUTOEN is on).",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Channels",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clr",
                    description: Some(
                        "Per-channel status clear register. Writing 1 clears the corresponding bit in STS.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Channels",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msg",
                    description: Some(
                        "32-bit inline message slot. Four slots are available to carry short messages without touching shared SRAM.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Msg",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Channels",
            extends: None,
            description: Some(
                "Per-channel 8-bit value (used by ISR / ISM / ENA / STS / SET / CLR). Channel n occupies bits 8n..8n+7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "Channel byte. Each of the 8 bits is an independent status / enable / set / clear flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some(
                "IPC control register. Each of the four channels occupies 8 bits (channel n at bits 8n..8n+7).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_cid",
                    description: Some(
                        "Sender core ID. 0=route TX interrupt to hart 0 (V3F), 1=route to hart 1 (V5F).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rx_cid",
                    description: Some(
                        "Receiver core ID. 0=route RX interrupt to hart 0, 1=route to hart 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tx_ier",
                    description: Some(
                        "TX interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rx_ier",
                    description: Some(
                        "RX interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "autoen",
                    description: Some(
                        "Status-bit auto-update. When set, writing to STS / SET also updates ENA to the same value so the interrupt fires immediately.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some(
                        "Channel configuration lock. Write 1 to freeze the channel's other control bits until reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Msg",
            extends: None,
            description: Some(
                "32-bit inline message register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "32-bit message value.",
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
    ],
    enums: &[],
};
