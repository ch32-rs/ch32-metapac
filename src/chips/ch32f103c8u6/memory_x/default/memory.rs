use crate::mem_layout::{Access, MemoryRegion, MemoryRegionKind, MemoryRole::*, Mode::*};

pub static MEMORY: &[MemoryRegion] = &[
    MemoryRegion {
        name: "USR_1",
        kind: MemoryRegionKind::Flash,
        role: Application,
        address: 0x8000000,
        size: 65536,
        modes: &[
            Fast {
                page_size: 128,
                load_size: 16,
            },
            Standard {
                erase_size: 1024,
                write_size: 2,
            },
        ],
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
        address: 0x1ffff000,
        size: 2048,
        modes: &[
            Fast {
                page_size: 128,
                load_size: 16,
            },
            Standard {
                erase_size: 1024,
                write_size: 2,
            },
        ],
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
        size: 128,
        modes: &[Standard {
            erase_size: 128,
            write_size: 2,
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
        address: 0x1ffff880,
        size: 128,
        modes: &[],
        access: Some(Access {
            read: true,
            write: false,
            execute: false,
        }),
    },
    MemoryRegion {
        name: "SYS_2",
        kind: MemoryRegionKind::Flash,
        role: System,
        address: 0x1ffff900,
        size: 1792,
        modes: &[
            Fast {
                page_size: 128,
                load_size: 16,
            },
            Standard {
                erase_size: 1024,
                write_size: 2,
            },
        ],
        access: Some(Access {
            read: true,
            write: true,
            execute: true,
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
