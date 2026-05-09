use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "BleAes",
        extends: None,
        description: Some("BLE AES engine — only +0x04 cleanup-bit pair decoded for ADV TX path."),
        items: &[BlockItem {
            name: "statr",
            description: Some(
                "AES op status (bit0 + bit1; cleared sequentially from the BB IRQ path).",
            ),
            array: None,
            byte_offset: 0x4,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Statr"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Statr",
        extends: None,
        description: Some("BLE_AES status — phase 1 / phase 2 cleanup bits."),
        bit_size: 32,
        fields: &[
            Field {
                name: "phase2",
                description: Some("AES op cleanup phase 2 (cleared after PHASE1)."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "phase1",
                description: Some("AES op cleanup phase 1 (cleared first if set)."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[],
};
