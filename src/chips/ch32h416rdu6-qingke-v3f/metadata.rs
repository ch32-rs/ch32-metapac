include!("../metadata_0001.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "CH32H416RDU6",
    family: "CH32H4 (QingKe RISC-V dual-core, high-end)",
    line: "Dual-core V3F+V5F, USB 3.0, 480K Flash variant, 60-pin QFN",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x0,
            size: 491520,
            settings: Some(FlashSettings {
                erase_size: 1024,
                write_size: 256,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "ITCM",
            kind: MemoryRegionKind::Ram,
            address: 0x200a0000,
            size: 131072,
            settings: None,
        },
        MemoryRegion {
            name: "DTCM",
            kind: MemoryRegionKind::Ram,
            address: 0x200c0000,
            size: 262144,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM_SHARED",
            kind: MemoryRegionKind::Ram,
            address: 0x20100000,
            size: 524288,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    // nvic_priority_bits: 0,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
