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
                    name: "link_lmp_rx_data0",
                    description: Some(
                        "LMP receives data 0 register.",
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
                        "LMP receives data 1 register.",
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
                        "LMP receives data 2 register.",
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
                    name: "tp_rx_data0",
                    description: Some(
                        "DEV_NOTIF-TP Data 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpRxData0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tp_rx_data1",
                    description: Some(
                        "DEV_NOTIF-TP Data 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpRxData1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tp_rx_data2",
                    description: Some(
                        "DEV_NOTIF-TP Data 2 Register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpRxData2",
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
                    name: "link_lmp_rx_data0",
                    description: Some(
                        "LMP receives data 0 register.",
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
                        "LMP receives data 1 register.",
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
                        "LMP receives data 2 register.",
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
                    name: "tp_rx_data0",
                    description: Some(
                        "DEV_NOTIF-TP Data 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpRxData0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tp_rx_data1",
                    description: Some(
                        "DEV_NOTIF-TP Data 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpRxData1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tp_rx_data2",
                    description: Some(
                        "DEV_NOTIF-TP Data 2 Register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpRxData2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_cfg",
                    description: Some(
                        "Endpoint 1 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_cr",
                    description: Some(
                        "Endpoint 1 control register.",
                    ),
                    array: None,
                    byte_offset: 0xc1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_seq",
                    description: Some(
                        "Endpoint 1 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0xc2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_st",
                    description: Some(
                        "Endpoint 1 status register.",
                    ),
                    array: None,
                    byte_offset: 0xc3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_chain_cr",
                    description: Some(
                        "Endpoint 1 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_chain_st",
                    description: Some(
                        "Endpoint 1 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0xc5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_chain_len",
                    description: Some(
                        "Endpoint 1 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0xc6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1TxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_chain_exp_nump",
                    description: Some(
                        "Number of NUMPs expected to be sent by endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1TxChainExpNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been sent by endpoint n.",
                    ),
                    array: None,
                    byte_offset: 0xc9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_dma_ofs",
                    description: Some(
                        "DMA offset length for endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0xca,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1TxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_dma",
                    description: Some(
                        "DMA start address for endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep1TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_cfg",
                    description: Some(
                        "Endpoint 1 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_cr",
                    description: Some(
                        "Endpoint 1 control register.",
                    ),
                    array: None,
                    byte_offset: 0xd1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_seq",
                    description: Some(
                        "Endpoint 1 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0xd2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_st",
                    description: Some(
                        "Endpoint 1 status register.",
                    ),
                    array: None,
                    byte_offset: 0xd3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_chain_cr",
                    description: Some(
                        "Endpoint 1 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_chain_st",
                    description: Some(
                        "Endpoint 1 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0xd5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_chain_len",
                    description: Some(
                        "Endpoint 1 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0xd6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1RxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_chain_max_nump",
                    description: Some(
                        "Number of NUMPs that Endpoint 1 can receive.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxChainMaxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been received by endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0xd9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_dma_ofs",
                    description: Some(
                        "DMA offset length for endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0xda,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1RxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_dma",
                    description: Some(
                        "DMA start address for endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep1RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_cfg",
                    description: Some(
                        "Endpoint 2 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_cr",
                    description: Some(
                        "Endpoint 2 control register.",
                    ),
                    array: None,
                    byte_offset: 0xe1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_seq",
                    description: Some(
                        "Endpoint 2 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0xe2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_st",
                    description: Some(
                        "Endpoint 2 status register.",
                    ),
                    array: None,
                    byte_offset: 0xe3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_chain_cr",
                    description: Some(
                        "Endpoint 2 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_chain_st",
                    description: Some(
                        "Endpoint 2 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0xe5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_chain_len",
                    description: Some(
                        "Endpoint 2 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0xe6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2TxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_chain_exp_nump",
                    description: Some(
                        "Number of NUMPs expected to be sent by endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2TxChainExpNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been sent by endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0xe9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_dma_ofs",
                    description: Some(
                        "DMA offset length for endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0xea,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2TxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_dma",
                    description: Some(
                        "DMA start address for endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep2TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_cfg",
                    description: Some(
                        "Endpoint 2 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_cr",
                    description: Some(
                        "Endpoint 2 control register.",
                    ),
                    array: None,
                    byte_offset: 0xf1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_seq",
                    description: Some(
                        "Endpoint 2 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0xf2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_st",
                    description: Some(
                        "Endpoint 2 status register.",
                    ),
                    array: None,
                    byte_offset: 0xf3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_chain_cr",
                    description: Some(
                        "Endpoint 2 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_chain_st",
                    description: Some(
                        "Endpoint 2 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0xf5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_chain_len",
                    description: Some(
                        "Endpoint 2 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0xf6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2RxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_chain_max_nump",
                    description: Some(
                        "Number of NUMPs that Endpoint 2 can receive.",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxChainMaxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been received by endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0xf9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0xfa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2RxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_dma",
                    description: Some(
                        "DMA start address for Endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep2RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_cfg",
                    description: Some(
                        "Endpoint 3 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_cr",
                    description: Some(
                        "Endpoint 3 control register.",
                    ),
                    array: None,
                    byte_offset: 0x101,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_seq",
                    description: Some(
                        "Endpoint 3 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x102,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_st",
                    description: Some(
                        "Endpoint 3 status register.",
                    ),
                    array: None,
                    byte_offset: 0x103,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_chain_cr",
                    description: Some(
                        "Endpoint 3 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_chain_st",
                    description: Some(
                        "Endpoint 3 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x105,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_chain_len",
                    description: Some(
                        "Endpoint 3 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x106,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3TxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_chain_exp_nump",
                    description: Some(
                        "Number of NUMPs expected to be sent by endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3TxChainExpNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been sent by endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0x109,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_dma_ofs",
                    description: Some(
                        "DMA offset length for endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0x10a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3TxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_dma",
                    description: Some(
                        "DMA start address for endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep3TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_cfg",
                    description: Some(
                        "Endpoint 3 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_cr",
                    description: Some(
                        "Endpoint 3 control register.",
                    ),
                    array: None,
                    byte_offset: 0x111,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_seq",
                    description: Some(
                        "Endpoint 3 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x112,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_st",
                    description: Some(
                        "Endpoint 3 status register.",
                    ),
                    array: None,
                    byte_offset: 0x113,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_chain_cr",
                    description: Some(
                        "Endpoint 3 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_chain_st",
                    description: Some(
                        "Endpoint 3 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x115,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_chain_len",
                    description: Some(
                        "Endpoint 3 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x116,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3RxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_chain_max_nump",
                    description: Some(
                        "Number of NUMPs that Endpoint 3 can receive.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxChainMaxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been received by Endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0x119,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0x11a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3RxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_dma",
                    description: Some(
                        "DMA start address for Endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep3RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_cfg",
                    description: Some(
                        "Endpoint 4 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_cr",
                    description: Some(
                        "Endpoint 4 control register.",
                    ),
                    array: None,
                    byte_offset: 0x121,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_seq",
                    description: Some(
                        "Endpoint 4 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x122,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_st",
                    description: Some(
                        "Endpoint 4 status register.",
                    ),
                    array: None,
                    byte_offset: 0x123,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_chain_cr",
                    description: Some(
                        "Endpoint 4 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_chain_st",
                    description: Some(
                        "Endpoint 4 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x125,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_chain_len",
                    description: Some(
                        "Endpoint 4 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x126,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4TxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_chain_exp_nump",
                    description: Some(
                        "Number of NUMPs expected to be sent by Endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4TxChainExpNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been sent by Endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0x129,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0x12a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4TxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_dma",
                    description: Some(
                        "DMA start address for Endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep4TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_cfg",
                    description: Some(
                        "Endpoint 4 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_cr",
                    description: Some(
                        "Endpoint 4 control register.",
                    ),
                    array: None,
                    byte_offset: 0x131,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_seq",
                    description: Some(
                        "Endpoint 4 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x132,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_st",
                    description: Some(
                        "Endpoint 4 status register.",
                    ),
                    array: None,
                    byte_offset: 0x133,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_chain_cr",
                    description: Some(
                        "Endpoint 4 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_chain_st",
                    description: Some(
                        "Endpoint 4 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x135,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_chain_len",
                    description: Some(
                        "Endpoint 4 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x136,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4RxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_chain_max_nump",
                    description: Some(
                        "Number of NUMPs that Endpoint 4 can receive.",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxChainMaxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been received by Endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0x139,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0x13a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4RxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_dma",
                    description: Some(
                        "DMA start address for Endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep4RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_cfg",
                    description: Some(
                        "Endpoint 5 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_cr",
                    description: Some(
                        "Endpoint 5 control register.",
                    ),
                    array: None,
                    byte_offset: 0x141,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_seq",
                    description: Some(
                        "Endpoint 5 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x142,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_st",
                    description: Some(
                        "Endpoint 5 status register.",
                    ),
                    array: None,
                    byte_offset: 0x143,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_chain_cr",
                    description: Some(
                        "Endpoint 5 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_chain_st",
                    description: Some(
                        "Endpoint 5 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x145,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_chain_len",
                    description: Some(
                        "Endpoint 5 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x146,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5TxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_chain_exp_nump",
                    description: Some(
                        "Number of NUMPs expected to be sent by Endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5TxChainExpNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been sent by Endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0x149,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0x14a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5TxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_dma",
                    description: Some(
                        "DMA start address for Endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep5TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_cfg",
                    description: Some(
                        "Endpoint 5 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_cr",
                    description: Some(
                        "Endpoint 5 control register.",
                    ),
                    array: None,
                    byte_offset: 0x151,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_seq",
                    description: Some(
                        "Endpoint 5 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x152,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_st",
                    description: Some(
                        "Endpoint 5 status register.",
                    ),
                    array: None,
                    byte_offset: 0x153,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_chain_cr",
                    description: Some(
                        "Endpoint 5 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_chain_st",
                    description: Some(
                        "Endpoint 5 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x155,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_chain_len",
                    description: Some(
                        "Endpoint 5 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x156,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5RxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_chain_max_nump",
                    description: Some(
                        "Number of NUMPs that Endpoint 5 can receive.",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxChainMaxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been received by Endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0x159,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0x15a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5RxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_dma",
                    description: Some(
                        "DMA start address for Endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep5RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_cfg",
                    description: Some(
                        "Endpoint 6 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_cr",
                    description: Some(
                        "Endpoint 6 control register.",
                    ),
                    array: None,
                    byte_offset: 0x161,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_seq",
                    description: Some(
                        "Endpoint 6 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x162,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_st",
                    description: Some(
                        "Endpoint 6 status register.",
                    ),
                    array: None,
                    byte_offset: 0x163,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_chain_cr",
                    description: Some(
                        "Endpoint 6 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_chain_st",
                    description: Some(
                        "Endpoint 6 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x165,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_chain_len",
                    description: Some(
                        "Endpoint 6 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x166,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6TxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_chain_exp_nump",
                    description: Some(
                        "Number of NUMPs expected to be sent by Endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6TxChainExpNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been sent by Endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0x169,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0x16a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6TxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_dma",
                    description: Some(
                        "DMA start address for Endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep6TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_cfg",
                    description: Some(
                        "Endpoint 6 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_cr",
                    description: Some(
                        "Endpoint 6 control register.",
                    ),
                    array: None,
                    byte_offset: 0x171,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_seq",
                    description: Some(
                        "Endpoint 6 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x172,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_st",
                    description: Some(
                        "Endpoint 6 status register.",
                    ),
                    array: None,
                    byte_offset: 0x173,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_chain_cr",
                    description: Some(
                        "Endpoint 6 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_chain_st",
                    description: Some(
                        "Endpoint 6 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x175,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_chain_len",
                    description: Some(
                        "Endpoint 6 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x176,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6RxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_chain_max_nump",
                    description: Some(
                        "Number of NUMPs that Endpoint 6 can receive.",
                    ),
                    array: None,
                    byte_offset: 0x178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxChainMaxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been received by Endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0x179,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0x17a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6RxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_dma",
                    description: Some(
                        "DMA start address for Endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0x17c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep6RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_cfg",
                    description: Some(
                        "Endpoint 7 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_cr",
                    description: Some(
                        "Endpoint 7 control register.",
                    ),
                    array: None,
                    byte_offset: 0x181,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_seq",
                    description: Some(
                        "Endpoint 7 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x182,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_st",
                    description: Some(
                        "Endpoint 7 status register.",
                    ),
                    array: None,
                    byte_offset: 0x183,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_chain_cr",
                    description: Some(
                        "Endpoint 7 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_chain_st",
                    description: Some(
                        "Endpoint 7 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x185,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_chain_len",
                    description: Some(
                        "Endpoint 7 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x186,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7TxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_chain_exp_nump",
                    description: Some(
                        "Number of NUMPs expected to be sent by Endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7TxChainExpNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been sent by Endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0x189,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0x18a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7TxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_dma",
                    description: Some(
                        "DMA start address for Endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep7TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_cfg",
                    description: Some(
                        "Endpoint 7 Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_cr",
                    description: Some(
                        "Endpoint 7 control register.",
                    ),
                    array: None,
                    byte_offset: 0x191,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_seq",
                    description: Some(
                        "Endpoint 7 Serial Number Register.",
                    ),
                    array: None,
                    byte_offset: 0x192,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxSeq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_st",
                    description: Some(
                        "Endpoint 7 status register.",
                    ),
                    array: None,
                    byte_offset: 0x193,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_chain_cr",
                    description: Some(
                        "Endpoint 7 CHAIN control register.",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxChainCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_chain_st",
                    description: Some(
                        "Endpoint 7 CHAIN state register.",
                    ),
                    array: None,
                    byte_offset: 0x195,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxChainSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_chain_len",
                    description: Some(
                        "Endpoint 7 CHAIN sends the last packet length.",
                    ),
                    array: None,
                    byte_offset: 0x196,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7RxChainLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_chain_max_nump",
                    description: Some(
                        "Number of NUMPs that Endpoint 7 can receive.",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxChainMaxNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_chain_nump",
                    description: Some(
                        "Number of NUMPs has been received by Endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0x199,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxChainNump",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_dma_ofs",
                    description: Some(
                        "DMA offset length for Endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0x19a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7RxDmaOfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_dma",
                    description: Some(
                        "DMA start address for Endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep7RxDma",
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
                    name: "link_lmp_rx_data0",
                    description: Some(
                        "LMP receives data 0 register.",
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
                        "LMP receives data 1 register.",
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
                        "LMP receives data 2 register.",
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
                    name: "uh_tx_dma_u3ep0_tx_dma",
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
                                "UhTxDmaU3ep0TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_dma_u3ep0_rx_dma",
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
                                "UhRxDmaU3ep0RxDma",
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
                    name: "tp_rx_data0",
                    description: Some(
                        "DEV_NOTIF-TP Data 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpRxData0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tp_rx_data1",
                    description: Some(
                        "DEV_NOTIF-TP Data 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpRxData1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tp_rx_data2",
                    description: Some(
                        "DEV_NOTIF-TP Data 2 Register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpRxData2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "HostRxFcStatus",
            extends: None,
            description: Some(
                "The host endpoint receives the flow control register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "epx_rx_fc",
                    description: Some(
                        "The flow control status of the host to receiving endpoints 1-15.",
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
                    name: "epx_tx_fc",
                    description: Some(
                        "The flow control status of the host to send endpoints 1-15.",
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
                        "Transmitter de-emphasis control.",
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
                    name: "link_lookback_act",
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
                        "High validity, after receiving the LGO_U2, the response LXU allows to enter the U2 state,. otherwise after receiving the LGO_U2, the response LXU refuses to enter the U2 state.",
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
                        "SPEC configure.",
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
                        "LINK ENTERS THE SS.ACTIVE STATE INTERRUPT FLAG.",
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
                        "he device receives the HOT RESET interrupt flag.",
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
                        "link_reset reset to 0.",
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
                        "A valid PORT_CAP-LMP is received, and the protocol stipulates that. the two LINK parties exchange PORT_CAP-LMP within 20 years after entering the U0 state.",
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
                "LMP receives data 0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_lmp_rx_data0",
                    description: Some(
                        "Once the LMP is received, the HP data is stored in this register [31:0].",
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
                "LMP receives data 1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_lmp_rx_data1",
                    description: Some(
                        "Once the LMP is received, the HP data is stored in this register [63:32].",
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
                "LMP receives data 2 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_lmp_rx_data1",
                    description: Some(
                        "Once the LMP is received, the HP data is stored in this register [95:64].",
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
                        "The data of the user-defined HP is sent [31:0].",
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
                        "The data of the user-defined HP is sent [63:32].",
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
                        "The data of the user-defined HP is sent [95:64].",
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
                    name: "uhif_erdy__udif_setup",
                    description: Some(
                        "Host Mode: Receive ERDY-TP Complete Interrupt Flag;. Device Mode: Receive SETUP Transaction Completion Interrupt Flag.",
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
                    name: "uhif_notif__udif_status",
                    description: Some(
                        "Host Mode: Receive DEV_NOTIF-TP Complete Interrupt Flag;. Device Mode: Receive STATUS Transaction Completion Interrupt Flag.",
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
            name: "TpRxData0",
            extends: None,
            description: Some(
                "DEV_NOTIF-TP Data 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usb3_notif_data0",
                    description: Some(
                        "After the DEV_NOTIF-TP is received, the HP data is stored in this register.",
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
            name: "TpRxData1",
            extends: None,
            description: Some(
                "DEV_NOTIF-TP Data 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usb3_notif_data1",
                    description: Some(
                        "After the DEV_NOTIF-TP is received, the HP data is stored in this register.",
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
            name: "TpRxData2",
            extends: None,
            description: Some(
                "DEV_NOTIF-TP Data 2 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usb3_notif_data2",
                    description: Some(
                        "After the DEV_NOTIF-TP is received, the HP data is stored in this register.",
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
            name: "Uep1RxCfg",
            extends: None,
            description: Some(
                "Endpoint 1 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_iso_mode",
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
                    name: "ep_rx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_rx_eob_mode",
                    description: Some(
                        "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH;  Software reconfiguration required.",
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
                        "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt.",
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
                    name: "ep_rx_fifo_mode",
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
                    name: "ep_rx_chain_auto",
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
            name: "Uep1RxChainCr",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_ret_sel",
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
            name: "Uep1RxChainLen",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_chain_rx_len",
                    description: Some(
                        "The length of the currently completed CHAIN to the last packet.",
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
            name: "Uep1RxChainMaxNump",
            extends: None,
            description: Some(
                "Number of NUMPs that Endpoint 1 can receive.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rx_chain_max_nump",
                    description: Some(
                        "The number of DPP packets that can be received by the CHAIN.",
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
            name: "Uep1RxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been received by endpoint 1.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that has received.",
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
            name: "Uep1RxChainSt",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_chain_no",
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
                    name: "ep_rx_dph_pp",
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
                    name: "ep_rx_nump_empty",
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
                    name: "ep_rx_lpf_flag",
                    description: Some(
                        "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH.",
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
                    name: "ep_rx_chain_en",
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
            name: "Uep1RxCr",
            extends: None,
            description: Some(
                "Endpoint 1 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_erdy_nump",
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
                    name: "ep_rx_chain_clr",
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
                    name: "ep_rx_clr",
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
                    name: "ep_rx_halt",
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
            name: "Uep1RxDma",
            extends: None,
            description: Some(
                "DMA start address for endpoint 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_rx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep1RxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for endpoint 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep1RxSeq",
            extends: None,
            description: Some(
                "Endpoint 1 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_seq_num",
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
            name: "Uep1RxSt",
            extends: None,
            description: Some(
                "Endpoint 1 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "er_rx_chain_en",
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
                    name: "ep_rx_chain_res",
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
                    name: "ep_rx_erdy_req",
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
                    name: "ep_rx_fc_st",
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
                    name: "ep_rx_int_flag",
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
            name: "Uep1TxCfg",
            extends: None,
            description: Some(
                "Endpoint 1 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_iso_mode",
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
                    name: "ep_tx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_tx_fifo_cfg",
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
                    name: "ep_tx_fifo_mode",
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
                    name: "ep_tx_chain_auto",
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
            name: "Uep1TxChainCr",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ret_sel",
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
            name: "Uep1TxChainExpNump",
            extends: None,
            description: Some(
                "Number of NUMPs expected to be sent by endpoint 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "tx_chain_exp_nump",
                    description: Some(
                        "The number of DPP packets that can be sent by the currently completed CHAIN.",
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
            name: "Uep1TxChainLen",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_len",
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
            name: "Uep1TxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been sent by endpoint n.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that have been transmitted.",
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
            name: "Uep1TxChainSt",
            extends: None,
            description: Some(
                "Endpoint 1 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_no",
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
                    name: "tx_dph_pp",
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
                    name: "tx_nump_empty",
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
                    name: "tx_eob_lpf",
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
                    name: "tx_chain_if",
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
                    name: "tx_chain_en",
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
            name: "Uep1TxCr",
            extends: None,
            description: Some(
                "Endpoint 1 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_erdy_nump",
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
                    name: "ep_tx_chain_clr",
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
                    name: "ep_tx_clr",
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
                    name: "ep_tx_halt",
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
            name: "Uep1TxDma",
            extends: None,
            description: Some(
                "DMA start address for endpoint 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_tx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep1TxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for endpoint 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep1TxSeq",
            extends: None,
            description: Some(
                "Endpoint 1 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ep_seq_num",
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
            name: "Uep1TxSt",
            extends: None,
            description: Some(
                "Endpoint 1 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_en",
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
                    name: "tx_chain_res",
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
                    name: "tx_ep_erdy_req",
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
                    name: "tx_ep_fc_st",
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
                    name: "tx_ep_int_flag",
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
            name: "Uep2RxCfg",
            extends: None,
            description: Some(
                "Endpoint 2 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_iso_mode",
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
                    name: "ep_rx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_rx_eob_mode",
                    description: Some(
                        "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH;  Software reconfiguration required.",
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
                        "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt.",
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
                    name: "ep_rx_fifo_mode",
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
                    name: "ep_rx_chain_auto",
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
            name: "Uep2RxChainCr",
            extends: None,
            description: Some(
                "Endpoint 2 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_ret_sel",
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
            name: "Uep2RxChainLen",
            extends: None,
            description: Some(
                "Endpoint 2 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_chain_rx_len",
                    description: Some(
                        "The length of the currently completed CHAIN to the last packet.",
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
            name: "Uep2RxChainMaxNump",
            extends: None,
            description: Some(
                "Number of NUMPs that Endpoint 2 can receive.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rx_chain_max_nump",
                    description: Some(
                        "The number of DPP packets that can be received by the CHAIN.",
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
            name: "Uep2RxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been received by endpoint 2.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that has received.",
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
            name: "Uep2RxChainSt",
            extends: None,
            description: Some(
                "Endpoint 2 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_chain_no",
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
                    name: "ep_rx_dph_pp",
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
                    name: "ep_rx_nump_empty",
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
                    name: "ep_rx_lpf_flag",
                    description: Some(
                        "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH.",
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
                    name: "ep_rx_chain_en",
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
            name: "Uep2RxCr",
            extends: None,
            description: Some(
                "Endpoint 2 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_erdy_nump",
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
                    name: "ep_rx_chain_clr",
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
                    name: "ep_rx_clr",
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
                    name: "ep_rx_halt",
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
            name: "Uep2RxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_rx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep2RxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep2RxSeq",
            extends: None,
            description: Some(
                "Endpoint 2 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_seq_num",
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
            name: "Uep2RxSt",
            extends: None,
            description: Some(
                "Endpoint 2 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "er_rx_chain_en",
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
                    name: "ep_rx_chain_res",
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
                    name: "ep_rx_erdy_req",
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
                    name: "ep_rx_fc_st",
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
                    name: "ep_rx_int_flag",
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
            name: "Uep2TxCfg",
            extends: None,
            description: Some(
                "Endpoint 2 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_iso_mode",
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
                    name: "ep_tx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_tx_fifo_cfg",
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
                    name: "ep_tx_fifo_mode",
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
                    name: "ep_tx_chain_auto",
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
            name: "Uep2TxChainCr",
            extends: None,
            description: Some(
                "Endpoint 2 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ret_sel",
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
            name: "Uep2TxChainExpNump",
            extends: None,
            description: Some(
                "Number of NUMPs expected to be sent by endpoint 2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "tx_chain_exp_nump",
                    description: Some(
                        "The number of DPP packets that can be sent by the currently completed CHAIN.",
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
            name: "Uep2TxChainLen",
            extends: None,
            description: Some(
                "Endpoint 2 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_len",
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
            name: "Uep2TxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been sent by endpoint 2.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that have been transmitted.",
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
            name: "Uep2TxChainSt",
            extends: None,
            description: Some(
                "Endpoint 2 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_no",
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
                    name: "tx_dph_pp",
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
                    name: "tx_nump_empty",
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
                    name: "tx_eob_lpf",
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
                    name: "tx_chain_if",
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
                    name: "tx_chain_en",
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
            name: "Uep2TxCr",
            extends: None,
            description: Some(
                "Endpoint 2 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_erdy_nump",
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
                    name: "ep_tx_chain_clr",
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
                    name: "ep_tx_clr",
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
                    name: "ep_tx_halt",
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
            name: "Uep2TxDma",
            extends: None,
            description: Some(
                "DMA start address for endpoint 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_tx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep2TxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for endpoint 2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep2TxSeq",
            extends: None,
            description: Some(
                "Endpoint 2 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ep_seq_num",
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
            name: "Uep2TxSt",
            extends: None,
            description: Some(
                "Endpoint 2 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_en",
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
                    name: "tx_chain_res",
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
                    name: "tx_ep_erdy_req",
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
                    name: "tx_ep_fc_st",
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
                    name: "tx_ep_int_flag",
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
            name: "Uep3RxCfg",
            extends: None,
            description: Some(
                "Endpoint 3 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_iso_mode",
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
                    name: "ep_rx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_rx_eob_mode",
                    description: Some(
                        "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH;  Software reconfiguration required.",
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
                        "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt.",
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
                    name: "ep_rx_fifo_mode",
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
                    name: "ep_rx_chain_auto",
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
            name: "Uep3RxChainCr",
            extends: None,
            description: Some(
                "Endpoint 3 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_ret_sel",
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
            name: "Uep3RxChainLen",
            extends: None,
            description: Some(
                "Endpoint 3 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_chain_rx_len",
                    description: Some(
                        "The length of the currently completed CHAIN to the last packet.",
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
            name: "Uep3RxChainMaxNump",
            extends: None,
            description: Some(
                "Number of NUMPs that Endpoint 3 can receive.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rx_chain_max_nump",
                    description: Some(
                        "The number of DPP packets that can be received by the CHAIN.",
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
            name: "Uep3RxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been received by Endpoint 3.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that has received.",
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
            name: "Uep3RxChainSt",
            extends: None,
            description: Some(
                "Endpoint 3 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_chain_no",
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
                    name: "ep_rx_dph_pp",
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
                    name: "ep_rx_nump_empty",
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
                    name: "ep_rx_lpf_flag",
                    description: Some(
                        "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH.",
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
                    name: "ep_rx_chain_en",
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
            name: "Uep3RxCr",
            extends: None,
            description: Some(
                "Endpoint 3 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_erdy_nump",
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
                    name: "ep_rx_chain_clr",
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
                    name: "ep_rx_clr",
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
                    name: "ep_rx_halt",
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
            name: "Uep3RxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_rx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep3RxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 3.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep3RxSeq",
            extends: None,
            description: Some(
                "Endpoint 3 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_seq_num",
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
            name: "Uep3RxSt",
            extends: None,
            description: Some(
                "Endpoint 3 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "er_rx_chain_en",
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
                    name: "ep_rx_chain_res",
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
                    name: "ep_rx_erdy_req",
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
                    name: "ep_rx_fc_st",
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
                    name: "ep_rx_int_flag",
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
            name: "Uep3TxCfg",
            extends: None,
            description: Some(
                "Endpoint 3 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_iso_mode",
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
                    name: "ep_tx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_tx_fifo_cfg",
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
                    name: "ep_tx_fifo_mode",
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
                    name: "ep_tx_chain_auto",
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
            name: "Uep3TxChainCr",
            extends: None,
            description: Some(
                "Endpoint 3 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ret_sel",
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
            name: "Uep3TxChainExpNump",
            extends: None,
            description: Some(
                "Number of NUMPs expected to be sent by endpoint 3.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "tx_chain_exp_nump",
                    description: Some(
                        "The number of DPP packets that can be sent by the currently completed CHAIN.",
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
            name: "Uep3TxChainLen",
            extends: None,
            description: Some(
                "Endpoint 3 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_len",
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
            name: "Uep3TxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been sent by endpoint 3.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that have been transmitted.",
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
            name: "Uep3TxChainSt",
            extends: None,
            description: Some(
                "Endpoint 3 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_no",
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
                    name: "tx_dph_pp",
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
                    name: "tx_nump_empty",
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
                    name: "tx_eob_lpf",
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
                    name: "tx_chain_if",
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
                    name: "tx_chain_en",
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
            name: "Uep3TxCr",
            extends: None,
            description: Some(
                "Endpoint 3 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_erdy_nump",
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
                    name: "ep_tx_chain_clr",
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
                    name: "ep_tx_clr",
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
                    name: "ep_tx_halt",
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
            name: "Uep3TxDma",
            extends: None,
            description: Some(
                "DMA start address for endpoint 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_tx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep3TxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for endpoint 3.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep3TxSeq",
            extends: None,
            description: Some(
                "Endpoint 3 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ep_seq_num",
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
            name: "Uep3TxSt",
            extends: None,
            description: Some(
                "Endpoint 3 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_en",
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
                    name: "tx_chain_res",
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
                    name: "tx_ep_erdy_req",
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
                    name: "tx_ep_fc_st",
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
                    name: "tx_ep_int_flag",
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
            name: "Uep4RxCfg",
            extends: None,
            description: Some(
                "Endpoint 4 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_iso_mode",
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
                    name: "ep_rx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_rx_eob_mode",
                    description: Some(
                        "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH;  Software reconfiguration required.",
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
                        "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt.",
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
                    name: "ep_rx_fifo_mode",
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
                    name: "ep_rx_chain_auto",
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
            name: "Uep4RxChainCr",
            extends: None,
            description: Some(
                "Endpoint 4 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_ret_sel",
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
            name: "Uep4RxChainLen",
            extends: None,
            description: Some(
                "Endpoint 4 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_chain_rx_len",
                    description: Some(
                        "The length of the currently completed CHAIN to the last packet.",
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
            name: "Uep4RxChainMaxNump",
            extends: None,
            description: Some(
                "Number of NUMPs that Endpoint 4 can receive.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rx_chain_max_nump",
                    description: Some(
                        "The number of DPP packets that can be received by the CHAIN.",
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
            name: "Uep4RxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been received by Endpoint 4.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that has received.",
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
            name: "Uep4RxChainSt",
            extends: None,
            description: Some(
                "Endpoint 4 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_chain_no",
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
                    name: "ep_rx_dph_pp",
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
                    name: "ep_rx_nump_empty",
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
                    name: "ep_rx_lpf_flag",
                    description: Some(
                        "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH.",
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
                    name: "ep_rx_chain_en",
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
            name: "Uep4RxCr",
            extends: None,
            description: Some(
                "Endpoint 4 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_erdy_nump",
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
                    name: "ep_rx_chain_clr",
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
                    name: "ep_rx_clr",
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
                    name: "ep_rx_halt",
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
            name: "Uep4RxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_rx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep4RxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 4.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep4RxSeq",
            extends: None,
            description: Some(
                "Endpoint 4 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_seq_num",
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
            name: "Uep4RxSt",
            extends: None,
            description: Some(
                "Endpoint 4 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "er_rx_chain_en",
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
                    name: "ep_rx_chain_res",
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
                    name: "ep_rx_erdy_req",
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
                    name: "ep_rx_fc_st",
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
                    name: "ep_rx_int_flag",
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
            name: "Uep4TxCfg",
            extends: None,
            description: Some(
                "Endpoint 4 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_iso_mode",
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
                    name: "ep_tx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_tx_fifo_cfg",
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
                    name: "ep_tx_fifo_mode",
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
                    name: "ep_tx_chain_auto",
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
            name: "Uep4TxChainCr",
            extends: None,
            description: Some(
                "Endpoint 4 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ret_sel",
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
            name: "Uep4TxChainExpNump",
            extends: None,
            description: Some(
                "Number of NUMPs expected to be sent by Endpoint 4.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "tx_chain_exp_nump",
                    description: Some(
                        "The number of DPP packets that can be sent by the currently completed CHAIN.",
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
            name: "Uep4TxChainLen",
            extends: None,
            description: Some(
                "Endpoint 4 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_len",
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
            name: "Uep4TxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been sent by Endpoint 4.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that have been transmitted.",
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
            name: "Uep4TxChainSt",
            extends: None,
            description: Some(
                "Endpoint 4 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_no",
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
                    name: "tx_dph_pp",
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
                    name: "tx_nump_empty",
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
                    name: "tx_eob_lpf",
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
                    name: "tx_chain_if",
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
                    name: "tx_chain_en",
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
            name: "Uep4TxCr",
            extends: None,
            description: Some(
                "Endpoint 4 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_erdy_nump",
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
                    name: "ep_tx_chain_clr",
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
                    name: "ep_tx_clr",
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
                    name: "ep_tx_halt",
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
            name: "Uep4TxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_tx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep4TxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 4.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep4TxSeq",
            extends: None,
            description: Some(
                "Endpoint 4 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ep_seq_num",
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
            name: "Uep4TxSt",
            extends: None,
            description: Some(
                "Endpoint 4 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_en",
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
                    name: "tx_chain_res",
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
                    name: "tx_ep_erdy_req",
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
                    name: "tx_ep_fc_st",
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
                    name: "tx_ep_int_flag",
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
            name: "Uep5RxCfg",
            extends: None,
            description: Some(
                "Endpoint 5 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_iso_mode",
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
                    name: "ep_rx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_rx_eob_mode",
                    description: Some(
                        "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH;  Software reconfiguration required.",
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
                        "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt.",
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
                    name: "ep_rx_fifo_mode",
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
                    name: "ep_rx_chain_auto",
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
            name: "Uep5RxChainCr",
            extends: None,
            description: Some(
                "Endpoint 5 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_ret_sel",
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
            name: "Uep5RxChainLen",
            extends: None,
            description: Some(
                "Endpoint 5 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_chain_rx_len",
                    description: Some(
                        "The length of the currently completed CHAIN to the last packet.",
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
            name: "Uep5RxChainMaxNump",
            extends: None,
            description: Some(
                "Number of NUMPs that Endpoint 5 can receive.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rx_chain_max_nump",
                    description: Some(
                        "The number of DPP packets that can be received by the CHAIN.",
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
            name: "Uep5RxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been received by Endpoint 5.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that has received.",
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
            name: "Uep5RxChainSt",
            extends: None,
            description: Some(
                "Endpoint 5 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_chain_no",
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
                    name: "ep_rx_dph_pp",
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
                    name: "ep_rx_nump_empty",
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
                    name: "ep_rx_lpf_flag",
                    description: Some(
                        "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH.",
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
                    name: "ep_rx_chain_en",
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
            name: "Uep5RxCr",
            extends: None,
            description: Some(
                "Endpoint 5 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_erdy_nump",
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
                    name: "ep_rx_chain_clr",
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
                    name: "ep_rx_clr",
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
                    name: "ep_rx_halt",
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
            name: "Uep5RxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_rx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep5RxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 5.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep5RxSeq",
            extends: None,
            description: Some(
                "Endpoint 5 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_seq_num",
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
            name: "Uep5RxSt",
            extends: None,
            description: Some(
                "Endpoint 5 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "er_rx_chain_en",
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
                    name: "ep_rx_chain_res",
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
                    name: "ep_rx_erdy_req",
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
                    name: "ep_rx_fc_st",
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
                    name: "ep_rx_int_flag",
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
            name: "Uep5TxCfg",
            extends: None,
            description: Some(
                "Endpoint 5 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_iso_mode",
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
                    name: "ep_tx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_tx_fifo_cfg",
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
                    name: "ep_tx_fifo_mode",
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
                    name: "ep_tx_chain_auto",
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
            name: "Uep5TxChainCr",
            extends: None,
            description: Some(
                "Endpoint 5 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ret_sel",
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
            name: "Uep5TxChainExpNump",
            extends: None,
            description: Some(
                "Number of NUMPs expected to be sent by Endpoint 5.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "tx_chain_exp_nump",
                    description: Some(
                        "The number of DPP packets that can be sent by the currently completed CHAIN.",
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
            name: "Uep5TxChainLen",
            extends: None,
            description: Some(
                "Endpoint 5 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_len",
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
            name: "Uep5TxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been sent by Endpoint 5.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that have been transmitted.",
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
            name: "Uep5TxChainSt",
            extends: None,
            description: Some(
                "Endpoint 5 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_no",
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
                    name: "tx_dph_pp",
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
                    name: "tx_nump_empty",
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
                    name: "tx_eob_lpf",
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
                    name: "tx_chain_if",
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
                    name: "tx_chain_en",
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
            name: "Uep5TxCr",
            extends: None,
            description: Some(
                "Endpoint 5 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_erdy_nump",
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
                    name: "ep_tx_chain_clr",
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
                    name: "ep_tx_clr",
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
                    name: "ep_tx_halt",
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
            name: "Uep5TxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_tx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep5TxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 5.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep5TxSeq",
            extends: None,
            description: Some(
                "Endpoint 5 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ep_seq_num",
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
            name: "Uep5TxSt",
            extends: None,
            description: Some(
                "Endpoint 5 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_en",
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
                    name: "tx_chain_res",
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
                    name: "tx_ep_erdy_req",
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
                    name: "tx_ep_fc_st",
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
                    name: "tx_ep_int_flag",
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
            name: "Uep6RxCfg",
            extends: None,
            description: Some(
                "Endpoint 6 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_iso_mode",
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
                    name: "ep_rx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_rx_eob_mode",
                    description: Some(
                        "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH;  Software reconfiguration required.",
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
                        "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt.",
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
                    name: "ep_rx_fifo_mode",
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
                    name: "ep_rx_chain_auto",
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
            name: "Uep6RxChainCr",
            extends: None,
            description: Some(
                "Endpoint 6 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_ret_sel",
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
            name: "Uep6RxChainLen",
            extends: None,
            description: Some(
                "Endpoint 6 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_chain_rx_len",
                    description: Some(
                        "The length of the currently completed CHAIN to the last packet.",
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
            name: "Uep6RxChainMaxNump",
            extends: None,
            description: Some(
                "Number of NUMPs that Endpoint 6 can receive.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rx_chain_max_nump",
                    description: Some(
                        "The number of DPP packets that can be received by the CHAIN.",
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
            name: "Uep6RxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been received by Endpoint 6.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that has received.",
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
            name: "Uep6RxChainSt",
            extends: None,
            description: Some(
                "Endpoint 6 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_chain_no",
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
                    name: "ep_rx_dph_pp",
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
                    name: "ep_rx_nump_empty",
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
                    name: "ep_rx_lpf_flag",
                    description: Some(
                        "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH.",
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
                    name: "ep_rx_chain_en",
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
            name: "Uep6RxCr",
            extends: None,
            description: Some(
                "Endpoint 6 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_erdy_nump",
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
                    name: "ep_rx_chain_clr",
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
                    name: "ep_rx_clr",
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
                    name: "ep_rx_halt",
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
            name: "Uep6RxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_rx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep6RxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 6.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep6RxSeq",
            extends: None,
            description: Some(
                "Endpoint 6 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_seq_num",
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
            name: "Uep6RxSt",
            extends: None,
            description: Some(
                "Endpoint 6 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "er_rx_chain_en",
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
                    name: "ep_rx_chain_res",
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
                    name: "ep_rx_erdy_req",
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
                    name: "ep_rx_fc_st",
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
                    name: "ep_rx_int_flag",
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
            name: "Uep6TxCfg",
            extends: None,
            description: Some(
                "Endpoint 6 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_iso_mode",
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
                    name: "ep_tx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_tx_fifo_cfg",
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
                    name: "ep_tx_fifo_mode",
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
                    name: "ep_tx_chain_auto",
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
            name: "Uep6TxChainCr",
            extends: None,
            description: Some(
                "Endpoint 6 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ret_sel",
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
            name: "Uep6TxChainExpNump",
            extends: None,
            description: Some(
                "Number of NUMPs expected to be sent by Endpoint 6.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "tx_chain_exp_nump",
                    description: Some(
                        "The number of DPP packets that can be sent by the currently completed CHAIN.",
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
            name: "Uep6TxChainLen",
            extends: None,
            description: Some(
                "Endpoint 6 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_len",
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
            name: "Uep6TxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been sent by Endpoint 6.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that have been transmitted.",
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
            name: "Uep6TxChainSt",
            extends: None,
            description: Some(
                "Endpoint 6 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_no",
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
                    name: "tx_dph_pp",
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
                    name: "tx_nump_empty",
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
                    name: "tx_eob_lpf",
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
                    name: "tx_chain_if",
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
                    name: "tx_chain_en",
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
            name: "Uep6TxCr",
            extends: None,
            description: Some(
                "Endpoint 6 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_erdy_nump",
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
                    name: "ep_tx_chain_clr",
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
                    name: "ep_tx_clr",
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
                    name: "ep_tx_halt",
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
            name: "Uep6TxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_tx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep6TxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 6.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep6TxSeq",
            extends: None,
            description: Some(
                "Endpoint 6 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ep_seq_num",
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
            name: "Uep6TxSt",
            extends: None,
            description: Some(
                "Endpoint 6 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_en",
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
                    name: "tx_chain_res",
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
                    name: "tx_ep_erdy_req",
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
                    name: "tx_ep_fc_st",
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
                    name: "tx_ep_int_flag",
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
            name: "Uep7RxCfg",
            extends: None,
            description: Some(
                "Endpoint 7 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_iso_mode",
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
                    name: "ep_rx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_rx_eob_mode",
                    description: Some(
                        "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH;  Software reconfiguration required.",
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
                        "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt.",
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
                    name: "ep_rx_fifo_mode",
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
                    name: "ep_rx_chain_auto",
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
            name: "Uep7RxChainCr",
            extends: None,
            description: Some(
                "Endpoint 7 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_ret_sel",
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
            name: "Uep7RxChainLen",
            extends: None,
            description: Some(
                "Endpoint 7 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ep_rx_chain_rx_len",
                    description: Some(
                        "The length of the currently completed CHAIN to the last packet.",
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
            name: "Uep7RxChainMaxNump",
            extends: None,
            description: Some(
                "Number of NUMPs that Endpoint 7 can receive.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rx_chain_max_nump",
                    description: Some(
                        "The number of DPP packets that can be received by the CHAIN.",
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
            name: "Uep7RxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been received by Endpoint 7.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that has received.",
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
            name: "Uep7RxChainSt",
            extends: None,
            description: Some(
                "Endpoint 7 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_chain_no",
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
                    name: "ep_rx_dph_pp",
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
                    name: "ep_rx_nump_empty",
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
                    name: "ep_rx_lpf_flag",
                    description: Some(
                        "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH.",
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
                    name: "ep_rx_chain_en",
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
            name: "Uep7RxCr",
            extends: None,
            description: Some(
                "Endpoint 7 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_erdy_nump",
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
                    name: "ep_rx_chain_clr",
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
                    name: "ep_rx_clr",
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
                    name: "ep_rx_halt",
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
            name: "Uep7RxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_rx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep7RxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 7.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep7RxSeq",
            extends: None,
            description: Some(
                "Endpoint 7 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_rx_seq_num",
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
            name: "Uep7RxSt",
            extends: None,
            description: Some(
                "Endpoint 7 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "er_rx_chain_en",
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
                    name: "ep_rx_chain_res",
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
                    name: "ep_rx_erdy_req",
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
                    name: "ep_rx_fc_st",
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
                    name: "ep_rx_int_flag",
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
            name: "Uep7TxCfg",
            extends: None,
            description: Some(
                "Endpoint 7 Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_iso_mode",
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
                    name: "ep_tx_seq_auto",
                    description: Some(
                        "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM.",
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
                        "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit.",
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
                    name: "ep_tx_fifo_cfg",
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
                    name: "ep_tx_fifo_mode",
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
                    name: "ep_tx_chain_auto",
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
            name: "Uep7TxChainCr",
            extends: None,
            description: Some(
                "Endpoint 7 CHAIN control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ret_sel",
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
            name: "Uep7TxChainExpNump",
            extends: None,
            description: Some(
                "Number of NUMPs expected to be sent by Endpoint 7.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "tx_chain_exp_nump",
                    description: Some(
                        "The number of DPP packets that can be sent by the currently completed CHAIN.",
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
            name: "Uep7TxChainLen",
            extends: None,
            description: Some(
                "Endpoint 7 CHAIN sends the last packet length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_len",
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
            name: "Uep7TxChainNump",
            extends: None,
            description: Some(
                "Number of NUMPs has been sent by Endpoint 7.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_nump",
                    description: Some(
                        "The number of DPP packets that have been transmitted.",
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
            name: "Uep7TxChainSt",
            extends: None,
            description: Some(
                "Endpoint 7 CHAIN state register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_no",
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
                    name: "tx_dph_pp",
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
                    name: "tx_nump_empty",
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
                    name: "tx_eob_lpf",
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
                    name: "tx_chain_if",
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
                    name: "tx_chain_en",
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
            name: "Uep7TxCr",
            extends: None,
            description: Some(
                "Endpoint 7 control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep_tx_erdy_nump",
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
                    name: "ep_tx_chain_clr",
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
                    name: "ep_tx_clr",
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
                    name: "ep_tx_halt",
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
            name: "Uep7TxDma",
            extends: None,
            description: Some(
                "DMA start address for Endpoint 7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chain_tx_dma",
                    description: Some(
                        "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM.",
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
            name: "Uep7TxDmaOfs",
            extends: None,
            description: Some(
                "DMA offset length for Endpoint 7.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "chain_tx_dma_ofs",
                    description: Some(
                        "The offset address of the DPP in that CHAIN.",
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
            name: "Uep7TxSeq",
            extends: None,
            description: Some(
                "Endpoint 7 Serial Number Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_ep_seq_num",
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
            name: "Uep7TxSt",
            extends: None,
            description: Some(
                "Endpoint 7 status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_chain_en",
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
                    name: "tx_chain_res",
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
                    name: "tx_ep_erdy_req",
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
                    name: "tx_ep_fc_st",
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
                    name: "tx_ep_int_flag",
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
                        "Endpoints 1~15 downpass enabled.",
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
                        "Endpoints 1~15 upload enabled.",
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
            name: "UhRxDmaU3ep0RxDma",
            extends: None,
            description: Some(
                "Receive buffer address registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "u3ep0_rx_dma",
                    description: Some(
                        "Host Mode:The start address of the host receiving buffer.. Device Mode:Endpoint 0 receives the start of the buffer.",
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
            name: "UhTxDmaU3ep0TxDma",
            extends: None,
            description: Some(
                "Send buffer address registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "u3ep0_tx_dma",
                    description: Some(
                        "Host Mode:The start address of the host sending buffer.. Device Mode:Endpoint 0 sends the start of the buffer.",
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
                        "When DPH is transmitted in bursts, the next packet of data status.",
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
                        "SETUP transaction throttling.",
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
                        "Packet Pending control bit for sending TP/DP packets.",
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
                    name: "udie_setup__uhie_erdy",
                    description: Some(
                        "Device Mode: Receive SETUP Transaction Completion Interrupt Enabled;. Host mode:Receive SETUP Transaction Completion Interrupt Enabled.",
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
                    name: "udie_status__uhie_notif",
                    description: Some(
                        "Device mode: Receiving STATUS transaction completion interrupt enabled;. Host mode: Interrupt enabled for receiving DEV_NOTIF-TP.",
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
                        "Host Mode.",
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
    enums: &[],
};
