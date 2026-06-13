use crate::metadata::ir::*;
pub(crate) static DESCRIPTOR: IR = IR {
    blocks: &[
        Block {
            name: "OB",
            extends: None,
            description: Some(
                "Persistent flash configuration — read protection, watchdog mode, low-power reset behavior, SRAM/CODE split, write protection, and two free user-data bytes.",
            ),
            items: &[
                BlockItem {
                    name: "RDPR",
                    description: Some(
                        "Read protection level.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "RDPR",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "NRDPR",
                    description: Some(
                        "Integrity complement of RDPR (managed automatically).",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "USER",
                    description: Some(
                        "User configuration — watchdog mode, stop/standby reset, and SRAM/CODE allocation.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "USER",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "NUSER",
                    description: Some(
                        "Integrity complement of USER (managed automatically).",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "DATA0",
                    description: Some(
                        "Free user data byte 0.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "NDATA0",
                    description: Some(
                        "Integrity complement of DATA0 (managed automatically).",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "DATA1",
                    description: Some(
                        "Free user data byte 1.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "NDATA1",
                    description: Some(
                        "Integrity complement of DATA1 (managed automatically).",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "WRPR0",
                    description: Some(
                        "Write protection for main-flash sectors 0-7. 1 bit per 4K sector (1 = writable, 0 = protected).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "NWRPR0",
                    description: Some(
                        "Integrity complement of WRPR0 (managed automatically).",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "WRPR1",
                    description: Some(
                        "Write protection for main-flash sectors 8-15. 1 bit per 4K sector (1 = writable, 0 = protected).",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "NWRPR1",
                    description: Some(
                        "Integrity complement of WRPR1 (managed automatically).",
                    ),
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "WRPR2",
                    description: Some(
                        "Write protection for main-flash sectors 16-23. 1 bit per 4K sector (1 = writable, 0 = protected).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "NWRPR2",
                    description: Some(
                        "Integrity complement of WRPR2 (managed automatically).",
                    ),
                    array: None,
                    byte_offset: 0xd,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "WRPR3",
                    description: Some(
                        "Write protection — bits 0-6 cover main-flash sectors 24-30 (1 bit per 4K sector); bit 7 covers sectors 31-127 as a group. 1 = writable, 0 = protected.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "NWRPR3",
                    description: Some(
                        "Integrity complement of WRPR3 (managed automatically).",
                    ),
                    array: None,
                    byte_offset: 0xf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "RDPR",
            extends: None,
            description: Some(
                "Read protection level.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "RDPR",
                    description: Some(
                        "Set UNPROTECTED (0xA5) to allow debug reads. Any other value blocks the debug interface and auto-write-protects pages 0-15 (4K each).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "ReadProtection",
                    ),
                },
            ],
        },
        FieldSet {
            name: "USER",
            extends: None,
            description: Some(
                "Watchdog mode, low-power reset behavior, and SRAM/CODE memory allocation.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "IWDGSW",
                    description: Some(
                        "How the independent watchdog starts.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "IwdgMode",
                    ),
                },
                Field {
                    name: "STOPRST",
                    description: Some(
                        "Whether entering Stop mode resets the chip.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "PowerModeReset",
                    ),
                },
                Field {
                    name: "STANDYRST",
                    description: Some(
                        "Whether entering Standby mode resets the chip.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "PowerModeReset",
                    ),
                },
                Field {
                    name: "RAM_CODE",
                    description: Some(
                        "SRAM/CODE memory allocation. Patterns 0b00x / 0b01x / 0b10x ignore bit 5; patterns 0b110 and 0b111 are distinct.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "RamCodeMode",
                    ),
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
        Enum {
            name: "RamCodeMode",
            description: Some(
                "SRAM/CODE memory allocation. Patterns 0b00x / 0b01x / 0b10x ignore bit 5; patterns 0b110 and 0b111 are distinct.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "C192_R128",
                    description: Some(
                        "192 KB code + 128 KB RAM.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "C224_R96",
                    description: Some(
                        "224 KB code + 96 KB RAM.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "C256_R64",
                    description: Some(
                        "256 KB code + 64 KB RAM (factory default).",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "C128_R192",
                    description: Some(
                        "128 KB code + 192 KB RAM (only on lots where the 6th-from-last lot-number digit is non-zero).",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "C288_R32",
                    description: Some(
                        "288 KB code + 32 KB RAM.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "ReadProtection",
            description: Some(
                "Whether flash contents are readable through the debug interface.",
            ),
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "UNPROTECTED",
                    description: Some(
                        "Allow the debug interface to read flash.",
                    ),
                    value: 165,
                },
            ],
        },
    ],
};
