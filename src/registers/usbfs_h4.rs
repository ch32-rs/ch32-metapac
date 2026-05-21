use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "UsbOtgFs",
            extends: None,
            description: Some(
                "USB FS OTG register.",
            ),
            items: &[
                BlockItem {
                    name: "usb_ctrl",
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
                                "UsbCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "udev_ctrl__uhost_ctrl",
                    description: Some(
                        "USB device/host physical prot control.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UdevCtrl_uhostCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_int_en",
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
                    name: "usb_mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0x5,
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
                    name: "usb_int_fg",
                    description: Some(
                        "USB interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0x6,
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
                    byte_offset: 0x7,
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
                    name: "usb_rx_len",
                    description: Some(
                        "USB receiving length.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_1_mod",
                    description: Some(
                        "endpoint 4/1 mode.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep41Mod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_3_mod__uh_ep_mod",
                    description: Some(
                        "endpoint 2/3 mode;host endpoint mode.",
                    ),
                    array: None,
                    byte_offset: 0xd,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep23Mod_uhEpMod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_6_mod",
                    description: Some(
                        "endpoint 5/6 mode.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep56Mod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_mod",
                    description: Some(
                        "endpoint 7 mode.",
                    ),
                    array: None,
                    byte_offset: 0xf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7Mod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_dma",
                    description: Some(
                        "endpoint 0 DMA buffer address.",
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
                    name: "uep1_dma",
                    description: Some(
                        "endpoint 1 DMA buffer address.",
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
                BlockItem {
                    name: "uep2_dma__uh_rx_dma",
                    description: Some(
                        "endpoint 2 DMA buffer address;host rx endpoint buffer high address.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_dma__uh_tx_dma",
                    description: Some(
                        "endpoint 3 DMA buffer address;host tx endpoint buffer high address.",
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
                    name: "uep4_dma",
                    description: Some(
                        "endpoint 4 DMA buffer address.",
                    ),
                    array: None,
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
                    name: "uep5_dma",
                    description: Some(
                        "endpoint 5 DMA buffer address.",
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
                    name: "uep6_dma",
                    description: Some(
                        "endpoint 6 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_dma",
                    description: Some(
                        "endpoint 7 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_t_len",
                    description: Some(
                        "endpoint 0 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_t_ctrl",
                    description: Some(
                        "endpoint 0 control.",
                    ),
                    array: None,
                    byte_offset: 0x32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep0TCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_r_ctrl",
                    description: Some(
                        "endpoint 0 control.",
                    ),
                    array: None,
                    byte_offset: 0x33,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep0RCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_t_len",
                    description: Some(
                        "endpoint 1 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_t_ctrl___usbhd_uh_setup",
                    description: Some(
                        "endpoint 1 control.",
                    ),
                    array: None,
                    byte_offset: 0x36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TCtrl_UsbhdUhSetup",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_r_ctrl",
                    description: Some(
                        "endpoint 1 control.",
                    ),
                    array: None,
                    byte_offset: 0x37,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_t_len__usbhd_uh_ep_pid",
                    description: Some(
                        "endpoint 2 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TLen_usbhdUhEpPid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_t_ctrl",
                    description: Some(
                        "endpoint 2 control.",
                    ),
                    array: None,
                    byte_offset: 0x3a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_r_ctrl__usbhd_uh_rx_ctrl",
                    description: Some(
                        "endpoint 2 control.",
                    ),
                    array: None,
                    byte_offset: 0x3b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RCtrl_usbhdUhRxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_t_len__usbhd_uh_tx_len",
                    description: Some(
                        "endpoint 3 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_t_ctrl__usbhd_uh_tx_ctrl",
                    description: Some(
                        "endpoint 3 control.",
                    ),
                    array: None,
                    byte_offset: 0x3e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TCtrl_usbhdUhTxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_r_ctrl_",
                    description: Some(
                        "endpoint 3 control.",
                    ),
                    array: None,
                    byte_offset: 0x3f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_t_len",
                    description: Some(
                        "endpoint 4 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_t_ctrl",
                    description: Some(
                        "endpoint 4 control.",
                    ),
                    array: None,
                    byte_offset: 0x42,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_r_ctrl_",
                    description: Some(
                        "endpoint 4 control.",
                    ),
                    array: None,
                    byte_offset: 0x43,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_t_len",
                    description: Some(
                        "endpoint 5 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_t_ctrl",
                    description: Some(
                        "endpoint 5 control.",
                    ),
                    array: None,
                    byte_offset: 0x46,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_r_ctrl_",
                    description: Some(
                        "endpoint 5 control.",
                    ),
                    array: None,
                    byte_offset: 0x47,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_t_len",
                    description: Some(
                        "endpoint 6 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_t_ctrl",
                    description: Some(
                        "endpoint 6 control.",
                    ),
                    array: None,
                    byte_offset: 0x4a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_r_ctrl_",
                    description: Some(
                        "endpoint 6 control.",
                    ),
                    array: None,
                    byte_offset: 0x4b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_t_len",
                    description: Some(
                        "endpoint 7 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_t_ctrl",
                    description: Some(
                        "endpoint 7 control.",
                    ),
                    array: None,
                    byte_offset: 0x4e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_r_ctrl_",
                    description: Some(
                        "endpoint 7 control.",
                    ),
                    array: None,
                    byte_offset: 0x4f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_otg_cr",
                    description: Some(
                        "usb otg control.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbOtgCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb_otg_sr",
                    description: Some(
                        "usb otg status.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbOtgSr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "UdevCtrl_uhostCtrl",
            extends: None,
            description: Some(
                "USB device/host physical prot control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_port_en__ud_port_en",
                    description: Some(
                        "enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached.",
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
                    name: "uh_bus_reset__ud_gp_bit",
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
                    name: "uh_low_speed__ud_low_speed",
                    description: Some(
                        "enable USB port low speed: 0=full speed, 1=low speed.",
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
                    name: "uh_dm_pin__ud_dm_pin",
                    description: Some(
                        "ReadOnly: indicate current UDM pin level.",
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
                    name: "uh_dp_pin__ud_dp_pin",
                    description: Some(
                        "USB device enable and internal pullup resistance enable.",
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
                    name: "uh_pd_dis__ud_pd_dis",
                    description: Some(
                        "disable USB UDP/UDM pulldown resistance: 0=enable pulldown, 1=disable.",
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
            name: "Uep0RCtrl",
            extends: None,
            description: Some(
                "endpoint 0 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep0TCtrl",
            extends: None,
            description: Some(
                "endpoint 0 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep1RCtrl",
            extends: None,
            description: Some(
                "endpoint 1 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep1TCtrl_UsbhdUhSetup",
            extends: None,
            description: Some(
                "endpoint 1 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
                    name: "uh_sof_en",
                    description: Some(
                        "USB host automatic SOF enable.",
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
                    name: "uh_pre_pid_en",
                    description: Some(
                        "USB host PRE PID enable for low speed device via hub.",
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
            name: "Uep23Mod_uhEpMod",
            extends: None,
            description: Some(
                "endpoint 2/3 mode;host endpoint mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep2_buf_mod__uh_ep_rbuf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 2;buffer mode of USB host IN endpoint.",
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
                    name: "uep2_tx_en",
                    description: Some(
                        "enable USB endpoint 2 transmittal (IN).",
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
                    name: "uep2_rx_en__uh_ep_rx_en",
                    description: Some(
                        "enable USB endpoint 2 receiving (OUT);enable USB host IN endpoint receiving.",
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
                    name: "uep3_buf_mod__uh_ep_tbuf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 3;buffer mode of USB host OUT endpoint.",
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
                    name: "uep3_tx_en__uh_ep_tx_en",
                    description: Some(
                        "enable USB endpoint 3 transmittal (IN);enable USB host OUT endpoint transmittal.",
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
                    name: "uep3_rx_en",
                    description: Some(
                        "enable USB endpoint 3 receiving (OUT).",
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
            name: "Uep2RCtrl_usbhdUhRxCtrl",
            extends: None,
            description: Some(
                "endpoint 2 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_r_res___uh_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_r_tog___uh_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog___uh_r_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep2TCtrl",
            extends: None,
            description: Some(
                "endpoint 2 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep2TLen_usbhdUhEpPid",
            extends: None,
            description: Some(
                "endpoint 2 transmittal length.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_endp_mask",
                    description: Some(
                        "bit mask of endpoint number for USB host transfer.",
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
                    name: "uh_token_mask",
                    description: Some(
                        "bit mask of token PID for USB host transfer.",
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
            name: "Uep3RCtrl",
            extends: None,
            description: Some(
                "endpoint 3 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep3TCtrl_usbhdUhTxCtrl",
            extends: None,
            description: Some(
                "endpoint 3 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res___uh_t_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_t_tog___uh_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog__uh_t_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep41Mod",
            extends: None,
            description: Some(
                "endpoint 4/1 mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep4_tx_en",
                    description: Some(
                        "enable USB endpoint 4 transmittal (IN).",
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
                    name: "uep4_rx_en",
                    description: Some(
                        "enable USB endpoint 4 receiving (OUT).",
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
                    name: "uep1_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 1.",
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
                    name: "uep1_tx_en",
                    description: Some(
                        "enable USB endpoint 1 transmittal (IN).",
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
                    name: "uep1_rx_en",
                    description: Some(
                        "enable USB endpoint 1 receiving (OUT).",
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
            name: "Uep4RCtrl",
            extends: None,
            description: Some(
                "endpoint 4 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep4TCtrl",
            extends: None,
            description: Some(
                "endpoint 4 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_t_tog___uh_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog__uh_t_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep56Mod",
            extends: None,
            description: Some(
                "endpoint 5/6 mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep5_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 5.",
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
                    name: "uep5_tx_en",
                    description: Some(
                        "enable USB endpoint 5 transmittal (IN).",
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
                    name: "uep5_rx_en",
                    description: Some(
                        "enable USB endpoint 5 receiving (OUT).",
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
                    name: "uep6_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 6.",
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
                    name: "uep6_tx_en",
                    description: Some(
                        "enable USB endpoint 6 transmittal (IN).",
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
                    name: "uep6_rx_en",
                    description: Some(
                        "enable USB endpoint 6 receiving (OUT).",
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
            name: "Uep5RCtrl",
            extends: None,
            description: Some(
                "endpoint 5 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep5TCtrl",
            extends: None,
            description: Some(
                "endpoint 5 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_t_tog___uh_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog__uh_t_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep6RCtrl",
            extends: None,
            description: Some(
                "endpoint 6 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep6TCtrl",
            extends: None,
            description: Some(
                "endpoint 6 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_t_tog___uh_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog__uh_t_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep7Mod",
            extends: None,
            description: Some(
                "endpoint 7 mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep7_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 7.",
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
                    name: "uep7_tx_en",
                    description: Some(
                        "enable USB endpoint 7 transmittal (IN).",
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
                    name: "uep7_rx_en",
                    description: Some(
                        "enable USB endpoint 7 receiving (OUT).",
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
            name: "Uep7RCtrl",
            extends: None,
            description: Some(
                "endpoint 7 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
            name: "Uep7TCtrl",
            extends: None,
            description: Some(
                "endpoint 7 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_t_tog___uh_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_auto_tog__uh_t_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle.",
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
                "USB base control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uc_dma_en",
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
                    name: "uc_clr_all",
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
                    name: "uc_rst_sie",
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
                    name: "uc_int_busy",
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
                    name: "mask_uc_sys_ctrl_rb_uc_dev_pu_en",
                    description: Some(
                        "USB device enable and internal pullup resistance enable.",
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
                    name: "uc_low_speed",
                    description: Some(
                        "enable USB low speed: 0=12Mbps, 1=1.5Mbps.",
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
                    name: "uc_host_mode",
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
                Field {
                    name: "uda_gp_bit",
                    description: Some(
                        "general purpose bit.",
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
            name: "UsbIntEn",
            extends: None,
            description: Some(
                "USB interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uie_bus_rst__uie_detect",
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
                    name: "uie_transfer",
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
                    name: "uie_suspend",
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
                    name: "uie_hst_sof",
                    description: Some(
                        "enable interrupt for host SOF timer action for USB host mode.",
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
                    name: "uie_fifo_ov",
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
                    name: "uie_dev_nak",
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
                    name: "uie_dev_sof",
                    description: Some(
                        "enable interrupt for SOF received for USB device mode.",
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
                "USB interrupt flag.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uif_bus_rst__uif_detect",
                    description: Some(
                        "bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear.",
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
                    name: "uif_transfer",
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
                    name: "uif_suspend",
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
                    name: "uif_hst_sof",
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
                    name: "uif_fifo_ov",
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
                    name: "u_sie_free",
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
                    name: "u_tog_ok",
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
                    name: "u_is_nak",
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
            name: "UsbIntSt",
            extends: None,
            description: Some(
                "USB interrupt status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uis_h_res__mask_uis_endp",
                    description: Some(
                        "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode.",
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
                    name: "mask_uis_token",
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
                    enumm: None,
                },
                Field {
                    name: "uis_tog_ok",
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
                    name: "uis_is_nak",
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
            name: "UsbMisSt",
            extends: None,
            description: Some(
                "USB miscellaneous status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ums_dev_attach",
                    description: Some(
                        "RO, indicate device attached status on USB host.",
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
                    name: "ums_dm_level",
                    description: Some(
                        "RO, indicate UDM level saved at device attached to USB host.",
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
                    name: "ums_suspend",
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
                    name: "ums_bus_reset",
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
                    name: "ums_r_fifo_rdy",
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
                    name: "ums_sie_free",
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
                    name: "ums_sof_act",
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
                    name: "ums_sof_pres",
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
            name: "UsbOtgCr",
            extends: None,
            description: Some(
                "usb otg control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usb_otg_cr_dischargevbus",
                    description: Some(
                        "usb otg control.",
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
                    name: "usb_otg_cr_chargevbus",
                    description: Some(
                        "usb otg control.",
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
                    name: "usb_otg_cr_idpu",
                    description: Some(
                        "usb otg control.",
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
                    name: "usb_otg_cr_otg_en",
                    description: Some(
                        "usb otg control.",
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
                    name: "usb_otg_cr_vbus",
                    description: Some(
                        "usb otg control.",
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
                    name: "usb_otg_cr_sess",
                    description: Some(
                        "usb otg control.",
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
            name: "UsbOtgSr",
            extends: None,
            description: Some(
                "usb otg status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usb_otg_sr_vbus_vld",
                    description: Some(
                        "usb otg status.",
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
                    name: "usb_otg_sr_sess_vld",
                    description: Some(
                        "usb otg status.",
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
                    name: "usb_otg_sr_sess_end",
                    description: Some(
                        "usb otg status.",
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
                    name: "usb_otg_sr_id_dig",
                    description: Some(
                        "usb otg status.",
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
