use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usbss",
            extends: None,
            description: Some(
                "USB3.0 ultra-high speed host /device controller.",
            ),
            items: &[
                BlockItem {
                    name: "link_cfg",
                    description: Some(
                        "LINK Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_ctrl",
                    description: Some(
                        "LINK Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_int_ctrl",
                    description: Some(
                        "LINK Interrupt Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkIntCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_int_flag",
                    description: Some(
                        "LINK Interrupt Flag Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkIntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_status",
                    description: Some(
                        "LINK Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_itp_pre",
                    description: Some(
                        "LINK ITP timeout mode register.",
                    ),
                    array: None,
                    byte_offset: 0x17,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LinkItpPre",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u2_inact_timer",
                    description: Some(
                        "LINK U2 Inactivity Timeout Counter Threshold Register.",
                    ),
                    array: None,
                    byte_offset: 0x1d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LinkU2InactTimer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u1_wkup_filter",
                    description: Some(
                        "U1 Wake Up LFPS Effective Time Register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LinkU1WkupFilter",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u2_wkup_filter",
                    description: Some(
                        "U2 Wake LFPS Effective Time Register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LinkU2WkupFilter",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u3_wkup_filter",
                    description: Some(
                        "U3 Wake LFPS Effective Time Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LinkU3WkupFilter",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_iso_dly",
                    description: Some(
                        "LINK Synchronous Delay Register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "LinkIsoDly",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_lpm_cr",
                    description: Some(
                        "Link Power Management Registers.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "LinkLpmCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_port_cap",
                    description: Some(
                        "PORT_CAP Register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkLmpPortCap",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_rx_data0",
                    description: Some(
                        "LMP Receive Data 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkLmpRxData0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_rx_data1",
                    description: Some(
                        "LMP Receives Data 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkLmpRxData1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_rx_data2",
                    description: Some(
                        "LMP Receives Data 2 Register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkLmpRxData2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_tx_data0",
                    description: Some(
                        "USB Custom HP Data 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkLmpTxData0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_tx_data1",
                    description: Some(
                        "USB Custom HP Data 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkLmpTxData1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_tx_data2",
                    description: Some(
                        "USB Custom HP Data 2 Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkLmpTxData2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "Flash capecity register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "USBSS Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itp",
                    description: Some(
                        "The Interval register for the ITP package in USB.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Itp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itp_adj",
                    description: Some(
                        "Interval adaptive registers for ITP packets in USB.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ItpAdj",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_en",
                    description: Some(
                        "Endpoint Send Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepTxEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_en",
                    description: Some(
                        "Endpoint Receive Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x82,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRxEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_tx_ctrl",
                    description: Some(
                        "Endpoint 0 Transmit Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep0TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_rx_ctrl",
                    description: Some(
                        "Endpoint 0 Receives Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep0RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_tx_dma",
                    description: Some(
                        "Sending Buffer Address Register.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep0TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_rx_dma",
                    description: Some(
                        "Receive Buffer Address Register.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep0RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_tx_dma_ofs",
                    description: Some(
                        "endpoint 0 transmit DMA offset.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_rx_dma_ofs",
                    description: Some(
                        "endpoint 0 receive DMA offset.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_cfg",
                    description: Some(
                        "Endpoint 1 configures the registers.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_cr",
                    description: Some(
                        "Endpoint 1 Control Register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_seq",
                    description: Some(
                        "Endpoint 1 serial number register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_st",
                    description: Some(
                        "Endpoint 1 state register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_chain_cr",
                    description: Some(
                        "Endpoint 1 CHAIN control register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_chain_st",
                    description: Some(
                        "Endpoint 1 CHAIN status register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_chain_len",
                    description: Some(
                        "Endpoint 1 CHAIN sends the last packet length.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepTxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_chain_exp_nump",
                    description: Some(
                        "Number of NUMPs expected to be sent by endpoint 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxChainExpNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_chain_nump",
                    description: Some(
                        "Number of NUMPs sent by endpoint 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_dma_ofs",
                    description: Some(
                        "DMA offset length of endpoint 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xca,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepTxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_dma",
                    description: Some(
                        "DMA start address of endpoint 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UepTxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_cfg",
                    description: Some(
                        "Endpoint 1 Configuration Register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_cr",
                    description: Some(
                        "Endpoint 1 Control Register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_seq",
                    description: Some(
                        "Endpoint 1 serial number register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_st",
                    description: Some(
                        "Endpoint 1 state register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_chain_cr",
                    description: Some(
                        "Endpoint 1 CHAIN control register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_chain_st",
                    description: Some(
                        "Endpoint 1 CHAIN status register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_chain_len",
                    description: Some(
                        "The length of the last packet received by the endpoint 1 CHAIN.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_chain_max_nump",
                    description: Some(
                        "Number of NUMPs that endpoint 1 can receive.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxChainMaxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_chain_nump",
                    description: Some(
                        "Number of NUMPs received by endpoint 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_dma_ofs",
                    description: Some(
                        "DMA offset length of endpoint 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xda,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_dma",
                    description: Some(
                        "DMA start address of endpoint 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UepRxDma",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "Flash capecity register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_en",
                    description: Some(
                        "Enable DMA.",
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
                    name: "usb_clr_all",
                    description: Some(
                        "Reset all software configuration registers, high efficiency, need software clearing.",
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
                    name: "force_rst",
                    description: Some(
                        "To reset the protocol layer and FIFO module, software clearing is required.",
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
                    name: "dma_mode",
                    description: Some(
                        "When performing burst DPH transmission, the operation of the next packet.",
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
                    name: "setup_flow",
                    description: Some(
                        "SETUP Transaction Flow Control.",
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
                    name: "tx_erdy_mode",
                    description: Some(
                        "Use in device mode.",
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
                    name: "uie_transfer",
                    description: Some(
                        "USB transaction completion interrupt enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udie_setup",
                    description: Some(
                        "Device mode: Receive SETUP transaction to complete interrupt enablement.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udie_status",
                    description: Some(
                        "Device mode: Receive STATUS transaction to complete interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uie_rx_ping",
                    description: Some(
                        "Receive PING-TP to complete the interrupt enablement.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uie_itp",
                    description: Some(
                        "Send ITP to complete interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uie_fifo_txov",
                    description: Some(
                        "Sending FIFO Overflow Interrupt Enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uie_fifo_rxov",
                    description: Some(
                        "Receive FIFO overflow interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dev_addr",
                    description: Some(
                        "The address of the USB device.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itp",
            extends: None,
            description: Some(
                "The Interval register for the ITP package in USB.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reg_itp_interval",
                    description: Some(
                        "The Bus interval Counter field in the received ITP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ItpAdj",
            extends: None,
            description: Some(
                "Interval adaptive registers for ITP packets in USB.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "itp_adj_cr",
                    description: Some(
                        "In device mode, the Bus Interval Adjustment Control field in the received ITP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "itp_delayed",
                    description: Some(
                        "In device mode, the delayed bit of the Link Control Word in the received ITP is 1 when the ITP is delayed.",
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
                    name: "itp_delta",
                    description: Some(
                        "The high 13bit (delta) of ITS in the ITP received in device mode represents the time difference from.  the start of the current ITP packet to the boundary of the previous bus interval.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LinkCfg",
            extends: None,
            description: Some(
                "LINK Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_down_mode",
                    description: Some(
                        "Peripheral Type.",
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
                    name: "link_rx_term_en",
                    description: Some(
                        "Receiving terminal resistance control.",
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
                    name: "link_ss_plr_swap",
                    description: Some(
                        "Swap SSTX and SSRX polarity as follows.",
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
                    name: "link_phy_reset",
                    description: Some(
                        "PIPE Interface Reset.",
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
                    name: "link_compliance_en",
                    description: Some(
                        "POLLING_LFPS whether the timeout enters COMPLIANCE mode.",
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
                    name: "link_lfps_rx_pd",
                    description: Some(
                        "LFPS receive control, where the bit 1 indicates that LFPS receive is disabled.",
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
                    name: "link_rx_eq_en",
                    description: Some(
                        "Receiver equalization enables control, protocol specifics are optional.",
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
                    name: "link_tx_swing",
                    description: Some(
                        "Transmitter signal swing control, low swing power consumption, but affect the transmission distance.",
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
                    name: "link_tx_deemph",
                    description: Some(
                        "Transmitter De-Emphasis control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_cp78_sel",
                    description: Some(
                        "In Compliance Pattern 7/8, send a length of consecutive 0 or consecutive 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_u2_det_en",
                    description: Some(
                        "Detection method of connected devices in U2 state.",
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
                    name: "link_loopback_en",
                    description: Some(
                        "Allow to enter the LOOPBACK enable bit, high efficiency, use with the LOOKBACK enable in TX/RX_TS_CFG [3], both are valid,.  LINK can enter the LOOKBACK mode (used to enable the LOOPBACK slave data loop and error counting, the LOOPBACK host maintains 0).",
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
                    name: "link_lookback_act",
                    description: Some(
                        "Used in LOOPBACK mode for LOOPBACK host control pattern transmission (LOOPBACK slave holds 0),.  ACT is a high-level signal that lasts for a period of time.",
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
                    name: "link_ltssm_mode",
                    description: Some(
                        "The link state machine enters DISABLE mode.",
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
                    name: "link_u1_allow",
                    description: Some(
                        "High efficiency, after receiving the LGO_U1, the response LAU is allowed to enter the U1 state,.  otherwise the response LXU refuses to enter the U1 state after receiving the LGO_U1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_u2_allow",
                    description: Some(
                        "High efficiency, after receiving the LGO_U2,.  the response LAU allows it to enter the U2 state,   otherwise after receiving the LGO_U2, the response LXU refuses to enter the U2 state.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_skp_mode",
                    description: Some(
                        "SKP rejection control for descrambling.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_u1_ping_en",
                    description: Some(
                        "U1 sends PING_FPFS enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_tout_mode",
                    description: Some(
                        "protocol version.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_reset",
                    description: Some(
                        "LINK reset, including the reset status machine and all interrupt flags, is highly effective.",
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
        FieldSet {
            name: "LinkCtrl",
            extends: None,
            description: Some(
                "LINK Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_pd_mode",
                    description: Some(
                        "Configure the current power mode of the PHY, corresponding to PO/P1/P2/P3 in the PIPE.",
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
                    name: "link_go_disabled",
                    description: Some(
                        "RW Set LINK to SS.DISABLED, which is highly effective and requires software to clear it.. If the hardware automatically goes to SS.DIABLED, then the bit is automatically set to 1 by the hardware.  In device mode, if the USB2.0 bus reset is received, the software should first set the PD_MODE to P2 mode, and then clear this bit,  LTSSM will automatically enter the SS.RX_DETECT, the device will be detected 8 times (every 12ms), if no TERM is detected, it will enter the DISABLE state,  and communicate in USB2.0 mode at the same time.  In host mode, a BUS_RESET signal should be sent through the USB2.0 controller, the software should first set the PD_MODE to P2 mode,  then clear this bit, and if a TERM is detected, a POLLING_LFPS handshake will be made.  Otherwise, the software should set the power mode to P3 and communicate in USB2.0 mode.",
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
                    name: "link_go_inactive",
                    description: Some(
                        "Set LINK to SS.INACTIVE, high efficiency, hardware automatic clearing.",
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
                    name: "link_go_recovery",
                    description: Some(
                        "Set the LINK to SS.RECOVERY, which is highly effective and the hardware is automatically cleared.",
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
                    name: "link_go_rx_det",
                    description: Some(
                        "Before setting this bit, the PD_MODE should be set to P2 mode, and the LINK should be set to enter the SS.RX_DETECT,.  and the software query TERM_PRESENT know whether there is a connection or whether the connection is disconnected.   High effectiveness, automatic clearing.",
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
                    name: "link_tx_warm_rst",
                    description: Some(
                        "High effectiveness, cleared by software. This bit is valid and will send WARM_RESET.. In device mode: After detecting that the host has continuously sent WARM_RESET for 18 ms,  the device controller will automatically declare that it has detected WARM_RESET and reset it.",
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
                    name: "link_tx_ux_exit",
                    description: Some(
                        "High efficiency, hardware automatic clearing.",
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
                    name: "link_u3_wkup_en",
                    description: Some(
                        "Receive U3 exti LPFS signal enable.",
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
                    name: "link_polling_en",
                    description: Some(
                        "High validity, if the SS.RX_DETECT detects the TERM, the POLLING handshake will be performed after the software sets the bit to be valid.",
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
                    name: "link_tx_lgo_u1",
                    description: Some(
                        "After the bit is valid, it will send LGO_U1, high efficiency, and the hardware will be automatically cleared.. If the other party responds with LXU, it indicates that it refuses to enter U1 mode;  If the query LINK status is U1, it indicates that the other party accepts to enter the U1 low-power state,  and the software should change the current power mode to P1.",
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
                    name: "link_tx_lgo_u2",
                    description: Some(
                        "After the bit is valid, it will send LGO_U2, high efficiency, and the hardware will be automatically cleared;. If the other party responds with LXU, it indicates that it refuses to enter the U2 state;  If the query LINK status is U2, it indicates that the other party accepts to enter the U2 low-power state,  and the software should change the current power mode to P2.",
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
                    name: "link_tx_lgo_u3",
                    description: Some(
                        "After the bit is valid, it will be sent LGO_U3, high effectiveness, and the hardware will be automatically cleared.",
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
                    name: "link_tx_ts_cfg",
                    description: Some(
                        "Link Configuration for Sending TS1/TS2 Training Sequences.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_rx_ts_cfg",
                    description: Some(
                        "Received link control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LinkIntCtrl",
            extends: None,
            description: Some(
                "LINK Interrupt Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_ie_ready",
                    description: Some(
                        "LINK is initialized, including two ports before (Heade Sequence Number Advertisement).  and (RX Headr Buffer Credit Advertisement) interrupt enabled.",
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
                    name: "link_ie_recovery",
                    description: Some(
                        "Due to the error LINK in RECOVERY interrupt enable.",
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
                    name: "link_ie_inactive",
                    description: Some(
                        "LINK is in INACTIVE interrupt enable.",
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
                    name: "link_ie_disable",
                    description: Some(
                        "LINK in DISABLED interrupt enable.",
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
                    name: "link_ie_go_u3",
                    description: Some(
                        "LINK is in U3 interrupt enable.",
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
                    name: "link_ie_go_u2",
                    description: Some(
                        "LINK is in 2 interrupt enable.",
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
                    name: "link_ie_go_u1",
                    description: Some(
                        "LINK is in U1 interrupt enable.",
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
                    name: "link_ie_go_u0",
                    description: Some(
                        "LINK in U0 interrupt enabled.",
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
                    name: "link_ie_u3_wk_tout",
                    description: Some(
                        "When requesting an exit U3 command timeout interrupt enabled.",
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
                    name: "link_ie_ux_rej",
                    description: Some(
                        "Send LGO_ Ux, receive LXU interrupt enablement that refuses to enter the ux send.",
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
                    name: "link_ie_term_pres",
                    description: Some(
                        "LINK entry RX_DETECT to detect the impedance detection interruption of the trace segment of the remote receiver enables the following functions.",
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
                    name: "link_ie_txeq",
                    description: Some(
                        "LINK into POLLING_RXEQ for receiver equalization training interrupt enablement.",
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
                    name: "link_ie_ux_exit",
                    description: Some(
                        "Receiving a request to exit the UX interrupts the enable.",
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
                    name: "link_ie_warm_rst",
                    description: Some(
                        "Warm reset (unconnected) interrupt enable with LPFS.",
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
                    name: "link_ie_u3_wakeup",
                    description: Some(
                        "In the U3 state, receive the low-frequency periodic signal (LFPS) wake-up interrupt enable.",
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
                    name: "link_ie_hot_rst",
                    description: Some(
                        "HOT RESET, which uses the reset interrupt of the TS1/TS2 ordered set.",
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
                    name: "link_ie_hpbuf_empty",
                    description: Some(
                        "HP_BUF send FIFO null interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_hpbuf_full",
                    description: Some(
                        "HP_BUF send FIFO full interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_compliance",
                    description: Some(
                        "Link entry compliance tests, interrupted with compatibility tests or physical layer conformance tests.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_loopback",
                    description: Some(
                        "Link enters loopback mode for testing and fault isolation interrupt enablement.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_rx_det",
                    description: Some(
                        "Link enters Rx.Detect state interrupt enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_rx_lmp",
                    description: Some(
                        "Receive a link command flag interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_tx_lmp",
                    description: Some(
                        "Successfully entering U0, you can send HP packet interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_rx_lmp_tout",
                    description: Some(
                        "Receive LMP timeout interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_cap_fail_int_en",
                    description: Some(
                        "The CAP of the other party receiving the link cannot be combined with its own CAP to make an interrupt enable of the uplink and downlink ports.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "config_done_int_en",
                    description: Some(
                        "Link receives or sends to PORT_Capability LMP interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_ux_exit_fail",
                    description: Some(
                        "Exit UX failure interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_tx_warmrst",
                    description: Some(
                        "Send WARM_RESET end interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_ux_fail",
                    description: Some(
                        "UX conversion failure interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_u2_tout",
                    description: Some(
                        "U2 timeout interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_u1_tout",
                    description: Some(
                        "U1 Timeout Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_ie_state_chg",
                    description: Some(
                        "Link state machine change flag interrupt enable.",
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
        FieldSet {
            name: "LinkIntFlag",
            extends: None,
            description: Some(
                "LINK Interrupt Flag Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_if_ready",
                    description: Some(
                        "LINK enters the U0 state and completes the LINK initialization interrupt flag.",
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
                    name: "link_if_recovery",
                    description: Some(
                        "LINK enters the SS.RECOVERY status interrupt flag.",
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
                    name: "link_if_inactive",
                    description: Some(
                        "LINK enters the SS.INACTIVE state interrupt flag.",
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
                    name: "link_if_disable",
                    description: Some(
                        "LINK to enter the SS.DISABLE state interrupt flag.",
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
                    name: "link_if_go_u3",
                    description: Some(
                        "LINK enters the U3 state interrupt flag.",
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
                    name: "link_if_go_u2",
                    description: Some(
                        "LINK enters the U2 state interrupt flag.",
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
                    name: "link_if_go_u1",
                    description: Some(
                        "LINK enters the U1 state interrupt flag.",
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
                    name: "link_if_go_u0",
                    description: Some(
                        "LINK enters the U0 state interrupt flag.",
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
                    name: "link_if_u3_wk_tout",
                    description: Some(
                        "Wake up timeout interrupt flag (10ms) from U3.",
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
                    name: "link_if_ux_rej",
                    description: Some(
                        "LINK refuses to enter low power mode (U1/U2) interrupt flag.",
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
                    name: "link_if_term_pres",
                    description: Some(
                        "TERM disconnected or connection interrupted flag detected.",
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
                    name: "link_if_txeq",
                    description: Some(
                        "LINK enters TXEQ state outage flag:. Indicates completion of the POLLING handshake and waits for the software  to set the PWR_MODE to P0 in order to enter the TXEQ stage.",
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
                    name: "link_if_ux_exit",
                    description: Some(
                        "LINK receives the LFPS ready to exit U1/U2/U3 request interrupt flag,.  after this position 1, the software should set the PWR_MODE to P0, after setting the PWR_MODE to P0,   the UX exit timer starts to time, after reaching U1_EXIT_TIMER,   U2_EXIT_TIMER, U3_WAKE_TIMER, LINK exits the Ux state, Go to Recovery.",
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
                    name: "link_if_warm_rst",
                    description: Some(
                        "The WARM_RESET state change (valid-> invalid or invalid->valid).  received by the device interrupted the flag.",
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
                    name: "link_if_wakeup",
                    description: Some(
                        "The power supply is in P3 mode and the LFPS signal interrupt flag is detected.",
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
                    name: "link_if_hot_rst",
                    description: Some(
                        "The device receives the HOT RESET interrupt flag.",
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
                    name: "link_if_hpbuf_empty",
                    description: Some(
                        "Header Packet buffer empty interrupt flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_hpbuf_full",
                    description: Some(
                        "Header Packet buffer full interrupt flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_compliance",
                    description: Some(
                        "LINK enters the COMPLIANCE status interrupt flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_loopback",
                    description: Some(
                        "LINK enters the LOOPBACK state interrupt flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_rx_det",
                    description: Some(
                        "After entering the RX_DETECT state interrupt flag, this position 1,.  the software will set the PWR_MODE to P2, ready for RX_DETECT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_rx_lmp",
                    description: Some(
                        "Receiving LMP interrupt flags.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_tx_lmp",
                    description: Some(
                        "LINK initializes the interrupt flag,.  and the software configures to send Port Capabilities LMP for port capability switching.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_rx_lmp_tout",
                    description: Some(
                        "After LINK initialization is complete,.  the Port Capabilities/Configuration LMPs timeout (20us) interrupt flag is exchanged.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_cap_fail_flag",
                    description: Some(
                        "The CAP of the other party received by the link cannot be combined with its own CAP to form an interrupt.  at the uplink and downlink ports (write 1 to clear it).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "config_done_flag",
                    description: Some(
                        "The link receives or sends to PORT_Capability LMP interrupt (write 1 to zero).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_ux_exit_fail",
                    description: Some(
                        "Exit UX fails to interrupt; (write 1 clear).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_tx_warmrst",
                    description: Some(
                        "Interruption at the end of the WARM_RESET send; (write 1 clear).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_ux_fail",
                    description: Some(
                        "Failed to enter the UX interrupted; (write 1 clear).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_u2_tout",
                    description: Some(
                        "U2 timeout interrupt; (write 1 clear).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_u1_tout",
                    description: Some(
                        "U1 timeout interrupt; (write 1 clear).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_if_state_chg",
                    description: Some(
                        "LINK_RESET reset to 0 (write 1 clear).",
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
        FieldSet {
            name: "LinkIsoDly",
            extends: None,
            description: Some(
                "LINK Synchronous Delay Register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "link_isoch_dly",
                    description: Some(
                        "The delay time for serial bit stream parsing to parallel data is 40ns by default,.  as the device writes the delay information to the register,   which is invalid by 3 bits lower (because the clock frequency is 125MHz, the delay time needs to be set to 8*n).",
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
            ],
        },
        FieldSet {
            name: "LinkItpPre",
            extends: None,
            description: Some(
                "LINK ITP timeout mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "itp_pre",
                    description: Some(
                        "ITP Timeout Mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LinkLmpPortCap",
            extends: None,
            description: Some(
                "PORT_CAP Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auto_cap",
                    description: Some(
                        "When the automatic configuration of Port Capability fails, the bit is automatically cleared.",
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
                    name: "cfg_otg_capable",
                    description: Some(
                        "Configure the OTG bits in the LMP of the port capability of this port; Used for whether to.  go to this OTG function.",
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
                    name: "cfg_down_mode",
                    description: Some(
                        "Configure the Direction low in the LMP of the port capability.",
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
                    name: "cfg_up_mode",
                    description: Some(
                        "Configure the Direction bit in the LMP of the port capability of this port.",
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
                    name: "cfg_tiebreaker",
                    description: Some(
                        "Configure the Tiebreaker bit in the LMP of the Port Capability of this port;. The higher the value, the lower exit.",
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
                    name: "link_down_mode",
                    description: Some(
                        "Decide on your port configuration according to your and the other party's PORT Capability.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_cap_dir",
                    description: Some(
                        "[17]: If the position is 1, it means that the port can be configured as a downbound port;. [18]: If the position is 1, it means that the port can be configured as an upbound port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_cap_otg",
                    description: Some(
                        "OTG bits in LMP that receive Port Capability; Used for whether to go to this OTG function.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_cap_tiebrk",
                    description: Some(
                        "Tiebreaker, used for PORT type quorum when two PORTs (REG_PORT_CAP[17:16]=11) with both uplink and.  downlink capabilities are connected, the port with the higher value becomes the downstream port,   and if the tiebreakers of the two PORTs are equal, the PORT_CAP will be re-exchanged.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_speed",
                    description: Some(
                        "[24] Position 1, indicating that the supported device supports USB3.2 Gen1 (5Gbps).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_lmp_tx_cap_vld",
                    description: Some(
                        "PORT Capability configuration completion flag (software operation, write 1 clear 0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_lmp_rx_cap_vld",
                    description: Some(
                        "Upon receipt of a valid PORT_CAP_LMP, the protocol stipulates that.  the two parties will exchange PORT_CAP_LMP within 20us after entering the U0 state.",
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
        FieldSet {
            name: "LinkLmpRxData0",
            extends: None,
            description: Some(
                "LMP Receive Data 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_lmp_rx_data0",
                    description: Some(
                        "After LMP reception is complete, HP data is stored in this register [31:0].",
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
            name: "LinkLmpRxData1",
            extends: None,
            description: Some(
                "LMP Receives Data 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_lmp_rx_data1",
                    description: Some(
                        "After LMP reception is complete, HP data is stored in this register [63:32].",
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
            name: "LinkLmpRxData2",
            extends: None,
            description: Some(
                "LMP Receives Data 2 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_lmp_rx_data2",
                    description: Some(
                        "After LMP reception is complete, HP data is stored in this register [95:64].",
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
            name: "LinkLmpTxData0",
            extends: None,
            description: Some(
                "USB Custom HP Data 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_lmp_tx_data0",
                    description: Some(
                        "User-defined data sent to HP [31:0].",
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
            name: "LinkLmpTxData1",
            extends: None,
            description: Some(
                "USB Custom HP Data 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_lmp_tx_data1",
                    description: Some(
                        "User-defined data sent to HP [63:32].",
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
            name: "LinkLmpTxData2",
            extends: None,
            description: Some(
                "USB Custom HP Data 2 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_lmp_tx_data2",
                    description: Some(
                        "User-defined data sent to HP [95:64].",
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
            name: "LinkLpmCr",
            extends: None,
            description: Some(
                "Link Power Management Registers.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "rxdet_exp",
                    description: Some(
                        "Detect external device counter registers.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpm_rst",
                    description: Some(
                        "Reset signal for LPM-related registers.",
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
                    name: "lpm_en",
                    description: Some(
                        "CLKLPM is fixed at 12MHz, when LINK_PD_MODE=3 and LPM_EN=0 and OSCOUTEN=0 and PLLALIV=0,.  CLKOSCO will stop and the crystal oscillator will stop.",
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
                    name: "lpm_term_chg",
                    description: Some(
                        "The connected device changes, plugs in the device or unplugs the device.",
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
                    name: "lpm_term_present",
                    description: Some(
                        "The PHY layer detects the device connection.",
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
                    name: "lpm_rxdet_en",
                    description: Some(
                        "When the LPM count reaches the desired value, it is confirmed that a device is connected to the PHY RXDET_EXP this bit is elevated.",
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
                    name: "phy_chsel_auto",
                    description: Some(
                        "The channel type of the PHY layer is automatically selected.",
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
            ],
        },
        FieldSet {
            name: "LinkStatus",
            extends: None,
            description: Some(
                "LINK Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_rx_term_pres",
                    description: Some(
                        "After RX_DETECT, if there is a receiver terminal resistor, the bit is 1.",
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
                    name: "link_rx_warm_rst",
                    description: Some(
                        "Receive a valid WARM_RESET signal from the host.  (the hardware will automatically pull up after receiving the host's WARM_RESET 18ms).",
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
                    name: "link_busy",
                    description: Some(
                        "LINK busy state, the bit is 1 when the switching PD_MODE is completed,.  and the hardware will automatically clear it after the switching is completed.",
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
                    name: "link_ready",
                    description: Some(
                        "When the LINK enters the U0 state, the position 1 will be automatically.  cleared after the initialization (broadcast) is completed,   and the bit will be automatically cleared when it exits the U0 state.",
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
                    name: "link_pd_mode_mask",
                    description: Some(
                        "Link Power Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_txeq",
                    description: Some(
                        "The link is in POLLING_RXEQ.",
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
                    name: "link_state",
                    description: Some(
                        "Link Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_rx_ux_exit_req",
                    description: Some(
                        "Received a request to exit the UX.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_rx_detect",
                    description: Some(
                        "The link is at P2 and is RX_DETECT underway.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_rx_lfps",
                    description: Some(
                        "The link receives an LFPS signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_wakup",
                    description: Some(
                        "A link wake-up signal is received.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_rxdet_sleep_allow",
                    description: Some(
                        "The link is in the RXDET state, allowing sleep.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_u2_sleep_allow",
                    description: Some(
                        "The link is in the U2 state, allowing sleep.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_u3_sleep_allow",
                    description: Some(
                        "The link is in the U3 state, allowing sleep.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_hpbuf_idle",
                    description: Some(
                        "HP_BUF the status of sending FIFO IDLE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_hpbuf_full",
                    description: Some(
                        "HP buffer is full.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_hpbuf_empty",
                    description: Some(
                        "HP buffer is empty.",
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
        FieldSet {
            name: "LinkU1WkupFilter",
            extends: None,
            description: Some(
                "U1 Wake Up LFPS Effective Time Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "u1_wkup_filter",
                    description: Some(
                        "The effective duration of the LFPS received when U1 exits..  When the receiving LFPS reaches this time, the received U1 EXIT can be considered valid, and the transmitted LFPS_LAST is raised, and the handshake is successful, which defaults to 600ns.   U1_WKUP_FILTER [7]: the unit of time that controls U1_WKUP_FILTER [6:0].",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LinkU2InactTimer",
            extends: None,
            description: Some(
                "LINK U2 Inactivity Timeout Counter Threshold Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "u2_inactive_timer",
                    description: Some(
                        "Values of the inactivity timeout counter threshold for U2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LinkU2WkupFilter",
            extends: None,
            description: Some(
                "U2 Wake LFPS Effective Time Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "u2_wkup_filter",
                    description: Some(
                        "When the received LFPS reaches this time, the transmitted LFPS_LAST is raised and the handshake is successful,.  and the received U2 EXIT is considered valid, the duration is (n)us, and the default is 2us.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LinkU3WkupFilter",
            extends: None,
            description: Some(
                "U3 Wake LFPS Effective Time Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "u3_wkup_filter",
                    description: Some(
                        "When the received LFPS reaches this time, the transmitted LFPS_LAST is raised and the handshake is successful,.  and the received U3 EXIT is considered valid, the duration is (n)us, and the default is 100us.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Status",
            extends: None,
            description: Some(
                "USBSS Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uif_transfer",
                    description: Some(
                        "USB transaction completion interrupt flag bit.",
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
                    name: "udif_setup",
                    description: Some(
                        "Device mode: Receives the SETUP transaction completion interrupt flag bit.",
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
                    name: "udif_status",
                    description: Some(
                        "Device mode: Receives the STATUS transaction to complete the interrupt flag.",
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
                    name: "uif_rx_ping",
                    description: Some(
                        "Receive PING-TP completion interrupt flag bit.",
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
                    name: "uif_itp",
                    description: Some(
                        "Send ITP completion interrupt flag bit.",
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
                    name: "uif_fifo_txov",
                    description: Some(
                        "Send FIFO overflow interrupt flag bits.",
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
                    name: "uif_fifo_rxov",
                    description: Some(
                        "Receive FIFO overflow interrupt flag bits.",
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
                    name: "ep_id",
                    description: Some(
                        "If multiple endpoints have an interrupt flag at the same time,.  the priority order is as follows (high-> low): Synchronous transmission > control transmission>block   transmission/interrupted transmission, the smaller the endpoint number of the same endpoint type,   the higher the priority.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep_dir",
                    description: Some(
                        "If the transmission and reception have an interrupt flag.  at the same time, the bit is 1, which prioritizes transmission   (SETUP/STATUS independent interrupt).",
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
            ],
        },
        FieldSet {
            name: "Uep0RxCtrl",
            extends: None,
            description: Some(
                "Endpoint 0 Receives Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ep0_rx_len",
                    description: Some(
                        "The endpoint receives the length register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep0_rx_seq",
                    description: Some(
                        "The serial number that the endpoint expects to receive.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep0_rx_res",
                    description: Some(
                        "Response to DPH.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep0_rx_erdy",
                    description: Some(
                        "The bit writes 1, sends ERDY, and the hardware of the bit is automatically cleared after the ERDY is sent.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep0_rx_pp",
                    description: Some(
                        "The PP bit in the received DPH.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uif_ep0_rx_act",
                    description: Some(
                        "The downstream transaction completes the interrupt flag,.  the software writes 0 to clear zero, and the hardware sets it to 1.",
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
        FieldSet {
            name: "Uep0RxDma",
            extends: None,
            description: Some(
                "Receive Buffer Address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "u3ep0_rx_dma",
                    description: Some(
                        "Endpoint 0 receives the buffer start address.",
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
            name: "Uep0TxCtrl",
            extends: None,
            description: Some(
                "Endpoint 0 Transmit Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ep0_tx_len",
                    description: Some(
                        "The endpoint sends length registers with a maximum value of 512B.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep0_tx_seq",
                    description: Some(
                        "The current sequence number of the endpoint is automatically cleared after receiving the SETUP transaction.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep0_tx_res",
                    description: Some(
                        "Response to ACK-TP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep0_tx_erdy",
                    description: Some(
                        "The bit writes 1, sends ERDY, and the hardware of the bit is automatically cleared after the ERDY is sent.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep0_tx_pp",
                    description: Some(
                        "The PP bit in the received ACK-TP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep0_tx_flow",
                    description: Some(
                        "Flag to complete NRDY-TP send (answer).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uif_ep0_tx_act",
                    description: Some(
                        "The upload transaction is interrupted, the software writes 0 to clear zero, and the hardware sets it to 1.",
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
        FieldSet {
            name: "Uep0TxDma",
            extends: None,
            description: Some(
                "Sending Buffer Address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "u3ep0_tx_dma",
                    description: Some(
                        "Endpoint 0 sends the buffer start address.",
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
            name: "UepRxCfg",
            extends: None,
            description: Some(
                "Endpoint 1 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_iso_mode",
                    description: Some(
                        "1 means that the current endpoint is a synchronous endpoint.",
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
                    name: "ep_rx_seq_auto",
                    description: Some(
                        "Allow the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                    name: "ep_rx_erdy_auto",
                    description: Some(
                        "ERDY automatic mode, the hardware will send ERDY, no software control required;.  The synchronous endpoint recommends clearing the bit.",
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
                    name: "ep_rx_nrdy_mode",
                    description: Some(
                        "This bit is 1, and when a short packet or DPH (PP=0) is received,.  all CHAIN_EN are cleared, which will cause the DPH to respond to NRDY.   Software reconfiguration required.",
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
                    name: "ep_rx_tout_mode",
                    description: Some(
                        "This bit is 1, and when the continuous burst packet is received, PP=1 and not a short packet,.  the DPH is not received after the timeout, resulting in TOUT_IF interrupt.",
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
                    name: "ep_rx_fifo_cfg",
                    description: Some(
                        "The object being operated on when the access offset address is 0xC~0xF.",
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
                    name: "ep_rx_fifo_mode",
                    description: Some(
                        "A bit of 1 indicates that the current endpoint is using the FIFO mode,.  and the start and end addresses of the FIFO should be configured before the bit is 1.",
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
                    name: "ep_rx_chain_auto",
                    description: Some(
                        "CHAIN automatically switches mode, this mode is recommended.",
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
            ],
        },
        FieldSet {
            name: "UepRxChainCr",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_ret_sel",
                    description: Some(
                        "When the FORCE_RET is valid, this bit indicates the returned CHAIN state and configuration.",
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
                    name: "ep_rx_force_ret",
                    description: Some(
                        "This bit effectively forces the return of the selected CHAIN state machine configuration.",
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
                    name: "ep_rx_cur_cfg",
                    description: Some(
                        "The currently configured CHAIN serial number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep_rx_cur_use",
                    description: Some(
                        "The CHAIN serial number currently in use.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRxChainLen",
            extends: None,
            description: Some(
                "The length of the last packet received by the endpoint 1 CHAIN.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_chain_rx_len",
                    description: Some(
                        "The currently completed CHAIN receives the length of the last package.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRxChainMaxNump",
            extends: None,
            description: Some(
                "Number of NUMPs that endpoint 1 can receive.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rx_chain_max_nump",
                    description: Some(
                        "The number of DPP packets that the CHAIN can receive.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs received by endpoint 1.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that the CHAIN has received.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRxChainSt",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_chain_no",
                    description: Some(
                        "The CHAIN serial number that is currently generating the outage.",
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
                    name: "ep_rx_dph_pp",
                    description: Some(
                        "The current status of the PP bit in the DPH.",
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
                    name: "ep_rx_nump_empty",
                    description: Some(
                        "If the NUMP in the current CHAIN is 0, then the position is 1.",
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
                    name: "ep_rx_iso_pkt_err",
                    description: Some(
                        "Only the synchronous downhaul endpoint is used, and there is a CRC32 error in the currently receiving DPP.",
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
                    name: "ep_rx_lpf_flag",
                    description: Some(
                        "Only the synchronous downhaul endpoint is used to receive the LPF status in the DPH currently received.",
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
                    name: "ep_rx_chain_if",
                    description: Some(
                        "The bit is only written, write 1 to release the current CHAIN_IF;. The interrupt occurs when any of the following conditions are met.",
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
                    name: "ep_rx_chain_en",
                    description: Some(
                        "The currently used CHAIN enables the hardware to.  automatically set 1 after the UEP_CHAIN_NUMP register is configured, and the CHAIN is automatically cleared   after the transmission is completed.",
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
            ],
        },
        FieldSet {
            name: "UepRxCr",
            extends: None,
            description: Some(
                "Endpoint 1 Control Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_erdy_nump",
                    description: Some(
                        "The hardware sends the NUMP field of ERDY-TP,.  and the general configuration value is the number of bursts supported.   For example, if you support level 16 bursts, the field is configured to be 16.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep_rx_chain_clr",
                    description: Some(
                        "Write 1 to clear all CHAIN configuration values and statuses.",
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
                    name: "ep_rx_clr",
                    description: Some(
                        "Write 1 to clear all configuration values and states of the endpoint, except for UEP_CFG.",
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
                    name: "ep_rx_halt",
                    description: Some(
                        "Endpoint stop, high efficiency, endpoint stop response STALL to DPH.",
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
            ],
        },
        FieldSet {
            name: "UepRxDma",
            extends: None,
            description: Some(
                "DMA start address of endpoint 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_dma_ba",
                    description: Some(
                        "In normal mode:. The DMA start address of the CHAIN receiving data.  In FIFO mode:  The FIFO start address is 16~23 bits of the mapping address in SRAM;  The FIFO ends 16~23 bits of the mapped address in SRAM.",
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
            name: "UepRxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length of endpoint 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in the CHAIN.",
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
            ],
        },
        FieldSet {
            name: "UepRxEn",
            extends: None,
            description: Some(
                "Endpoint Receive Enable Register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_en",
                    description: Some(
                        "Endpoint 1~15 downpass Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRxSeq",
            extends: None,
            description: Some(
                "Endpoint 1 serial number register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_seq_num",
                    description: Some(
                        "The current serial number of the endpoint is writable in non-SEQ_AUTO mode and read-only in SEQ_AUTO mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRxSt",
            extends: None,
            description: Some(
                "Endpoint 1 state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_chain_en",
                    description: Some(
                        "CHAIN enabled state, corresponding to 4 independent CHAINs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep_rx_chain_res",
                    description: Some(
                        "CHAIN response state, corresponding to 4 independent CHAINs.",
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
                    name: "ep_rx_erdy_req",
                    description: Some(
                        "It indicates that it is currently in the sending ERDY;. Writing 1 to that bit will send ERDY, used in non-ERDY_AUTO mode.",
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
                    name: "ep_rx_fc_st",
                    description: Some(
                        "The endpoint is currently in the flow control state, write 1 to clear it.",
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
                    name: "ep_rx_int_flag",
                    description: Some(
                        "The endpoint is currently interrupted, the bit is read-only, and if all CHAIN_IF is 0, the bit is 0, otherwise it is 1.",
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
            ],
        },
        FieldSet {
            name: "UepTxCfg",
            extends: None,
            description: Some(
                "Endpoint 1 configures the registers.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_iso_mode",
                    description: Some(
                        "1 means that the current endpoint is a synchronous endpoint.",
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
                    name: "ep_tx_seq_auto",
                    description: Some(
                        "Allow the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                    name: "ep_tx_erdy_auto",
                    description: Some(
                        "ERDY automatic mode, the hardware will send ERDY, no software control required;.  The synchronous endpoint recommends clearing the bit.",
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
                    name: "ep_tx_eob_mode",
                    description: Some(
                        "If a short packet is sent, then EOB/LPF in ACK-TP.",
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
                    name: "ep_tx_fifo_cfg",
                    description: Some(
                        "The object being operated on when the access offset address is 0xC~0xF.",
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
                    name: "ep_tx_fifo_mode",
                    description: Some(
                        "A bit of 1 indicates that the current endpoint is using the FIFO mode,.  and the start and end addresses of the FIFO should be configured before the bit is 1.",
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
                    name: "ep_tx_chain_auto",
                    description: Some(
                        "CHAIN automatically switches mode, this mode is recommended.",
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
            ],
        },
        FieldSet {
            name: "UepTxChainCr",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ret_sel",
                    description: Some(
                        "When the FORCE_RET is valid, this bit indicates the returned CHAIN state and configuration.",
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
                    name: "tx_force_ret",
                    description: Some(
                        "This bit effectively forces the return of the selected CHAIN state machine configuration.",
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
                    name: "tx_cur_cfg",
                    description: Some(
                        "The currently configured CHAIN serial number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_cur_use",
                    description: Some(
                        "The CHAIN serial number currently in use.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTxChainExpNump",
            extends: None,
            description: Some(
                "Number of NUMPs expected to be sent by endpoint 1.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_exp_nump",
                    description: Some(
                        "The number of DPP packets that can be sent by the CHAIN that has been completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTxChainLen",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_len",
                    description: Some(
                        "The CHAIN sends the length of the last packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs sent by endpoint 1.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that have been transmitted; When the CHIN_EN is 0,.  the register is automatically cleared at the same time as the UEP_CHAIN_NUMP is written.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTxChainSt",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_no",
                    description: Some(
                        "The CHAIN serial number that is currently generating the outage.",
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
                    name: "tx_dph_pp",
                    description: Some(
                        "The current status of the PP bit in the DPH.",
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
                    name: "tx_nump_empty",
                    description: Some(
                        "If the NUMP in the current CHAIN is 0, then the position is 1.",
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
                    name: "tx_eob_lpf",
                    description: Some(
                        "The EOB/LPF bits in the last pack of DPH of the current CHAIN.",
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
                    name: "tx_chain_if",
                    description: Some(
                        "The bit is only written, write 1 to release the current CHAIN_IF;. The current CHAIN NUMP_EMPTY is 1 to cause this outage.",
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
                    name: "tx_chain_en",
                    description: Some(
                        "The currently used CHAIN enables the hardware to.  automatically set 1 after the UEP_CHAIN_NUMP register is configured, and the CHAIN is automatically cleared   after the transmission is completed.",
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
            ],
        },
        FieldSet {
            name: "UepTxCr",
            extends: None,
            description: Some(
                "Endpoint 1 Control Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_erdy_nump",
                    description: Some(
                        "The hardware sends the NUMP field of ERDY-TP,.  and the general configuration value is the number of bursts supported.   For example, if you support level 16 bursts, the field is configured to be 16.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep_tx_chain_clr",
                    description: Some(
                        "Write 1 to clear all CHAIN configuration values and statuses.",
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
                    name: "ep_tx_clr",
                    description: Some(
                        "Write 1 to clear all configuration values and states of the endpoint, except for UEP_CFG.",
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
                    name: "ep_tx_halt",
                    description: Some(
                        "Endpoint stop, high efficiency, endpoint stop response STALL to DPH.",
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
            ],
        },
        FieldSet {
            name: "UepTxDma",
            extends: None,
            description: Some(
                "DMA start address of endpoint 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_tx_dma",
                    description: Some(
                        "In normal mode:. The DMA start address of the CHAIN sending data.  In FIFO mode:  The FIFO start address is 16~23 bits of the mapping address in SRAM;  The FIFO ends 16~23 bits of the mapped address in SRAM.",
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
            name: "UepTxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length of endpoint 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in the CHAIN.",
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
            ],
        },
        FieldSet {
            name: "UepTxEn",
            extends: None,
            description: Some(
                "Endpoint Send Enable Register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_tx_en",
                    description: Some(
                        "Endpoint 1~15 Upload Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTxSeq",
            extends: None,
            description: Some(
                "Endpoint 1 serial number register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ep_seq_num",
                    description: Some(
                        "The current serial number of the endpoint is writable in non-SEQ_AUTO mode and read-only in SEQ_AUTO mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTxSt",
            extends: None,
            description: Some(
                "Endpoint 1 state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_en",
                    description: Some(
                        "CHAIN enabled state, corresponding to 4 independent CHAINs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_chain_res",
                    description: Some(
                        "CHAIN response state, corresponding to 4 independent CHAINs.",
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
                    name: "tx_ep_erdy_req",
                    description: Some(
                        "It indicates that it is currently in the sending ERDY;. Writing 1 to that bit will send ERDY, used in non-ERDY_AUTO mode.",
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
                    name: "tx_ep_fc_st",
                    description: Some(
                        "The endpoint is currently in the flow control state, write 1 to clear it.",
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
                    name: "tx_ep_int_flag",
                    description: Some(
                        "The endpoint is currently interrupted, the bit is read-only, and if all CHAIN_IF is 0, the bit is 0, otherwise it is 1.",
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
            ],
        },
    ],
    enums: &[],
};
