include!("../metadata_0021.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
#[path = "../../registers/esig_common.rs"]
pub mod nv_esig;
#[path = "../../registers/ob_m030.rs"]
pub mod nv_ob;
pub static NV_STRUCTS: &[NvStructBinding] = &[
    NvStructBinding {
        region: "VND",
        structs: &[NvStruct {
            name: "ESIG",
            offset: 0x20,
            kind: "esig",
            version: "common",
            block: "ESIG",
            defaults: &[],
            ir: &nv_esig::DESCRIPTOR,
        }],
    },
    NvStructBinding {
        region: "OPT",
        structs: &[NvStruct {
            name: "OB",
            offset: 0x0,
            kind: "ob",
            version: "m030",
            block: "OB",
            defaults: &[("NRDPR", 90), ("NUSER", 64), ("RDPR", 165), ("USER", 191)],
            ir: &nv_ob::DESCRIPTOR,
        }],
    },
];
pub static METADATA: Metadata = Metadata {
    name: "CH32M030G8R7",
    family: "QingKe RISC-V-based, motor control MCU",
    line: "Motor control, half-bridge gate driver",
    memory: crate::memory_select::MEMORY,
    nv_structs: NV_STRUCTS,
    memory_options: &[MemoryOption {
        name: "default",
        region_sizes: &[("RAM", 12288), ("USR_1", 65536)],
    }],
    default_memory_option: "default",
    peripherals: PERIPHERALS,
    // nvic_priority_bits: 0,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
