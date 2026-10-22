use crate::mem_layout::{Access, MemoryRegion, MemoryRegionKind, MemoryRole::*, Mode::*};

pub static MEMORY: &[MemoryRegion] = &[
    MemoryRegion {
        name: "USR_1",
        kind: MemoryRegionKind::Flash,
        role: Application,
        address: 0x8000000,
        size: 65536,
        modes: &[Fast {
            page_size: 256,
            load_size: 4,
        }],
        access: Some(Access {
            read: true,
            write: true,
            execute: true,
        }),
    },
    MemoryRegion {
        name: "SYS_1",
        kind: MemoryRegionKind::Flash,
        role: System,
        address: 0x1fff0000,
        size: 3328,
        modes: &[Fast {
            page_size: 256,
            load_size: 4,
        }],
        access: Some(Access {
            read: true,
            write: true,
            execute: true,
        }),
    },
    MemoryRegion {
        name: "OPT",
        kind: MemoryRegionKind::Flash,
        role: OptionBytes,
        address: 0x1ffff800,
        size: 256,
        modes: &[Fast {
            page_size: 256,
            load_size: 4,
        }],
        access: Some(Access {
            read: true,
            write: true,
            execute: false,
        }),
    },
    MemoryRegion {
        name: "VND",
        kind: MemoryRegionKind::Flash,
        role: Vendor,
        address: 0x1ffff700,
        size: 256,
        modes: &[],
        access: Some(Access {
            read: true,
            write: false,
            execute: false,
        }),
    },
    MemoryRegion {
        name: "RAM",
        kind: MemoryRegionKind::Ram,
        role: Ram,
        address: 0x20000000,
        size: 20480,
        modes: &[],
        access: Some(Access {
            read: true,
            write: true,
            execute: true,
        }),
    },
];
