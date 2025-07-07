use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usb",
            extends: None,
            description: Some(
                "USB register. USBHS, host/device interface, for V305, and V307",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "USB base control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "USB interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dev_ad",
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
                                "DevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "frame_no",
                    description: Some(
                        "FRAME_NO.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "FrameNo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "suspend",
                    description: Some(
                        "indicate USB suspend status.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Suspend",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "speed_type",
                    description: Some(
                        "USB current speed type.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "SpeedType",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "MisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_fg",
                    description: Some(
                        "USB interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "IntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_len",
                    description: Some(
                        "USB receiving length.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Usbd",
            extends: Some(
                "USB",
            ),
            description: Some(
                "USB device endpoint register block",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "USB base control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "USB interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dev_ad",
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
                                "DevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "frame_no",
                    description: Some(
                        "FRAME_NO.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "FrameNo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "suspend",
                    description: Some(
                        "indicate USB suspend status.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Suspend",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "speed_type",
                    description: Some(
                        "USB current speed type.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "SpeedType",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "MisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_fg",
                    description: Some(
                        "USB interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "IntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_len",
                    description: Some(
                        "USB receiving length.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ep_config",
                    description: Some(
                        "USB endpoint configuration.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EpConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep_type",
                    description: Some(
                        "Endpoint type configuration register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EpType",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep_buf_mod",
                    description: Some(
                        "USB endpoint buffer mode.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EpBufMod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep0_dma",
                    description: Some(
                        "endpoint 0 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ep_rx_dma",
                    description: Some(
                        "endpoint n (n=1-15) DMA RX buffer address.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ep_tx_dma",
                    description: Some(
                        "endpoint n (n=1-15) DMA TX buffer address.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ep_max_len",
                    description: Some(
                        "endpoint n (n=0-15) max acceptable length.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EpLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep_t_len",
                    description: Some(
                        "endpoint n (n=0-15) send length.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EpLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep_tx_ctrl",
                    description: Some(
                        "endpoint n (n=0-15) send control.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xda,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpTxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep_rx_ctrl",
                    description: Some(
                        "endpoint n (n=0-15) receive control.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xdb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EpRxCtrl",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Usbh",
            extends: Some(
                "USB",
            ),
            description: Some(
                "USB host register block",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "USB HOST control.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "USB interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dev_ad",
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
                                "DevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "frame_no",
                    description: Some(
                        "FRAME_NO.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "FrameNo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "suspend",
                    description: Some(
                        "indicate USB suspend status.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Suspend",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "speed_type",
                    description: Some(
                        "USB current speed type.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "SpeedType",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "MisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_fg",
                    description: Some(
                        "USB interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "IntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_len",
                    description: Some(
                        "USB receiving length.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "config",
                    description: Some(
                        "USB endpoint configuration.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep_type",
                    description: Some(
                        "USB endpoint type.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhEpType",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_dma",
                    description: Some(
                        "USB host receive buffer start address",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "tx_dma",
                    description: Some(
                        "USB host transmit buffer start address",
                    ),
                    array: None,
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
                    name: "rx_max_len",
                    description: Some(
                        "USB host receive maximum length packet register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EpLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep_pid",
                    description: Some(
                        "USB host token setup register",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhEpPid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_ctrl",
                    description: Some(
                        "USB host receive control register",
                    ),
                    array: None,
                    byte_offset: 0xe3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhRxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_len",
                    description: Some(
                        "USB host transmit length register",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EpLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_ctrl",
                    description: Some(
                        "USB host transmit control register",
                    ),
                    array: None,
                    byte_offset: 0xe6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhTxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "split_data",
                    description: Some(
                        "USB host transmit data of the SPLIT packet",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UhSplitData",
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
                "USB base control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_en",
                    description: Some(
                        "DMA enable and DMA interrupt enable for USB.",
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
                    name: "clr_all",
                    description: Some(
                        "force clear FIFO and count of USB.",
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
                    name: "reset_sie",
                    description: Some(
                        "force reset USB SIE, need software clear.",
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
                    name: "int_busy",
                    description: Some(
                        "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid.",
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
                    name: "dev_pu_en",
                    description: Some(
                        "USB device enable and internal pullup resistance enable.",
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
                    name: "speed_type",
                    description: Some(
                        "enable USB low speed: 00=full speed, 01=high speed, 10 =low speed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "SpeedType",
                    ),
                },
                Field {
                    name: "host_mode",
                    description: Some(
                        "enable USB host mode: 0=device mode, 1=host mode.",
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
            name: "DevAd",
            extends: None,
            description: Some(
                "USB device address.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "USB device address.",
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
            name: "EpBufMod",
            extends: None,
            description: Some(
                "USB endpoint buffer mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint.",
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
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "iso_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EpConfig",
            extends: None,
            description: Some(
                "USB endpoint configuration register (R32_UEP_CONFIG)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_en",
                    description: Some(
                        "Endpoint 1 to 15 transmit enable",
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
                Field {
                    name: "r_en",
                    description: Some(
                        "Endpoint 1 to 15 receive enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
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
            name: "EpLen",
            extends: None,
            description: Some(
                "endpoint n acceptable length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "len",
                    description: Some(
                        "endpoint n acceptable length.",
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
            name: "EpRxCtrl",
            extends: None,
            description: Some(
                "endpoint n receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_r_res",
                    description: Some(
                        "MASK_UEP_R_RES",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "EpRxResponse",
                    ),
                },
                Field {
                    name: "mask_uep_r_tog",
                    description: Some(
                        "MASK_UEP_R_TOG",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "EpTog",
                    ),
                },
                Field {
                    name: "r_tog_auto",
                    description: Some(
                        "endpoint n synchronous trigger bit automatic filp enables the control bit.",
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
            name: "EpTxCtrl",
            extends: None,
            description: Some(
                "endpoint n send control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
                    description: Some(
                        "MASK_UEP_T_RES",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "EpTxResponse",
                    ),
                },
                Field {
                    name: "mask_uep_t_tog",
                    description: Some(
                        "MASK_UEP_T_TOG",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "EpTog",
                    ),
                },
                Field {
                    name: "t_tog_auto",
                    description: Some(
                        "endpoint n synchronous trigger bit automatic filp enables the control bit.",
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
            name: "EpType",
            extends: None,
            description: Some(
                "USB endpoint type control register (R32_UEP_TYPE)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_type",
                    description: Some(
                        "Endpoint 1 to 15 transmit type, 1 means synchronous transmission",
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
                    enumm: Some(
                        "EndpointType",
                    ),
                },
                Field {
                    name: "r_type",
                    description: Some(
                        "Endpoint 1 to 15 receive type, 1 means synchronous transmission",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
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
                    enumm: Some(
                        "EndpointType",
                    ),
                },
            ],
        },
        FieldSet {
            name: "FrameNo",
            extends: None,
            description: Some(
                "FRAME_NO.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "frame_no",
                    description: Some(
                        "FRAME_NO.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "USB interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bus_rst",
                    description: Some(
                        "enable interrupt for USB bus reset event for USB device mode.",
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
                    name: "detect",
                    description: Some(
                        "enable interrupt for USB device detected event for USB host mode.",
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
                    name: "transfer",
                    description: Some(
                        "enable interrupt for USB transfer completion.",
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
                    name: "suspend",
                    description: Some(
                        "enable interrupt for USB suspend or resume event.",
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
                    name: "sof_act",
                    description: Some(
                        "indicate host SOF timer action status for USB host.",
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
                    name: "fifo_ov",
                    description: Some(
                        "enable interrupt for FIFO overflow.",
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
                    name: "setup_act",
                    description: Some(
                        "indicate host SETUP timer action status for USB host.",
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
                    name: "iso_act",
                    description: Some(
                        "enable interrupt for NAK responded for USB device mode.",
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
                    name: "dev_nak",
                    description: Some(
                        "enable interrupt for NAK responded for USB device mode.",
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
            name: "IntFg",
            extends: None,
            description: Some(
                "USB interrupt flag.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bus_rst",
                    description: Some(
                        "in USB device mode, USB bus reset event interrupt flag, write 1 to clear.",
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
                    name: "detect",
                    description: Some(
                        "in USB host mode, USB device connect or disconnect event interrupt flag, write 1 to clear.",
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
                    name: "transfer",
                    description: Some(
                        "USB transfer completion interrupt flag, direct bit address clear or write 1 to clear.",
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
                    name: "suspend",
                    description: Some(
                        "USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear.",
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
                    name: "hst_sof",
                    description: Some(
                        "host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear.",
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
                    name: "fifo_ov",
                    description: Some(
                        "FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear.",
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
                    name: "setup_act",
                    description: Some(
                        "SETUP transaction completion interrupt flag, write 1 to clear.",
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
                    name: "iso_act",
                    description: Some(
                        "isochronous transmission starts to send/receive data interrupt flag, write 1 to clear.",
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
                    name: "is_nak",
                    description: Some(
                        "RO, indicate current USB transfer is NAK received.",
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
            name: "IntSt",
            extends: None,
            description: Some(
                "USB interrupt status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "endp",
                    description: Some(
                        "RO, bit mask of current transfer endpoint number for USB device mode.",
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
                    name: "h_res",
                    description: Some(
                        "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received.",
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
                    name: "token",
                    description: Some(
                        "RO, bit mask of current token PID code received for USB device mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "UsbToken",
                    ),
                },
                Field {
                    name: "tog_ok",
                    description: Some(
                        "RO, indicate current USB transfer toggle is OK.",
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
                    name: "is_nak",
                    description: Some(
                        "RO, indicate current USB transfer is NAK received for USB device mode.",
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
            name: "MisSt",
            extends: None,
            description: Some(
                "USB miscellaneous status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "split_can",
                    description: Some(
                        "RO, in USB host mode, SPLIT packet transmit enable.",
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
                    name: "dev_attach",
                    description: Some(
                        "RO, USB device attach status for the port in USB host mode.",
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
                    name: "suspend",
                    description: Some(
                        "RO, indicate USB suspend status.",
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
                    name: "bus_reset",
                    description: Some(
                        "RO, indicate USB bus reset status.",
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
                    name: "r_fifo_rdy",
                    description: Some(
                        "RO, indicate USB receiving FIFO ready status (not empty).",
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
                    name: "sie_free",
                    description: Some(
                        "RO, indicate USB SIE free status.",
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
                    name: "sof_act",
                    description: Some(
                        "RO, indicate host SOF timer action status for USB host.",
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
                    name: "sof_pres",
                    description: Some(
                        "RO, indicate host SOF timer presage status.",
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
            name: "SpeedType",
            extends: None,
            description: Some(
                "USB current speed type.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "speed_type",
                    description: Some(
                        "in host mode, it indicates the speed type of the currently connected device; in device mode, it indicates the speed type of the current device.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "SpeedType",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Suspend",
            extends: None,
            description: Some(
                "indicate USB suspend status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "sys_mod",
                    description: Some(
                        "SYS_MOD.",
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
                    name: "wakeup",
                    description: Some(
                        "remote resume.",
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
                    name: "linestate",
                    description: Some(
                        "LINESTATE.",
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
            ],
        },
        FieldSet {
            name: "UhConfig",
            extends: None,
            description: Some(
                "USB endpoint configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "h_tx_en",
                    description: Some(
                        "host TX enable.",
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
                    name: "h_rx_en",
                    description: Some(
                        "host RX enable.",
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
            ],
        },
        FieldSet {
            name: "UhCtrl",
            extends: None,
            description: Some(
                "USB HOST control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_bus_reset",
                    description: Some(
                        "USB host bus reset status.",
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
                    name: "tx_bus_suspend",
                    description: Some(
                        "the host sends hang sigal.",
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
                    name: "tx_bus_resume",
                    description: Some(
                        "host wake up device.",
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
                    name: "remote_wkup",
                    description: Some(
                        "the remoke wake-up.",
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
                    name: "phy_suspendm",
                    description: Some(
                        "USB-PHY thesuspended state the internal USB-PLL is turned off.",
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
                    name: "sof_free",
                    description: Some(
                        "the bus is idle.",
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
                    name: "sof_en",
                    description: Some(
                        "automatically generate the SOF packet enabling control bit.",
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
            name: "UhEpPid",
            extends: None,
            description: Some(
                "host token setup register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "endp",
                    description: Some(
                        "set the endpoint number of the target device.",
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
                    name: "token",
                    description: Some(
                        "set the token PID of this usb transaction.",
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
            ],
        },
        FieldSet {
            name: "UhEpType",
            extends: None,
            description: Some(
                "USB endpoint type.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "h_tx_type",
                    description: Some(
                        "host TX type.",
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
                    name: "h_rx_type",
                    description: Some(
                        "host RX type.",
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
            ],
        },
        FieldSet {
            name: "UhRxCtrl",
            extends: None,
            description: Some(
                "USB host receive control register",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "r_res",
                    description: Some(
                        "host control of the accept response to IN transactions.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "HostRxResponse",
                    ),
                },
                Field {
                    name: "r_res_no",
                    description: Some(
                        "H_R_RES_NO",
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
                    name: "r_tog",
                    description: Some(
                        "host synchronous trigger bit for the accept to prepare.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Tog",
                    ),
                },
                Field {
                    name: "r_auto_tog",
                    description: Some(
                        "host synchronization trigger bit auto toggle enable.",
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
                    name: "r_data_no",
                    description: Some(
                        "expect data packet (IN)",
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
            name: "UhSplitData",
            extends: None,
            description: Some(
                "data content of the split packet sent by the host.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "split_data",
                    description: Some(
                        "data content of the split packet sent by the host.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhTxCtrl",
            extends: None,
            description: Some(
                "USB host transmit control register",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "t_res",
                    description: Some(
                        "USB host transmitter response control bits to SETUP/OUT transactions",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "HostTxResponse",
                    ),
                },
                Field {
                    name: "t_res_no",
                    description: Some(
                        "expect a response after sending data successfully.",
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
                    name: "t_tog",
                    description: Some(
                        "sync trigger bit prepared by USB host transmitter (handling SETUP/OUT transactions)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Tog",
                    ),
                },
                Field {
                    name: "t_auto_tog",
                    description: Some(
                        "host synchronization trigger bit auto toggle enable.",
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
                    name: "t_data_no",
                    description: Some(
                        "send data packets (OUT/SETUP).",
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
    ],
    enums: &[
        Enum {
            name: "EndpointType",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NISO",
                    description: Some(
                        "Non Isochronous (Interrupt/Bulk)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ISO",
                    description: Some(
                        "Isochronous Transfer",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "EpRxResponse",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ACK",
                    description: Some(
                        "Respond with ACK",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NYET",
                    description: Some(
                        "Respond NYET",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "NAK",
                    description: Some(
                        "Respond with NAK(Busy)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "STALL",
                    description: Some(
                        "Respond with STALL(Error)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "EpTog",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DATA0",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DATA1",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DATA2",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "MDATA",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "EpTxResponse",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ACK",
                    description: Some(
                        "Respond with DATA0/DATA1 and expect ACK",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NAK",
                    description: Some(
                        "Respond with NAK or Busy",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "STALL",
                    description: Some(
                        "Respond with STALL or Error",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "HostRxResponse",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ACK",
                    description: Some(
                        "Respond with ACK",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "HostTxResponse",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ACK",
                    description: Some(
                        "Expect ACK",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NYET",
                    description: Some(
                        "Expect NYET",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "NAK",
                    description: Some(
                        "Expect NAK(Busy)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "STALL",
                    description: Some(
                        "Expect STALL(Error)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "SpeedType",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FULLSPEED",
                    description: Some(
                        "USB Full Speed (12Mbps)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGHSPEED",
                    description: Some(
                        "USB High Speed (480Mbps)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LOWSPEED",
                    description: Some(
                        "USB Low Speed (1.5Mbps)",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Tog",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DATA0",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DATA1",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DATA2",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "MDATA",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "UsbToken",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "OUT",
                    description: Some(
                        "OUT Packet",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SOF",
                    description: Some(
                        "Start of Frame",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "IN",
                    description: Some(
                        "IN Packet",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SETUP",
                    description: Some(
                        "SETUP Packet",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
