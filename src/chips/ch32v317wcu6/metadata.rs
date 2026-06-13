include!("../metadata_0034.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
#[path = "../../registers/esig_common.rs"]
pub mod nv_esig;
#[path = "../../registers/ob_v3_ram_code.rs"]
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
            version: "v3_ram_code",
            block: "OB",
            defaults: &[("NRDPR", 90), ("NUSER", 96), ("RDPR", 165), ("USER", 159)],
            ir: &nv_ob::DESCRIPTOR,
        }],
    },
];
pub static METADATA: Metadata = Metadata {
    name: "CH32V317WCU6",
    family: "QingKe RISC-V-based, general-purpose MCU",
    line: "Interconnectivity (USB high-speed, CAN, Ethernet, DVP, SDIO, FSMC)",
    memory: crate::memory_select::MEMORY,
    nv_structs: NV_STRUCTS,
    memory_options: &[
        MemoryOption {
            name: "c192_r128",
            region_sizes: &[("USR_1", 196608), ("USR_2", 294912), ("RAM", 131072)],
        },
        MemoryOption {
            name: "c224_r96",
            region_sizes: &[("USR_1", 229376), ("USR_2", 262144), ("RAM", 98304)],
        },
        MemoryOption {
            name: "c256_r64",
            region_sizes: &[("USR_1", 262144), ("USR_2", 229376), ("RAM", 65536)],
        },
        MemoryOption {
            name: "c128_r192",
            region_sizes: &[("USR_1", 131072), ("USR_2", 360448), ("RAM", 196608)],
        },
        MemoryOption {
            name: "c288_r32",
            region_sizes: &[("USR_1", 294912), ("USR_2", 196608), ("RAM", 32768)],
        },
    ],
    default_memory_option: "c256_r64",
    peripherals: PERIPHERALS,
    // nvic_priority_bits: 0,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
