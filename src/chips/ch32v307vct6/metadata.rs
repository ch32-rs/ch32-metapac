include!("../metadata_0021.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "CH32V307VCT6",
    family: "QingKe RISC-V-based, general-purpose MCU",
    line: "Interconnectivity (USB high-speed, CAN, Ethernet, DVP, SDIO, FSMC)",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x0,
            size: 262144,
            settings: Some(FlashSettings {
                erase_size: 1024,
                write_size: 256,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 65536,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    // nvic_priority_bits: 0,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
