use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock control.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr",
                    description: Some(
                        "Clock control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr0",
                    description: Some(
                        "Clock configuration register (RCC_CFGR0).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllcfgr",
                    description: Some(
                        "PLL clock configuration register (RCC_PLLCFGR).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intr",
                    description: Some(
                        "Clock interrupt register (RCC_INTR).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hb2prstr",
                    description: Some(
                        "HB2 peripheral reset register (RCC_HB2PRSTR).",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hb2prstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hb1prstr",
                    description: Some(
                        "HB1 peripheral reset register (RCC_HB1PRSTR).",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hb1prstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hbpcenr",
                    description: Some(
                        "HB Peripheral Clock enable register (RCC_HBPCENR).",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hbpcenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hb2pcenr",
                    description: Some(
                        "HB2 peripheral clock enable register (RCC_HB2PCENR).",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hb2pcenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hb1pcenr",
                    description: Some(
                        "HB1 peripheral clock enable register (RCC_HB1PCENR).",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hb1pcenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdctlr",
                    description: Some(
                        "Backup domain control register (RCC_BDCTLR).",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rstsckr",
                    description: Some(
                        "Control/status register (RCC_RSTSCKR).",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rstsckr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hbrstr",
                    description: Some(
                        "HB reset register (RCC_PHBRSTR).",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hbrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "Clock configuration register2 (RCC_CFGR2).",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                    name: "pllcfgr2",
                    description: Some(
                        "PLL Clock configuration register2 (RCC_PLLCFGR2).",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllcfgr2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bdctlr",
            extends: None,
            description: Some(
                "Backup domain control register (RCC_BDCTLR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "External Low Speed oscillator enable.",
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
                    name: "lserdy",
                    description: Some(
                        "External Low Speed oscillator ready.",
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
                    name: "lsebyp",
                    description: Some(
                        "External Low Speed oscillator bypass.",
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
                    name: "cco",
                    description: Some(
                        "calibrate the clock output selection.",
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
                    name: "asoe",
                    description: Some(
                        "TAMPER pin enables pulse output.",
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
                    name: "asos",
                    description: Some(
                        "TAMPER pin alarm/second pulse output.",
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
                    name: "rtcsel",
                    description: Some(
                        "RTC clock source selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rtcsel",
                    ),
                },
                Field {
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable.",
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
                    name: "rtccal",
                    description: Some(
                        "RTC calibration value.",
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
                Field {
                    name: "bdrst",
                    description: Some(
                        "Backup domain software reset.",
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
            name: "Cfgr0",
            extends: None,
            description: Some(
                "Clock configuration register (RCC_CFGR0).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "System clock switch.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "sws",
                    description: Some(
                        "System clock switch status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "hpre",
                    description: Some(
                        "HB (AHB) prescaler — divides SYS_CLK to produce HCLK.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre1",
                    description: Some(
                        "PB1 (low-speed APB) prescaler.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre2",
                    description: Some(
                        "PB2 (high-speed APB) prescaler, also feeds ADC prescaler.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "adcpre",
                    description: Some(
                        "ADC prescaler (divides PB2 clock).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Adcpre",
                    ),
                },
                Field {
                    name: "fpre",
                    description: Some(
                        "V5F core prescaler — divides SYS_CLK to produce the V5F core clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fpre",
                    ),
                },
                Field {
                    name: "rgmiion",
                    description: Some(
                        "1000 Mb/s Ethernet RGMII interface and clock enable.",
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
                    name: "pipeon",
                    description: Some(
                        "PIPE clock gating enable.",
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
                    name: "utmion",
                    description: Some(
                        "UTMI clock gating enable.",
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
                    name: "mco",
                    description: Some(
                        "Microcontroller clock output selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mco",
                    ),
                },
                Field {
                    name: "adc_duty_sel",
                    description: Some(
                        "ADC clock duty cycle. 0=50%, 1=75%. Only effective when ADCSRC=HCLK.",
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
                    name: "adcsrc",
                    description: Some(
                        "ADC input clock source selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adcsrc",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "Clock configuration register2 (RCC_CFGR2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uhsifdiv",
                    description: Some(
                        "UHSIF prescaler (1..64).",
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
                    name: "uhsifsrc",
                    description: Some(
                        "UHSIF clock source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Uhsifsrc",
                    ),
                },
                Field {
                    name: "ltdcdiv",
                    description: Some(
                        "LTDC prescaler (1..64).",
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
                    name: "ltdcsrc",
                    description: Some(
                        "LTDC clock source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ltdcsrc",
                    ),
                },
                Field {
                    name: "usbfsdiv",
                    description: Some(
                        "USBFS 48 MHz prescaler.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Usbfsdiv",
                    ),
                },
                Field {
                    name: "usbfssrc",
                    description: Some(
                        "USBFS 48 MHz clock source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usbfssrc",
                    ),
                },
                Field {
                    name: "rngsrc",
                    description: Some(
                        "RNG clock source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ClkSrcPll",
                    ),
                },
                Field {
                    name: "i2s2src",
                    description: Some(
                        "I2S2 clock source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ClkSrcPll",
                    ),
                },
                Field {
                    name: "i2s3src",
                    description: Some(
                        "I2S3 clock source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ClkSrcPll",
                    ),
                },
                Field {
                    name: "hsadcsrc",
                    description: Some(
                        "HSADC clock source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hsadcsrc",
                    ),
                },
                Field {
                    name: "eth1gsrc",
                    description: Some(
                        "Gigabit Ethernet 125 MHz clock source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Eth1gsrc",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some(
                "Clock control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsion",
                    description: Some(
                        "Internal High Speed clock enable.",
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
                    name: "hsirdy",
                    description: Some(
                        "Internal High Speed clock ready flag.",
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
                    name: "hsitrim",
                    description: Some(
                        "Internal High Speed clock trimming.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsical",
                    description: Some(
                        "Internal High Speed clock calibration (hardware-set, read-only).",
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
                    name: "hseon",
                    description: Some(
                        "External High Speed clock enable.",
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
                    name: "hserdy",
                    description: Some(
                        "External High Speed clock ready flag.",
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
                    name: "hsebyp",
                    description: Some(
                        "External High Speed clock Bypass.",
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
                    name: "csson",
                    description: Some(
                        "Clock Security System enable.",
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
                    name: "usbhs_pllon",
                    description: Some(
                        "USBHS PLL clock enable.",
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
                    name: "usbhs_pllrdy",
                    description: Some(
                        "USBHS PLL clock ready flag.",
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
                    name: "usbss_pllon",
                    description: Some(
                        "USBSS PLL clock enable.",
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
                    name: "usbss_pllrdy",
                    description: Some(
                        "USBSS PLL clock ready flag.",
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
                    name: "pllon",
                    description: Some(
                        "PLL clock enable.",
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
                    name: "pllrdy",
                    description: Some(
                        "PLL clock ready flag.",
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
                    name: "eth_pllon",
                    description: Some(
                        "ETH PLL clock enable.",
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
                    name: "eth_pllrdy",
                    description: Some(
                        "ETH PLL clock ready flag.",
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
                    name: "serdes_pllon",
                    description: Some(
                        "SERDES PLL clock enable.",
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
                    name: "serdes_pllrdy",
                    description: Some(
                        "SERDES PLL clock ready flag.",
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
                    name: "css_hse_dis",
                    description: Some(
                        "Upon the occurrence of an HSE failure event with CSSON enabled.",
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
            name: "Hb1pcenr",
            extends: None,
            description: Some(
                "HB1 peripheral clock enable register (RCC_HB1PCENR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "Timer 2 clock enable.",
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
                    name: "tim3en",
                    description: Some(
                        "Timer 3 clock enable.",
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
                    name: "tim4en",
                    description: Some(
                        "Timer 4 clock enable.",
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
                    name: "tim5en",
                    description: Some(
                        "Timer 5 clock enable.",
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
                    name: "tim6en",
                    description: Some(
                        "Timer 6 clock enable.",
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
                    name: "tim7en",
                    description: Some(
                        "Timer 7 clock enable.",
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
                    name: "usart6en",
                    description: Some(
                        "USART 6 clock enable.",
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
                    name: "usart7en",
                    description: Some(
                        "USART 7 clock enable.",
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
                    name: "usart8en",
                    description: Some(
                        "USART 8 clock enable.",
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
                    name: "lptim1en",
                    description: Some(
                        "LPTIM1 clock enable.",
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
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 clock enable.",
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
                    name: "wwdgen",
                    description: Some(
                        "Window watchdog clock enable.",
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
                    name: "qspi1en",
                    description: Some(
                        "QSPI1 clock enable.",
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
                    name: "qspi2en",
                    description: Some(
                        "QSPI2 clock enable.",
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
                    name: "spi2en",
                    description: Some(
                        "SPI2 clock enable.",
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
                    name: "spi3en",
                    description: Some(
                        "SPI3 clock enable.",
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
                    name: "spi4en",
                    description: Some(
                        "SPI4 clock enable.",
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
                    name: "usart2en",
                    description: Some(
                        "USART2 clock enable.",
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
                    name: "usart3en",
                    description: Some(
                        "USART3 clock enable.",
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
                    name: "usart4en",
                    description: Some(
                        "USART4 clock enable.",
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
                    name: "usart5en",
                    description: Some(
                        "USART5 clock enable.",
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
                    name: "i2c1en",
                    description: Some(
                        "I2C1 clock enable.",
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
                    name: "i2c2en",
                    description: Some(
                        "I2C2 clock enable.",
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
                    name: "can3en",
                    description: Some(
                        "CAN3 clock enable.",
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
                    name: "can1en",
                    description: Some(
                        "CAN1 clock enable.",
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
                    name: "can2en",
                    description: Some(
                        "CAN2 clock enable.",
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
                    name: "bkpen",
                    description: Some(
                        "Backup interface clock enable.",
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
                    name: "pwren",
                    description: Some(
                        "Power interface clock enable.",
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
                    name: "dacen",
                    description: Some(
                        "DAC interface clock enable.",
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
                    name: "i2c3en",
                    description: Some(
                        "I2C3 clock enable.",
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
                    name: "swpmien",
                    description: Some(
                        "SWPMI clock enable.",
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
            name: "Hb1prstr",
            extends: None,
            description: Some(
                "HB1 peripheral reset register (RCC_HB1PRSTR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "Timer 2 reset.",
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
                    name: "tim3rst",
                    description: Some(
                        "Timer 3 reset.",
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
                    name: "tim4rst",
                    description: Some(
                        "Timer 4 reset.",
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
                    name: "tim5rst",
                    description: Some(
                        "Timer 5 reset.",
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
                    name: "tim6rst",
                    description: Some(
                        "Timer 6 reset.",
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
                    name: "tim7rst",
                    description: Some(
                        "Timer 7 reset.",
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
                    name: "usart6rst",
                    description: Some(
                        "USART 6 reset.",
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
                    name: "usart7rst",
                    description: Some(
                        "USART 7 reset.",
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
                    name: "usart8rst",
                    description: Some(
                        "USART 8 reset.",
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
                    name: "lptim1rst",
                    description: Some(
                        "LPTIM1 reset.",
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
                    name: "lptim2rst",
                    description: Some(
                        "LPTIM2 reset.",
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
                    name: "wwdgrst",
                    description: Some(
                        "Window watchdog reset.",
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
                    name: "qspi1rst",
                    description: Some(
                        "QSPI1 reset.",
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
                    name: "qspi2rst",
                    description: Some(
                        "QSPI2 reset.",
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
                    name: "spi2rst",
                    description: Some(
                        "SPI2 reset.",
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
                    name: "spi3rst",
                    description: Some(
                        "SPI3 reset.",
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
                    name: "spi4rst",
                    description: Some(
                        "SPI4 reset.",
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
                    name: "usart2rst",
                    description: Some(
                        "USART 2 reset.",
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
                    name: "usart3rst",
                    description: Some(
                        "USART 3 reset.",
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
                    name: "usart4rst",
                    description: Some(
                        "USART 4 reset.",
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
                    name: "usart5rst",
                    description: Some(
                        "USART 5 reset.",
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
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 reset.",
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
                    name: "i2c2rst",
                    description: Some(
                        "I2C2 reset.",
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
                    name: "can3rst",
                    description: Some(
                        "CAN3 reset.",
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
                    name: "can1rst",
                    description: Some(
                        "CAN1 reset.",
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
                    name: "can2rst",
                    description: Some(
                        "CAN2 reset.",
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
                    name: "bkprst",
                    description: Some(
                        "Backup interface reset.",
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
                    name: "pwrrst",
                    description: Some(
                        "Power interface reset.",
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
                    name: "dacrst",
                    description: Some(
                        "DAC interface reset.",
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
                    name: "i2c3rst",
                    description: Some(
                        "I2C3 reset.",
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
                    name: "swpmirst",
                    description: Some(
                        "SWPMI reset.",
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
            name: "Hb2pcenr",
            extends: None,
            description: Some(
                "HB2 peripheral clock enable register (RCC_HB2PCENR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afioen",
                    description: Some(
                        "Alternate function I/O clock enable.",
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
                    name: "hsadcen",
                    description: Some(
                        "HSADC clock enable.",
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
                    name: "iopaen",
                    description: Some(
                        "I/O port A clock enable.",
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
                    name: "iopben",
                    description: Some(
                        "I/O port B clock enable.",
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
                    name: "iopcen",
                    description: Some(
                        "I/O port C clock enable.",
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
                    name: "iopden",
                    description: Some(
                        "I/O port D clock enable.",
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
                    name: "iopeen",
                    description: Some(
                        "I/O port E clock enable.",
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
                    name: "iopfen",
                    description: Some(
                        "I/O port F clock enable.",
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
                    name: "adc1en",
                    description: Some(
                        "ADC1 interface clock enable.",
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
                    name: "adc2en",
                    description: Some(
                        "ADC 2 interface clock enable.",
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
                    name: "tim1en",
                    description: Some(
                        "TIM1 Timer clock enable.",
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
                    name: "spi1en",
                    description: Some(
                        "SPI 1 clock enable.",
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
                    name: "tim8en",
                    description: Some(
                        "TIM8 Timer clock enable.",
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
                    name: "usart1en",
                    description: Some(
                        "USART1 clock enable.",
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
                    name: "i2c4en",
                    description: Some(
                        "I2C4 clock enable.",
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
                    name: "saien",
                    description: Some(
                        "SAI clock enable.",
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
                    name: "sdioen",
                    description: Some(
                        "SDIO clock enable.",
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
                    name: "tim9en",
                    description: Some(
                        "TIM9 Timer clock enable.",
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
                    name: "tim10en",
                    description: Some(
                        "TIM10 Timer clock enable.",
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
                    name: "tim11en",
                    description: Some(
                        "TIM11 Timer clock enable.",
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
                    name: "tim12en",
                    description: Some(
                        "TIM12 Timer clock enable.",
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
                    name: "opcmen",
                    description: Some(
                        "OPA and CMP clock enable.",
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
                    name: "dfsdmen",
                    description: Some(
                        "DFSDM clock enable.",
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
                    name: "ecdcen",
                    description: Some(
                        "ECDC clock enable.",
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
                    name: "gphaen",
                    description: Some(
                        "GPHA clock enable.",
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
                    name: "ltdcen",
                    description: Some(
                        "LTDC clock enable.",
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
                    name: "i3cen",
                    description: Some(
                        "I3C clock enable.",
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
            name: "Hb2prstr",
            extends: None,
            description: Some(
                "HB2 peripheral reset register (RCC_HB2PRSTR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afiorst",
                    description: Some(
                        "Alternate function I/O reset.",
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
                    name: "hsadcrst",
                    description: Some(
                        "HSADC reset.",
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
                    name: "ioparst",
                    description: Some(
                        "IO port A reset.",
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
                    name: "iopbrst",
                    description: Some(
                        "IO port B reset.",
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
                    name: "iopcrst",
                    description: Some(
                        "IO port C reset.",
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
                    name: "iopdrst",
                    description: Some(
                        "IO port D reset.",
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
                    name: "ioperst",
                    description: Some(
                        "IO port E reset.",
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
                    name: "iopfrst",
                    description: Some(
                        "IO port F reset.",
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
                    name: "adc1rst",
                    description: Some(
                        "ADC 1 interface reset.",
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
                    name: "adc2rst",
                    description: Some(
                        "ADC 2 interface reset.",
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
                    name: "tim1rst",
                    description: Some(
                        "TIM1 timer reset.",
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
                    name: "spi1rst",
                    description: Some(
                        "SPI 1 reset.",
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
                    name: "tim8rst",
                    description: Some(
                        "TIM8 timer reset.",
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
                    name: "usart1rst",
                    description: Some(
                        "USART1 reset.",
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
                    name: "i2c4rst",
                    description: Some(
                        "I2C4 reset.",
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
                    name: "sairst",
                    description: Some(
                        "SAI reset.",
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
                    name: "sdiorst",
                    description: Some(
                        "SDIO reset.",
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
                    name: "tim9rst",
                    description: Some(
                        "TIM9 timer reset.",
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
                    name: "tim10rst",
                    description: Some(
                        "TIM10 timer reset.",
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
                    name: "tim11rst",
                    description: Some(
                        "TIM11 timer reset.",
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
                    name: "tim12rst",
                    description: Some(
                        "TIM12 timer reset.",
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
                    name: "opcmrst",
                    description: Some(
                        "OPA and CMP reset.",
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
                    name: "dfsdmrst",
                    description: Some(
                        "DFSDM reset.",
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
                    name: "ecdcrst",
                    description: Some(
                        "ECDC reset.",
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
                    name: "gpharst",
                    description: Some(
                        "GPHA reset.",
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
                    name: "ltdcrst",
                    description: Some(
                        "LTDC reset.",
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
                    name: "i3crst",
                    description: Some(
                        "I3C reset.",
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
            name: "Hbpcenr",
            extends: None,
            description: Some(
                "HB Peripheral Clock enable register (RCC_HBPCENR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1en",
                    description: Some(
                        "DMA clock enable.",
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
                    name: "dma2en",
                    description: Some(
                        "DMA2 clock enable.",
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
                    name: "crcen",
                    description: Some(
                        "CRC clock enable.",
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
                    name: "fmcen",
                    description: Some(
                        "FMC clock enable.",
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
                    name: "rngen",
                    description: Some(
                        "RNG clock enable.",
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
                    name: "sdmmcen",
                    description: Some(
                        "SDMMC clock enable.",
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
                    name: "usbhsen",
                    description: Some(
                        "USBHS clock enable.",
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
                    name: "usbssen",
                    description: Some(
                        "USBSS clock enable.",
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
                    name: "dvpen",
                    description: Some(
                        "DVP clock enable.",
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
                    name: "ethmacen",
                    description: Some(
                        "Ethernet MAC clock enable.",
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
                    name: "otgfsen",
                    description: Some(
                        "USBFS_OTG_FS clock enable.",
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
                    name: "uhsifen",
                    description: Some(
                        "UHSIF clock enable.",
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
                    name: "usbpden",
                    description: Some(
                        "USBPD clock enable.",
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
                    name: "serdesen",
                    description: Some(
                        "SERDES clock enable.",
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
                    name: "piocen",
                    description: Some(
                        "PIOC clock enable.",
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
            ],
        },
        FieldSet {
            name: "Hbrstr",
            extends: None,
            description: Some(
                "HB reset register (RCC_PHBRSTR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1rst",
                    description: Some(
                        "DMA1 reset.",
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
                    name: "dma2rst",
                    description: Some(
                        "DMA2 reset.",
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
                    name: "fmcrst",
                    description: Some(
                        "FMC reset.",
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
                    name: "rngrst",
                    description: Some(
                        "RNG reset.",
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
                    name: "sdmmcrst",
                    description: Some(
                        "SDMMC reset.",
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
                    name: "usbhsrst",
                    description: Some(
                        "USBHS reset.",
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
                    name: "usbssrst",
                    description: Some(
                        "USBSS reset.",
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
                    name: "dvprst",
                    description: Some(
                        "DVP reset.",
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
                    name: "ethmacrst",
                    description: Some(
                        "Ethernet MAC reset.",
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
                    name: "otgfsrst",
                    description: Some(
                        "USBFS_OTG_FS eset.",
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
                    name: "uhsifrst",
                    description: Some(
                        "UHSIF reset.",
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
                    name: "usbpdrst",
                    description: Some(
                        "USBPD reset.",
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
                    name: "serdesrst",
                    description: Some(
                        "SERDES reset.",
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
                    name: "piocrst",
                    description: Some(
                        "PIOC reset.",
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
            ],
        },
        FieldSet {
            name: "Intr",
            extends: None,
            description: Some(
                "Clock interrupt register (RCC_INTR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI Ready Interrupt flag.",
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
                    name: "lserdyf",
                    description: Some(
                        "LSE Ready Interrupt flag.",
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
                    name: "hsirdyf",
                    description: Some(
                        "HSI Ready Interrupt flag.",
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
                    name: "hserdyf",
                    description: Some(
                        "HSE Ready Interrupt flag.",
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
                    name: "pllrdyf",
                    description: Some(
                        "PLL Ready Interrupt flag.",
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
                    name: "ethpllrdyf",
                    description: Some(
                        "ETH Ready Interrupt flag.",
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
                    name: "serdespllrdyf",
                    description: Some(
                        "SERDES_PLL Ready Interrupt flag.",
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
                    name: "cssf",
                    description: Some(
                        "Clock Security System Interrupt flag.",
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
                    name: "lsirdyie",
                    description: Some(
                        "LSI Ready Interrupt Enable.",
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
                    name: "lserdyie",
                    description: Some(
                        "LSE Ready Interrupt Enable.",
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
                    name: "hsirdyie",
                    description: Some(
                        "HSI Ready Interrupt Enable.",
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
                    name: "hserdyie",
                    description: Some(
                        "HSE Ready Interrupt Enable.",
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
                    name: "pllrdyie",
                    description: Some(
                        "PLL Ready Interrupt Enable.",
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
                    name: "ethpllrdyie",
                    description: Some(
                        "ETHPLL Ready Interrupt Enable.",
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
                    name: "serdespllrdyie",
                    description: Some(
                        "SERDESPLL Ready Interrupt Enable.",
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
                    name: "lsirdyc",
                    description: Some(
                        "LSI Ready Interrupt Clear.",
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
                    name: "lserdyc",
                    description: Some(
                        "LSE Ready Interrupt Clear.",
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
                    name: "hsirdyc",
                    description: Some(
                        "HSI Ready Interrupt Clear.",
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
                    name: "hserdyc",
                    description: Some(
                        "HSE Ready Interrupt Clear.",
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
                    name: "pllrdyc",
                    description: Some(
                        "PLL Ready Interrupt Clear.",
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
                    name: "ethpllrdyc",
                    description: Some(
                        "ETH PLL Ready Interrupt Clear.",
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
                    name: "serdespllrdyc",
                    description: Some(
                        "SERDES Ready Interrupt Clear.",
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
                    name: "cssc",
                    description: Some(
                        "Clock security system interrupt clear.",
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
            name: "Pllcfgr",
            extends: None,
            description: Some(
                "PLL clock configuration register (RCC_PLLCFGR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllmul",
                    description: Some(
                        "Main PLL multiplication factor. Writable only while PLLON=0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Pllmul",
                    ),
                },
                Field {
                    name: "pllsrc",
                    description: Some(
                        "PLL input clock source. Writable only while PLLON=0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "pll_src_div",
                    description: Some(
                        "PLL input prescaler (1..64). Writable only while PLLON=0.",
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
                    name: "syspll_sel",
                    description: Some(
                        "System-clock-to-PLL selection. Writable only while SYSPLL_GATE=0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SyspllSel",
                    ),
                },
                Field {
                    name: "syspll_gate",
                    description: Some(
                        "System-clock-to-PLL gate. Must be set before switching SYSCLK to PLL output.",
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
            name: "Pllcfgr2",
            extends: None,
            description: Some(
                "PLL Clock configuration register2 (RCC_PLLCFGR2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usbhspllsrc",
                    description: Some(
                        "USBHS PLL input clock source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usbhspllsrc",
                    ),
                },
                Field {
                    name: "usbhspll_refsel",
                    description: Some(
                        "USBHS_PLL reference clock frequency. Writable only while USBHS_PLLON=0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "UsbhspllRefsel",
                    ),
                },
                Field {
                    name: "usbsspll_refsel",
                    description: Some(
                        "USBSS_PLL reference clock frequency. Writable only while USBSS_PLLON=0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "UsbsspllRefsel",
                    ),
                },
                Field {
                    name: "usbhspll_in_div",
                    description: Some(
                        "USBHS_PLL input prescaler from SYS_PLL (1..32). Writable only while USBHS_PLLON=0.",
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
                    name: "serdespll_mul",
                    description: Some(
                        "SERDES_PLL multiplication factor. Writable only while SERDES_PLLON=0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "SerdespllMul",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Rstsckr",
            extends: None,
            description: Some(
                "Control/status register (RCC_RSTSCKR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsion",
                    description: Some(
                        "Internal low speed oscillator enable.",
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
                    name: "lsirdy",
                    description: Some(
                        "Internal low speed oscillator ready.",
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
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag.",
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
                    name: "pinrstf",
                    description: Some(
                        "PIN reset flag.",
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
                    name: "porrstf",
                    description: Some(
                        "POR/PDR reset flag.",
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
                    name: "sftrstf",
                    description: Some(
                        "Software reset flag.",
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
                    name: "iwdgrstf",
                    description: Some(
                        "Independent watchdog reset flag.",
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
                    name: "wwdgrstf",
                    description: Some(
                        "Window watchdog reset flag.",
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
                    name: "lkuprstf",
                    description: Some(
                        "LOCKUP reset flag.",
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
    ],
    enums: &[
        Enum {
            name: "Adcpre",
            description: Some(
                "ADC prescaler (divides PB2 clock).",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PB2 divided by 2.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PB2 divided by 4.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PB2 divided by 6.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PB2 divided by 8.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Adcsrc",
            description: Some(
                "ADC input clock source selection.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HCLK",
                    description: Some(
                        "HCLK feeds the ADC prescaler.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USBHS_PLL",
                    description: Some(
                        "USBHS_PLL (480 MHz) feeds the ADC prescaler.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "ClkSrcPll",
            description: Some(
                "Single-bit clock source selecting between SYSCLK and PLL_CLK.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL_CLK",
                    description: Some(
                        "PLL_CLK selected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Eth1gsrc",
            description: Some(
                "Gigabit Ethernet 125 MHz clock source.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLL_CLK",
                    description: Some(
                        "PLL_CLK selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USBSS_PLL",
                    description: Some(
                        "USBSS_PLL selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ETH_PLL_DIV4",
                    description: Some(
                        "ETH_PLL divided by 4 selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SERDES_PLL_DIV8",
                    description: Some(
                        "SERDES_PLL divided by 8 selected.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Fpre",
            description: Some(
                "V5F core clock prescaler. Note 1xx encodes \"divide by 4\" (the MSB is don't-care for that case).",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "V5F clock not divided.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "V5F clock divided by 2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "V5F clock divided by 4.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Hpre",
            description: Some(
                "HB (AHB) prescaler.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "SYS_CLK not divided.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "SYS_CLK divided by 2.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "SYS_CLK divided by 4.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "SYS_CLK divided by 8.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "SYS_CLK divided by 16.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "SYS_CLK divided by 64.",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "SYS_CLK divided by 128.",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "SYS_CLK divided by 256.",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: Some(
                        "SYS_CLK divided by 512.",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Hsadcsrc",
            description: Some(
                "HSADC clock source. Encoding follows the WCH SDK header (RCC_HSADCSource_* in ch32h417_rcc.h) and the HSADC example which sets value 0 to obtain PLL_CLK. RM V1.7 section 3.4.13 lists the encoding for values 00 / 01 swapped (SYSCLK / PLL_CLK) — believed to be a doc typo since the SDK example is the canonical shipping code.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLL_CLK",
                    description: Some(
                        "PLL_CLK selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "USBHS_PLL",
                    description: Some(
                        "USBHS_PLL (480 MHz) selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ETH_PLL",
                    description: Some(
                        "ETH_PLL (500 MHz) selected.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ltdcsrc",
            description: Some(
                "LTDC clock source.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLL_CLK",
                    description: Some(
                        "PLL_CLK selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SERDES_PLL_DIV2",
                    description: Some(
                        "SERDES_PLL divided by 2 selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ETH_PLL",
                    description: Some(
                        "ETH_PLL selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "USBHS_PLL",
                    description: Some(
                        "USBHS_PLL selected.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mco",
            description: Some(
                "Microcontroller clock output (MCO pin) source selection.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NO_CLK",
                    description: Some(
                        "No clock output (encoded values 0000-0011).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK output.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI (25 MHz internal RC) output.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE output.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "PLL_DIV2",
                    description: Some(
                        "PLL clock divided by 2 output.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "UTMI",
                    description: Some(
                        "UTMI clock output.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "USBSS_PLL_DIV2",
                    description: Some(
                        "USBSS_PLL divided by 2 output.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "ETH_PLL_DIV8",
                    description: Some(
                        "ETH_PLL divided by 8 output.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "SERDES_PLL_DIV16",
                    description: Some(
                        "SERDES_PLL divided by 16 output.",
                    ),
                    value: 11,
                },
            ],
        },
        Enum {
            name: "Pllmul",
            description: Some(
                "Main PLL multiplication factor (writable only while PLLON=0).",
            ),
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "MUL4",
                    description: Some(
                        "PLL x 4.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MUL6",
                    description: Some(
                        "PLL x 6.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MUL7",
                    description: Some(
                        "PLL x 7.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MUL8",
                    description: Some(
                        "PLL x 8.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MUL8_5",
                    description: Some(
                        "PLL x 8.5.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "MUL9",
                    description: Some(
                        "PLL x 9.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "MUL9_5",
                    description: Some(
                        "PLL x 9.5.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "MUL10",
                    description: Some(
                        "PLL x 10.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "MUL10_5",
                    description: Some(
                        "PLL x 10.5.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "MUL11",
                    description: Some(
                        "PLL x 11.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "MUL11_5",
                    description: Some(
                        "PLL x 11.5.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "MUL12",
                    description: Some(
                        "PLL x 12.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "MUL12_5",
                    description: Some(
                        "PLL x 12.5.",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "MUL13",
                    description: Some(
                        "PLL x 13.",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "MUL14",
                    description: Some(
                        "PLL x 14.",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "MUL15",
                    description: Some(
                        "PLL x 15.",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "MUL16",
                    description: Some(
                        "PLL x 16.",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "MUL17",
                    description: Some(
                        "PLL x 17.",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "MUL18",
                    description: Some(
                        "PLL x 18.",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "MUL19",
                    description: Some(
                        "PLL x 19.",
                    ),
                    value: 19,
                },
                EnumVariant {
                    name: "MUL20",
                    description: Some(
                        "PLL x 20.",
                    ),
                    value: 20,
                },
                EnumVariant {
                    name: "MUL22",
                    description: Some(
                        "PLL x 22.",
                    ),
                    value: 21,
                },
                EnumVariant {
                    name: "MUL24",
                    description: Some(
                        "PLL x 24.",
                    ),
                    value: 22,
                },
                EnumVariant {
                    name: "MUL26",
                    description: Some(
                        "PLL x 26.",
                    ),
                    value: 23,
                },
                EnumVariant {
                    name: "MUL28",
                    description: Some(
                        "PLL x 28.",
                    ),
                    value: 24,
                },
                EnumVariant {
                    name: "MUL30",
                    description: Some(
                        "PLL x 30.",
                    ),
                    value: 25,
                },
                EnumVariant {
                    name: "MUL32",
                    description: Some(
                        "PLL x 32.",
                    ),
                    value: 26,
                },
                EnumVariant {
                    name: "MUL34",
                    description: Some(
                        "PLL x 34.",
                    ),
                    value: 27,
                },
                EnumVariant {
                    name: "MUL36",
                    description: Some(
                        "PLL x 36.",
                    ),
                    value: 28,
                },
                EnumVariant {
                    name: "MUL38",
                    description: Some(
                        "PLL x 38.",
                    ),
                    value: 29,
                },
                EnumVariant {
                    name: "MUL40",
                    description: Some(
                        "PLL x 40.",
                    ),
                    value: 30,
                },
                EnumVariant {
                    name: "MUL59",
                    description: Some(
                        "PLL x 59.",
                    ),
                    value: 31,
                },
            ],
        },
        Enum {
            name: "Pllsrc",
            description: Some(
                "Main PLL input clock source.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as PLL input.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as PLL input.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "USBHS_PLL",
                    description: Some(
                        "USBHS_PLL (480 MHz) selected as PLL input.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "ETH_PLL",
                    description: Some(
                        "ETH_PLL (500 MHz) selected as PLL input.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "USBSS_PLL",
                    description: Some(
                        "USBSS_PLL (125 MHz) selected as PLL input.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "SERDES_PLL_DIV2",
                    description: Some(
                        "SERDES_PLL divided by 2 selected as PLL input.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Ppre",
            description: Some(
                "PB (APB) prescaler.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "HCLK not divided.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "HCLK divided by 2.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "HCLK divided by 4.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "HCLK divided by 8.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "HCLK divided by 16.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Rtcsel",
            description: Some(
                "RTC clock source selection.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_CLK",
                    description: Some(
                        "No clock.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator selected as RTC clock.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator selected as RTC clock.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE_DIV512",
                    description: Some(
                        "HSE oscillator divided by 512 selected as RTC clock.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "SerdespllMul",
            description: Some(
                "SERDES_PLL multiplication factor.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "MUL25",
                    description: Some(
                        "SERDES_PLL x 25.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MUL28",
                    description: Some(
                        "SERDES_PLL x 28.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MUL30",
                    description: Some(
                        "SERDES_PLL x 30.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MUL32",
                    description: Some(
                        "SERDES_PLL x 32.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MUL35",
                    description: Some(
                        "SERDES_PLL x 35.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "MUL38",
                    description: Some(
                        "SERDES_PLL x 38.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "MUL40",
                    description: Some(
                        "SERDES_PLL x 40.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "MUL45",
                    description: Some(
                        "SERDES_PLL x 45.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "MUL50",
                    description: Some(
                        "SERDES_PLL x 50.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "MUL56",
                    description: Some(
                        "SERDES_PLL x 56.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "MUL60",
                    description: Some(
                        "SERDES_PLL x 60.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "MUL64",
                    description: Some(
                        "SERDES_PLL x 64.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "MUL70",
                    description: Some(
                        "SERDES_PLL x 70.",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "MUL76",
                    description: Some(
                        "SERDES_PLL x 76.",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "MUL80",
                    description: Some(
                        "SERDES_PLL x 80.",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "MUL90",
                    description: Some(
                        "SERDES_PLL x 90.",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: Some(
                "System clock source.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as system clock.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as system clock.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "PLL selected as system clock.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "SyspllSel",
            description: Some(
                "System-clock-to-PLL output selection (writable only while SYSPLL_GATE=0).",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL_CLK",
                    description: Some(
                        "PLL_CLK (covers any encoding 0xx).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USBHS_PLL",
                    description: Some(
                        "USBHS_PLL (480 MHz).",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "ETH_PLL",
                    description: Some(
                        "ETH_PLL (500 MHz).",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "SERDES_PLL_DIV2",
                    description: Some(
                        "SERDES_PLL divided by 2.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "USBSS_PLL",
                    description: Some(
                        "USBSS_PLL (125 MHz).",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Uhsifsrc",
            description: Some(
                "UHSIF clock source.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL_CLK",
                    description: Some(
                        "PLL_CLK selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "USBHS_PLL",
                    description: Some(
                        "USBHS_PLL (480 MHz) selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ETH_PLL",
                    description: Some(
                        "ETH_PLL (500 MHz) selected.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Usbfsdiv",
            description: Some(
                "USBFS 48 MHz prescaler. Selects integer or half-integer divisors of the source clock.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "Divide by 1.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Divide by 2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "Divide by 3.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Divide by 4.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "Divide by 5.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "Divide by 6.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "Divide by 8.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "Divide by 10.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV1_5",
                    description: Some(
                        "Divide by 1.5.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV2_5",
                    description: Some(
                        "Divide by 2.5.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV3_5",
                    description: Some(
                        "Divide by 3.5.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV4_5",
                    description: Some(
                        "Divide by 4.5.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV5_5",
                    description: Some(
                        "Divide by 5.5.",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV6_5",
                    description: Some(
                        "Divide by 6.5.",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV7_5",
                    description: Some(
                        "Divide by 7.5.",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV9_5",
                    description: Some(
                        "Divide by 9.5.",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Usbfssrc",
            description: Some(
                "USBFS 48 MHz clock source.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "PLL clock selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USBHS_PLL",
                    description: Some(
                        "USBHS_PLL clock selected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "UsbhspllRefsel",
            description: Some(
                "USBHS_PLL reference clock frequency.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "F25MHZ",
                    description: Some(
                        "25 MHz.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "F20MHZ",
                    description: Some(
                        "20 MHz.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "F24MHZ",
                    description: Some(
                        "24 MHz.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "F32MHZ",
                    description: Some(
                        "32 MHz.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Usbhspllsrc",
            description: Some(
                "USBHS_PLL input clock source.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as USBHS_PLL input.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as USBHS_PLL input.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ETHCLK_20M",
                    description: Some(
                        "ETHCLK_20M selected as USBHS_PLL input.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SYS_PLL_DIV",
                    description: Some(
                        "SYS_PLL_CLK divided by USBHSPLL_IN_DIV selected as USBHS_PLL input.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "UsbsspllRefsel",
            description: Some(
                "USBSS_PLL reference clock frequency.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "F20MHZ",
                    description: Some(
                        "20 MHz.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "F24MHZ",
                    description: Some(
                        "24 MHz.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "F25MHZ",
                    description: Some(
                        "25 MHz.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "F30MHZ",
                    description: Some(
                        "30 MHz.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "F32MHZ",
                    description: Some(
                        "32 MHz.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "F40MHZ",
                    description: Some(
                        "40 MHz.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "F60MHZ",
                    description: Some(
                        "60 MHz.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "F80MHZ",
                    description: Some(
                        "80 MHz.",
                    ),
                    value: 7,
                },
            ],
        },
    ],
};
