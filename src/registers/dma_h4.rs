use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ch",
            extends: None,
            description: Some(
                "DMA channel cluster (CFGR / CNTR / PADDR / MADDR / M1ADDR). M1ADDR holds the second memory address when double-buffer mode is enabled.",
            ),
            items: &[
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "Channel configuration register (DMA_CFGR).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cntr",
                    description: Some(
                        "Channel number-of-data register (DMA_CNTR).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "paddr",
                    description: Some(
                        "Channel peripheral address register (DMA_PADDR).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "maddr",
                    description: Some(
                        "Channel memory address register (DMA_MADDR, also used as memory0 in double-buffer mode).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "m1addr",
                    description: Some(
                        "Channel second memory address (DMA_M1ADDR, double-buffer mode only).",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Dma",
            extends: None,
            description: Some(
                "DMA controller (8 channels per instance). H4 introduces double-buffer mode via M1ADDR and routes requests via DMAMUX.",
            ),
            items: &[
                BlockItem {
                    name: "intfr",
                    description: Some(
                        "Interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Intfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intfcr",
                    description: Some(
                        "Interrupt flag clear register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Intfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Channel cluster: CFGR / CNTR / PADDR / MADDR / M1ADDR. Stride 20 bytes per channel.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 20,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Ch",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "DMA channel configuration register (DMA_CFGR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Channel enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcie",
                    description: Some(
                        "Transfer complete interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "htie",
                    description: Some(
                        "Half Transfer interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "teie",
                    description: Some(
                        "Transfer error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dir",
                    description: Some(
                        "Data transfer direction.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "circ",
                    description: Some(
                        "Circular mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pinc",
                    description: Some(
                        "Peripheral increment mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "minc",
                    description: Some(
                        "Memory increment mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psize",
                    description: Some(
                        "Peripheral size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msize",
                    description: Some(
                        "Memory size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pl",
                    description: Some(
                        "Channel Priority level.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mem2mem",
                    description: Some(
                        "Memory to memory mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "double_mode",
                    description: Some(
                        "Double buffer mode enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flag_cur_mem",
                    description: Some(
                        "Memory address selection setting.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cntr",
            extends: None,
            description: Some(
                "DMA channel 1 number of data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ndt",
                    description: Some(
                        "Number of data to transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intfcr",
            extends: None,
            description: Some(
                "DMA interrupt flag clear register (DMA_INTFCR). Per-channel clear bits (write 1 to clear).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cgif",
                    description: Some(
                        "Channel global interrupt clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ctcif",
                    description: Some(
                        "Channel transfer complete clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "chtif",
                    description: Some(
                        "Channel half-transfer complete clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "cteif",
                    description: Some(
                        "Channel transfer error clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intfr",
            extends: None,
            description: Some(
                "DMA interrupt status register (DMA_INTFR). Per-channel global / TC / HT / TE flags.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gif",
                    description: Some(
                        "Channel global interrupt flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tcif",
                    description: Some(
                        "Channel transfer complete flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "htif",
                    description: Some(
                        "Channel half-transfer complete flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "teif",
                    description: Some(
                        "Channel transfer error flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
