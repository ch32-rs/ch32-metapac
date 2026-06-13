include!("../metadata_0024.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
#[path = "../../registers/esig_common.rs"]
pub mod nv_esig;
#[path = "../../registers/ob_v2_ram_code.rs"]
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
            version: "v2_ram_code",
            block: "OB",
            defaults: &[("NRDPR", 90), ("NUSER", 224), ("RDPR", 165), ("USER", 31)],
            ir: &nv_ob::DESCRIPTOR,
        }],
    },
];
pub static METADATA: Metadata = Metadata {
    name: "CH32V208RBT6",
    family: "QingKe RISC-V-based, general-purpose MCU",
    line: "Wireless (BLE5.X, CAN, USB, Ethernet)",
    memory: crate::memory_select::MEMORY,
    nv_structs: NV_STRUCTS,
    memory_options: &[
        MemoryOption {
            name: "c128_r64",
            region_sizes: &[("USR_1", 131072), ("USR_2", 360448), ("RAM", 65536)],
        },
        MemoryOption {
            name: "c144_r48",
            region_sizes: &[("USR_1", 147456), ("USR_2", 344064), ("RAM", 49152)],
        },
        MemoryOption {
            name: "c160_r32",
            region_sizes: &[("USR_1", 163840), ("USR_2", 327680), ("RAM", 32768)],
        },
    ],
    default_memory_option: "c128_r64",
    peripherals: PERIPHERALS,
    // nvic_priority_bits: 0,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
