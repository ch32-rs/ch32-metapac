
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "PFIC",
        address: 0xe000e000,
        registers: Some(PeripheralRegisters {
            kind: "pfic",
            version: "v4x7",
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
            version: "rv4",
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
            version: "x315",
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI7_0",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI7_0",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI15_8",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15_8",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "x315",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[PeripheralPin {
            pin: "PA8",
            signal: "MCO",
            remap: None,
            af: None,
        }],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "x315",
            block: "DMA1",
            ir: &dma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
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
                interrupt: "DMA1_CH1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CH2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CH3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CH4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CH5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CH6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CH7",
            },
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "DMA1_CH8",
            },
            PeripheralInterrupt {
                signal: "CH9",
                interrupt: "DMA1_CH9",
            },
            PeripheralInterrupt {
                signal: "CH10",
                interrupt: "DMA1_CH10",
            },
            PeripheralInterrupt {
                signal: "CH11",
                interrupt: "DMA1_CH11",
            },
        ],
    },
    Peripheral {
        name: "PWR",
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "x315",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "PWRRST",
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
            version: "h4",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "IOPAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
        name: "GPIOB",
        address: 0x40010c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "h4",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "IOPBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "IOPBRST",
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
            version: "h4",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "IOPCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
            version: "h4",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "IOPDEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
        name: "AFIO",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "afio",
            version: "x315",
            block: "AFIO",
            ir: &afio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "AFIOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
        name: "RTC",
        address: 0x40002800,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "common",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v00x",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "WWDGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "WWDGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v00x",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "h4",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "CRCEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTEND",
        address: 0x400220c0,
        registers: Some(PeripheralRegisters {
            kind: "extend",
            version: "x315",
            block: "EXTEND",
            ir: &extend::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "x315",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("ADC"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "ADC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "ADC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: Some("DMA1_CH1"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 0x40012800,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "x315",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("ADC"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "ADC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "ADC2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC2",
            channel: Some("DMA1_CH10"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC2",
        }],
    },
    Peripheral {
        name: "ADC3",
        address: 0x40013c00,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "x315",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("ADC"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "ADC3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "ADC3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC3",
            channel: Some("DMA1_CH9"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC3",
        }],
    },
    Peripheral {
        name: "ADC4",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "x315",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("ADC"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "ADC4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "ADC4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC4",
            channel: Some("DMA1_CH8"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC4",
        }],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "x315",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "TIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
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
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "x315",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
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
                signal: "CH4",
                channel: Some("DMA1_CH8"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM2",
        }],
    },
    Peripheral {
        name: "TIM3",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "x315",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "TIM3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH11"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH9"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM3",
        }],
    },
    Peripheral {
        name: "TIM4",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "x315",
            block: "GPTM32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH10"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM4",
        }],
    },
    Peripheral {
        name: "USART1",
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "USART1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
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
        name: "USART2",
        address: 0x40004400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "USART2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
    },
    Peripheral {
        name: "USART3",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "USART3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
    },
    Peripheral {
        name: "USART4",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "USART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "USART4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH10"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH11"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART4",
        }],
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
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
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
        name: "SPI2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v0",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v0",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
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
                register: "HB1PCENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH7"),
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
        name: "I2C2",
        address: 0x40005800,
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
                register: "HB1PCENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "I2C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ER",
            },
        ],
    },
    Peripheral {
        name: "ARGB",
        address: 0x40025000,
        registers: Some(PeripheralRegisters {
            kind: "argb",
            version: "x315",
            block: "ARGB",
            ir: &argb::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "ARGBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "ARGBRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ARGB",
            channel: Some("DMA1_CH11"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ARGB",
        }],
    },
    Peripheral {
        name: "USBHS",
        address: 0x40030000,
        registers: Some(PeripheralRegisters {
            kind: "usbhs",
            version: "x315",
            block: "USB",
            ir: &usbhs::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("USBHSPLLCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "USBHSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "USBHSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USBHS",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "USBHS_WKUP",
            },
        ],
    },
    Peripheral {
        name: "USBPD",
        address: 0x40024400,
        registers: Some(PeripheralRegisters {
            kind: "usbpd",
            version: "x315",
            block: "USBPD",
            ir: &usbpd::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "USBPDEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "USBPDRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USBPD",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USBPD_WKUP",
            },
        ],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 16,
    },
    Interrupt {
        name: "EXTI15_8",
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
        name: "SPI2",
        number: 21,
    },
    Interrupt {
        name: "DMA1_CH1",
        number: 22,
    },
    Interrupt {
        name: "DMA1_CH2",
        number: 23,
    },
    Interrupt {
        name: "DMA1_CH3",
        number: 24,
    },
    Interrupt {
        name: "DMA1_CH4",
        number: 25,
    },
    Interrupt {
        name: "DMA1_CH5",
        number: 26,
    },
    Interrupt {
        name: "DMA1_CH6",
        number: 27,
    },
    Interrupt {
        name: "DMA1_CH7",
        number: 28,
    },
    Interrupt {
        name: "DMA1_CH8",
        number: 29,
    },
    Interrupt {
        name: "DMA1_CH9",
        number: 30,
    },
    Interrupt {
        name: "DMA1_CH10",
        number: 31,
    },
    Interrupt {
        name: "DMA1_CH11",
        number: 32,
    },
    Interrupt {
        name: "USART2",
        number: 33,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 34,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 35,
    },
    Interrupt {
        name: "USART1",
        number: 36,
    },
    Interrupt {
        name: "SPI1",
        number: 37,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 38,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 39,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 40,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 41,
    },
    Interrupt {
        name: "TIM2",
        number: 42,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 43,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 44,
    },
    Interrupt {
        name: "USBPD",
        number: 45,
    },
    Interrupt {
        name: "USBPD_WKUP",
        number: 46,
    },
    Interrupt {
        name: "ARGB",
        number: 47,
    },
    Interrupt {
        name: "TIM3",
        number: 48,
    },
    Interrupt {
        name: "TIM4",
        number: 49,
    },
    Interrupt {
        name: "SPI3",
        number: 50,
    },
    Interrupt {
        name: "USART3",
        number: 51,
    },
    Interrupt {
        name: "USART4",
        number: 52,
    },
    Interrupt {
        name: "USBSS",
        number: 53,
    },
    Interrupt {
        name: "USBSS_LINK",
        number: 54,
    },
    Interrupt {
        name: "USBHS",
        number: 55,
    },
    Interrupt {
        name: "USBHS_WKUP",
        number: 56,
    },
    Interrupt {
        name: "USBSS_WKUP",
        number: 57,
    },
    Interrupt {
        name: "PVD",
        number: 58,
    },
    Interrupt {
        name: "ADC1",
        number: 59,
    },
    Interrupt {
        name: "ADC2",
        number: 60,
    },
    Interrupt {
        name: "ADC3",
        number: 61,
    },
    Interrupt {
        name: "RTCALARM",
        number: 62,
    },
    Interrupt {
        name: "RTC",
        number: 63,
    },
    Interrupt {
        name: "USART_WKUP",
        number: 64,
    },
    Interrupt {
        name: "ADC4",
        number: 65,
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
    DmaChannel {
        name: "DMA1_CH8",
        dma: "DMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH9",
        dma: "DMA1",
        channel: 8,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH10",
        dma: "DMA1",
        channel: 9,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH11",
        dma: "DMA1",
        channel: 10,
        dmamux: None,
        dmamux_channel: None,
    },
];
#[path = "../registers/adc_x315.rs"]
pub mod adc;
#[path = "../registers/afio_x315.rs"]
pub mod afio;
#[path = "../registers/argb_x315.rs"]
pub mod argb;
#[path = "../registers/crc_h4.rs"]
pub mod crc;
#[path = "../registers/dma_x315.rs"]
pub mod dma;
#[path = "../registers/extend_x315.rs"]
pub mod extend;
#[path = "../registers/exti_common.rs"]
pub mod exti;
#[path = "../registers/flash_x315.rs"]
pub mod flash;
#[path = "../registers/gpio_h4.rs"]
pub mod gpio;
#[path = "../registers/i2c_v0.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v00x.rs"]
pub mod iwdg;
#[path = "../registers/pfic_v4x7.rs"]
pub mod pfic;
#[path = "../registers/pwr_x315.rs"]
pub mod pwr;
#[path = "../registers/rcc_x315.rs"]
pub mod rcc;
#[path = "../registers/rtc_common.rs"]
pub mod rtc;
#[path = "../registers/spi_v0.rs"]
pub mod spi;
#[path = "../registers/systick_rv4.rs"]
pub mod systick;
#[path = "../registers/timer_x315.rs"]
pub mod timer;
#[path = "../registers/usart_h4.rs"]
pub mod usart;
#[path = "../registers/usbhs_x315.rs"]
pub mod usbhs;
#[path = "../registers/usbpd_x315.rs"]
pub mod usbpd;
#[path = "../registers/wwdg_v00x.rs"]
pub mod wwdg;
