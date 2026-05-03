use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Opa",
            extends: None,
            description: Some(
                "OPA and CMP configuration.",
            ),
            items: &[
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "OPA Configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "OPA Control register 1.",
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
                    name: "cfgr2",
                    description: Some(
                        "OPA Configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr2",
                    description: Some(
                        "OPA Control register 2.",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "opa_key",
                    description: Some(
                        "OPA unlock key register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OpaKey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp_key",
                    description: Some(
                        "CMP unlock key register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmpKey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "poll_key",
                    description: Some(
                        "POLL lock key register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PollKey",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "OPA Configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "poll_en",
                    description: Some(
                        "OPA1 front-end polling enable.",
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
                    name: "poll1_num",
                    description: Some(
                        "Number of OPA1 polling positive ends. 00 = 1 channel, 01 = 2 channels, 10 = 3 channels.",
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
                    name: "rst_en1",
                    description: Some(
                        "OPA1 reset system enable. When set, an OPA1 polling result of high will reset the system.",
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
                    name: "setup_cfg",
                    description: Some(
                        "OPA establishment time configuration. 00/10 = 0.5us, 01 = 0.312us, 11 = 0.77us.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "auto_adc_cfg",
                    description: Some(
                        "OPA polling automatic ADC trigger configuration.",
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
                    name: "ie_out1",
                    description: Some(
                        "OPA1 output interrupt enable.",
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
                    name: "nmi_en",
                    description: Some(
                        "OPA NMI interrupt enable.",
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
                    name: "if_out_poll_ch1",
                    description: Some(
                        "1st polling channel OPA1 output high interrupt flag.",
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
                    name: "if_out_poll_ch2",
                    description: Some(
                        "2nd polling channel OPA1 output high interrupt flag.",
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
                    name: "if_out_poll_ch3",
                    description: Some(
                        "3rd polling channel OPA1 output high interrupt flag.",
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
                    name: "poll_ch1",
                    description: Some(
                        "OPA polling order, 1st polling channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "poll_ch2",
                    description: Some(
                        "OPA polling order, 2nd polling channel.",
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
                    name: "poll_ch3",
                    description: Some(
                        "OPA polling order, 3rd polling channel.",
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
                    name: "poll_swstrt",
                    description: Some(
                        "OPA polling software trigger. Set by software, cleared by hardware once polling starts.",
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
                    name: "poll_sel",
                    description: Some(
                        "OPA polling trigger event selection. 000 = software, 001 = TIM1_CH4, 010 = TIM2_CH4, 011 = TIM3_CH1, 100 = TIM3_CH2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "poll_lock",
                    description: Some(
                        "POLL lock status. Cleared only by module reset.",
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
            name: "Cfgr2",
            extends: None,
            description: Some(
                "OPA Configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "poll_en1",
                    description: Some(
                        "CMP1 front-end polling enable.",
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
                    name: "poll1_num",
                    description: Some(
                        "Number of CMP1 polling positive ends. 00 = 1, 01 = 2, 10 = 3.",
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
                    name: "rst_en1",
                    description: Some(
                        "CMP1 reset system enable.",
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
                    name: "rst_en2",
                    description: Some(
                        "CMP2 reset system enable.",
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
                    name: "ie_out1",
                    description: Some(
                        "CMP1 interrupt enable.",
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
                    name: "ie_cnt",
                    description: Some(
                        "CMP1 polling interval end interrupt enable.",
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
                    name: "if_out_poll_ch1",
                    description: Some(
                        "1st CMP1 polling channel output-high flag.",
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
                    name: "if_out_poll_ch2",
                    description: Some(
                        "2nd CMP1 polling channel output-high flag.",
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
                    name: "if_out_poll_ch3",
                    description: Some(
                        "3rd CMP1 polling channel output-high flag.",
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
                    name: "if_cnt",
                    description: Some(
                        "CMP1 polling interval end flag.",
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
                    name: "poll_vlu",
                    description: Some(
                        "CMP1 front-end polling interval. Polling interval = (POLL_VLU + 1) * 1us.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "poll_ch1",
                    description: Some(
                        "CMP1 polling order, 1st polling channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "poll_ch2",
                    description: Some(
                        "CMP1 polling order, 2nd polling channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "poll_ch3",
                    description: Some(
                        "CMP1 polling order, 3rd polling channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CmpKey",
            extends: None,
            description: Some(
                "CMP unlock key register. Write KEY1 = 0x45670123, then KEY2 = 0xCDEF89AB to unlock.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp_key",
                    description: Some(
                        "CMP unlock key.",
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
            name: "Ctlr1",
            extends: None,
            description: Some(
                "OPA Control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opa_en1",
                    description: Some(
                        "OPA1 enable.",
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
                        "OPA1 output channel selection. 00 = PD4 plus internal CMP2 input, 01 = PA5 plus internal CMP2 input, 1x = internal CMP2 input only.",
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
                    name: "psel1",
                    description: Some(
                        "OPA1 positive (P) input channel selection. 00 = PA2, 01 = PD7, 10 = PD3, 11 = PD1.",
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
                    name: "nsel1",
                    description: Some(
                        "OPA1 negative input channel and PGA gain selection. 000 = PA1, 001 = PD0, 011 = PGA gain 4, 100 = PGA gain 8, 101 = PGA gain 16, 110 = PGA gain 32.",
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
                    name: "fb_en1",
                    description: Some(
                        "OPA1 internal feedback resistor enable. Must be set when NSEL1 is in PGA mode.",
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
                    name: "pgadif",
                    description: Some(
                        "Differential input PGA mode enable. The negative end is connected to OPA_CHN2 (PA4).",
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
                    name: "vben",
                    description: Some(
                        "PGA mode positive reference voltage enable.",
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
                    name: "vbsel",
                    description: Some(
                        "PGA mode positive reference voltage selection. 0 = VDD/2, 1 = VDD/4.",
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
                    name: "vbcmpsel",
                    description: Some(
                        "CMP2 negative reference voltage selection. Only valid when VBEN = 1. 11 = off.",
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
                    name: "opa_hs1",
                    description: Some(
                        "OPA1 high-speed mode enable. Increases the slew rate to 40V/us.",
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
                    name: "opa_lock",
                    description: Some(
                        "OPA lock. Write 1 to lock, write 0 has no effect.",
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
            name: "Ctlr2",
            extends: None,
            description: Some(
                "OPA Control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp_en1",
                    description: Some(
                        "CMP1 enable.",
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
                        "CMP1 output mode selection. 00 = output to PC0, 01 = TIM1_CH4 internal, 10 = TIM2_CH4 internal.",
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
                    name: "nsel1",
                    description: Some(
                        "CMP1 negative input channel selection. 00 = PC2, 01 = PD5, 10 = PA6, 11 = invalid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psel1",
                    description: Some(
                        "CMP1 positive input channel selection. 00 = PC5, 01 = PB3, 10 = PD2, 11 = invalid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hyen1",
                    description: Some(
                        "CMP1 hysteresis function enable (+/- 24mV).",
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
                    name: "rmid1",
                    description: Some(
                        "CMP1 positive input channel virtual center point enable.",
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
                    name: "cmp_en2",
                    description: Some(
                        "CMP2 enable.",
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
                    name: "filt_en",
                    description: Some(
                        "CMP digital filter enable.",
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
                    name: "filt_sel",
                    description: Some(
                        "CMP output digital filter length selection. 0 = 0.33us, 1 = 0.5us.",
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
                    name: "bkin_cfg",
                    description: Some(
                        "TIM1 brake source configuration. 00 = IO, 01 = CMP1, 10 = CMP2, 11 = OPA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmp_lock",
                    description: Some(
                        "CMP lock. Write 1 to lock, write 0 has no effect.",
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
            name: "OpaKey",
            extends: None,
            description: Some(
                "OPA unlock key register. Write KEY1 = 0x45670123, then KEY2 = 0xCDEF89AB to unlock.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opa_key",
                    description: Some(
                        "OPA unlock key.",
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
            name: "PollKey",
            extends: None,
            description: Some(
                "POLL lock key register. Write KEY1 = 0x45670123, then KEY2 = 0xCDEF89AB; once locked the module must be reset to unlock.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "poll_key",
                    description: Some(
                        "POLL lock key.",
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
