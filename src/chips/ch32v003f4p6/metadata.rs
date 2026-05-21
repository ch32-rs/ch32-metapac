include!("../metadata_0008.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "CH32V003F4P6",
    family: "QingKe RISC-V-based, general-purpose MCU",
    line: "General-purpose",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x0,
            size: 16384,
            settings: Some(FlashSettings {
                erase_size: 1024,
                write_size: 64,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 2048,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    // nvic_priority_bits: 0,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
