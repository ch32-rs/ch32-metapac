use crate::metadata::ir::*;
pub(crate) static DESCRIPTOR: IR = IR {
    blocks: &[
        Block {
            name: "OB",
            extends: None,
            description: Some(
                "Persistent flash configuration — read protection, watchdog/boot mode, write protection, and two free user-data bytes.",
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
                        "User configuration — boot mode, reset and download options.",
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
                        "Write protection for main-flash sectors 24-31. 1 bit per 4K sector (1 = writable, 0 = protected).",
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
            bit_size: 32,
            fields: &[
                Field {
                    name: "RDPR",
                    description: Some(
                        "0xA5 (with NRDPR 0x5A) disables read protection.",
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
            name: "USER",
            extends: None,
            description: Some(
                "User configuration byte.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "IWDGSW",
                    description: Some(
                        "IWDG software enable (1 = software start only).",
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
                    name: "CFGRSTEN",
                    description: Some(
                        "External reset pin enable (1 = disabled).",
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
                    name: "START_MD",
                    description: Some(
                        "Boot mode (1 = boot from the system flash when BOOT is present).",
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
                    name: "USBHSDLEN",
                    description: Some(
                        "Enable USBHS download in the bootloader.",
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
                    name: "USARTDLEN",
                    description: Some(
                        "Enable USART keyless download in the bootloader.",
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
    enums: &[],
};
