use crate::metadata::ir::*;
pub(crate) static DESCRIPTOR: IR = IR {
    blocks: &[
        Block {
            name: "OB",
            extends: None,
            description: Some(
                "Persistent flash configuration — read protection, watchdog mode, low-power reset behavior, write protection, and two free user-data bytes.",
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
                        "User configuration — watchdog mode and stop/standby reset behavior.",
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
                "User configuration byte.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "STOPRST",
                    description: Some(
                        "Stop low-power reset disable.",
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
                    name: "STANDYRST",
                    description: Some(
                        "Standby low-power reset disable.",
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
                    name: "RST_MODE",
                    description: Some(
                        "Reset pin mode.",
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
                    name: "RST_PIN_SEL",
                    description: Some(
                        "Reset pin select (1 = PA15, 0 = PC0).",
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
                    name: "OV_CFG",
                    description: Some(
                        "Overvoltage protection source (1 = PWR CTLR).",
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
    ],
    enums: &[
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
