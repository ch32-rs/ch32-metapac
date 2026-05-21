use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usbhs",
            extends: None,
            description: Some(
                "USB register.",
            ),
            items: &[
                BlockItem {
                    name: "usb_ctrl",
                    description: Some(
                        "USB base control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_base_mode",
                    description: Some(
                        "USB mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbBaseMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_en",
                    description: Some(
                        "USB interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_dev_ad",
                    description: Some(
                        "USB device address.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbDevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_wake_ctrl",
                    description: Some(
                        "USB remote wake up register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbWakeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_test_mode",
                    description: Some(
                        "USB test mode register.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbTestMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_lpm_data",
                    description: Some(
                        "USB power management register.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbLpmData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_fg",
                    description: Some(
                        "USB interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbMisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_fram_no",
                    description: Some(
                        "USB frame number.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbFramNo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_bus",
                    description: Some(
                        "USB bus.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbBus",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "UsbhsDevice",
            extends: Some(
                "USBHS",
            ),
            description: Some(
                "USBHS in device mode. Endpoint configuration / TX / RX registers (UEP* family).",
            ),
            items: &[
                BlockItem {
                    name: "usb_ctrl",
                    description: Some(
                        "USB base control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_base_mode",
                    description: Some(
                        "USB mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbBaseMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_en",
                    description: Some(
                        "USB interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_dev_ad",
                    description: Some(
                        "USB device address.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbDevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_wake_ctrl",
                    description: Some(
                        "USB remote wake up register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbWakeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_test_mode",
                    description: Some(
                        "USB test mode register.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbTestMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_lpm_data",
                    description: Some(
                        "USB power management register.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbLpmData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_fg",
                    description: Some(
                        "USB interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbMisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_fram_no",
                    description: Some(
                        "USB frame number.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbFramNo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_bus",
                    description: Some(
                        "USB bus.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbBus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_en",
                    description: Some(
                        "USB endpoint sends the enable register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                        "USB endpoint receive the enable registers.",
                    ),
                    array: None,
                    byte_offset: 0x12,
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
                    name: "uep_t_tog_auto",
                    description: Some(
                        "USB endpoint sends the auto-filp enable register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepTTogAuto",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_r_tog_auto",
                    description: Some(
                        "USB endpoint receive the auto-filp enable register.",
                    ),
                    array: None,
                    byte_offset: 0x16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRTogAuto",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_t_burst",
                    description: Some(
                        "USB endpoint sends a burst register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTBurst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_t_burst_mode",
                    description: Some(
                        "USB endpoint send the mode register.",
                    ),
                    array: None,
                    byte_offset: 0x19,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTBurstMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_r_burst",
                    description: Some(
                        "USB endpoint receives the burst register.",
                    ),
                    array: None,
                    byte_offset: 0x1a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRBurst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_r_res_mode",
                    description: Some(
                        "USB endpoint reply mode register.",
                    ),
                    array: None,
                    byte_offset: 0x1b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRResMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_af_mode",
                    description: Some(
                        "USB endpoint muitiplexing register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UepAfMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_dma",
                    description: Some(
                        "The start address register of the endpoint 0 buffer.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep0Dma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_dma",
                    description: Some(
                        "endpoint 1 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                    name: "uep2_rx_dma",
                    description: Some(
                        "endpoint 2 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                    name: "uep3_rx_dma",
                    description: Some(
                        "endpoint 3 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                    name: "uep4_rx_dma",
                    description: Some(
                        "endpoint 4 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                    name: "uep5_rx_dma",
                    description: Some(
                        "endpoint 5 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x34,
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
                    name: "uep6_rx_dma",
                    description: Some(
                        "endpoint 6 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x38,
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
                    name: "uep7_rx_dma",
                    description: Some(
                        "endpoint 7 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
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
                BlockItem {
                    name: "uep1_tx_dma",
                    description: Some(
                        "endpoint 1 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x40,
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
                    name: "uep2_tx_dma",
                    description: Some(
                        "endpoint 2 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x44,
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
                    name: "uep3_tx_dma",
                    description: Some(
                        "endpoint 3 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x48,
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
                    name: "uep4_tx_dma",
                    description: Some(
                        "endpoint 4 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
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
                    name: "uep5_tx_dma",
                    description: Some(
                        "endpoint 5 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x50,
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
                    name: "uep6_tx_dma",
                    description: Some(
                        "endpoint 6 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x54,
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
                    name: "uep7_tx_dma",
                    description: Some(
                        "endpoint 7 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x58,
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
                    name: "uep0_max_len",
                    description: Some(
                        "endpoint 0 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep0MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_max_len",
                    description: Some(
                        "endpoint 1 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep1MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_max_len",
                    description: Some(
                        "endpoint 2 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep2MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_max_len",
                    description: Some(
                        "endpoint 3 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep3MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_max_len",
                    description: Some(
                        "endpoint 4 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep4MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_max_len",
                    description: Some(
                        "endpoint 5 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep5MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_max_len",
                    description: Some(
                        "endpoint 6 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep6MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_max_len",
                    description: Some(
                        "endpoint 7 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep7MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_rx_len",
                    description: Some(
                        "endpoint 0 acceptable length.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep0RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_len",
                    description: Some(
                        "endpoint 1 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0x82,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_len",
                    description: Some(
                        "endpoint 2 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0x86,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_len",
                    description: Some(
                        "endpoint 3 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0x8a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_len",
                    description: Some(
                        "endpoint 4 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0x8e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_len",
                    description: Some(
                        "endpoint 5 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0x92,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_len",
                    description: Some(
                        "endpoint 6 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0x96,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_len",
                    description: Some(
                        "endpoint 7 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0x9a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_t_len",
                    description: Some(
                        "endpoint 0 send the length.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep0TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_tx_ctrl",
                    description: Some(
                        "endpoint 0 send control register.",
                    ),
                    array: None,
                    byte_offset: 0x9e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep0TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_rx_ctrl",
                    description: Some(
                        "endpoint 0 send control register.",
                    ),
                    array: None,
                    byte_offset: 0x9f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep0RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_t_len",
                    description: Some(
                        "endpoint 1 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_ctrl",
                    description: Some(
                        "endpoint 1 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xa2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_ctrl",
                    description: Some(
                        "endpoint 1 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xa3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_t_len",
                    description: Some(
                        "endpoint 2 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_ctrl",
                    description: Some(
                        "endpoint 2 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xa6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_ctrl",
                    description: Some(
                        "endpoint 2 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xa7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_t_len",
                    description: Some(
                        "endpoint 3 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_ctrl",
                    description: Some(
                        "endpoint 3 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xaa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_ctrl",
                    description: Some(
                        "endpoint 3 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xab,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_t_len",
                    description: Some(
                        "endpoint 4 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_ctrl",
                    description: Some(
                        "endpoint 4 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xae,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_ctrl",
                    description: Some(
                        "endpoint 4 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xaf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_t_len",
                    description: Some(
                        "endpoint 5 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_ctrl",
                    description: Some(
                        "endpoint 5 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xb2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_ctrl",
                    description: Some(
                        "endpoint 5 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xb3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_t_len",
                    description: Some(
                        "endpoint 6 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_ctrl",
                    description: Some(
                        "endpoint 6 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xb6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_ctrl",
                    description: Some(
                        "endpoint 6 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xb7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_t_len",
                    description: Some(
                        "endpoint 7 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_ctrl",
                    description: Some(
                        "endpoint 7 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xba,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_ctrl",
                    description: Some(
                        "endpoint 7 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xbb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_t_iso",
                    description: Some(
                        "usb endpoint sends a synchronous mode enable register.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepTIso",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_r_iso",
                    description: Some(
                        "usb endpoint receives a synchronous mode enable register.",
                    ),
                    array: None,
                    byte_offset: 0xbe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRIso",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_fifo",
                    description: Some(
                        "Receive FIFO address of usb endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep1RxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_fifo",
                    description: Some(
                        "Receive FIFO address of usb endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep2RxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_fifo",
                    description: Some(
                        "Receive FIFO address of usb endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep3RxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_fifo",
                    description: Some(
                        "Receive FIFO address of usb endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep4RxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_fifo",
                    description: Some(
                        "Receive FIFO address of usb endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep5RxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_fifo",
                    description: Some(
                        "Receive FIFO address of usb endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep6RxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_fifo",
                    description: Some(
                        "Receive FIFO address of usb endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep7RxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_fifo",
                    description: Some(
                        "The sending FIFO address of usb endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep1TxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_fifo",
                    description: Some(
                        "The sending FIFO address of usb endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep2TxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_fifo",
                    description: Some(
                        "The sending FIFO address of usb endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep3TxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_fifo",
                    description: Some(
                        "The sending FIFO address of usb endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep4TxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_fifo",
                    description: Some(
                        "The sending FIFO address of usb endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep5TxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_fifo",
                    description: Some(
                        "The sending FIFO address of usb endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep6TxFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_fifo",
                    description: Some(
                        "The sending FIFO address of usb endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep7TxFifo",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "UsbhsHost",
            extends: Some(
                "USBHS",
            ),
            description: Some(
                "USBHS in host mode. UH_* host control registers; offsets overlap with UEP* device-mode pairs.",
            ),
            items: &[
                BlockItem {
                    name: "usb_ctrl",
                    description: Some(
                        "USB base control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_base_mode",
                    description: Some(
                        "USB mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbBaseMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_en",
                    description: Some(
                        "USB interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_dev_ad",
                    description: Some(
                        "USB device address.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbDevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_wake_ctrl",
                    description: Some(
                        "USB remote wake up register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbWakeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_test_mode",
                    description: Some(
                        "USB test mode register.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbTestMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_lpm_data",
                    description: Some(
                        "USB power management register.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbLpmData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_fg",
                    description: Some(
                        "USB interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbIntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "UsbMisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_fram_no",
                    description: Some(
                        "USB frame number.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbFramNo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_bus",
                    description: Some(
                        "USB bus.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UsbBus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_cfg",
                    description: Some(
                        "USB host Configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_int_en",
                    description: Some(
                        "USB host interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x102,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_dev_ad",
                    description: Some(
                        "USB host device address register.",
                    ),
                    array: None,
                    byte_offset: 0x103,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhDevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_control",
                    description: Some(
                        "USB host control register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_int_flag",
                    description: Some(
                        "USB HOST interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhIntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_int_st",
                    description: Some(
                        "USB host interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x109,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhIntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_mis_st",
                    description: Some(
                        "USB host miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0x10a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhMisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_lpm_data",
                    description: Some(
                        "USB host power management data register.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhLpmData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_split_data",
                    description: Some(
                        "USB host SPLIT register.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhSplitData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_frame",
                    description: Some(
                        "USB host frame register.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhFrame",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_tx_len",
                    description: Some(
                        "USB host send length register.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhTxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_len",
                    description: Some(
                        "USB host receive length register.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhRxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_max_len",
                    description: Some(
                        "USB host receives the maximum length register.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhRxMaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_dma",
                    description: Some(
                        "DMA receive address register.",
                    ),
                    array: None,
                    byte_offset: 0x124,
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
                    name: "uh_tx_dma",
                    description: Some(
                        "DMA send address register.",
                    ),
                    array: None,
                    byte_offset: 0x128,
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
                    name: "uh_port_ctrl",
                    description: Some(
                        "USB host port control register.",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhPortCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_port_cfg",
                    description: Some(
                        "USB host port Configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhPortCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_port_int_en",
                    description: Some(
                        "USB host port interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x132,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhPortIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_port_test_ct",
                    description: Some(
                        "USB host port test mode register.",
                    ),
                    array: None,
                    byte_offset: 0x133,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhPortTestCt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_port_st",
                    description: Some(
                        "USB host port status register.",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UhPortSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_port_chg",
                    description: Some(
                        "USB host port state charge register.",
                    ),
                    array: None,
                    byte_offset: 0x136,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UhPortChg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_bc_ctrl",
                    description: Some(
                        "USB host BC charging control register.",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhBcCtrl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Uep0Dma",
            extends: None,
            description: Some(
                "The start address register of the endpoint 0 buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep0_dma",
                    description: Some(
                        "The start address of the endpoint 0 buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep0MaxLen",
            extends: None,
            description: Some(
                "endpoint 0 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep0_max_len",
                    description: Some(
                        "endpoint 0 max acceptable offset length.",
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
            name: "Uep0RxCtrl",
            extends: None,
            description: Some(
                "endpoint 0 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mask",
                    description: Some(
                        "endpoint 0 has control over the received response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "the reception of endpoint 0 expects a synchronous trigger. bit.",
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
                    name: "uep_r_setup_is",
                    description: Some(
                        "whether endpoint 0 receives a SETUP transaction.",
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
                    name: "uep_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint 0 for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint 0 receives the end of NAK flag.",
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
                    name: "uep_r_done",
                    description: Some(
                        "endpoint 0 receives the end flag.",
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
            name: "Uep0RxLen",
            extends: None,
            description: Some(
                "endpoint 0 acceptable length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep0_rx_len",
                    description: Some(
                        "endpoint 0 acceptable data length.",
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
            name: "Uep0TLen",
            extends: None,
            description: Some(
                "endpoint 0 send the length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep0_t_len",
                    description: Some(
                        "endpoint 0 send the length.",
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
            name: "Uep0TxCtrl",
            extends: None,
            description: Some(
                "endpoint 0 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_res_mask",
                    description: Some(
                        "endpoint 0 control of the send response to IN transactions.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "endpoint 0 synchronous trigger bit for the sender to. prepare.",
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
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint 0 sends the end flag.",
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
                    name: "uep_t_done",
                    description: Some(
                        "endpoint 0 sends the end of NAK flag.",
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
            name: "Uep1MaxLen",
            extends: None,
            description: Some(
                "endpoint 1 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep1_max_len",
                    description: Some(
                        "endpoint 1 max acceptable offset length.",
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
            name: "Uep1RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep1_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 1.",
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
            name: "Uep1RxCtrl",
            extends: None,
            description: Some(
                "endpoint 1 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mask",
                    description: Some(
                        "endpoint 1 has control over the received response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "the reception of endpoint 1 expects a synchronous trigger. bit.",
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
                    name: "uep_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint 1 for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint 1 receives the end of NAK flag.",
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
                    name: "uep_r_done",
                    description: Some(
                        "endpoint 1 receives the end flag.",
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
                "endpoint 1 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep1_rx_dma",
                    description: Some(
                        "endpoint receives the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep1RxFifo",
            extends: None,
            description: Some(
                "Receive FIFO address of usb endpoint 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_rx_fifo_s",
                    description: Some(
                        "The FIFO start address of the receiving endpoint.",
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
                    name: "uep_rx_fifo_e",
                    description: Some(
                        "The end FIFO address of the receiving endpoint.",
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
            name: "Uep1RxLen",
            extends: None,
            description: Some(
                "endpoint 1 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep1_rx_len",
                    description: Some(
                        "the length of data received by endpoint 1 in a single pass.",
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
            name: "Uep1TLen",
            extends: None,
            description: Some(
                "endpoint 1 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep1_t_len",
                    description: Some(
                        "endpoint 1 send the length.",
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
            name: "Uep1TxCtrl",
            extends: None,
            description: Some(
                "endpoint 1 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_res_mask",
                    description: Some(
                        "endpoint 1 control of the send response to IN transactions.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "endpoint 1 synchronous trigger bit for the sender to. prepare.",
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
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint 1 sends the end flag.",
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
                    name: "uep_t_done",
                    description: Some(
                        "endpoint 1 sends the end of NAK flag.",
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
                "endpoint 1 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep1_tx_dma",
                    description: Some(
                        "endpoint sends the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep1TxFifo",
            extends: None,
            description: Some(
                "The sending FIFO address of usb endpoint 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_tx_fifo_s",
                    description: Some(
                        "The FIFO start address of the sending endpoint.",
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
                    name: "uep_tx_fifo_e",
                    description: Some(
                        "The end FIFO address of the sending endpoint.",
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
            name: "Uep2MaxLen",
            extends: None,
            description: Some(
                "endpoint 2 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep2_max_len",
                    description: Some(
                        "endpoint 2 max acceptable offset length.",
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
            name: "Uep2RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep2_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 2.",
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
            name: "Uep2RxCtrl",
            extends: None,
            description: Some(
                "endpoint 2 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mask",
                    description: Some(
                        "endpoint 2 has control over the received response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "the reception of endpoint 2 expects a synchronous trigger. bit.",
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
                    name: "uep_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint 2 for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint 2 receives the end of NAK flag.",
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
                    name: "uep_r_done",
                    description: Some(
                        "endpoint 2 receives the end flag.",
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
                "endpoint 2 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep2_rx_dma",
                    description: Some(
                        "endpoint receives the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep2RxFifo",
            extends: None,
            description: Some(
                "Receive FIFO address of usb endpoint 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_rx_fifo_s",
                    description: Some(
                        "The FIFO start address of the receiving endpoint.",
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
                    name: "uep_rx_fifo_e",
                    description: Some(
                        "The end FIFO address of the receiving endpoint.",
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
            name: "Uep2RxLen",
            extends: None,
            description: Some(
                "endpoint 2 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep2_rx_len",
                    description: Some(
                        "the length of data received by endpoint 2 in a single pass.",
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
            name: "Uep2TLen",
            extends: None,
            description: Some(
                "endpoint 2 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep2_t_len",
                    description: Some(
                        "endpoint 2 send the length.",
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
            name: "Uep2TxCtrl",
            extends: None,
            description: Some(
                "endpoint 2 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_res_mask",
                    description: Some(
                        "endpoint 2 control of the send response to IN transactions.",
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
                    name: "uep_t_tog_mas",
                    description: Some(
                        "endpoint 2 synchronous trigger bit for the sender to. prepare.",
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
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint 2 sends the end flag.",
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
                    name: "uep_t_done",
                    description: Some(
                        "endpoint 2 sends the end of NAK flag.",
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
                "endpoint 2 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep2_tx_dma",
                    description: Some(
                        "endpoint sends the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep2TxFifo",
            extends: None,
            description: Some(
                "The sending FIFO address of usb endpoint 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_tx_fifo_s",
                    description: Some(
                        "The FIFO start address of the sending endpoint.",
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
                    name: "uep_tx_fifo_e",
                    description: Some(
                        "The end FIFO address of the sending endpoint.",
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
            name: "Uep3MaxLen",
            extends: None,
            description: Some(
                "endpoint 3 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep3_max_len",
                    description: Some(
                        "endpoint 3 max acceptable offset length.",
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
            name: "Uep3RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 3.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep3_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 3.",
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
            name: "Uep3RxCtrl",
            extends: None,
            description: Some(
                "endpoint 3 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mask",
                    description: Some(
                        "endpoint 3 has control over the received response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "the reception of endpoint 3 expects a synchronous trigger. bit.",
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
                    name: "uep_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint 3 for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint 3 receives the end of NAK flag.",
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
                    name: "uep_r_done",
                    description: Some(
                        "endpoint 3 receives the end flag.",
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
                "endpoint 3 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep3_rx_dma",
                    description: Some(
                        "endpoint receives the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep3RxFifo",
            extends: None,
            description: Some(
                "Receive FIFO address of usb endpoint 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_rx_fifo_s",
                    description: Some(
                        "The FIFO start address of the receiving endpoint.",
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
                    name: "uep_rx_fifo_e",
                    description: Some(
                        "The end FIFO address of the receiving endpoint.",
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
            name: "Uep3RxLen",
            extends: None,
            description: Some(
                "endpoint 3 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep3_rx_len",
                    description: Some(
                        "the length of data received by endpoint 3 in a single pass.",
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
            name: "Uep3TLen",
            extends: None,
            description: Some(
                "endpoint 3 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep3_t_len",
                    description: Some(
                        "endpoint 3 send the length.",
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
            name: "Uep3TxCtrl",
            extends: None,
            description: Some(
                "endpoint 3 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_res_mask",
                    description: Some(
                        "endpoint 3 control of the send response to IN transactions.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "endpoint 3 synchronous trigger bit for the sender to. prepare.",
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
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint 3 sends the end flag.",
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
                    name: "uep_t_done",
                    description: Some(
                        "endpoint 3 sends the end of NAK flag.",
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
                "endpoint 3 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep3_tx_dma",
                    description: Some(
                        "endpoint sends the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep3TxFifo",
            extends: None,
            description: Some(
                "The sending FIFO address of usb endpoint 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_tx_fifo_s",
                    description: Some(
                        "The FIFO start address of the sending endpoint.",
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
                    name: "uep_tx_fifo_e",
                    description: Some(
                        "The end FIFO address of the sending endpoint.",
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
            name: "Uep4MaxLen",
            extends: None,
            description: Some(
                "endpoint 4 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep4_max_len",
                    description: Some(
                        "endpoint 4 max acceptable offset length.",
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
            name: "Uep4RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 4.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep4_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 4.",
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
            name: "Uep4RxCtrl",
            extends: None,
            description: Some(
                "endpoint 4 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mask",
                    description: Some(
                        "endpoint 4 has control over the received response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "the reception of endpoint 4 expects a synchronous trigger. bit.",
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
                    name: "uep_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint 4 for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint 4 receives the end of NAK flag.",
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
                    name: "uep_r_done",
                    description: Some(
                        "endpoint 4 receives the end flag.",
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
                "endpoint 4 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep4_rx_dma",
                    description: Some(
                        "endpoint receives the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep4RxFifo",
            extends: None,
            description: Some(
                "Receive FIFO address of usb endpoint 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_rx_fifo_s",
                    description: Some(
                        "The FIFO start address of the receiving endpoint.",
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
                    name: "uep_rx_fifo_e",
                    description: Some(
                        "The end FIFO address of the receiving endpoint.",
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
            name: "Uep4RxLen",
            extends: None,
            description: Some(
                "endpoint 4 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep4_rx_len",
                    description: Some(
                        "the length of data received by endpoint 4 in a single pass.",
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
            name: "Uep4TLen",
            extends: None,
            description: Some(
                "endpoint 4 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep4_t_len",
                    description: Some(
                        "endpoint 4 send the length.",
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
            name: "Uep4TxCtrl",
            extends: None,
            description: Some(
                "endpoint 4 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_res_mask",
                    description: Some(
                        "endpoint 4 control of the send response to IN transactions.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "endpoint 4 synchronous trigger bit for the sender to. prepare.",
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
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint 4 sends the end flag.",
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
                    name: "uep_t_done",
                    description: Some(
                        "endpoint 4 sends the end of NAK flag.",
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
                "endpoint 4 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep4_tx_dma",
                    description: Some(
                        "endpoint sends the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep4TxFifo",
            extends: None,
            description: Some(
                "The sending FIFO address of usb endpoint 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_tx_fifo_s",
                    description: Some(
                        "The FIFO start address of the sending endpoint.",
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
                    name: "uep_tx_fifo_e",
                    description: Some(
                        "The end FIFO address of the sending endpoint.",
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
            name: "Uep5MaxLen",
            extends: None,
            description: Some(
                "endpoint 5 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep5_max_len",
                    description: Some(
                        "endpoint 5 max acceptable offset length.",
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
            name: "Uep5RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 5.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep5_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 5.",
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
            name: "Uep5RxCtrl",
            extends: None,
            description: Some(
                "endpoint 5 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mask",
                    description: Some(
                        "endpoint 5 has control over the received response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "the reception of endpoint 5 expects a synchronous trigger. bit.",
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
                    name: "uep_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint 5 for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint 5 receives the end of NAK flag.",
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
                    name: "uep_r_done",
                    description: Some(
                        "endpoint 5 receives the end flag.",
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
                "endpoint 5 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep5_rx_dma",
                    description: Some(
                        "endpoint receives the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep5RxFifo",
            extends: None,
            description: Some(
                "Receive FIFO address of usb endpoint 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_rx_fifo_s",
                    description: Some(
                        "The FIFO start address of the receiving endpoint.",
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
                    name: "uep_rx_fifo_e",
                    description: Some(
                        "The end FIFO address of the receiving endpoint.",
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
            name: "Uep5RxLen",
            extends: None,
            description: Some(
                "endpoint 5 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep5_rx_len",
                    description: Some(
                        "the length of data received by endpoint 5 in a single pass.",
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
            name: "Uep5TLen",
            extends: None,
            description: Some(
                "endpoint 5 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep5_t_len",
                    description: Some(
                        "endpoint 5 send the length.",
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
            name: "Uep5TxCtrl",
            extends: None,
            description: Some(
                "endpoint 5 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_res_mask",
                    description: Some(
                        "endpoint 5 control of the send response to IN transactions.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "endpoint 5 synchronous trigger bit for the sender to. prepare.",
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
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint 5 sends the end flag.",
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
                    name: "uep_t_done",
                    description: Some(
                        "endpoint 5 sends the end of NAK flag.",
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
                "endpoint 5 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep5_tx_dma",
                    description: Some(
                        "endpoint sends the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep5TxFifo",
            extends: None,
            description: Some(
                "The sending FIFO address of usb endpoint 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_tx_fifo_s",
                    description: Some(
                        "The FIFO start address of the sending endpoint.",
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
                    name: "uep_tx_fifo_e",
                    description: Some(
                        "The end FIFO address of the sending endpoint.",
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
            name: "Uep6MaxLen",
            extends: None,
            description: Some(
                "endpoint 6 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep6_max_len",
                    description: Some(
                        "endpoint 6 max acceptable offset length.",
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
            name: "Uep6RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 6.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep6_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 6.",
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
            name: "Uep6RxCtrl",
            extends: None,
            description: Some(
                "endpoint 6 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mask",
                    description: Some(
                        "endpoint 6 has control over the received response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "the reception of endpoint 6 expects a synchronous trigger. bit.",
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
                    name: "uep_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint 6 for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint 6 receives the end of NAK flag.",
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
                    name: "uep_r_done",
                    description: Some(
                        "endpoint 6 receives the end flag.",
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
                "endpoint 6 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep6_rx_dma",
                    description: Some(
                        "endpoint receives the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep6RxFifo",
            extends: None,
            description: Some(
                "Receive FIFO address of usb endpoint 6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_rx_fifo_s",
                    description: Some(
                        "The FIFO start address of the receiving endpoint.",
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
                    name: "uep_rx_fifo_e",
                    description: Some(
                        "The end FIFO address of the receiving endpoint.",
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
            name: "Uep6RxLen",
            extends: None,
            description: Some(
                "endpoint 6 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep6_rx_len",
                    description: Some(
                        "the length of data received by endpoint 6 in a single pass.",
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
            name: "Uep6TLen",
            extends: None,
            description: Some(
                "endpoint 6 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep6_t_len",
                    description: Some(
                        "endpoint 6 send the length.",
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
            name: "Uep6TxCtrl",
            extends: None,
            description: Some(
                "endpoint 6 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_res_mask",
                    description: Some(
                        "endpoint 6 control of the send response to IN transactions.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "endpoint 6 synchronous trigger bit for the sender to. prepare.",
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
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint 6 sends the end flag.",
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
                    name: "uep_t_done",
                    description: Some(
                        "endpoint 6 sends the end of NAK flag.",
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
                "endpoint 6 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep6_tx_dma",
                    description: Some(
                        "endpoint sends the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep6TxFifo",
            extends: None,
            description: Some(
                "The sending FIFO address of usb endpoint 6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_tx_fifo_s",
                    description: Some(
                        "The FIFO start address of the sending endpoint.",
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
                    name: "uep_tx_fifo_e",
                    description: Some(
                        "The end FIFO address of the sending endpoint.",
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
            name: "Uep7MaxLen",
            extends: None,
            description: Some(
                "endpoint 7 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep7_max_len",
                    description: Some(
                        "endpoint 7 max acceptable offset length.",
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
            name: "Uep7RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 7.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep7_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 7.",
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
            name: "Uep7RxCtrl",
            extends: None,
            description: Some(
                "endpoint 7 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mask",
                    description: Some(
                        "endpoint 7 has control over the received response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "the reception of endpoint 7 expects a synchronous trigger. bit.",
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
                    name: "uep_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint 7 for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint 7 receives the end of NAK flag.",
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
                    name: "uep_r_done",
                    description: Some(
                        "endpoint 7 receives the end flag.",
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
                "endpoint 7 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep7_rx_dma",
                    description: Some(
                        "endpoint receives the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep7RxFifo",
            extends: None,
            description: Some(
                "Receive FIFO address of usb endpoint 7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_rx_fifo_s",
                    description: Some(
                        "The FIFO start address of the receiving endpoint.",
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
                    name: "uep_rx_fifo_e",
                    description: Some(
                        "The end FIFO address of the receiving endpoint.",
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
            name: "Uep7RxLen",
            extends: None,
            description: Some(
                "endpoint 7 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep7_rx_len",
                    description: Some(
                        "the length of data received by endpoint 7 in a single pass.",
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
            name: "Uep7TLen",
            extends: None,
            description: Some(
                "endpoint 7 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep7_t_len",
                    description: Some(
                        "endpoint 7 send the length.",
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
            name: "Uep7TxCtrl",
            extends: None,
            description: Some(
                "endpoint 7 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_res_mask",
                    description: Some(
                        "endpoint 7 control of the send response to IN transactions.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "endpoint 7 synchronous trigger bit for the sender to. prepare.",
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
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint 7 sends the end flag.",
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
                    name: "uep_t_done",
                    description: Some(
                        "endpoint 7 sends the end of NAK flag.",
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
                "endpoint 7 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep7_tx_dma",
                    description: Some(
                        "endpoint sends the start address register of the buffer.",
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
            ],
        },
        FieldSet {
            name: "Uep7TxFifo",
            extends: None,
            description: Some(
                "The sending FIFO address of usb endpoint 7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_tx_fifo_s",
                    description: Some(
                        "The FIFO start address of the sending endpoint.",
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
                    name: "uep_tx_fifo_e",
                    description: Some(
                        "The end FIFO address of the sending endpoint.",
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
            name: "UepAfMode",
            extends: None,
            description: Some(
                "USB endpoint muitiplexing register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_t_af",
                    description: Some(
                        "1 to 7 endpoint muitiplexing enables.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRBurst",
            extends: None,
            description: Some(
                "USB endpoint receives the burst register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_burst_en",
                    description: Some(
                        "0 to 7 endpoint receive burst enable.",
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
            name: "UepRIso",
            extends: None,
            description: Some(
                "usb endpoint receives a synchronous mode enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uepn_r_iso_en",
                    description: Some(
                        "down endpoint(OUT) synchronization mode enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_r_fifo_en",
                    description: Some(
                        "The FIFO mode of the TX of the endpoint is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRResMode",
            extends: None,
            description: Some(
                "USB endpoint reply mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mode",
                    description: Some(
                        "0 to 7 endpoint reply mode.",
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
            name: "UepRTogAuto",
            extends: None,
            description: Some(
                "USB endpoint receive the auto-filp enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_r_tog_auto",
                    description: Some(
                        "0 to 7 endpoint synchronization triggers bit auto-filp. enable.",
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
            name: "UepRxEn",
            extends: None,
            description: Some(
                "USB endpoint receive the enable registers.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_rx_en",
                    description: Some(
                        "Endpoint 0 to 15 receive enable.",
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
            name: "UepTBurst",
            extends: None,
            description: Some(
                "USB endpoint sends a burst register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_burst_en",
                    description: Some(
                        "0 to 7 endpoint send burst enable.",
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
            name: "UepTBurstMode",
            extends: None,
            description: Some(
                "USB endpoint send the mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_burst_mode",
                    description: Some(
                        "0 to 7 endpoint send the mode enable.",
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
            name: "UepTIso",
            extends: None,
            description: Some(
                "usb endpoint sends a synchronous mode enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uepn_t_iso_en",
                    description: Some(
                        "upload endpoint(IN) synchronization mode enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_fifo_en",
                    description: Some(
                        "The FIFO mode of the TX of the endpoint is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTTogAuto",
            extends: None,
            description: Some(
                "USB endpoint sends the auto-filp enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_t_tog_auto",
                    description: Some(
                        "0 to 7 endpoint synchronization triggers bit auto-filp. enable.",
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
            name: "UepTxEn",
            extends: None,
            description: Some(
                "USB endpoint sends the enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_tx_en",
                    description: Some(
                        "Endpoint 0 to 15 sends enable.",
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
            name: "UhBcCtrl",
            extends: None,
            description: Some(
                "USB host BC charging control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "udp_bc_cmpo",
                    description: Some(
                        "UDP pin BC protocol comparator status.",
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
                    name: "udm_bc_cmpo",
                    description: Some(
                        "UDM pin BC protocol comparator status.",
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
                    name: "udp_bc_cmpe",
                    description: Some(
                        "UDP pin BC protocol comparator enables.",
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
                    name: "udm_bc_cmpe",
                    description: Some(
                        "UDM pin BC protocol comparator enables.",
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
                    name: "bc_auto_mode",
                    description: Some(
                        "Automatic mode enables.",
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
                    name: "udp_bc_vsrc",
                    description: Some(
                        "UDP pin BC protocol source voltage enabled.",
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
                    name: "udm_bc_vsrc",
                    description: Some(
                        "UDM pin BC protocol source voltage enabled.",
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
                    name: "udm_vsrc_act",
                    description: Some(
                        "In automatic mode, UDP outputs VBC_SRC ,otherwise it is. controlled by UDM_BC_CMPE.",
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
            name: "UhCfg",
            extends: None,
            description: Some(
                "USB host Configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_rst_link",
                    description: Some(
                        "USB connection controller module resets.",
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
                    name: "uh_rst_sie",
                    description: Some(
                        "USB protocol processor reset.",
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
                    name: "uh_clr_all",
                    description: Some(
                        "clear all interrupt flags.",
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
                    name: "uh_phy_suspendm",
                    description: Some(
                        "PHY suspend,close utmi clock.",
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
                    name: "uh_dma_en",
                    description: Some(
                        "DMA transfer enabled.",
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
                    name: "uh_sof_en",
                    description: Some(
                        "sof packet sending function enabled.",
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
                    name: "uh_force_fs",
                    description: Some(
                        "forced full speed FS.",
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
                    name: "uh_lpm_en",
                    description: Some(
                        "LPM enable.",
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
            name: "UhControl",
            extends: None,
            description: Some(
                "USB host control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_t_token_mask",
                    description: Some(
                        "transaction token packet PID.",
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
                    name: "uh_t_endp_mask",
                    description: Some(
                        "transaction token packet endpoint number.",
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
                    name: "uh_t_tog_mask",
                    description: Some(
                        "send data PID.",
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
                    name: "uh_buf_mode",
                    description: Some(
                        "DATA cache control bit.",
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
                    name: "uh_host_action",
                    description: Some(
                        "host enables the transaction.",
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
                    name: "uh_lpm_valid",
                    description: Some(
                        "valid to send LMP packet.",
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
                    name: "uh_split_valid",
                    description: Some(
                        "valid to send SPLIT packet.",
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
                    name: "uh_pre_pid_en",
                    description: Some(
                        "this bit is enabled when the port is operating at full. speed and needs to send low speed packets(PRE).",
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
                    name: "uh_tx_no_data",
                    description: Some(
                        "OUT or SETUP token packet packets are followed by no data. packets for hign-speed SPLIT packets.",
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
                    name: "uh_rx_no_data",
                    description: Some(
                        "DATA packets are not expected after IN token packets and. are used for  high-speed SPLIT packets.",
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
                    name: "uh_tx_no_res",
                    description: Some(
                        "OUT or SETUP DATA no reply is extended and is used for. synchronous transmission or hign-speed SPLIT packets.",
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
                    name: "uh_rx_no_res",
                    description: Some(
                        "IN-DATA no answer,used for synchronous transfer or. high-speed SPLIT packets.",
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
            name: "UhDevAd",
            extends: None,
            description: Some(
                "USB host device address register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_dev_addr",
                    description: Some(
                        "device address.",
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
            name: "UhFrame",
            extends: None,
            description: Some(
                "USB host frame register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_frame_no",
                    description: Some(
                        "the frame number.",
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
                    name: "uh_mframe_no",
                    description: Some(
                        "microfame number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uh_sof_cnt_en",
                    description: Some(
                        "SOF count enabled.",
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
                    name: "uh_sof_cnt_clr",
                    description: Some(
                        "the SOF count is cleared.",
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
            ],
        },
        FieldSet {
            name: "UhIntEn",
            extends: None,
            description: Some(
                "USB host interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uhie_wkup_act",
                    description: Some(
                        "Wakeup interrupt enabled.",
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
                    name: "uhie_resume_act",
                    description: Some(
                        "bus recovery interrupt was enabled.",
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
                    name: "uhie_transfer",
                    description: Some(
                        "USB transfer end interrupt enabled.",
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
                    name: "uhie_sof_act",
                    description: Some(
                        "sof packet sending interruption is enabled.",
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
                    name: "uhie_tx_halt",
                    description: Some(
                        "send pause interrupt enable.",
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
                    name: "uhie_fifo_over",
                    description: Some(
                        "FIFO overflow interrupt enable.",
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
            name: "UhIntFlag",
            extends: None,
            description: Some(
                "USB HOST interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uhif_wkup_act",
                    description: Some(
                        "Wakeup interrupt flag.",
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
                    name: "uhif_resume_act_if",
                    description: Some(
                        "bus recovery interrupt flag.",
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
                    name: "uhif_transfer",
                    description: Some(
                        "USB transaction transfer completion interrupt flag.",
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
                    name: "uhif_sof_act",
                    description: Some(
                        "sof packet delivert completion interrupt flag.",
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
                    name: "uhif_tx_halt",
                    description: Some(
                        "send pause interrupt flag.",
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
                    name: "uhif_fifo_over",
                    description: Some(
                        "FIFO overflow interrupt flag.",
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
            name: "UhIntSt",
            extends: None,
            description: Some(
                "USB host interrupt status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_r_token_mask",
                    description: Some(
                        "received PID.",
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
                    name: "uhis_port_rx_resume",
                    description: Some(
                        "A bit of indicates that the port received a wakeup signal.",
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
            ],
        },
        FieldSet {
            name: "UhLpmData",
            extends: None,
            description: Some(
                "USB host power management data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_lpm_data",
                    description: Some(
                        "power management data.",
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
            name: "UhMisSt",
            extends: None,
            description: Some(
                "USB host miscellaneous status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uhms_sof_free",
                    description: Some(
                        "port enabled state.",
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
                    name: "uhms_sof_pre",
                    description: Some(
                        "The state of the USB SOF packet is indicated as follows.",
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
                    name: "uhms_sof_act",
                    description: Some(
                        "USB bus SOF status.",
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
                    name: "uhms_usb_wakeup",
                    description: Some(
                        "USB bus wakes up.",
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
                    name: "uhms_linestate",
                    description: Some(
                        "USB bus status.",
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
                    name: "uhms_bus_j",
                    description: Some(
                        "J on USB bus.",
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
                    name: "uhms_bus_se0",
                    description: Some(
                        "SE0 on USB bus.",
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
            name: "UhPortCfg",
            extends: None,
            description: Some(
                "USB host port Configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_host_en",
                    description: Some(
                        "USB port mode selection.",
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
                    name: "uh_pd_en",
                    description: Some(
                        "the 15k resistance drop-down function is enable in host. mode.",
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
            name: "UhPortChg",
            extends: None,
            description: Some(
                "USB host port state charge register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uhif_port_connect",
                    description: Some(
                        "port connection state charge.",
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
                    name: "uhif_port_en",
                    description: Some(
                        "port enable state charge.",
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
                    name: "uhif_port_susp",
                    description: Some(
                        "port pause state charge.",
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
                    name: "uhif_port_reset",
                    description: Some(
                        "port reset state charge.",
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
                    name: "uhif_port_slp",
                    description: Some(
                        "port sleep state charge.",
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
            name: "UhPortCtrl",
            extends: None,
            description: Some(
                "USB host port control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_set_port_reset",
                    description: Some(
                        "PORT send reset.",
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
                    name: "uh_set_port_susp",
                    description: Some(
                        "the PORT is in a suspended state.",
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
                    name: "uh_clr_port_susp",
                    description: Some(
                        "the PORT exits the suspended state.",
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
                    name: "uh_set_port_sleep",
                    description: Some(
                        "the PORT goes to sleep(LPM).",
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
                    name: "uh_clr_port_en",
                    description: Some(
                        "PORT exits the enabled state and enters the DISABLED state.",
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
                    name: "uh_clr_port_connect",
                    description: Some(
                        "bring the PORT into the port state.",
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
                    name: "uh_clr_port_sleep",
                    description: Some(
                        "the PORT exits the sleep state (LPM).",
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
                    name: "uh_port_sleep_besl",
                    description: Some(
                        "wake-up time control.",
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
                    name: "uh_bus_rst_long",
                    description: Some(
                        "BUS reset time selection.",
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
            ],
        },
        FieldSet {
            name: "UhPortIntEn",
            extends: None,
            description: Some(
                "USB host port interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uhie_port_connect",
                    description: Some(
                        "port connection state change interrupt enabled.",
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
                    name: "uhie_port_en",
                    description: Some(
                        "port enable state change interrupt enabled.",
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
                    name: "uhie_port_susp",
                    description: Some(
                        "port pause state change interrupt enabled.",
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
                    name: "uhie_port_reset",
                    description: Some(
                        "port reset state change interrupt enabled.",
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
                    name: "uhie_port_slp",
                    description: Some(
                        "port sleep state change interrupt enabled.",
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
            name: "UhPortSt",
            extends: None,
            description: Some(
                "USB host port status register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uhis_port_c0nnect",
                    description: Some(
                        "port connection state.",
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
                    name: "uhis_port_en",
                    description: Some(
                        "port enabled.",
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
                    name: "uhis_port_susp",
                    description: Some(
                        "port pause state.",
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
                    name: "uhis_port_rst",
                    description: Some(
                        "port reset status.",
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
                    name: "uhis_port_slp",
                    description: Some(
                        "port sleep.",
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
                    name: "uhis_port_ls",
                    description: Some(
                        "whether the port connection speed is low.",
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
                    name: "uhis_port_hs",
                    description: Some(
                        "whether the port connection speed is hign.",
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
                    name: "uhis_port_test",
                    description: Some(
                        "whether the port is in test mode.",
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
            ],
        },
        FieldSet {
            name: "UhPortTestCt",
            extends: None,
            description: Some(
                "USB host port test mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_test_j",
                    description: Some(
                        "test output J.",
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
                    name: "uh_test_k",
                    description: Some(
                        "test output K.",
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
                    name: "uh_test_force_en",
                    description: Some(
                        "test mode was enabled.",
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
                    name: "uh_test_packet",
                    description: Some(
                        "test packet.",
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
                    name: "uh_test_se0_nak",
                    description: Some(
                        "test SE0 NAK.",
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
            ],
        },
        FieldSet {
            name: "UhRxDma",
            extends: None,
            description: Some(
                "DMA receive address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_dma",
                    description: Some(
                        "Received address.",
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
            ],
        },
        FieldSet {
            name: "UhRxLen",
            extends: None,
            description: Some(
                "USB host receive length register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_len",
                    description: Some(
                        "Received data length.",
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
            name: "UhRxMaxLen",
            extends: None,
            description: Some(
                "USB host receives the maximum length register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_max_len",
                    description: Some(
                        "receives the maximum length.",
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
            name: "UhSplitData",
            extends: None,
            description: Some(
                "USB host SPLIT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_split_data",
                    description: Some(
                        "SPLIT management data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhTxDma",
            extends: None,
            description: Some(
                "DMA send address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_tx_dma",
                    description: Some(
                        "Send address.",
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
            ],
        },
        FieldSet {
            name: "UhTxLen",
            extends: None,
            description: Some(
                "USB host send length register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_tx_len",
                    description: Some(
                        "send data length.",
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
            name: "UsbBaseMode",
            extends: None,
            description: Some(
                "USB mode control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ud_speed_type",
                    description: Some(
                        "The desired speed mode of the device.",
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
            ],
        },
        FieldSet {
            name: "UsbBus",
            extends: None,
            description: Some(
                "USB bus.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "usb_wakeup",
                    description: Some(
                        "USB wakeup.",
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
                    name: "usb_dp_st",
                    description: Some(
                        "UDP status.",
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
                    name: "usb_dm_st",
                    description: Some(
                        "UDM status.",
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
        FieldSet {
            name: "UsbCtrl",
            extends: None,
            description: Some(
                "USB base control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ud_rst_link",
                    description: Some(
                        "LINK layer reset,highly effective.",
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
                    name: "ud_rst_sie",
                    description: Some(
                        "USB protocol processor reset.",
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
                    name: "ud_clr_all",
                    description: Some(
                        "clear all interrupt flags.",
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
                    name: "ud_phy_suspendm",
                    description: Some(
                        "USB PHY suspend.",
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
                    name: "ud_dma_en",
                    description: Some(
                        "DMA transfer enabled.",
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
                    name: "ud_dev_en",
                    description: Some(
                        "USB device enabled.",
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
                    name: "ud_lpm_en",
                    description: Some(
                        "LPM enable.",
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
            name: "UsbDevAd",
            extends: None,
            description: Some(
                "USB device address.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_usb_addr",
                    description: Some(
                        "bit mask for USB device address.",
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
            name: "UsbFramNo",
            extends: None,
            description: Some(
                "USB frame number.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ud_frame_no",
                    description: Some(
                        "Received frame number.",
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
                    name: "ud_mframe_no",
                    description: Some(
                        "Received micro frame number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbIntEn",
            extends: None,
            description: Some(
                "USB interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "udie_bus_rst",
                    description: Some(
                        "USB bus reset interrupt enabled.",
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
                    name: "udie_suspend",
                    description: Some(
                        "USB bus pause interrupt enabled.",
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
                    name: "udie_bus_sleep",
                    description: Some(
                        "USB bus sleep interrupt enabled.",
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
                    name: "udie_lpm_act",
                    description: Some(
                        "LMP transfer end interrupt enabled.",
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
                    name: "udie_transfer",
                    description: Some(
                        "USB transfer end interrupt enabled.",
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
                    name: "udie_sof_act",
                    description: Some(
                        "Receive SOF packet interrupt enable.",
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
                    name: "udie_link_rdy",
                    description: Some(
                        "USB connection interrupt enable.",
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
                    name: "udie_fifo_over",
                    description: Some(
                        "USB Overflow interrupt enable.",
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
            name: "UsbIntFg",
            extends: None,
            description: Some(
                "USB interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "udif_bus_rst",
                    description: Some(
                        "USB bus reset interrupt flag.",
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
                    name: "udif_suspend",
                    description: Some(
                        "USB bus suspend interrupt flag.",
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
                    name: "udif_bus_sleep",
                    description: Some(
                        "USB bus sleep interrupt flag.",
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
                    name: "udif_lpm_act",
                    description: Some(
                        "LPM transmission end interrupt flag.",
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
                    name: "udif_rtx_act",
                    description: Some(
                        "USB transmission end interrupt flag.",
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
                    name: "udif_rx_sof",
                    description: Some(
                        "Receive SOF packet interrupt flag.",
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
                    name: "udif_link_rdy",
                    description: Some(
                        "USB connection interrupt flag.",
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
                    name: "udif_fifo_ov",
                    description: Some(
                        "USB Overflow interrupt flag.",
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
            name: "UsbIntSt",
            extends: None,
            description: Some(
                "USB interrupt status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "udis_ep_id_mask",
                    description: Some(
                        "The endpoint number at which the data transfer occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udis_ep_dir",
                    description: Some(
                        "Endpoint data transmission direction.",
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
            ],
        },
        FieldSet {
            name: "UsbLpmData",
            extends: None,
            description: Some(
                "USB power management register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ud_lpm_data",
                    description: Some(
                        "power management data.",
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
                    name: "ud_lpm_busy",
                    description: Some(
                        "power management busy.",
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
            name: "UsbMisSt",
            extends: None,
            description: Some(
                "USB miscellaneous status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "udms_ready",
                    description: Some(
                        "USB connection status.",
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
                    name: "udms_suspend",
                    description: Some(
                        "USB suspend status.",
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
                    name: "udms_sleep",
                    description: Some(
                        "USB sleep status.",
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
                    name: "udms_sie_free",
                    description: Some(
                        "USB free status.",
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
                    name: "udms_susp_req",
                    description: Some(
                        "USB suspends the request.",
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
                    name: "udms_hs_mod",
                    description: Some(
                        "whether the host is high-speed.",
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
            name: "UsbTestMode",
            extends: None,
            description: Some(
                "USB test mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ud_test_j",
                    description: Some(
                        "test mode,output J.",
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
                    name: "ud_test_k",
                    description: Some(
                        "test mode,output K.",
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
                    name: "ud_test_pkt",
                    description: Some(
                        "test mode,output a packet.",
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
                    name: "ud_test_se0nak",
                    description: Some(
                        "test mode,output SEO.",
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
                    name: "ud_test_en",
                    description: Some(
                        "test mode enable.",
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
            name: "UsbWakeCtrl",
            extends: None,
            description: Some(
                "USB remote wake up register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ud_remote_wkup",
                    description: Some(
                        "remote wake up.",
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
    ],
    enums: &[],
};
