use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "EpRx",
            extends: None,
            description: Some(
                "Receive (OUT) registers of one device endpoint, 1 to 7. USBSS_EP_RX_TypeDef in the EVT header.",
            ),
            items: &[
                BlockItem {
                    name: "cfg",
                    description: Some(
                        "Endpoint n configuration register (R8_UEPn_RX_CFG).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpRxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Endpoint n control register (R8_UEPn_RX_CR).",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpRxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq",
                    description: Some(
                        "Endpoint n sequence number register (R8_UEPn_RX_SEQ).",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpRxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "st",
                    description: Some(
                        "Endpoint n status register (R8_UEPn_RX_ST).",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpRxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chain_cr",
                    description: Some(
                        "Endpoint n CHAIN control register (R8_UEPn_RX_CHAIN_CR).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpRxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chain_st",
                    description: Some(
                        "Endpoint n CHAIN status register (R8_UEPn_RX_CHAIN_ST).",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpRxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chain_len",
                    description: Some(
                        "Length of the last packet the CHAIN received (R16_UEPn_RX_CHAIN_LEN).",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EpRxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chain_max_nump",
                    description: Some(
                        "Number of DPPs the CHAIN can receive (R8_UEPn_RX_CHAIN_MAX_NUMP).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "chain_nump",
                    description: Some(
                        "Number of DPPs the CHAIN has received (R8_UEPn_RX_CHAIN_NUMP).",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_ofs",
                    description: Some(
                        "Offset of a DPP in the CHAIN, the distance between two packets in the buffer (R16_UEPn_RX_DMA_OFS). Resets to 0x0400.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma",
                    description: Some(
                        "Normal mode: DMA start address of the CHAIN. FIFO mode: FIFO start and end addresses (R32_UEPn_RX_DMA).",
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
            ],
        },
        Block {
            name: "EpTx",
            extends: None,
            description: Some(
                "Transmit (IN) registers of one device endpoint, 1 to 7. USBSS_EP_TX_TypeDef in the EVT header.",
            ),
            items: &[
                BlockItem {
                    name: "cfg",
                    description: Some(
                        "Endpoint n configuration register (R8_UEPn_TX_CFG).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpTxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Endpoint n control register (R8_UEPn_TX_CR).",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpTxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq",
                    description: Some(
                        "Endpoint n sequence number register (R8_UEPn_TX_SEQ).",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpTxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "st",
                    description: Some(
                        "Endpoint n status register (R8_UEPn_TX_ST).",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpTxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chain_cr",
                    description: Some(
                        "Endpoint n CHAIN control register (R8_UEPn_TX_CHAIN_CR).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpTxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chain_st",
                    description: Some(
                        "Endpoint n CHAIN status register (R8_UEPn_TX_CHAIN_ST).",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpTxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chain_len",
                    description: Some(
                        "Length of the last packet the CHAIN sends (R16_UEPn_TX_CHAIN_LEN).",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EpTxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chain_exp_nump",
                    description: Some(
                        "Number of DPPs the CHAIN is to send (R8_UEPn_TX_CHAIN_EXP_NUMP).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "chain_nump",
                    description: Some(
                        "Number of DPPs the CHAIN has sent (R8_UEPn_TX_CHAIN_NUMP).",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_ofs",
                    description: Some(
                        "Offset of a DPP in the CHAIN, the distance between two packets in the buffer (R16_UEPn_TX_DMA_OFS). Resets to 0x0400.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma",
                    description: Some(
                        "Normal mode: DMA start address of the CHAIN. FIFO mode: FIFO start and end addresses (R32_UEPn_TX_DMA).",
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
            ],
        },
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
                        "LINK control registers.",
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
                        "LINK interrupt enable register.",
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
                        "LINK ITP Timeout Mode Register.",
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
                        "U1 wakes up the LFPS Duration Register.",
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
                    name: "link_u1_wkup_tmr",
                    description: Some(
                        "U1 wake-up timer. The RM register table omits this register; the EVT device header exposes it and the device SDK writes 120.",
                    ),
                    array: None,
                    byte_offset: 0x2a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "LinkU1WkupTmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u2_wkup_filter",
                    description: Some(
                        "U2 wakes up the LFPS Duration Register.",
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
                        "U3 wakes up the LFPS validity duration register.",
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
                    name: "link_u3_wkup_tmr",
                    description: Some(
                        "U3 wake-up timer. The RM register table omits this register; the EVT device header exposes it.",
                    ),
                    array: None,
                    byte_offset: 0x32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "LinkU3WkupTmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u1_exit_cfg",
                    description: Some(
                        "U1 exit configuration. The RM register table omits this register; the EVT device header exposes it and the device SDK sets bit 9 before link training.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkU1ExitCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u2_exit_cfg",
                    description: Some(
                        "U2 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkU2ExitCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u3_exit_cfg",
                    description: Some(
                        "U3 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkU3ExitCfg",
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
                    name: "link_tx_route",
                    description: Some(
                        "Hub routing for host transactions. The RM register table omits this register; the EVT host header exposes it and the hub host SDK writes the downstream hub port number to reach a device behind a hub, or 0 for the root hub port.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
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
                        "PORT_CAP Registers.",
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
                    name: "link_lmp_rx_data",
                    description: Some(
                        "Header packet of the last LMP received; word n holds bits [32n+31:32n].",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_tx_data",
                    description: Some(
                        "User-defined header packet to send, such as an LMP; word n holds bits [32n+31:32n].",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "usbss_ctrl",
                    description: Some(
                        "USBSS Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbssCtrl",
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
                        "Interval Registers for ITP packets in USB.",
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
                        "Interval Adaptive Registers for ITP packets in USB.",
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
                    name: "tp_rx_data",
                    description: Some(
                        "Header packet of the last DEV_NOTIF-TP received; word n holds bits [32n+31:32n].",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cfg_cr",
                    description: Some(
                        "Indirect PHY configuration command. The RM does not document this register; the device SDK writes `(1 << 23) | (addr << 16) | data`.",
                    ),
                    array: None,
                    byte_offset: 0x1f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCfgCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cfg_dat",
                    description: Some(
                        "Indirect PHY configuration data. The RM does not document this register; the device SDK reads the PHY response from it.",
                    ),
                    array: None,
                    byte_offset: 0x1fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCfgDat",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "UsbssDevice",
            extends: Some(
                "USBSS",
            ),
            description: Some(
                "USBSS in device mode. Endpoint configuration / TX / RX registers (UEP* family). Offsets overlap with USBSS_HOST host-mode registers; chiptool's overlap check is intra-block only so the two views coexist via the extends mechanism.",
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
                        "LINK control registers.",
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
                        "LINK interrupt enable register.",
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
                        "LINK ITP Timeout Mode Register.",
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
                        "U1 wakes up the LFPS Duration Register.",
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
                    name: "link_u1_wkup_tmr",
                    description: Some(
                        "U1 wake-up timer. The RM register table omits this register; the EVT device header exposes it and the device SDK writes 120.",
                    ),
                    array: None,
                    byte_offset: 0x2a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "LinkU1WkupTmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u2_wkup_filter",
                    description: Some(
                        "U2 wakes up the LFPS Duration Register.",
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
                        "U3 wakes up the LFPS validity duration register.",
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
                    name: "link_u3_wkup_tmr",
                    description: Some(
                        "U3 wake-up timer. The RM register table omits this register; the EVT device header exposes it.",
                    ),
                    array: None,
                    byte_offset: 0x32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "LinkU3WkupTmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u1_exit_cfg",
                    description: Some(
                        "U1 exit configuration. The RM register table omits this register; the EVT device header exposes it and the device SDK sets bit 9 before link training.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkU1ExitCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u2_exit_cfg",
                    description: Some(
                        "U2 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkU2ExitCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u3_exit_cfg",
                    description: Some(
                        "U3 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkU3ExitCfg",
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
                    name: "link_tx_route",
                    description: Some(
                        "Hub routing for host transactions. The RM register table omits this register; the EVT host header exposes it and the hub host SDK writes the downstream hub port number to reach a device behind a hub, or 0 for the root hub port.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
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
                        "PORT_CAP Registers.",
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
                    name: "link_lmp_rx_data",
                    description: Some(
                        "Header packet of the last LMP received; word n holds bits [32n+31:32n].",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_tx_data",
                    description: Some(
                        "User-defined header packet to send, such as an LMP; word n holds bits [32n+31:32n].",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "usbss_ctrl",
                    description: Some(
                        "USBSS Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbssCtrl",
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
                        "Interval Registers for ITP packets in USB.",
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
                        "Interval Adaptive Registers for ITP packets in USB.",
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
                        "Endpoint Sends Enable Register.",
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
                        "Endpoint 0 sends control registers.",
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
                        "Endpoint 0 receives control registers.",
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
                        "Start address of the endpoint 0 transmit buffer.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_rx_dma",
                    description: Some(
                        "Start address of the endpoint 0 receive buffer.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_tx_dma_ofs",
                    description: Some(
                        "Endpoint 0 transmit DMA offset. The RM register table omits this register; the EVT device header exposes it.",
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
                        "Endpoint 0 receive DMA offset. The RM register table omits this register; the EVT device header exposes it.",
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
                    name: "tp_rx_data",
                    description: Some(
                        "Header packet of the last DEV_NOTIF-TP received; word n holds bits [32n+31:32n].",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ep_tx",
                    description: Some(
                        "Transmit (IN) registers of endpoints 1 to 7. Index 0 is endpoint 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "EpTx",
                        },
                    ),
                },
                BlockItem {
                    name: "ep_rx",
                    description: Some(
                        "Receive (OUT) registers of endpoints 1 to 7. Index 0 is endpoint 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "EpRx",
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cfg_cr",
                    description: Some(
                        "Indirect PHY configuration command. The RM does not document this register; the device SDK writes `(1 << 23) | (addr << 16) | data`.",
                    ),
                    array: None,
                    byte_offset: 0x1f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCfgCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cfg_dat",
                    description: Some(
                        "Indirect PHY configuration data. The RM does not document this register; the device SDK reads the PHY response from it.",
                    ),
                    array: None,
                    byte_offset: 0x1fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCfgDat",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "UsbssHost",
            extends: Some(
                "USBSS",
            ),
            description: Some(
                "USBSS in host mode. UH_* / HOST_* registers. Offsets overlap with USBSS_DEVICE.",
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
                        "LINK control registers.",
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
                        "LINK interrupt enable register.",
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
                        "LINK ITP Timeout Mode Register.",
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
                        "U1 wakes up the LFPS Duration Register.",
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
                    name: "link_u1_wkup_tmr",
                    description: Some(
                        "U1 wake-up timer. The RM register table omits this register; the EVT device header exposes it and the device SDK writes 120.",
                    ),
                    array: None,
                    byte_offset: 0x2a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "LinkU1WkupTmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u2_wkup_filter",
                    description: Some(
                        "U2 wakes up the LFPS Duration Register.",
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
                        "U3 wakes up the LFPS validity duration register.",
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
                    name: "link_u3_wkup_tmr",
                    description: Some(
                        "U3 wake-up timer. The RM register table omits this register; the EVT device header exposes it.",
                    ),
                    array: None,
                    byte_offset: 0x32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "LinkU3WkupTmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u1_exit_cfg",
                    description: Some(
                        "U1 exit configuration. The RM register table omits this register; the EVT device header exposes it and the device SDK sets bit 9 before link training.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkU1ExitCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u2_exit_cfg",
                    description: Some(
                        "U2 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkU2ExitCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "link_u3_exit_cfg",
                    description: Some(
                        "U3 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LinkU3ExitCfg",
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
                    name: "link_tx_route",
                    description: Some(
                        "Hub routing for host transactions. The RM register table omits this register; the EVT host header exposes it and the hub host SDK writes the downstream hub port number to reach a device behind a hub, or 0 for the root hub port.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
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
                        "PORT_CAP Registers.",
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
                    name: "link_lmp_rx_data",
                    description: Some(
                        "Header packet of the last LMP received; word n holds bits [32n+31:32n].",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "link_lmp_tx_data",
                    description: Some(
                        "User-defined header packet to send, such as an LMP; word n holds bits [32n+31:32n].",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "usbss_ctrl",
                    description: Some(
                        "USBSS Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbssCtrl",
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
                        "Interval Registers for ITP packets in USB.",
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
                        "Interval Adaptive Registers for ITP packets in USB.",
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
                        "Endpoint Transmit Enable Register. The RM lists it only among the device registers; the EVT host header exposes it and the host SDK sets bit 1.",
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
                        "Endpoint Receive Enable Register. The RM lists it only among the device registers; the EVT host header exposes it and the host SDK sets bit 1.",
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
                    name: "uh_tx_ctrl",
                    description: Some(
                        "Host Transmit Control Registers.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhTxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_ctrl",
                    description: Some(
                        "Host receives control registers.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhRxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_tx_dma",
                    description: Some(
                        "Send buffer address registers.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhTxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_dma",
                    description: Some(
                        "Receive buffer address registers.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhRxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_tx_dma_ofs",
                    description: Some(
                        "Host Transmit Address Offset Register.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhTxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_dma_ofs",
                    description: Some(
                        "Host Receive Address Offset Register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhRxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "host_tx_nump",
                    description: Some(
                        "The host transmits the NUMP register.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "HostTxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "host_rx_nump",
                    description: Some(
                        "The host receives the NUMP register.",
                    ),
                    array: None,
                    byte_offset: 0x9e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "HostRxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "host_status",
                    description: Some(
                        "Host Status Registers.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HostStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "host_tx_fc_status",
                    description: Some(
                        "The host endpoint sends flow control register.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "HostTxFcStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "host_rx_fc_status",
                    description: Some(
                        "The host endpoint receives the flow control register.",
                    ),
                    array: None,
                    byte_offset: 0xa6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "HostRxFcStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tp_rx_data",
                    description: Some(
                        "Header packet of the last DEV_NOTIF-TP received; word n holds bits [32n+31:32n].",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cfg_cr",
                    description: Some(
                        "Indirect PHY configuration command. The RM does not document this register; the device SDK writes `(1 << 23) | (addr << 16) | data`.",
                    ),
                    array: None,
                    byte_offset: 0x1f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCfgCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cfg_dat",
                    description: Some(
                        "Indirect PHY configuration data. The RM does not document this register; the device SDK reads the PHY response from it.",
                    ),
                    array: None,
                    byte_offset: 0x1fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCfgDat",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "EpRxCfg",
            extends: None,
            description: Some(
                "Endpoint n receive configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "iso_mode",
                    description: Some(
                        "A value of 1 indicates that the current endpoint is a synchronous endpoint.",
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
                    name: "seq_auto",
                    description: Some(
                        "1: software cannot write SEQ_NUM, hardware maintains it. 0: software may write SEQ_NUM.",
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
                    name: "erdy_auto",
                    description: Some(
                        "ERDY automatic mode, the hardware sends ERDY without software control. The RM recommends clearing this bit on a synchronous endpoint.",
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
                    name: "nrdy_mode",
                    description: Some(
                        "After a short packet or a DPH with PP=0, clear every CHAIN_EN, so that later DPHs get NRDY until software configures a CHAIN again. RB_EP_RX_NRDY_MODE in the RM; the EVT header still calls it USBSS_EP_RX_EOB_MODE.",
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
                    name: "tout_mode",
                    description: Some(
                        "After a burst packet with PP=1 that is not a short packet, raise the TOUT_IF interrupt when no DPH follows within the timeout.",
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
                    name: "fifo_cfg",
                    description: Some(
                        "Access offset address 0xC~0xF, and the operation object.",
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
                    name: "fifo_mode",
                    description: Some(
                        "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1.",
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
                    name: "chain_auto",
                    description: Some(
                        "CHAIN automatically switches modes, and this mode is recommended.",
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
            name: "EpRxChainCr",
            extends: None,
            description: Some(
                "Endpoint n receive CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ret_sel",
                    description: Some(
                        "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration.",
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
                    name: "force_ret",
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
                    name: "cur_cfg",
                    description: Some(
                        "The chain serial number of the current configuration.",
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
                    name: "cur_use",
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
            name: "EpRxChainLen",
            extends: None,
            description: Some(
                "Endpoint n CHAIN receive packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "len",
                    description: Some(
                        "The length of the last packet received by the CHAIN.",
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
            name: "EpRxChainSt",
            extends: None,
            description: Some(
                "Endpoint n receive CHAIN status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "chain_no",
                    description: Some(
                        "The serial number of the CHAIN that is currently interrupting.",
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
                    name: "dph_pp",
                    description: Some(
                        "The status of the PP bits in the currently received DPH.",
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
                    name: "nump_empty",
                    description: Some(
                        "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1.",
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
                    name: "iso_pkt_err",
                    description: Some(
                        "Synchronous OUT endpoints only. The DPP just received has a CRC32 error.",
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
                    name: "lpf_flag",
                    description: Some(
                        "Synchronous OUT endpoints only. LPF bit of the DPH just received.",
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
                    name: "chain_if",
                    description: Some(
                        "This bit is only written, and 1 is written to release the current CHAIN_IF.",
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
                    name: "chain_en",
                    description: Some(
                        "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed.",
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
            name: "EpRxCr",
            extends: None,
            description: Some(
                "Endpoint n receive control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "erdy_nump",
                    description: Some(
                        "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts.  For example, if you support 16 bursts, the value of this field is 16.",
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
                    name: "chain_clr",
                    description: Some(
                        "Write 1 clears all CHAIN configuration values and statuses.",
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
                    name: "clr",
                    description: Some(
                        "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG.",
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
                    name: "halt",
                    description: Some(
                        "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH.",
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
            name: "EpRxSeq",
            extends: None,
            description: Some(
                "Endpoint n receive sequence number register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "seq_num",
                    description: Some(
                        "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode,  read-only in SEQ_AUTO mode.",
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
            name: "EpRxSt",
            extends: None,
            description: Some(
                "Endpoint n receive status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "chain_en",
                    description: Some(
                        "The CHAIN enabled state, which corresponds to 4 independent CHAINS.",
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
                    name: "chain_res",
                    description: Some(
                        "CHAIN response state, corresponding to 4 separate CHAINS.",
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
                    name: "erdy_req",
                    description: Some(
                        "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode.",
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
                    name: "fc_st",
                    description: Some(
                        "The endpoint is in the current throttling state, and write 1 is cleared to zero.",
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
                    name: "int_flag",
                    description: Some(
                        "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1.",
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
            name: "EpTxCfg",
            extends: None,
            description: Some(
                "Endpoint n transmit configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "iso_mode",
                    description: Some(
                        "A value of 1 indicates that the current endpoint is a synchronous endpoint.",
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
                    name: "seq_auto",
                    description: Some(
                        "1: software cannot write SEQ_NUM, hardware maintains it. 0: software may write SEQ_NUM.",
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
                    name: "erdy_auto",
                    description: Some(
                        "ERDY automatic mode, the hardware sends ERDY without software control. The RM recommends clearing this bit on a synchronous endpoint.",
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
                    name: "eob_mode",
                    description: Some(
                        "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP.",
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
                    name: "fifo_cfg",
                    description: Some(
                        "Access offset address 0xC~0xF, and the operation object.",
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
                    name: "fifo_mode",
                    description: Some(
                        "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1.",
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
                    name: "chain_auto",
                    description: Some(
                        "CHAIN automatically switches modes, and this mode is recommended.",
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
            name: "EpTxChainCr",
            extends: None,
            description: Some(
                "Endpoint n transmit CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ret_sel",
                    description: Some(
                        "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration.",
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
                    name: "force_ret",
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
                    name: "cur_cfg",
                    description: Some(
                        "The chain serial number of the current configuration.",
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
                    name: "cur_use",
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
            name: "EpTxChainLen",
            extends: None,
            description: Some(
                "Endpoint n CHAIN transmit packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "len",
                    description: Some(
                        "The length of the last packet sent by the CHAIN.",
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
            name: "EpTxChainSt",
            extends: None,
            description: Some(
                "Endpoint n transmit CHAIN status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "chain_no",
                    description: Some(
                        "The serial number of the CHAIN that is currently interrupting.",
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
                    name: "dph_pp",
                    description: Some(
                        "The status of the PP bits in the currently received DPH.",
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
                    name: "nump_empty",
                    description: Some(
                        "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1.",
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
                    name: "eob_lpf",
                    description: Some(
                        "EOB/LPF bits in the last packet of DPH in the current CHAIN.",
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
                    name: "chain_if",
                    description: Some(
                        "This bit is only written, and 1 is written to release the current CHAIN_IF.",
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
                    name: "chain_en",
                    description: Some(
                        "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed.",
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
            name: "EpTxCr",
            extends: None,
            description: Some(
                "Endpoint n transmit control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "erdy_nump",
                    description: Some(
                        "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts.  For example, if you support 16 bursts, the value of this field is 16.",
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
                    name: "chain_clr",
                    description: Some(
                        "Write 1 clears all CHAIN configuration values and statuses.",
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
                    name: "clr",
                    description: Some(
                        "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG.",
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
                    name: "halt",
                    description: Some(
                        "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH.",
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
            name: "EpTxSeq",
            extends: None,
            description: Some(
                "Endpoint n transmit sequence number register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "seq_num",
                    description: Some(
                        "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode,  read-only in SEQ_AUTO mode.",
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
            name: "EpTxSt",
            extends: None,
            description: Some(
                "Endpoint n transmit status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "chain_en",
                    description: Some(
                        "The CHAIN enabled state, which corresponds to 4 independent CHAINS.",
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
                    name: "chain_res",
                    description: Some(
                        "CHAIN response state, corresponding to 4 separate CHAINS.",
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
                    name: "erdy_req",
                    description: Some(
                        "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode.",
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
                    name: "fc_st",
                    description: Some(
                        "The endpoint is in the current throttling state, and write 1 is cleared to zero.",
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
                    name: "int_flag",
                    description: Some(
                        "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1.",
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
            name: "HostRxFcStatus",
            extends: None,
            description: Some(
                "The host endpoint receives the flow control register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_fc",
                    description: Some(
                        "The flow control status of the host to receiving endpoints 1-15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HostRxNump",
            extends: None,
            description: Some(
                "The host receives the NUMP register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uh_rx_nump",
                    description: Some(
                        "The number of packets that the host expects to receive,. if it is a synchronous transmission, will automatically subtract 1 from the hardware for each packet it receives.",
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
                    name: "uh_rx_dpp_num",
                    description: Some(
                        "The number of DPPs that have been accepted by the host.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HostStatus",
            extends: None,
            description: Some(
                "Host Status Registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_erdy_ep",
                    description: Some(
                        "ERDY is received from the device, and the segment represents the endpoint number of the device ERDY.",
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
                    name: "uh_rx_erdy_nump",
                    description: Some(
                        "ERDY received from the device,. which indicates the number of packets that can be sent/received by the device's corresponding endpoint.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uh_rx_erdy_dir",
                    description: Some(
                        "ERDY is received from the device, and the segment represents the direction of the endpoint.",
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
                    name: "uh_rx_eob_lpf",
                    description: Some(
                        "This bit represents the EOB/LPF status in the received packet.",
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
                    name: "uh_rx_iso_pkt_err",
                    description: Some(
                        "A CRC error was received for the packet (DPP) during synchronous transmission.",
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
                    name: "uh_itp_presage",
                    description: Some(
                        "In host mode, this bit indicates the time when the ITP packet was sent.",
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
            ],
        },
        FieldSet {
            name: "HostTxFcStatus",
            extends: None,
            description: Some(
                "The host endpoint sends flow control register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_tx_fc",
                    description: Some(
                        "The flow control status of the host to send endpoints 1-15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HostTxNump",
            extends: None,
            description: Some(
                "The host transmits the NUMP register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uh_tx_nump",
                    description: Some(
                        "The number of packets that the host expects to send; the hardware subtracts 1 for each packet it sends.",
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
            name: "Itp",
            extends: None,
            description: Some(
                "Interval Registers for ITP packets in USB.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reg_itp_interval",
                    description: Some(
                        "Bus interval Counter field in the received ITP.",
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
                "Interval Adaptive Registers for ITP packets in USB.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "itp_adj_cr",
                    description: Some(
                        "In device mode, the Bus Interval Adjustment Control field in the received ITP should be 0 after power-on reset or device disconnection.",
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
                        "In device mode, the Delayed bit of Link Control Word in the received ITP is 1 when the ITP is delayed.",
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
                        "The higher 13-bit delta of the ITS in. the ITP received in device mode indicates the time difference from the start of the current  ITP packet to the previous bus interval boundary.",
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
                        "Receiver Termination Resistance Control.",
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
                        "Exchange SSTX and SSRX polarities as follows.",
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
                        "The PIPE interface is reset.",
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
                        "POLLING_LFPS timeout is in COMPLIANCE mode.",
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
                        "LFPS receive control, this bit is 1 to disable LFPS reception.",
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
                        "Receiver equalization enable control, optional protocol specifications.",
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
                        "The transmitter signal swing control, low swing power consumption, but affect the transmission distance.",
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
                        "Transmitter de-emphasis control, -3.5 dB after reset. The RM allows only -6.0 dB and -3.5 dB in normal operation; the device SDK writes the reserved value 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "TxDeemph",
                    ),
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
                        "Detection mode of connected devices in U2 state.",
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
                        "The LOOPBACK enable bit is allowed, which is highly active, and can be used with the LOOKBACK enable in TX/RX_TS_CFG [3],. both of which are valid before LINK can enter the LOOKBACK mode  (used to enable the data loopback and error counting of the LOOPBACK slave,  and the LOOPBACK master remains 0).",
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
                    name: "link_loopback_act",
                    description: Some(
                        "It is used in LOOPBACK mode for the LOOPBACK master to control the pattern transmission (the LOOPBACK slave keeps 0),. and the ACT is a high-level signal that lasts for a period of time.",
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
                        "High validity, after receiving the LGO_U1, the response LAU allows to enter the U1 state,. otherwise, after receiving the LGO_U1, the response LXU refuses to enter the U1 state.",
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
                        "High validity, after receiving the LGO_U2, the response LAU allows to enter the U2 state,. otherwise after receiving the LGO_U2, the response LXU refuses to enter the U2 state.",
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
                        "SKP discard control for descrambling. 0: a byte pair is not descrambled when its low byte is SKP; 1 or 3: when both of its bytes are SKP; 2: when either of its bytes is SKP.",
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
                        "Send PING_FPFS under U1 to enable.",
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
                        "Link timer values. 1: USB 3.2 (PM_LC_TIMER 4 us, PM_ENTRY_TIMER 8 us, PENDING_HP_TIMER 10 us); 0: USB 3.0 (3 us, 6 us, 3 us).",
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
                    name: "link_force_polling",
                    description: Some(
                        "Not in the RM, which marks bits 30:22 reserved. Defined by the EVT header; the SDK examples do not use it.",
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
                    name: "link_force_rxterm",
                    description: Some(
                        "Not in the RM, which marks bits 30:22 reserved. Defined by the EVT header; the SDK examples do not use it.",
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
                    name: "link_reset",
                    description: Some(
                        "LINK reset, including the reset state machine and all interrupt flags, is highly effective.",
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
                "LINK control registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_pd_mode",
                    description: Some(
                        "Configure the current power mode of the PHY, corresponding to P0/P1/P2/P3 in the PIPE. P3 after reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "PdMode",
                    ),
                },
                Field {
                    name: "link_go_disabled",
                    description: Some(
                        "SET LINK TO ENTER SS.DISABLED, WHICH IS HIGHLY VALID,. AND REQUIRES SOFTWARE TO CLEAR.",
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
                        "SET THE LINK TO ENTER SS.INACTIVE,. WHICH IS HIGHLY VALID, AND THE HARDWARE IS AUTOMATICALLY CLEARED.",
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
                        "SET THE LINK TO ENTER SS.RECOVERY,. WHICH IS HIGHLY EFFECTIVE, AND THE HARDWARE IS AUTOMATICALLY CLEARED.",
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
                        "Before setting this bit, the PD_MODE should be set to P2 mode,. and the LINK should be set to enter the SS.RX_DETECT,  and the software will query TERM_PRESENT to know whether there is a connection or whether the connection is disconnected.  High effectiveness, automatic clearing.",
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
                        "High validity, cleared by software. A valid bit will send a warm-reset.",
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
                        "High validity, hardware automatic zeroing.",
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
                    name: "link_lup_ldn_en",
                    description: Some(
                        "In the U0 state, if there is no data, whether to send LUP and LDN packets every 10us.",
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
                    name: "link_reg_rout_en",
                    description: Some(
                        "Enable the routing function of the HUB, which is highly effective, with registers and no interfaces.",
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
                    name: "link_polling_en",
                    description: Some(
                        "If the TERM is detected by the SS.RX_DETECT,. a POLLING handshake will be performed after the software sets the bit to be valid.",
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
                        "After the bit is valid, the LGO_U1 is sent, the high is active, and the hardware is automatically cleared.",
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
                        "After the bit is valid, the LGO_U2 is sent, the high is active, and the hardware is automatically cleared.",
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
                        "After the bit is valid, the LGO_U3 is transmitted, and the hardware is automatically cleared.",
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
                        "Send the link configuration of the TS1/TS2 training sequence.",
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
                        "Received Link Control.",
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
                "LINK interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_ie_ready",
                    description: Some(
                        "LINK is initialized, including two ports before (Header Sequence Number Advertisement). and (RX Headr Buffer Credit Advertisement) interrupt enabled.",
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
                        "DUE TO ERROR LINK IN RECOVERY INTERRUPTED ENABLED.",
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
                        "LINK is enabled on INACTIVE interrupt.",
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
                        "LINK is disabled, interrupt enabled.",
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
                        "LINK is enabled when the U3 interrupt is enabled.",
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
                        "LINK is interrupted at U2 and enabled.",
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
                        "LINK is interrupted at U1 to enable the following.",
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
                        "LINK is interrupted at U0 to enable the following.",
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
                        "Interrupt the U3 command when requesting to exit U3 command timeout.",
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
                        "Transmit LGO_ Ux, receive LXU interrupt enabled that refuses to enter the Ux.",
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
                        "LINK enters the RX_DETECT to detect the impedance of the remote receiver trace segment and detect the interrupt enabled.",
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
                        "LINK enters the POLLING_RXEQ for receiver equalization training interrupt enable.",
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
                        "Request to exit UX is received Interrupt Enable.",
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
                        "Warm reset (not connected) interrupt enabled with LPFS.",
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
                        "In the U3 state, the Low Frequency Periodic Signal (LFPS) is received to wake up interrupt enabled.",
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
                        "hot reset, which uses the reset interrupt enable of the TS1/TS2 ordered set.",
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
                        "BUF sends FIFO null interrupt enable.",
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
                        "BUF sends FIFO full interrupt enable.",
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
                    name: "link_ie_rx_set_fc",
                    description: Some(
                        "EVT header name for bit 17, which the RM documents as LINK_IE_HPBUF_FULL. The device SDK enables it on chip revision 3 and later only.",
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
                        "The link enters the compliance test,. and the interrupt is enabled by the compatibility test or the physical layer conformance test.",
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
                        "The link goes into loopback mode for testing and error isolation interrupt enablement.",
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
                        "The link enters the Rx.Detect state and is interrupted.",
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
                        "Received Link Command Flag Interrupt Enabled.",
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
                        "If you successfully enter U0, you can send an HP packet to interrupt enable.",
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
                        "Receive LMP Timeout Interrupt Enabled.",
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
                    name: "link_ie_ux_exit_fail",
                    description: Some(
                        "Exit UX failed to interrupt enabled.",
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
                        "Send warm_reset end interrupt enabled.",
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
                        "UX Conversion Failure Interrupt Enabled.",
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
                        "U2 Timeout Interrupt Enabled.",
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
                        "U1 Timeout Interrupt Enabled.",
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
                        "Link State Machine Change Flag Interrupt Enabled.",
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
                        "LINK enters the U0 state and completes LINK initialization of the interrupt flag.",
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
                        "LINK INTO THE SS.RECOVERY STATE INTERRUPT FLAG.",
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
                        "LINK ENTERS THE SS.INACTIVE STATE INTERRUPT FLAG.",
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
                        "LINK INTO SS.DISABLE STATE INTERRUPT FLAG.",
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
                        "Wake up from U3 to timeout interrupt flag (10ms).",
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
                        "LINK refuses to enter the low-power mode (U1/U2) interrupt flag.",
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
                        "The TERM disconnected or disconnected flag was detected.",
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
                        "LINK enters TXEQ state interrupt flag:. INDICATE THAT THE POLLING HANDSHAKE IS COMPLETE,  AND WAIT FOR THE SOFTWARE TO SET THE PWR_MODE TO P0 IN ORDER TO ENTER THE TXEQ PHASE.",
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
                        "LINK receives the LFPS ready to exit U1/U2/U3 request interrupt flag.",
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
                        "The WARM RESET status change (active-> inactive,. or invalid->active) interrupt flag received by the device.",
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
                    name: "link_if_rx_set_fc",
                    description: Some(
                        "EVT header name for bit 17, which the RM documents as LINK_IF_HPBUF_FULL. On chip revision 3 and later the device SDK takes it as a received Set Link Function LMP and then reads FORCE_PM in LINK_LMP_PORT_CAP.",
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
                        "LINK enters the COMPLIANCE state with an outage flag.",
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
                        "LINK enters the LOOPBACK state and the LINK is interrupted.",
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
                        "After entering the RX_DETECT state interrupt flag, this position 1,. the software sets the PWR_MODE to P2, ready for RX_DETECT.",
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
                        "Receive LMP Interrupt Flag.",
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
                        "After the LINK initialization is completed,. the software is configured to send Port Capabilities LMP for port capability switching.",
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
                        "After the LINK initialization is complete,. the Port Capabilities/Configuration LMPs timeout (20us) interrupt flag is exchanged.",
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
                    name: "link_if_ux_exit_fail",
                    description: Some(
                        "Exit UX fails to interrupt.",
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
                        "send warm_reset to end interrupt.",
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
                        "Enter the Ux failed to interrupt.",
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
                        "U2 timeout interrupted.",
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
                        "U1 timeout interrupted.",
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
                        "Link state machine change flag. Write 1 to clear; LINK_RESET also clears it.",
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
                        "The delay time for the serial bitstream to parse to the parallel data is 40ns by default,. and the SET ISOCH DELAY host sends this information to the device when enumerated,  and writes the delay information to the register as the device, and the lower 3 bits of the register are invalid  (because the clock frequency is 125MHz, the delay time needs to be set to 8*n).",
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
                "LINK ITP Timeout Mode Register.",
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
                "PORT_CAP Registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_reg_port_cap",
                    description: Some(
                        "link port configure.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
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
                    name: "force_pm",
                    description: Some(
                        "Force_LinkPM_Accept of the received Set Link Function LMP. Not in the RM, where bit 29 is part of LINK_SPEED. Defined by the EVT header; the device SDK reads it after LINK_IF_RX_SET_FC on chip revision 3 and later.",
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
                    name: "link_lmp_tx_cap_vld",
                    description: Some(
                        "PORT Capability configuration completion flag.",
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
                        "A valid PORT_CAP-LMP is received, and the protocol stipulates that. the two LINK parties exchange PORT_CAP-LMP within 20 microseconds after entering the U0 state.",
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
                        "Detects external device counter registers.",
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
                        "Reset signal for LPM related register.",
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
                        "LPM Enable.",
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
                        "The PHY layer detects a device connection.",
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
                        "When the lpm count reaches the expected value RXDET_EXP it is confirmed that. a device is connected to the PHY, this bit is raised.",
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
                        "Automatic selection of the PHY layer transceiver channel type.",
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
                        "After RX_DETECT, if a receive termination resistor is present, the bit is 1.",
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
                        "A valid warm-reset signal is received from the host. (the hardware automatically pulls up after receiving the host's warm_reset 18ms).",
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
                        "When the LINK is busy, the bit is 1 when the switchover is PD_MODE,. and the hardware will automatically clear the link after the switchover is completed.",
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
                        "When the LINK enters the U0 state, the position 1 exits the U0 state after the initialization. (broadcast) is completed, and the bit is automatically cleared.",
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
                        "Current power mode of the PHY.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "PdMode",
                    ),
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
                        "Link state. DISABLED after reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "LinkState",
                    ),
                },
                Field {
                    name: "link_rx_ux_exit_req",
                    description: Some(
                        "Received a request to exit Ux.",
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
                        "The link is at P2 and is in rx_detect.",
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
                        "The link is in the U2 state and sleep is allowed.",
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
                        "The link is in the U3 state and sleep is allowed.",
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
                        "BUF sends the status of the FIFO IDLE.",
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
                        "The HP buffer is full.",
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
                        "The HP buffer is empty.",
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
            name: "LinkU1ExitCfg",
            extends: None,
            description: Some(
                "U1 exit configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "u1_exit_time",
                    description: Some(
                        "U1 exit time enable. The device SDK sets this bit before link training.",
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
            ],
        },
        FieldSet {
            name: "LinkU1WkupFilter",
            extends: None,
            description: Some(
                "U1 wakes up the LFPS Duration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "u1_wkup_filter",
                    description: Some(
                        "The duration of the LFPS received by U1 when exiting.. When the receiving LFPS reaches this time, the received U1 EXIT is considered valid,  and the handshake is successful when the transmitting LFPS_last is raised, which is 600ns by default  U1_WKUP_FILTER [7]: the unit of time that controls the U1_WKUP_FILTER [6:0].",
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
            name: "LinkU1WkupTmr",
            extends: None,
            description: Some(
                "U1 wake-up timer.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "u1_wkup_tmr",
                    description: Some(
                        "U1 wake-up timer value. The device SDK writes 120.",
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
            name: "LinkU2ExitCfg",
            extends: None,
            description: Some(
                "U2 exit configuration. The EVT header gives no bit definitions.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exit_cfg",
                    description: Some(
                        "Undocumented configuration value.",
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
                        "The value of the inactivity timeout counter threshold for U2.",
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
                "U2 wakes up the LFPS Duration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "u2_wkup_filter",
                    description: Some(
                        "When the receiving LFPS reaches this time, the sending LFPS_last is pulled up and the handshake is successful,. and the received U2 EXIT can be considered valid with the duration of (n)us, the default is 2us.",
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
            name: "LinkU3ExitCfg",
            extends: None,
            description: Some(
                "U3 exit configuration. The EVT header gives no bit definitions.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exit_cfg",
                    description: Some(
                        "Undocumented configuration value.",
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
            name: "LinkU3WkupFilter",
            extends: None,
            description: Some(
                "U3 wakes up the LFPS validity duration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "u3_wkup_filter",
                    description: Some(
                        "When the receiving LFPS reaches this time, the LFPS_last is raised, and the handshake is successful,. and the received U3 EXIT is considered valid for (n)us, with a default of 100us.",
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
            name: "LinkU3WkupTmr",
            extends: None,
            description: Some(
                "U3 wake-up timer.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "u3_wkup_tmr",
                    description: Some(
                        "U3 wake-up timer value.",
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
            name: "PhyCfgCr",
            extends: None,
            description: Some(
                "Indirect PHY configuration command.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_cfg_data",
                    description: Some(
                        "Data written to the addressed PHY register.",
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
                    name: "phy_cfg_addr",
                    description: Some(
                        "PHY register address.",
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
                    name: "phy_cfg_write",
                    description: Some(
                        "Command strobe. The device SDK always sets it together with the address and data.",
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
            ],
        },
        FieldSet {
            name: "PhyCfgDat",
            extends: None,
            description: Some(
                "Indirect PHY configuration data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_cfg_data",
                    description: Some(
                        "Data read back from the PHY.",
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
                        "USB Transaction Completion Interrupt Flag.",
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
                        "Receive SETUP Transaction Completion Interrupt Flag.",
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
                    name: "uhif_erdy",
                    description: Some(
                        "Receive ERDY-TP Complete Interrupt Flag.",
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
                        "Receive STATUS Transaction Completion Interrupt Flag.",
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
                    name: "uhif_notif",
                    description: Some(
                        "Receive DEV_NOTIF-TP Complete Interrupt Flag.",
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
                        "Receive PING-TP Complete Interrupt Flag.",
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
                        "Send the ITP Complete Interrupt flag.",
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
                        "Send FIFO overflow interrupt flag.",
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
                        "Receive FIFO overflow interrupt flag.",
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
                        "If multiple endpoints have interrupt flags at the same time, the order of endpoint priority is as follows.",
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
                        "The direction of the endpoint that currently has an interrupt flag for transmission completion,.",
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
                    name: "host_ack_nump",
                    description: Some(
                        "This bit is invalid for synchronous transmission, and is defined as follows in Control Transfer,. Block Transfer, and Interrupt Transfer.",
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
                    name: "htx_res",
                    description: Some(
                        "Received a reply TP from the device returning.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hrx_res",
                    description: Some(
                        "A reply TP is received from the device back.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep0RxCtrl",
            extends: None,
            description: Some(
                "Endpoint 0 receives control registers.",
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
                        "The sequence number of the endpoint expects to accept.",
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
                        "Response to DPH. Cleared by hardware when the OUT transaction completes. A SETUP packet is always answered with ACK-TP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ep0RxRes",
                    ),
                },
                Field {
                    name: "ep0_rx_erdy",
                    description: Some(
                        "ERDY received.",
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
                        "PP bits in the received DPH.",
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
                        "Upload the transaction completion interrupt flag, the software writes 0 to zero, and the hardware sets 1.",
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
            name: "Uep0TxCtrl",
            extends: None,
            description: Some(
                "Endpoint 0 sends control registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ep0_tx_len",
                    description: Some(
                        "Endpoint transmits length registers with a maximum value of 512B.",
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
                        "The current sequence number of the endpoint,. which is aotomatically reset after receiving the SETUP transaction.",
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
                        "Response to an ACK-TP with a non-zero NumP. Cleared by hardware when the IN transaction completes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ep0TxRes",
                    ),
                },
                Field {
                    name: "ep0_tx_erdy",
                    description: Some(
                        "ERDY received.",
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
                        "A sign that completes the NRDY-TP send (answer).",
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
                        "The upload transaction is interrupted, the software writes 0 to zero, and the hardware sets 1.",
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
                        "Endpoints 1~15 downpass enabled. Index 0 is endpoint 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTxEn",
            extends: None,
            description: Some(
                "Endpoint Sends Enable Register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_tx_en",
                    description: Some(
                        "Endpoints 1~15 upload enabled. Index 0 is endpoint 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhRxCtrl",
            extends: None,
            description: Some(
                "Host receives control registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_len",
                    description: Some(
                        "The endpoint receives the length register,. which for burst transmissions indicates the packet length of the last packet of the burst transmission,  and the other packets must be 1024B as specified in the protocol.",
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
                    name: "uh_rx_ep",
                    description: Some(
                        "Indicates the source (device endpoint number) from which the packet was received in host mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uh_rx_seq",
                    description: Some(
                        "The SEQ _NUM that the endpoint expects to receive, the hardware automatically adds 1, except for endpoint 0.",
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
                    name: "uh_rx_res",
                    description: Some(
                        "Response to DPH+DPP or STATUS-TP.",
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
                    name: "uh_rx_nump",
                    description: Some(
                        "The number of packets (DPP) that the endpoint is capable of receiving (burst transmission).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uh_rx_iso",
                    description: Some(
                        "Received Packets (DPP) are transmitted synchronously.",
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
                    name: "uh_rx_act",
                    description: Some(
                        "The OUT transaction completes the interrupt flag, the software writes 0 to zero, and the hardware sets 1.",
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
            name: "UhRxDma",
            extends: None,
            description: Some(
                "Receive buffer address registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_dma",
                    description: Some(
                        "The start address of the host receiving buffer.",
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
            name: "UhRxDmaOfs",
            extends: None,
            description: Some(
                "Host Receive Address Offset Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_dma_ofs",
                    description: Some(
                        "After the host receives it, the DMA's address is offset by a large amount.",
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
            name: "UhTxCtrl",
            extends: None,
            description: Some(
                "Host Transmit Control Registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_tx_len",
                    description: Some(
                        "The endpoint receives the length register,. which for burst transmissions indicates the packet length of the last packet of the burst transmission,  and the other packets must be 1024B as specified in the protocol.",
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
                    name: "uh_tx_ep",
                    description: Some(
                        "Indicates the destination of the packet sent in host mode (the target endpoint number of the device).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uh_tx_seq",
                    description: Some(
                        "The SEQ _NUM for which the endpoint receives the packet, the hardware automatically adds 1, except for endpoint 0.",
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
                    name: "uh_tx_res",
                    description: Some(
                        "Response to DPH+DPP.",
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
                    name: "uh_tx_lpf",
                    description: Some(
                        "For burst transmissions, this bit simply represents the LPF/EOB of the last packet,. and the LPF/EOB of the preceding packet uses a fixed value of 0.",
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
                    name: "uh_tx_status",
                    description: Some(
                        "Indicates that the packet sent by the host is STATUS TP.",
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
                    name: "uh_tx_setup",
                    description: Some(
                        "Indicates that the packet sent by the host is a Setup packet, and the setup flag is set.",
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
                    name: "uh_tx_iso",
                    description: Some(
                        "The host prepares to send ISO packets.",
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
                    name: "uh_tx_act",
                    description: Some(
                        "The IN transaction completes the interrupt flag, the software writes 0 to zero, and the hardware sets 1.",
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
            name: "UhTxDma",
            extends: None,
            description: Some(
                "Send buffer address registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_tx_dma",
                    description: Some(
                        "The start address of the host sending buffer.",
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
            name: "UhTxDmaOfs",
            extends: None,
            description: Some(
                "Host Transmit Address Offset Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_tx_dma_ofs",
                    description: Some(
                        "After the host sends the DMA, the size of the DMA address offset.",
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
            name: "UsbssCtrl",
            extends: None,
            description: Some(
                "USBSS Control Register.",
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
                        "Reset all software configuration registers, high validity, software clearance required.",
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
                        "The protocol layer and FIFO module are reset and need to be cleared by software.",
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
                        "1: do not prefetch the next packet of a burst into the FIFO; 0: prefetch it (reset value), which shortens the gap between the DPHs of a burst.",
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
                    name: "dir_abort",
                    description: Some(
                        "Not in the RM, which marks bit 4 reserved. Defined by the EVT header; the SDK examples do not use it.",
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
                    name: "setup_flow",
                    description: Some(
                        "SETUP flow control. 1: the control endpoint enters flow control when a SETUP transaction arrives; 0: it does not.",
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
                    name: "itp_en",
                    description: Some(
                        "In host mode, send ITP enabled.",
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
                    name: "host_mode",
                    description: Some(
                        "USB Operating Mode Selection Bits.",
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
                    name: "reg_hp_pend",
                    description: Some(
                        "Packet Pending bit of the TPs and DPs sent. In a burst, bit 0 is the PP of the last packet and bit 1 the PP of the others; otherwise only bit 0 is used.",
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
                    name: "tx_erdy_mode",
                    description: Some(
                        "Device mode. 1: no ERDY-TP is inserted between two DPHs; 0: an ERDY-TP may be inserted between two DPHs.",
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
                    name: "chain_conflict",
                    description: Some(
                        "Not in the RM, which marks bit 15 reserved. Defined by the EVT header; the SDK examples do not use it.",
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
                    name: "uie_transfer",
                    description: Some(
                        "USB Transaction Completion Interrupt Enabled.",
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
                        "Receive SETUP Transaction Completion Interrupt Enabled.",
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
                    name: "uhie_erdy",
                    description: Some(
                        "Receive ERDY-TP Complete Interrupt Enabled.",
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
                        "Receiving STATUS transaction completion interrupt enabled.",
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
                    name: "uhie_notif",
                    description: Some(
                        "Interrupt enabled for receiving DEV_NOTIF-TP.",
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
                        "Interrupt Enabled for Receiving PING-TP.",
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
                        "Sending FIFO overflow interrupt enables.",
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
                        "Receive FIFO overflow interrupt enabled.",
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
                        "Host mode: address of the device being accessed. Device mode: address of this device.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Ep0RxRes",
            description: Some(
                "Endpoint 0 response to a DPH. The value 3 is invalid (no response).",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ACK",
                    description: Some(
                        "ACK-TP.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "NRDY",
                    description: Some(
                        "NRDY-TP.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STALL",
                    description: Some(
                        "STALL-TP.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ep0TxRes",
            description: Some(
                "Endpoint 0 response to an ACK-TP. The value 3 is invalid (no response).",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DPH",
                    description: Some(
                        "Data packet.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "NRDY",
                    description: Some(
                        "NRDY-TP.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STALL",
                    description: Some(
                        "STALL-TP.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "LinkState",
            description: Some(
                "Link state (LTSSM). Other values are reserved.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "COMPLIANCE",
                    description: Some(
                        "Compliance Mode.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "SS.Disabled.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HOTRST",
                    description: Some(
                        "Hot Reset.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "INACTIVE",
                    description: Some(
                        "SS.Inactive.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "LOOPBACK",
                    description: Some(
                        "Loopback.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "POLLING",
                    description: Some(
                        "Polling.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "RECOVERY",
                    description: Some(
                        "Recovery.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "RXDET",
                    description: Some(
                        "Rx.Detect.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "U0",
                    description: Some(
                        "U0.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "U1",
                    description: Some(
                        "U1.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "U2",
                    description: Some(
                        "U2.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "U3",
                    description: Some(
                        "U3.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "PdMode",
            description: Some(
                "PHY power mode, P0 to P3 of the PIPE.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "P0",
                    description: Some(
                        "Normal operation.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "P1",
                    description: Some(
                        "Low power. Both ends of the port send PING.LFPS periodically to keep the connection.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "P2",
                    description: Some(
                        "Lower power than P1, with a longer exit time.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "P3",
                    description: Some(
                        "Lowest power, the 125 MHz PHY clock is off.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "TxDeemph",
            description: Some(
                "Transmitter de-emphasis, TxDeemph of the PIPE. The value 3 is reserved.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DB3_5",
                    description: Some(
                        "-3.5 dB.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DB6",
                    description: Some(
                        "-6.0 dB.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "No de-emphasis.",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
