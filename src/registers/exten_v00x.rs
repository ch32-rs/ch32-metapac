use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Exten",
        extends: None,
        description: Some("Extended configuration."),
        items: &[BlockItem {
            name: "ctr",
            description: Some("extended control register."),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Ctr"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Ctr",
        extends: None,
        description: Some("extended control register."),
        bit_size: 32,
        fields: &[
            Field {
                name: "lkupen",
                description: Some("LOCKUP monitor enable."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "lkuprst",
                description: Some("LOCKUP reset flag."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "tim2_dma_remap",
                description: Some("TIM2 channel 4 DMA remap."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[],
};
