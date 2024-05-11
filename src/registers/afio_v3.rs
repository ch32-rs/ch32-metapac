use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Afio",
        extends: None,
        description: Some("Alternate function I/O."),
        items: &[
            BlockItem {
                name: "ecr",
                description: Some("Event Control Register (AFIO_ECR)."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ecr"),
                }),
            },
            BlockItem {
                name: "pcfr1",
                description: Some("AF remap and debug I/O configuration register (AFIO_PCFR1)."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pcfr1"),
                }),
            },
            BlockItem {
                name: "exticr",
                description: Some("External interrupt configuration registers (AFIO_EXTICRs)."),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Exticr"),
                }),
            },
            BlockItem {
                name: "pcfr2",
                description: Some("AF remap and debug I/O configuration register (AFIO_PCFR2)."),
                array: None,
                byte_offset: 0x1c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pcfr2"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ecr",
            extends: None,
            description: Some("Event Control Register (AFIO_ECR)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pin",
                    description: Some("Pin selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "port",
                    description: Some("Port selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "evoe",
                    description: Some("Event Output Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some("External interrupt configuration register (AFIO_EXTICRx)."),
            bit_size: 32,
            fields: &[Field {
                name: "exti",
                description: Some("EXTI configuration."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Pcfr1",
            extends: None,
            description: Some("AF remap and debug I/O configuration register (AFIO_PCFR1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1_rm",
                    description: Some("SPI1 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1_rm",
                    description: Some("I2C1 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1_rm",
                    description: Some("USART1 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2_rm",
                    description: Some("USART2 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3_rm",
                    description: Some("USART3 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1_rm",
                    description: Some("TIM1 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim2_rm",
                    description: Some("TIM2 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3_rm",
                    description: Some("TIM3 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim4_rm",
                    description: Some("TIM4 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can1_rm",
                    description: Some("CAN1 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pd01_rm",
                    description: Some("Port D0/Port D1 mapping on OSCIN/OSCOUT."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim5ch4_rm",
                    description: Some("TIM5 channel4 internal remap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1_etrginj_rm",
                    description: Some("ADC 1 External trigger injected conversion remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1_etrgreg_rm",
                    description: Some("ADC 1 external trigger regular conversion remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2_etrginj_rm",
                    description: Some("ADC 2 External trigger injected conversion remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2_etrgreg_rm",
                    description: Some("ADC 2 external trigger regular conversion remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eth_rm",
                    description: Some("Ethernet remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can2_rm",
                    description: Some("CAN2 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mii_rmii_sel",
                    description: Some("MII_RMII_SEL."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swcfg",
                    description: Some("Serial wire JTAG configuration."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3_rm",
                    description: Some("SPI3 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim2itra_rm",
                    description: Some("TIM2 internally triggers 1 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 29 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ptp_ppsp_rm",
                    description: Some("Ethernet PTP_PPS remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcfr2",
            extends: None,
            description: Some("AF remap and debug I/O configuration register (AFIO_PCFR2)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim8_rm",
                    description: Some("TIM8 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim9_rm",
                    description: Some("TIM9 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim10_rm",
                    description: Some("TIM10 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fsmc_nadv",
                    description: Some("FSMC_NADV."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart4_rm",
                    description: Some("USART4 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart5_rm",
                    description: Some("USART5 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart6_rm",
                    description: Some("USART6 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart7_rm",
                    description: Some("USART7 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart8_rm",
                    description: Some("USART8 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1_rm2",
                    description: Some("USART1 remapping."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
