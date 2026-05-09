use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "BleBb",
        extends: None,
        description: Some("BLE Baseband — link-layer CTRL/GO/MODE/CFG, IRQ STATUS."),
        items: &[
            BlockItem {
                name: "ctrl",
                description: Some(
                    "Baseband control + GO strobes (bit23/bit28 enables, bits[8:7] analog gate).",
                ),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ctrl"),
                }),
            },
            BlockItem {
                name: "mode",
                description: Some("Baseband mode register (default 0x00090083)."),
                array: None,
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cfg",
                description: Some(
                    "Baseband CFG (bits[30:25] PHY mode = rf_flag, base 0x80010EC8).",
                ),
                array: None,
                byte_offset: 0x2c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfg"),
                }),
            },
            BlockItem {
                name: "timing",
                description: Some("Baseband timing (default 0x000001D0 = 464)."),
                array: None,
                byte_offset: 0x34,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "statr",
                description: Some("IRQ status register (W1C). Read=live status; write 1 to clear."),
                array: None,
                byte_offset: 0x38,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Statr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cfg",
            extends: None,
            description: Some("BLE_BB CFG — PHY mode + base configuration."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "base",
                    description: Some("Base config bits[24:0] (default 0x010EC8)."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rf_flag",
                    description: Some("PHY mode flag (BB_RF_FLAG_1M = 0x09; bit0=1M PHY enable)."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 25 }),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hw_reserved",
                    description: Some("Hardware reserved bit (always set, base=0x80010EC8)."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some("BLE_BB CTRL — baseband enable + analog gate."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "analog_clear",
                    description: Some(
                        "Analog clear bits[8:7] cleared pre-init, bit7 set post-init.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "analog_gate",
                    description: Some(
                        "Analog gate (BLE_RegInit pre-init clears bit13 sets bit12).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bb_en",
                    description: Some("Baseband enable (permanent, set in bb_dev_init)."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hw_en",
                    description: Some("Hardware enable (permanent, set in bb_dev_init)."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Statr",
            extends: None,
            description: Some("BLE_BB IRQ status (W1C — write 1 to clear)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bit4",
                    description: Some(
                        "IRQ status bit4 — `.L4` cleanup; sets gBleIPPara[4]=1 (re-arm).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bit5",
                    description: Some(
                        "IRQ status bit5 — paired with bit6 in W1C mask 0x60 (PLL-related).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pll_ready",
                    description: Some(
                        "PLL-ready (triggers `.L6` TX advance path; W1C with bit5 via 0x60).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bit7",
                    description: Some(
                        "IRQ status bit7 — `.L8` cleanup; sets gBleIPPara[4]=1 (re-arm).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
