use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Hsadc",
            extends: None,
            description: Some(
                "High speed ADC.",
            ),
            items: &[
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "High-speed ADC configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "HSADC Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                        "HSADC Control Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "statr",
                    description: Some(
                        "HSADC Status Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Statr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "datar",
                    description: Some(
                        "HSADC Data Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Datar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addr0",
                    description: Some(
                        "HSADC DMA Receive Address Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addr1",
                    description: Some(
                        "HSADC DMA Receive Address Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addr1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addr0",
            extends: None,
            description: Some(
                "HSADC DMA Receive Address Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_addr0",
                    description: Some(
                        "The DMA transmission address is configured with the 0 configuration bit.",
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
            name: "Addr1",
            extends: None,
            description: Some(
                "HSADC DMA Receive Address Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_addr1",
                    description: Some(
                        "DMA transport address 1 configuration bit.",
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
            name: "Cfgr",
            extends: None,
            description: Some(
                "High-speed ADC configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "High-speed ADC enable.",
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
                    name: "dmaen",
                    description: Some(
                        "Direct Storage Access (DMA) mode enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "setup",
                    description: Some(
                        "First Transition Establishment Time Configuration Bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ppmode",
                    description: Some(
                        "Ping Pong storage mode enable.",
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
                    name: "burst_en",
                    description: Some(
                        "DMA transmission length configuration bit.",
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
                Field {
                    name: "dma_len",
                    description: Some(
                        "DMA transmission length configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlr1",
            extends: None,
            description: Some(
                "HSADC Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "Initiating High-Speed ADC Conversion.",
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
                    name: "burst_end",
                    description: Some(
                        "Abort a burst transmission.",
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
                    name: "eocie",
                    description: Some(
                        "Transition Completion Interrupt Enables.",
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
                    name: "dmaie",
                    description: Some(
                        "Transition Completion Interrupt Enables.",
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
                    name: "burstie",
                    description: Some(
                        "Interrupt enables when the burst transmission is complete.",
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
                "HSADC Control Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "burst_dma_len",
                    description: Some(
                        "The final DMA transmission length configuration bit for burst transmission..  If the number of data transmitted in the final burst is not aligned with the DMA transmission length,   the DMA transmission length is configured.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "burst_len",
                    description: Some(
                        "Burst transmission length configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Datar",
            extends: None,
            description: Some(
                "HSADC Data Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dr",
                    description: Some(
                        "Convert data register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Statr",
            extends: None,
            description: Some(
                "HSADC Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eocif",
                    description: Some(
                        "Transition Complete Interrupt flag.",
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
                    name: "dmaif",
                    description: Some(
                        "DMA Transmission Complete Interrupt flag.",
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
                    name: "burstif",
                    description: Some(
                        "Burst transfer completed interrupt flag.",
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
                    name: "rxne",
                    description: Some(
                        "The data register is not empty flag, that is, the conversion is completed.  and the data is stored in the data register, which is the position bit;   A read operation on the data register clears the bit.",
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
                    name: "pp_addr",
                    description: Some(
                        "Ping-pong storage mode cache address indicator bit.",
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
                    name: "fifo_rdy",
                    description: Some(
                        "Receives a FIFO non-null status flag.",
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
                    name: "fifo_full",
                    description: Some(
                        "Receive FIFO full status flags.",
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
                    name: "fifo_ov",
                    description: Some(
                        "Receives the FIFO overflow status flag.",
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
                    name: "fifo_cnt",
                    description: Some(
                        "Receives the FIFO current count value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
