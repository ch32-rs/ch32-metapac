use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "BleRfend",
            extends: None,
            description: Some(
                "BLE RF Frontend — analog config (PLL/VCO/filter/bias) + calibration tables.",
            ),
            items: &[
                BlockItem {
                    name: "cal_trig",
                    description: Some(
                        "Calibration trigger/path bits (TX_tune_trigger bit0, TX_cal_mode bit4, TXF_enable bit8, RX_filter_mode bit12, RX_ADC_config bit16).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CalTrig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "path_en",
                    description: Some(
                        "TX/RX path enable (RX_ADC bit16, TX cal bit17, TX/RX PLL pre-enable bits[20:21]).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PathEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "RFEND control + reset (reset sequence writes 0x1101 → 0 → 0x1101; RX_filter_strobe bit4, ADC_ref_strobe bit8).",
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
                BlockItem {
                    name: "cfg0",
                    description: Some(
                        "RFEND CFG0 (default 0x480, hw-confirmed).",
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
                    name: "pll_vco",
                    description: Some(
                        "PLL/VCO config (post_cal_enable bit4, channel-lock release bit1).",
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
                    name: "loop_filter",
                    description: Some(
                        "PLL loop filter config (bits[3:0]/[7:4]/[10:8]/[22:20]/[30:28]).",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "cfg4",
                    description: Some(
                        "PLL enable (bits[25:19] cleared, bit20 set, bit31 set).",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "cfg5_freq",
                    description: Some(
                        "CFG5 + frequency code (bits[15:12]=8, bits[26:24], bits[31:30]; freq_code at bits[15:8] BF00=2401MHz, D300=2440MHz, E700=2480MHz; nCO2440 bits[5:0], nGA2440 bits[30:24]).",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg5Freq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll_div",
                    description: Some(
                        "PLL integer/frac divider (int_div bits[24:20], frac_div bits[13:0]).",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PllDiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf0",
                    description: Some(
                        "Analog RF0 config (RF analog enable bit31, bits[3:0]=9, bits[26:24]=0b100, bit29).",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rf1",
                    description: Some(
                        "Analog RF1 bias (bits[2:0]=3, bits[6:4]=3, bits[10:8]=3, bit24=0, bit25=1).",
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
                    name: "rf2",
                    description: Some(
                        "Analog RF2 (bit14 = 1).",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rf2b",
                    description: Some(
                        "Analog RF2b (bits[3:0]=12, bit7=1, bit12=0).",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "adc_ref",
                    description: Some(
                        "RX ADC reference enable (bit16).",
                    ),
                    array: None,
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
                    name: "tune_result",
                    description: Some(
                        "PLL tune result (CO bits[5:0], tune_done bit25, tune_active bit26). Reading also latches GA into CO_RESULT2.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TuneResult",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ga_result",
                    description: Some(
                        "GA result (bits[16:10], latched by TUNE_RESULT read).",
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
                    name: "rx_filter_result",
                    description: Some(
                        "RX filter calibration result (bits[4:0]) + done bit8.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "co_table1",
                    description: Some(
                        "CO calibration table 1 — nibble-packed `delta_low * (39-ch)/39` for ch0..ch39 (5×u32).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "co_table2",
                    description: Some(
                        "CO calibration table 2 — nibble-packed `delta_high * (ch+1)/40` for ch0..ch39 (5×u32).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ga_table",
                    description: Some(
                        "GA calibration table + CO2 overflow guard (3×u32 covering nibbles for ch40..41 + GA1[0..9] + GA2[0..9]).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xc8,
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
    ],
    fieldsets: &[
        FieldSet {
            name: "CalTrig",
            extends: None,
            description: Some(
                "RFEND calibration trigger/path bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_tune_trigger",
                    description: Some(
                        "TX tune trigger pulse for PLL measurement.",
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
                    name: "tx_cal_mode",
                    description: Some(
                        "TX calibration mode (set during cal, cleared post).",
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
                    name: "txf_enable",
                    description: Some(
                        "TX filter enable (set in RFEND_TXFtune; cleared post-cal Bug",
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
                    name: "rx_filter_mode",
                    description: Some(
                        "RX filter calibration mode (cleared post Bug",
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
                    name: "rx_adc_config",
                    description: Some(
                        "RX ADC config (cleared then set in RFEND_RXAdc).",
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
            name: "Cfg5Freq",
            extends: None,
            description: Some(
                "CFG5 + frequency-code register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nco2440",
                    description: Some(
                        "Stored CO anchor for default-channel comp (bits[5:0]).",
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
                    name: "freq_code",
                    description: Some(
                        "Frequency code bits[15:8] (BF=2401MHz, D3=2440MHz, E7=2480MHz).",
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
                Field {
                    name: "nga2440",
                    description: Some(
                        "Stored GA anchor for default-channel comp (bits[30:24]).",
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
        FieldSet {
            name: "PathEn",
            extends: None,
            description: Some(
                "RFEND TX/RX path enable mask (pre-init = 0x00330000 enables bits 16/17/20/21).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_adc_path",
                    description: Some(
                        "RX ADC path enable.",
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
                    name: "tx_cal_path",
                    description: Some(
                        "TX calibration path enable.",
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
                    name: "tx_pll_pre",
                    description: Some(
                        "TX PLL pre-enable.",
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
                    name: "rx_filter_path",
                    description: Some(
                        "RX filter path enable.",
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
            ],
        },
        FieldSet {
            name: "PllDiv",
            extends: None,
            description: Some(
                "PLL channel divider (programmed by listener.set_channel_freq).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frac_div",
                    description: Some(
                        "Fractional divider (`((freq_khz % 64000) << 10) / 250` masked to 14 bits).",
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
                Field {
                    name: "int_div",
                    description: Some(
                        "Integer divider (`freq_khz / 64000` masked to 5 bits).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TuneResult",
            extends: None,
            description: Some(
                "PLL tune result (CO, tune_done, tune_active).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "co",
                    description: Some(
                        "CO calibration result (read after PLL lock).",
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
                    name: "tune_done",
                    description: Some(
                        "Tune done flag — second-check by RFEND_WaitTune.",
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
                    name: "tune_active",
                    description: Some(
                        "Tune active flag — set first; double-check semantic.",
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
            ],
        },
    ],
    enums: &[],
};
