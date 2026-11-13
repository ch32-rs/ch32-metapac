use crate::mem_layout::{Access, MemoryRegion, MemoryRegionKind, MemoryRole::*, Mode::*};

pub static MEMORY: &[MemoryRegion] = &[
    MemoryRegion {
        name: "USR_1",
        kind: MemoryRegionKind::Flash,
        role: Application,
        address: 0x8000000,
        size: 262144,
        modes: &[
            Fast {
                page_size: 256,
                load_size: 4,
            },
            Standard {
                erase_size: 4096,
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
        address: 0x1fff8000,
        size: 28672,
        modes: &[
            Fast {
                page_size: 256,
                load_size: 4,
            },
            Standard {
                erase_size: 4096,
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
        size: 65536,
        modes: &[],
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
        name: "USR_2",
        kind: MemoryRegionKind::Flash,
        role: Application,
        address: 0x8040000,
        size: 229376,
        modes: &[
            Fast {
                page_size: 256,
                load_size: 4,
            },
            Standard {
                erase_size: 4096,
                write_size: 2,
            },
        ],
        access: Some(Access {
            read: true,
            write: true,
            execute: true,
        }),
    },
];
