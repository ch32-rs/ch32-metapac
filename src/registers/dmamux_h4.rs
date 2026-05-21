use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dmamux",
            extends: None,
            description: Some(
                "DMA request multiplexer. Routes peripheral DMA request lines to DMA1 channels 1-8 (DMAMUX channels 1-8) and DMA2 channels 1-8 (DMAMUX channels 9-16).",
            ),
            items: &[
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "DMA request multiplexer channel configuration register. Each register holds 4 channels (7-bit MUX selector per channel).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "DMAMUX channel configuration. Four 7-bit MUX selectors packed into a 32-bit register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "channel_mux",
                    description: Some(
                        "DMA request multiplexer input source for the channel (peripheral request ID, see RM table for assignments).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
