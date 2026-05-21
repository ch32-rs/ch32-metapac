use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Afio",
            extends: None,
            description: Some(
                "Alternate function I/O.",
            ),
            items: &[
                BlockItem {
                    name: "pcfr1",
                    description: Some(
                        "AF remap and debug I/O configuration register (AFIO_PCFR1).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcfr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpio_afr",
                    description: Some(
                        "GPIO alternate-function selection. 6 ports (A..F) x 2 halves (low pins 0..7, high pins 8..15) = 12 registers. Index as `gpio_afr(port_idx * 2 + (pin / 8)).afr(pin % 8) = af_num`, where port_idx is A=0, B=1, ..., F=5.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Afr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exticr",
                    description: Some(
                        "External interrupt configuration register (AFIO_EXTICRx). EXTICR[0] holds EXTI lines 0..7, EXTICR[1] holds lines 8..15.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Exticr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Afr",
            extends: None,
            description: Some(
                "GPIO alternate-function selection. Each 4-bit field selects the AF for one pin (8 pins per register).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afr",
                    description: Some(
                        "AF selection for the pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some(
                "External interrupt configuration register (AFIO_EXTICRx). Each 4-bit field selects which GPIO port drives the corresponding EXTI line.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "GPIO port that drives the EXTI line.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: Some(
                        "ExtiPort",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pcfr1",
            extends: None,
            description: Some(
                "AF remap and debug I/O configuration register (AFIO_PCFR1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd0pd1_rm",
                    description: Some(
                        "PD0PD1 remapping.",
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
                    name: "adc1_etrgreg_rm",
                    description: Some(
                        "ADC1_ETRGREG remapping.",
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
                    name: "adc1_etrginj_rm",
                    description: Some(
                        "ADC1_ETRGINJ remapping.",
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
                    name: "adc2_etrgreg_rm",
                    description: Some(
                        "ADC2_ETRGREG remapping.",
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
                    name: "adc2_etrginj_rm",
                    description: Some(
                        "ADC2_ETRGINJ remapping.",
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
                    name: "uhsif_clk_rm",
                    description: Some(
                        "UHSIF_CLK remapping.",
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
                Field {
                    name: "uhsif_port_rm",
                    description: Some(
                        "UHSIF_PORT remapping.",
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
                    name: "sdmmc_rm",
                    description: Some(
                        "SDMMC remapping.",
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
                    name: "tim2itr1_rm",
                    description: Some(
                        "TIM2ITR1 remapping.",
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
                    name: "vio18_io_hslv",
                    description: Some(
                        "VIO18 IO speed configuration at low voltage.",
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
                    name: "vio33_io_hslv",
                    description: Some(
                        "VIO33 IO speed configuration at low voltage.",
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
                    name: "vdd33_io_hslv",
                    description: Some(
                        "VDD33 IO speed configuration at low voltage.",
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
                    name: "usbpd_cc_hvt",
                    description: Some(
                        "CC pin input channel threshold adjustment.",
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
                    name: "sw_cfg",
                    description: Some(
                        "Serial wire JTAG configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "ExtiPort",
            description: Some(
                "GPIO port selection for an EXTI line (encoding matches SDK GPIO_PortSourceGPIO*). Only EXTI lines 0..15 are GPIO-routable; lines 16..26 are peripheral wake events.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "PA",
                    description: Some(
                        "GPIOA pin selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PB",
                    description: Some(
                        "GPIOB pin selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PC",
                    description: Some(
                        "GPIOC pin selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PD",
                    description: Some(
                        "GPIOD pin selected.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PE",
                    description: Some(
                        "GPIOE pin selected.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PF",
                    description: Some(
                        "GPIOF pin selected (CH32H4 family only).",
                    ),
                    value: 5,
                },
            ],
        },
    ],
};
