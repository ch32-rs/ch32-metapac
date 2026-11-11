use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tkey",
            extends: None,
            description: Some(
                "Touch-key controller.",
            ),
            items: &[
                BlockItem {
                    name: "tkey_ctlr",
                    description: Some(
                        "TKEY control register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TkeyCtlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tkey_trans_cfg",
                    description: Some(
                        "Charge Shift Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TkeyTransCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cap_cfg",
                    description: Some(
                        "Capacitor Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CapCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "drv_cfg",
                    description: Some(
                        "Driver Shield Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DrvCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tkey_cdcc",
                    description: Some(
                        "TKEY Charge-Discharge Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TkeyCdcc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tkey_sr",
                    description: Some(
                        "TKEY Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TkeySr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "CapCfg",
            extends: None,
            description: Some(
                "Capacitor Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "selcx",
                    description: Some(
                        "In charge transfer mode, the channel with an external reference capacitor is configured,.  and in current source charging mode, this bit is invalid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "selcs",
                    description: Some(
                        "In charge transfer mode, a channel for charge transfer is configured.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DrvCfg",
            extends: None,
            description: Some(
                "Driver Shield Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "drven",
                    description: Some(
                        "Drive Shielding Function Enable Position.",
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
                    name: "drvoutsel",
                    description: Some(
                        "Drive shield channel selection, active high.",
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
            name: "TkeyCdcc",
            extends: None,
            description: Some(
                "TKEY Charge-Discharge Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tkcd",
                    description: Some(
                        "In current source charging mode, the discharge time is configured.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tkcc",
                    description: Some(
                        "In the current source charging mode, the charging time is configured.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TkeyCtlr",
            extends: None,
            description: Some(
                "TKEY control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tken",
                    description: Some(
                        "TKEY enabler.",
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
                    name: "itune",
                    description: Some(
                        "Charging current configuration of the TKEY module.",
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
                    name: "mode",
                    description: Some(
                        "TKEY mode configuration.",
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
                    name: "chsel",
                    description: Some(
                        "Channel selection.",
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
                    name: "tkact",
                    description: Some(
                        "TKEY triggers the control.",
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
                    name: "scanie",
                    description: Some(
                        "TKEY Channel Scan Interrupt Enable.",
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
                    name: "scanen",
                    description: Some(
                        "TKEY channel sweep function enable.",
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
                    name: "scannum",
                    description: Some(
                        "Configure the number of TKEY scanning channels.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TkeySr",
            extends: None,
            description: Some(
                "TKEY Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scanif",
                    description: Some(
                        "The TKEY channel scan is interrupted at the end of the mark,.  the software writes 0 clear, and the write 1 is invalid.",
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
            ],
        },
        FieldSet {
            name: "TkeyTransCfg",
            extends: None,
            description: Some(
                "Charge Shift Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "transcc",
                    description: Some(
                        "In charge transfer mode, touch the button capacitor charge charge T2 and charge transfer time T4 configuration bits.",
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
                    name: "transn",
                    description: Some(
                        "The actual number of transfer cycles is TRANSN+1, that is,.  the maximum number of transfer cycles is 1024.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tksw",
                    description: Some(
                        "TKEY switch switching time configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
