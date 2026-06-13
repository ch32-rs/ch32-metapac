include!("../metadata_0019.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
#[path = "../../registers/esig_common.rs"]
pub mod nv_esig;
#[path = "../../registers/ob_v2_v3.rs"]
pub mod nv_ob;
pub static NV_STRUCTS: &[NvStructBinding] = &[
    NvStructBinding {
        region: "VND",
        structs: &[NvStruct {
            name: "ESIG",
            offset: 0xe0,
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
            version: "v2_v3",
            block: "OB",
            defaults: &[("NRDPR", 90), ("NUSER", 192), ("RDPR", 165), ("USER", 63)],
            ir: &nv_ob::DESCRIPTOR,
        }],
    },
];
pub static METADATA: Metadata = Metadata {
    name: "CH32V203F8U6",
    family: "QingKe RISC-V-based, general-purpose MCU",
    line: "General-purpose",
    memory: crate::memory_select::MEMORY,
    nv_structs: NV_STRUCTS,
    memory_options: &[MemoryOption {
        name: "default",
        region_sizes: &[("RAM", 20480), ("USR_1", 65536)],
    }],
    default_memory_option: "default",
    peripherals: PERIPHERALS,
    // nvic_priority_bits: 0,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
