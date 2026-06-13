use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "FLASH.",
            ),
            items: &[
                BlockItem {
                    name: "keyr",
                    description: Some(
                        "Flash key register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Keyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obkeyr",
                    description: Some(
                        "Flash option key register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Obkeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "statr",
                    description: Some(
                        "Status register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Statr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr",
                    description: Some(
                        "Control register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addr",
                    description: Some(
                        "Flash address register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Addr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obr",
                    description: Some(
                        "Option byte register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Obr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpr",
                    description: Some(
                        "Write protection register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "modekeyr",
                    description: Some(
                        "Mode select register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Modekeyr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addr",
            extends: None,
            description: Some(
                "Flash address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "far",
                    description: Some(
                        "Flash Address.",
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
            name: "Ctlr",
            extends: None,
            description: Some(
                "Control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some(
                        "Programming.",
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
                    name: "per",
                    description: Some(
                        "Page Erase.",
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
                    name: "mer",
                    description: Some(
                        "Mass Erase.",
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
                    name: "obpg",
                    description: Some(
                        "Option byte programming.",
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
                    name: "ober",
                    description: Some(
                        "Option byte erase.",
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
                    name: "strt",
                    description: Some(
                        "Start.",
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
                    name: "lock",
                    description: Some(
                        "Lock.",
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
                    name: "obwre",
                    description: Some(
                        "Option bytes write enable.",
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
                    name: "errie",
                    description: Some(
                        "Error interrupt enable.",
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
                    name: "eopie",
                    description: Some(
                        "End of operation interrupt enable.",
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
                    name: "flock",
                    description: Some(
                        "Fast programmable lock.",
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
                    name: "page_pg",
                    description: Some(
                        "Fast programming.",
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
                    name: "page_er",
                    description: Some(
                        "Fast erase.",
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
                Field {
                    name: "ber32",
                    description: Some(
                        "Block Erase 32K.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ber64",
                    description: Some(
                        "Block Erase 64K.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgstart",
                    description: Some(
                        "Page Programming Start.",
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
                    name: "rsenact",
                    description: Some(
                        "Reset Flash Enhance read mode.",
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
                    name: "enhancemode",
                    description: Some(
                        "Flash Enhance read mode.",
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
                    name: "sckmode",
                    description: Some(
                        "Flash SCK mode.",
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
                    name: "btpg",
                    description: Some(
                        "Undocumented; OR with PAGE_PG and PGSTART to program the BOOT region (0x1FFF8000). Guessed name.",
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
                    name: "bter",
                    description: Some(
                        "Undocumented; OR with PER (not PAGE_ER) for 4 KB erase of the BOOT region (0x1FFF8000). Guessed name.",
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
            ],
        },
        FieldSet {
            name: "Keyr",
            extends: None,
            description: Some(
                "Flash key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "keyr",
                    description: Some(
                        "FPEC key.",
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
            name: "Modekeyr",
            extends: None,
            description: Some(
                "Mode select register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "modekeyr",
                    description: Some(
                        "Mode select.",
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
            name: "Obkeyr",
            extends: None,
            description: Some(
                "Flash option key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "optkey",
                    description: Some(
                        "Option byte key.",
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
            name: "Obr",
            extends: None,
            description: Some(
                "Option byte register — read-only mirror of the Option Bytes loaded from flash at reset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oberr",
                    description: Some(
                        "Option byte integrity error — a value/complement pair did not match when loading.",
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
                    name: "rdprt",
                    description: Some(
                        "Read protection is currently active.",
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
                    name: "iwdg_sw",
                    description: Some(
                        "Independent watchdog start mode currently in effect.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "IwdgMode",
                    ),
                },
                Field {
                    name: "stop_rst",
                    description: Some(
                        "Stop-mode reset behavior currently in effect.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "PowerModeReset",
                    ),
                },
                Field {
                    name: "standy_rst",
                    description: Some(
                        "Standby-mode reset behavior currently in effect.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "PowerModeReset",
                    ),
                },
                Field {
                    name: "sram_code_mode",
                    description: Some(
                        "SRAM/CODE allocation currently in effect. Mirrors the upper two bits of OB.USER.RAM_CODE; the variant table is chip-specific (V2 D8/D8W vs V3 high-memory). On V3 parts the 110/111 OB encodings are not distinguishable here — read OB.USER.RAM_CODE directly to tell them apart.",
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
            ],
        },
        FieldSet {
            name: "Statr",
            extends: None,
            description: Some(
                "Status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bsy",
                    description: Some(
                        "Busy.",
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
                    name: "wr_bsy",
                    description: Some(
                        "Quick page programming.",
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
                    name: "wrprterr",
                    description: Some(
                        "Write protection error.",
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
                    name: "eop",
                    description: Some(
                        "End of operation.",
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
                    name: "enhance_mod_sta",
                    description: Some(
                        "Enhance mode start.",
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
            ],
        },
        FieldSet {
            name: "Wpr",
            extends: None,
            description: Some(
                "Write protection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp",
                    description: Some(
                        "Write protect.",
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
    ],
    enums: &[
        Enum {
            name: "IwdgMode",
            description: Some(
                "When the independent watchdog turns on after reset.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HARDWARE",
                    description: Some(
                        "Watchdog starts automatically at power-on.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SOFTWARE",
                    description: Some(
                        "Watchdog stays off until software enables it.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PowerModeReset",
            description: Some(
                "Whether entering a low-power mode triggers a system reset.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset the chip on entering the mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NO_RESET",
                    description: Some(
                        "Keep running on entering the mode.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
