include!("../metadata_0042.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
#[path = "../../registers/esig_common.rs"]
pub mod nv_esig;
#[path = "../../registers/ob_v4x7_ram_code.rs"]
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
            version: "v4x7_ram_code",
            block: "OB",
            defaults: &[("NRDPR", 90), ("NUSER", 64), ("RDPR", 165), ("USER", 191)],
            ir: &nv_ob::DESCRIPTOR,
        }],
    },
];
pub static METADATA: Metadata = Metadata {
    name: "CH32V407RET6",
    family: "QingKe RISC-V-based, general-purpose MCU",
    line: "Interconnectivity (USB high-speed, Ethernet, CAN, DVP, SDIO, LTDC)",
    memory: crate::memory_select::MEMORY,
    nv_structs: NV_STRUCTS,
    memory_options: &[
        MemoryOption {
            name: "c512_r200",
            region_sizes: &[("USR_1", 524288), ("USR_2", 491520), ("RAM", 204800)],
        },
        MemoryOption {
            name: "c576_r136",
            region_sizes: &[("USR_1", 589824), ("USR_2", 425984), ("RAM", 139264)],
        },
    ],
    default_memory_option: "c512_r200",
    peripherals: PERIPHERALS,
    // nvic_priority_bits: 0,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
