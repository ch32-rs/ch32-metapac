use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Tkey",
        extends: None,
        description: Some(
            "Touch key. Aliases ADC IDATAR1/RDATAR when ADC TKENABLE=1; enable via ADC.",
        ),
        items: &[
            BlockItem {
                name: "tkey_chg",
                description: Some("charge time configuration register."),
                array: None,
                byte_offset: 0x3c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("TkeyChg"),
                }),
            },
            BlockItem {
                name: "tkey_dischg",
                description: Some("start and discharge time register."),
                array: None,
                byte_offset: 0x4c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("TkeyDischg"),
                }),
            },
            BlockItem {
                name: "tkey_dr",
                description: Some("data register."),
                array: None,
                byte_offset: 0x4c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("TkeyDr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "TkeyChg",
            extends: None,
            description: Some("charge time configuration register."),
            bit_size: 32,
            fields: &[Field {
                name: "tkcharge",
                description: Some("Touch key charge time."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 11,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "TkeyDischg",
            extends: None,
            description: Some("start and discharge time register."),
            bit_size: 32,
            fields: &[Field {
                name: "tkact_dcg",
                description: Some("Touch key start and discharge time."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 11,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "TkeyDr",
            extends: None,
            description: Some("data register."),
            bit_size: 32,
            fields: &[Field {
                name: "data",
                description: Some("Converted data."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
