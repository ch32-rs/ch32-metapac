use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Eth",
            extends: None,
            description: Some(
                "Ethernet MAC, integrated PHY, MMC and IEEE 1588.",
            ),
            items: &[
                BlockItem {
                    name: "maccr",
                    description: Some(
                        "Ethernet MAC configuration register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macffr",
                    description: Some(
                        "Ethernet MAC frame filter register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macffr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "machthr",
                    description: Some(
                        "Ethernet MAC hash table high register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Machthr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "machtlr",
                    description: Some(
                        "Ethernet MAC hash table low register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Machtlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macmiiar",
                    description: Some(
                        "Ethernet MAC MII address register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macmiiar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macmiidr",
                    description: Some(
                        "Ethernet MAC MII data register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macmiidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macfcr",
                    description: Some(
                        "Ethernet MAC flow control register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macvlantr",
                    description: Some(
                        "Ethernet MAC VLAN tag register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macvlantr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macrwuffr",
                    description: Some(
                        "Ethernet MAC remote wakeup frame filter register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "macpmtcsr",
                    description: Some(
                        "Ethernet MAC PMT control and status register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macpmtcsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macsr",
                    description: Some(
                        "Ethernet MAC interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macimr",
                    description: Some(
                        "Ethernet MAC interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca0hr",
                    description: Some(
                        "Ethernet MAC address 0 high register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca0hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca0lr",
                    description: Some(
                        "Ethernet MAC address 0 low register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca0lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca1hr",
                    description: Some(
                        "Ethernet MAC address 1 high register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca1hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca1lr",
                    description: Some(
                        "Ethernet MAC address 1 low register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca1lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca2hr",
                    description: Some(
                        "Ethernet MAC address 2 high register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca2hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca2lr",
                    description: Some(
                        "Ethernet MAC address 2 low register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca2lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca3hr",
                    description: Some(
                        "Ethernet MAC address 3 high register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca3hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca3lr",
                    description: Some(
                        "Ethernet MAC address 3 low register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca3lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cr",
                    description: Some(
                        "PHY configure register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmccr",
                    description: Some(
                        "Ethernet MMC control register (ETH_MMCCR).",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrir",
                    description: Some(
                        "Ethernet MMC receive interrupt register (ETH_MMCRIR).",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctir",
                    description: Some(
                        "Ethernet MMC transmit interrupt register (ETH_MMCTIR).",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrimr",
                    description: Some(
                        "Ethernet MMC receive interrupt mask register (ETH_MMCRIMR).",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctimr",
                    description: Some(
                        "Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR).",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctgfsccr",
                    description: Some(
                        "Ethernet MMC transmitted good frames after a single collision counter.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctgfsccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctgfmsccr",
                    description: Some(
                        "Ethernet MMC transmitted good frames after more than a single collision.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctgfmsccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctgfcr",
                    description: Some(
                        "Ethernet MMC transmitted good frames counter register.",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctgfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrfcecr",
                    description: Some(
                        "Ethernet MMC received frames with CRC error counter register.",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrfcecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrfaecr",
                    description: Some(
                        "Ethernet MMC received frames with alignment error counter register.",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrfaecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrafcr",
                    description: Some(
                        "Ethernet MMC received frames counter register.",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrafcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "state",
                    description: Some(
                        "Ethernet state machine register.",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "State",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrgufcr",
                    description: Some(
                        "MMC received good unicast frames counter register.",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrgufcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptscr",
                    description: Some(
                        "Ethernet PTP time stamp control register (ETH_PTPTSCR).",
                    ),
                    array: None,
                    byte_offset: 0x700,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptpssir",
                    description: Some(
                        "Ethernet PTP subsecond increment register.",
                    ),
                    array: None,
                    byte_offset: 0x704,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptpssir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptshr",
                    description: Some(
                        "Ethernet PTP time stamp high register.",
                    ),
                    array: None,
                    byte_offset: 0x708,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptshr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptslr",
                    description: Some(
                        "Ethernet PTP time stamp low register (ETH_PTPTSLR).",
                    ),
                    array: None,
                    byte_offset: 0x70c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptslr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptshur",
                    description: Some(
                        "Ethernet PTP time stamp high update register.",
                    ),
                    array: None,
                    byte_offset: 0x710,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptshur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptslur",
                    description: Some(
                        "Ethernet PTP time stamp low update register (ETH_PTPTSLUR).",
                    ),
                    array: None,
                    byte_offset: 0x714,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptslur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptsar",
                    description: Some(
                        "Ethernet PTP time stamp addend register.",
                    ),
                    array: None,
                    byte_offset: 0x718,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptsar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptthr",
                    description: Some(
                        "Ethernet PTP target time high register.",
                    ),
                    array: None,
                    byte_offset: 0x71c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptthr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptpttlr",
                    description: Some(
                        "Ethernet PTP target time low register.",
                    ),
                    array: None,
                    byte_offset: 0x720,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptpttlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmabmr",
                    description: Some(
                        "Ethernet DMA bus mode register",
                    ),
                    array: None,
                    byte_offset: 0x1000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmabmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmatpdr",
                    description: Some(
                        "Ethernet DMA transmit poll demand register",
                    ),
                    array: None,
                    byte_offset: 0x1004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmatpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmarpdr",
                    description: Some(
                        "Ethernet DMA receive poll demand register",
                    ),
                    array: None,
                    byte_offset: 0x1008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmarpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmardlar",
                    description: Some(
                        "Ethernet DMA receive descriptor list address register",
                    ),
                    array: None,
                    byte_offset: 0x100c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmardlar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmatdlar",
                    description: Some(
                        "Ethernet DMA transmit descriptor list address register",
                    ),
                    array: None,
                    byte_offset: 0x1010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmatdlar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmasr",
                    description: Some(
                        "Ethernet DMA status register",
                    ),
                    array: None,
                    byte_offset: 0x1014,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmasr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaomr",
                    description: Some(
                        "Ethernet DMA operation mode register",
                    ),
                    array: None,
                    byte_offset: 0x1018,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaomr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaier",
                    description: Some(
                        "Ethernet DMA interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x101c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmamfbocr",
                    description: Some(
                        "Ethernet DMA missed frame and buffer overflow counter register",
                    ),
                    array: None,
                    byte_offset: 0x1020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamfbocr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachtdr",
                    description: Some(
                        "Ethernet DMA current host transmit descriptor register",
                    ),
                    array: None,
                    byte_offset: 0x1048,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachtdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachrdr",
                    description: Some(
                        "Ethernet DMA current host receive descriptor register",
                    ),
                    array: None,
                    byte_offset: 0x104c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachrdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachtbar",
                    description: Some(
                        "Ethernet DMA current host transmit buffer address register",
                    ),
                    array: None,
                    byte_offset: 0x1050,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachtbar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachrbar",
                    description: Some(
                        "Ethernet DMA current host receive buffer address register",
                    ),
                    array: None,
                    byte_offset: 0x1054,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachrbar",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Dmabmr",
            extends: None,
            description: Some(
                "Ethernet DMA bus mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sr",
                    description: Some(
                        "Software reset",
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
                    name: "da",
                    description: Some(
                        "DMA arbitration",
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
                    name: "dsl",
                    description: Some(
                        "Descriptor skip length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pbl",
                    description: Some(
                        "Programmable burst length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtpr",
                    description: Some(
                        "Rx Tx priority ratio",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fb",
                    description: Some(
                        "Fixed burst",
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
                Field {
                    name: "rdp",
                    description: Some(
                        "Rx DMA PBL",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usp",
                    description: Some(
                        "Use separate PBL",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpm",
                    description: Some(
                        "4xPBL mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aab",
                    description: Some(
                        "Address-aligned beats",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmachrbar",
            extends: None,
            description: Some(
                "Ethernet DMA current host receive buffer address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hrbap",
                    description: Some(
                        "Host receive buffer address pointer",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmachrdr",
            extends: None,
            description: Some(
                "Ethernet DMA current host receive descriptor register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hrdap",
                    description: Some(
                        "Host receive descriptor address pointer",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmachtbar",
            extends: None,
            description: Some(
                "Ethernet DMA current host transmit buffer address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htbap",
                    description: Some(
                        "Host transmit buffer address pointer",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmachtdr",
            extends: None,
            description: Some(
                "Ethernet DMA current host transmit descriptor register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htdap",
                    description: Some(
                        "Host transmit descriptor address pointer",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmaier",
            extends: None,
            description: Some(
                "Ethernet DMA interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tie",
                    description: Some(
                        "Transmit interrupt enable",
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
                    name: "tpsie",
                    description: Some(
                        "Transmit process stopped interrupt enable",
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
                    name: "tbuie",
                    description: Some(
                        "Transmit buffer unavailable interrupt enable",
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
                    name: "tjtie",
                    description: Some(
                        "Transmit jabber timeout interrupt enable",
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
                    name: "roie",
                    description: Some(
                        "Overflow interrupt enable",
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
                    name: "tuie",
                    description: Some(
                        "Underflow interrupt enable",
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
                    name: "rie",
                    description: Some(
                        "Receive interrupt enable",
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
                    name: "rbuie",
                    description: Some(
                        "Receive buffer unavailable interrupt enable",
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
                    name: "rpsie",
                    description: Some(
                        "Receive process stopped interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rwtie",
                    description: Some(
                        "Receive watchdog timeout interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "etie",
                    description: Some(
                        "Early transmit interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fbeie",
                    description: Some(
                        "Fatal bus error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erie",
                    description: Some(
                        "Early receive interrupt enable",
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
                    name: "aise",
                    description: Some(
                        "Abnormal interrupt summary enable",
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
                    name: "nise",
                    description: Some(
                        "Normal interrupt summary enable",
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
                Field {
                    name: "iple",
                    description: Some(
                        "10M PHY link status change interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmamfbocr",
            extends: None,
            description: Some(
                "Ethernet DMA missed frame and buffer overflow counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfc",
                    description: Some(
                        "Missed frames by the controller",
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
                Field {
                    name: "omfc",
                    description: Some(
                        "Overflow bit for missed frame counter",
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
                Field {
                    name: "mfa",
                    description: Some(
                        "Missed frames by the application",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ofoc",
                    description: Some(
                        "Overflow bit for FIFO overflow counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmaomr",
            extends: None,
            description: Some(
                "Ethernet DMA operation mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sr",
                    description: Some(
                        "Start/stop receive",
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
                    name: "osf",
                    description: Some(
                        "Operate on second frame",
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
                    name: "rtc",
                    description: Some(
                        "Receive threshold control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fugf",
                    description: Some(
                        "Forward undersized good frames",
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
                    name: "fef",
                    description: Some(
                        "Forward error frames",
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
                    name: "st",
                    description: Some(
                        "Start/stop transmission",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ttc",
                    description: Some(
                        "Transmit threshold control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ftf",
                    description: Some(
                        "Flush transmit FIFO",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsf",
                    description: Some(
                        "Transmit store and forward",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfrf",
                    description: Some(
                        "Disable flushing of received frames",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsf",
                    description: Some(
                        "Receive store and forward",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtcefd",
                    description: Some(
                        "Dropping of TCP/IP checksum error frames disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmardlar",
            extends: None,
            description: Some(
                "Ethernet DMA receive descriptor list address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srl",
                    description: Some(
                        "Start of receive list",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmarpdr",
            extends: None,
            description: Some(
                "Ethernet DMA receive poll demand register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rpd",
                    description: Some(
                        "Receive poll demand",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmasr",
            extends: None,
            description: Some(
                "Ethernet DMA status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ts",
                    description: Some(
                        "Transmit status",
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
                    name: "tpss",
                    description: Some(
                        "Transmit process stopped status",
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
                    name: "tbus",
                    description: Some(
                        "Transmit buffer unavailable status",
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
                    name: "tjts",
                    description: Some(
                        "Transmit jabber timeout status",
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
                    name: "ros",
                    description: Some(
                        "Receive overflow status",
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
                    name: "tus",
                    description: Some(
                        "Transmit underflow status",
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
                    name: "rs",
                    description: Some(
                        "Receive status",
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
                    name: "rbus",
                    description: Some(
                        "Receive buffer unavailable status",
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
                    name: "rpss",
                    description: Some(
                        "Receive process stopped status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rwts",
                    description: Some(
                        "Receive watchdog timeout status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ets",
                    description: Some(
                        "Early transmit status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fbes",
                    description: Some(
                        "Fatal bus error status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ers",
                    description: Some(
                        "Early receive status",
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
                    name: "ais",
                    description: Some(
                        "Abnormal interrupt summary",
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
                    name: "nis",
                    description: Some(
                        "Normal interrupt summary",
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
                Field {
                    name: "rps",
                    description: Some(
                        "Receive process state",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tps",
                    description: Some(
                        "Transmit process state",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ebs",
                    description: Some(
                        "Error bits status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmcs",
                    description: Some(
                        "MMC status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pmts",
                    description: Some(
                        "PMT status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsts",
                    description: Some(
                        "Time stamp trigger status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ipls",
                    description: Some(
                        "10M PHY physical layer status (write 1 to clear)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmatdlar",
            extends: None,
            description: Some(
                "Ethernet DMA transmit descriptor list address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stl",
                    description: Some(
                        "Start of transmit list",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmatpdr",
            extends: None,
            description: Some(
                "Ethernet DMA transmit poll demand register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tpd",
                    description: Some(
                        "Transmit poll demand",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca0hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 0 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca0h",
                    description: Some(
                        "MAC address0 high",
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
                Field {
                    name: "mo",
                    description: Some(
                        "Always 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca0lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 0 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca0l",
                    description: Some(
                        "MAC address0 low",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca1hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 1 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca1h",
                    description: Some(
                        "MAC address1 high",
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
                Field {
                    name: "mbc",
                    description: Some(
                        "Mask byte control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "Source address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae",
                    description: Some(
                        "Address enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca1lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 1 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca1l",
                    description: Some(
                        "MAC address1 low",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca2hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 2 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca2h",
                    description: Some(
                        "MAC address2 high",
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
                Field {
                    name: "mbc",
                    description: Some(
                        "Mask byte control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "Source address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae",
                    description: Some(
                        "Address enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca2lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 2 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca2l",
                    description: Some(
                        "MAC address2 low",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca3hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 3 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca3h",
                    description: Some(
                        "MAC address3 high",
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
                Field {
                    name: "mbc",
                    description: Some(
                        "Mask byte control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "Source address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae",
                    description: Some(
                        "Address enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca3lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 3 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca3l",
                    description: Some(
                        "MAC address3 low",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maccr",
            extends: None,
            description: Some(
                "Ethernet MAC configuration register (ETH_MACCR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "re",
                    description: Some(
                        "Receiver enable.",
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
                    name: "te",
                    description: Some(
                        "Transmitter enable.",
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
                    name: "apcs",
                    description: Some(
                        "Automatic pad/CRC stripping.",
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
                    name: "ipco",
                    description: Some(
                        "IPv4 checksum offload.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dm",
                    description: Some(
                        "Duplex mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lm",
                    description: Some(
                        "Loopback mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fes",
                    description: Some(
                        "Fast Ethernet speed.",
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
                    name: "ifg",
                    description: Some(
                        "Interframe gap.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jd",
                    description: Some(
                        "Jabber disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wd",
                    description: Some(
                        "Watchdog disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macfcr",
            extends: None,
            description: Some(
                "Ethernet MAC flow control register (ETH_MACFCR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fcb",
                    description: Some(
                        "Flow control busy.",
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
                    name: "tfce",
                    description: Some(
                        "Transmit flow control enable.",
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
                    name: "rfce",
                    description: Some(
                        "Receive flow control enable.",
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
                    name: "upfd",
                    description: Some(
                        "Unicast pause frame detect.",
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
                    name: "pt",
                    description: Some(
                        "Pass control frames.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macffr",
            extends: None,
            description: Some(
                "Ethernet MAC frame filter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pm",
                    description: Some(
                        "Promiscuous mode",
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
                    name: "hu",
                    description: Some(
                        "Hash unicast",
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
                    name: "hm",
                    description: Some(
                        "Hash multicast",
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
                    name: "daif",
                    description: Some(
                        "Destination address inverse filtering",
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
                    name: "pam",
                    description: Some(
                        "Pass all multicast",
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
                    name: "bfd",
                    description: Some(
                        "Broadcast frames disable",
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
                    name: "pcf",
                    description: Some(
                        "Pass control frames",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saif",
                    description: Some(
                        "Source address inverse filtering",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saf",
                    description: Some(
                        "Source address filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpf",
                    description: Some(
                        "Hash or perfect filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ra",
                    description: Some(
                        "Receive all",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Machthr",
            extends: None,
            description: Some(
                "Ethernet MAC hash table high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hth",
                    description: Some(
                        "Hash table high",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Machtlr",
            extends: None,
            description: Some(
                "Ethernet MAC hash table low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htl",
                    description: Some(
                        "Hash table low",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macimr",
            extends: None,
            description: Some(
                "Ethernet MAC interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pmtim",
                    description: Some(
                        "PMT interrupt mask",
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
                    name: "tstim",
                    description: Some(
                        "Time stamp trigger interrupt mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macmiiar",
            extends: None,
            description: Some(
                "Ethernet MAC MII address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mb",
                    description: Some(
                        "MII busy",
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
                    name: "mw",
                    description: Some(
                        "MII write",
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
                    name: "cr",
                    description: Some(
                        "Clock range",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mr",
                    description: Some(
                        "MII register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa",
                    description: Some(
                        "PHY address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macmiidr",
            extends: None,
            description: Some(
                "Ethernet MAC MII data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md",
                    description: Some(
                        "MII data",
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
            name: "Macpmtcsr",
            extends: None,
            description: Some(
                "Ethernet MAC PMT control and status register (ETH_MACPMTCSR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "Power down.",
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
                    name: "mpe",
                    description: Some(
                        "Magic Packet enable.",
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
                    name: "wfe",
                    description: Some(
                        "Wakeup frame enable.",
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
                    name: "phye",
                    description: Some(
                        "The PMEB enabling bit of PHY.",
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
                    name: "mpr",
                    description: Some(
                        "Magic packet received.",
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
                    name: "wfr",
                    description: Some(
                        "Wakeup frame received.",
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
                    name: "phyr",
                    description: Some(
                        "the Wakeup frame flag bit of PHY.",
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
                    name: "gu",
                    description: Some(
                        "Global unicast.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wffrpr",
                    description: Some(
                        "Wakeup frame filter register pointer reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macsr",
            extends: None,
            description: Some(
                "Ethernet MAC interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pmts",
                    description: Some(
                        "PMT status",
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
                    name: "mmcs",
                    description: Some(
                        "MMC status",
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
                    name: "mmcrs",
                    description: Some(
                        "MMC receive status",
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
                    name: "mmcts",
                    description: Some(
                        "MMC transmit status",
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
                    name: "tsts",
                    description: Some(
                        "Time stamp trigger status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macvlantr",
            extends: None,
            description: Some(
                "Ethernet MAC VLAN tag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlanti",
                    description: Some(
                        "VLAN tag identifier (for receive frames)",
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
                Field {
                    name: "vlantc",
                    description: Some(
                        "12-bit VLAN tag comparison",
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
            name: "Mmccr",
            extends: None,
            description: Some(
                "Ethernet MMC control register (ETH_MMCCR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr",
                    description: Some(
                        "Counter reset.",
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
                    name: "csr",
                    description: Some(
                        "Counter stop rollover.",
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
                    name: "ror",
                    description: Some(
                        "Reset on read.",
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
                    name: "mcf",
                    description: Some(
                        "MMC counter freeze.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrafcr",
            extends: None,
            description: Some(
                "Ethernet MMC received frames counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rafcr",
                    description: Some(
                        "the number of all frames Received.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrfaecr",
            extends: None,
            description: Some(
                "Ethernet MMC received frames with alignment error counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfaecr",
                    description: Some(
                        "Received frames with alignment error counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrfcecr",
            extends: None,
            description: Some(
                "Ethernet MMC received frames with CRC error counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfcfcr",
                    description: Some(
                        "Received frames with CRC error counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrgufcr",
            extends: None,
            description: Some(
                "MMC received good unicast frames counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rgufc",
                    description: Some(
                        "Received good unicast frames counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrimr",
            extends: None,
            description: Some(
                "Ethernet MMC receive interrupt mask register (ETH_MMCRIMR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfcem",
                    description: Some(
                        "Received frame CRC error mask.",
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
                    name: "rfaem",
                    description: Some(
                        "Received more than half of the unaligned error frame interrupt masking bit.",
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
                    name: "rgufm",
                    description: Some(
                        "Received good unicast frames mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrir",
            extends: None,
            description: Some(
                "Ethernet MMC receive interrupt register (ETH_MMCRIR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfces",
                    description: Some(
                        "Received frames CRC error status.",
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
                    name: "rgufs",
                    description: Some(
                        "Received Good Unicast Frames Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmctgfcr",
            extends: None,
            description: Some(
                "Ethernet MMC transmitted good frames counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfc",
                    description: Some(
                        "Transmitted good frames counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmctgfmsccr",
            extends: None,
            description: Some(
                "Ethernet MMC transmitted good frames after more than a single collision.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfmsccr",
                    description: Some(
                        "Transmitted good frames after more than a single collision counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmctgfsccr",
            extends: None,
            description: Some(
                "Ethernet MMC transmitted good frames after a single collision counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfsccr",
                    description: Some(
                        "Transmitted good frames after a single collision counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmctimr",
            extends: None,
            description: Some(
                "Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfscm",
                    description: Some(
                        "send a single conflict flags for a good frames.",
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
                    name: "tgfmscm",
                    description: Some(
                        "send multiple conflict flags for good frames.",
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
                    name: "tgfm",
                    description: Some(
                        "Transmitted good frames mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmctir",
            extends: None,
            description: Some(
                "Ethernet MMC transmit interrupt register (ETH_MMCTIR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfsgs",
                    description: Some(
                        "Transmitted good frames status.",
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
                    name: "tgfmscs",
                    description: Some(
                        "Transmitted good frames status.",
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
                    name: "tgfs",
                    description: Some(
                        "Transmitted good frames status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PhyCr",
            extends: None,
            description: Some(
                "PHY configure register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rephyaddr",
                    description: Some(
                        "When bit7 is set to 1,bit4 to 0 serve as the address of the PHY.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phyaddr_en",
                    description: Some(
                        "configure the address of the PHY.",
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
                    name: "speed",
                    description: Some(
                        "Speed selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dupl_ex",
                    description: Some(
                        "connection mode selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phy_pd",
                    description: Some(
                        "PHY power-down mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phy_rstn",
                    description: Some(
                        "Ethernet PHY global reset signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptpssir",
            extends: None,
            description: Some(
                "Ethernet PTP subsecond increment register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stssi",
                    description: Some(
                        "System time subsecond increment.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptsar",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp addend register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsa",
                    description: Some(
                        "Time stamp addend.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptscr",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp control register (ETH_PTPTSCR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tse",
                    description: Some(
                        "Time stamp enable.",
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
                    name: "tsfcu",
                    description: Some(
                        "Time stamp fine or coarse update.",
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
                    name: "tssti",
                    description: Some(
                        "Time stamp system time initialize.",
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
                    name: "tsstu",
                    description: Some(
                        "Time stamp system time update.",
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
                    name: "tsite",
                    description: Some(
                        "Time stamp interrupt trigger enable.",
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
                    name: "tsaru",
                    description: Some(
                        "Time stamp addend register update.",
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
            ],
        },
        FieldSet {
            name: "Ptptshr",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp high register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sts",
                    description: Some(
                        "System time second.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptshur",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp high update register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsus",
                    description: Some(
                        "Time stamp update second.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptslr",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp low register (ETH_PTPTSLR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stss",
                    description: Some(
                        "System time subseconds.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpns",
                    description: Some(
                        "System time positive or negative sign.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptslur",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp low update register (ETH_PTPTSLUR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsuss",
                    description: Some(
                        "Time stamp update subseconds.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptthr",
            extends: None,
            description: Some(
                "Ethernet PTP target time high register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ttsh",
                    description: Some(
                        "Target time stamp high.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptpttlr",
            extends: None,
            description: Some(
                "Ethernet PTP target time low register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ttsl",
                    description: Some(
                        "Target time stamp low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "State",
            extends: None,
            description: Some(
                "Ethernet state machine register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "etx_state",
                    description: Some(
                        "Ethernet Receiving state machine.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erx_state",
                    description: Some(
                        "Ethernet sending state machine.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
