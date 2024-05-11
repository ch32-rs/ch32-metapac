
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Exti",
            extends: None,
            description: Some(
                "EXTI.",
            ),
            items: &[
                BlockItem {
                    name: "intenr",
                    description: Some(
                        "Interrupt mask register (EXTI_INTENR).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "evenr",
                    description: Some(
                        "Event mask register (EXTI_EVENR).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Evenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtenr",
                    description: Some(
                        "Rising Trigger selection register (EXTI_RTENR).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rtenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ftenr",
                    description: Some(
                        "Falling Trigger selection register (EXTI_FTENR).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ftenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "swievr",
                    description: Some(
                        "Software interrupt event register (EXTI_SWIEVR).",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Swievr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intfr",
                    description: Some(
                        "Interrupt flag register (EXTI_INTFR).",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intfr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Evenr",
            extends: None,
            description: Some(
                "Event mask register (EXTI_EVENR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mr",
                    description: Some(
                        "Event Mask on line 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 30,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ftenr",
            extends: None,
            description: Some(
                "Falling Trigger selection register (EXTI_FTENR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tr",
                    description: Some(
                        "Falling trigger event configuration of line 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 30,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intenr",
            extends: None,
            description: Some(
                "Interrupt mask register (EXTI_INTENR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mr",
                    description: Some(
                        "Interrupt Mask on line 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 30,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intfr",
            extends: None,
            description: Some(
                "Interrupt flag register (EXTI_INTFR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "if_",
                    description: Some(
                        "Interrupt flag bit 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 30,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rtenr",
            extends: None,
            description: Some(
                "Rising Trigger selection register (EXTI_RTENR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tr",
                    description: Some(
                        "Rising trigger event configuration of line 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 30,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Swievr",
            extends: None,
            description: Some(
                "Software interrupt event register (EXTI_SWIEVR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swie",
                    description: Some(
                        "Software Interrupt on line 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 30,
                                stride: 1,
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
                