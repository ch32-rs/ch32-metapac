use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Exten",
            extends: None,
            description: Some(
                "Extend configuration.",
            ),
            items: &[
                BlockItem {
                    name: "exten_ctlr0",
                    description: Some(
                        "Configure the extended control register 0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExtenCtlr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exten_keyr",
                    description: Some(
                        "Configure unlock register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExtenKeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exten_ctlr1",
                    description: Some(
                        "Configure the extended control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExtenCtlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isink1_cfgr",
                    description: Some(
                        "ISINK1 configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Isink1Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isink2_cfgr",
                    description: Some(
                        "ISINK2 configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Isink2Cfgr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ExtenCtlr0",
            extends: None,
            description: Some(
                "Configure the extended control register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lkupen",
                    description: Some(
                        "LOCKUP_Enable.",
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
                    name: "lkuprst",
                    description: Some(
                        "LOCKUP RESET.",
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
                    name: "wr_lock",
                    description: Some(
                        "Control register 0 and register 1 for configuration locking.",
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
                    name: "usbpd0_cc_ref",
                    description: Some(
                        "USBPD0 CC pin analog reference enable.",
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
                    name: "usbpd1_cc_ref",
                    description: Some(
                        "USBPD1 CC pin analog reference enable.",
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
                    name: "usbpd0_cc_hvt",
                    description: Some(
                        "USBPD0 CC pin high threshold input mode enabled.",
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
                    name: "usbpd1_cc_hvt",
                    description: Some(
                        "USBPD1 CC pin high threshold input mode enabled.",
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
                    name: "usbpd0_lve_t",
                    description: Some(
                        "CC pin of USBPD0 1.12V output Tr,Tf difference adjustment.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbpd1_lve_t",
                    description: Some(
                        "CC pin of USBPD1 1.12V output Tr,Tf difference adjustment.",
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
                    name: "isrc1_en",
                    description: Some(
                        "Source current 1 enable.",
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
                    name: "isrc1_sel",
                    description: Some(
                        "Source current 1 selection.",
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
                    name: "isrc2_en",
                    description: Some(
                        "Source current 2 enable.",
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
                    name: "isrc2_sel",
                    description: Some(
                        "Source current 2 selection.",
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
            ],
        },
        FieldSet {
            name: "ExtenCtlr1",
            extends: None,
            description: Some(
                "Configure the extended control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "udp_bufoe",
                    description: Some(
                        "UDP pin DAC_BUF output enable.",
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
                    name: "udp_pcs",
                    description: Some(
                        "Select pull-up current or pull-down current.  built into the UDP pin.",
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
                    name: "udp_pue",
                    description: Some(
                        "UPD port DAC and programmable pull-down.  and pull-up mode selection.",
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
                    name: "udp_pde",
                    description: Some(
                        "UPD port DAC and programmable pull-down.  and pull-up mode selection.",
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
                    name: "udp_dac",
                    description: Some(
                        "UPD port DAC data/programmable pull-down.  resistor and pull-up resistor data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udp_ae",
                    description: Some(
                        "Enable of the UDP pin buffer/comparator DAC_BUF.",
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
                    name: "udp_ai",
                    description: Some(
                        "Output of the UDP pin comparator DAC_BUF.",
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
                    name: "udp_rat",
                    description: Some(
                        "UDP pin speed selection.",
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
                    name: "udp_ie",
                    description: Some(
                        "UDP pin is input enabled when acting as a GPIO.",
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
                    name: "udm_bufoe",
                    description: Some(
                        "UDM pin DAC_BUF output enable.",
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
                    name: "udm_pcs",
                    description: Some(
                        "Select pull-up current or pull-down current.  built into the UDM pin.",
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
                    name: "udm_pue",
                    description: Some(
                        "UPM port DAC and programmable pull-down.  and pull-up mode selection.",
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
                    name: "udm_pde",
                    description: Some(
                        "UPM port DAC and programmable pull-down.  and pull-up mode selection.",
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
                    name: "udm_dac",
                    description: Some(
                        "UPM port DAC data/programmable pull-down.  resistor and pull-up resistor data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udm_ae",
                    description: Some(
                        "Enable of the UDM pin buffer/comparator DAC_BUF.",
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
                    name: "udm_ai",
                    description: Some(
                        "Output of the UDM pin comparator DAC_BUF.",
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
                    name: "udm_rat",
                    description: Some(
                        "UDM pin speed selection.",
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
                    name: "udm_ie",
                    description: Some(
                        "UDM pin is input enabled when acting as a GPIO.",
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
                    name: "udu_shrt",
                    description: Some(
                        "UDP pin and UDM pin are shorted to enable.",
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
            name: "ExtenKeyr",
            extends: None,
            description: Some(
                "Configure unlock register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "keyr",
                    description: Some(
                        "Control register 0 and register 1 for configuration unlocking.",
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
            name: "Isink1Cfgr",
            extends: None,
            description: Some(
                "ISINK1 configuration register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "isink1_adj",
                    description: Some(
                        "Sink current 1 calibration value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "isink1_dat",
                    description: Some(
                        "Sink current 1 programmable data bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isink2Cfgr",
            extends: None,
            description: Some(
                "ISINK2 configuration register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "isink2_adj",
                    description: Some(
                        "Sink current 2 calibration value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "isink2_dat",
                    description: Some(
                        "Sink current 2 programmable data bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
