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
                load_size: 4,
            },
            Standard {
                erase_size: 128,
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
        address: 0x1ffff380,
        size: 128,
        modes: &[],
        access: Some(Access {
            read: true,
            write: false,
            execute: false,
        }),
    },
    MemoryRegion {
        name: "OPT",
        kind: MemoryRegionKind::Flash,
        role: OptionBytes,
        address: 0x1ffff300,
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
        name: "RAM",
        kind: MemoryRegionKind::Ram,
        role: Ram,
        address: 0x20000000,
        size: 12288,
        modes: &[],
        access: Some(Access {
            read: true,
            write: true,
            execute: true,
        }),
    },
];
