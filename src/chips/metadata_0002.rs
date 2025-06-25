
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "PFIC",
        address: 0xe000e000,
        registers: Some(PeripheralRegisters {
            kind: "pfic",
            version: "rv2",
            block: "PFIC",
            ir: &pfic::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SYSTICK",
        address: 0xe000f000,
        registers: Some(PeripheralRegisters {
            kind: "systick",
            version: "rv2",
            block: "SYSTICK",
            ir: &systick::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "v0",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 0x40010400,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "common",
            block: "EXTI",
            ir: &exti::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EXTI0",
            interrupt: "EXTI7_0",
        }],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "v0",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[PeripheralPin {
            pin: "PC4",
            signal: "MCO",
            remap: None,
        }],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v1",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "AHBPCENR",
                field: "DMA1EN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CHANNEL7",
            },
        ],
    },
    Peripheral {
        name: "AFIO",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "afio",
            version: "v0",
            block: "AFIO",
            ir: &afio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "AFIOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "AFIORST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x40010800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v0",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "IOPAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "IOPARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 0x40011000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v0",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "IOPCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "IOPCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOD",
        address: 0x40011400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v0",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "IOPDEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "IOPDRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "TIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "TIM1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PC5",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "BKIN",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "CH1N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH2N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "CH3N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "ETR",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH3",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CH4",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "BKIN",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH2N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "CH3N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "ETR",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH3",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH4",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "BKIN",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "CH1N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH2N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "CH3N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "ETR",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH1",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH3",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CH4",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "BKIN",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CH2N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH3N",
                remap: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "TIM2_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PD4",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "ETR",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CH3",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH4",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "ETR",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH3",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CH4",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "ETR",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CH3",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "CH4",
                remap: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
        ],
    },
    Peripheral {
        name: "USART1",
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "common",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "USART1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PD4",
                signal: "CK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "RTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "RTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "TX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "RX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CK",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "TX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RTS",
                remap: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v0",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PC2",
                signal: "SCL",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SDA",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "SCL",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "SDA",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SCL",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "SDA",
                remap: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1_ER",
            },
        ],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v0",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "SPI1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PC1",
                signal: "NSS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SCK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "MISO",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "MOSI",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "NSS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SCK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "MISO",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "MOSI",
                remap: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v0",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "ADC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "ADC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "IN0",
                remap: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                remap: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN2",
                remap: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "IN3",
                remap: None,
            },
            PeripheralPin {
                pin: "PD3",
                signal: "IN4",
                remap: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "IN5",
                remap: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "IN6",
                remap: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "IN7",
                remap: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: Some("DMA1_CH1"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "OPA",
        address: 0x40023800,
        registers: Some(PeripheralRegisters {
            kind: "extend",
            version: "v0",
            block: "EXTEND",
            ir: &extend::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "NEG",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "POS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "NEG",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "POS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "OUT",
                remap: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 16,
    },
    Interrupt {
        name: "PVD",
        number: 17,
    },
    Interrupt {
        name: "FLASH",
        number: 18,
    },
    Interrupt {
        name: "RCC",
        number: 19,
    },
    Interrupt {
        name: "EXTI7_0",
        number: 20,
    },
    Interrupt {
        name: "AWU",
        number: 21,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 22,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 23,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 24,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 25,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 26,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 27,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 28,
    },
    Interrupt {
        name: "ADC",
        number: 29,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 30,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 31,
    },
    Interrupt {
        name: "USART1",
        number: 32,
    },
    Interrupt {
        name: "SPI1",
        number: 33,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 34,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 35,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 36,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 37,
    },
    Interrupt {
        name: "TIM2",
        number: 38,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
];
#[path = "../registers/adc_v0.rs"]
pub mod adc;
#[path = "../registers/afio_v0.rs"]
pub mod afio;
#[path = "../registers/dma_v1.rs"]
pub mod dma;
#[path = "../registers/extend_v0.rs"]
pub mod extend;
#[path = "../registers/exti_common.rs"]
pub mod exti;
#[path = "../registers/flash_v0.rs"]
pub mod flash;
#[path = "../registers/gpio_v0.rs"]
pub mod gpio;
#[path = "../registers/i2c_v0.rs"]
pub mod i2c;
#[path = "../registers/pfic_rv2.rs"]
pub mod pfic;
#[path = "../registers/rcc_v0.rs"]
pub mod rcc;
#[path = "../registers/spi_v0.rs"]
pub mod spi;
#[path = "../registers/systick_rv2.rs"]
pub mod systick;
#[path = "../registers/timer_v3.rs"]
pub mod timer;
#[path = "../registers/usart_common.rs"]
pub mod usart;
