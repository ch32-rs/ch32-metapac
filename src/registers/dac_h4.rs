use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dac",
            extends: None,
            description: Some(
                "Digital to analog converter.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr",
                    description: Some(
                        "Control register (DAC_CR).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "swtr",
                    description: Some(
                        "DAC software trigger register (DAC_SWTRIGR).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Swtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "r12bdhr1",
                    description: Some(
                        "DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "R12bdhr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l12bdhr1",
                    description: Some(
                        "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L12bdhr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "r8bdhr1",
                    description: Some(
                        "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1).",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "R8bdhr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "r12bdhr2",
                    description: Some(
                        "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2).",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "R12bdhr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l12bdhr2",
                    description: Some(
                        "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2).",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L12bdhr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "r8bdhr2",
                    description: Some(
                        "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2).",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "R8bdhr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rd12bdhr",
                    description: Some(
                        "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rd12bdhr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ld12bdhr",
                    description: Some(
                        "DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ld12bdhr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rd8bdhr",
                    description: Some(
                        "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rd8bdhr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dor1",
                    description: Some(
                        "DAC channel1 data output register (DAC_DOR1).",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dor1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dor2",
                    description: Some(
                        "DAC channel2 data output register (DAC_DOR2).",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dor2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctlr",
            extends: None,
            description: Some(
                "Control register (DAC_CR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en1",
                    description: Some(
                        "DAC channel1 enable.",
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
                    name: "boff1",
                    description: Some(
                        "DAC channel1 output buffer disable.",
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
                    name: "ten1",
                    description: Some(
                        "DAC channel1 trigger enable.",
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
                    name: "tsel1",
                    description: Some(
                        "DAC channel1 trigger selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wave1",
                    description: Some(
                        "DAC channel1 noise/triangle wave generation enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mamp1",
                    description: Some(
                        "DAC channel1 mask/amplitude selector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaen1",
                    description: Some(
                        "DAC channel1 DMA enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "en2",
                    description: Some(
                        "DAC channel2 enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boff2",
                    description: Some(
                        "DAC channel2 output buffer disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ten2",
                    description: Some(
                        "DAC channel2 trigger enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsel2",
                    description: Some(
                        "DAC channel2 trigger selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wave2",
                    description: Some(
                        "DAC channel2 noise/triangle wave generation enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mamp2",
                    description: Some(
                        "DAC channel2 mask/amplitude selector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaen2",
                    description: Some(
                        "DAC channel2 DMA enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dor1",
            extends: None,
            description: Some(
                "DAC channel1 data output register (DAC_DOR1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc1dor",
                    description: Some(
                        "DAC channel1 data output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dor2",
            extends: None,
            description: Some(
                "DAC channel2 data output register (DAC_DOR2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc2dor",
                    description: Some(
                        "DAC channel2 data output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L12bdhr1",
            extends: None,
            description: Some(
                "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc1dhr",
                    description: Some(
                        "DAC channel1 12-bit left-aligned data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L12bdhr2",
            extends: None,
            description: Some(
                "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc2dhr",
                    description: Some(
                        "DAC channel2 12-bit left-aligned data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ld12bdhr",
            extends: None,
            description: Some(
                "DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc1dhr",
                    description: Some(
                        "DAC channel1 12-bit left-aligned data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dacc2dhr",
                    description: Some(
                        "DAC channel2 12-bit right-aligned data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "R12bdhr1",
            extends: None,
            description: Some(
                "DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc1dhr",
                    description: Some(
                        "DAC channel1 12-bit right-aligned data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "R12bdhr2",
            extends: None,
            description: Some(
                "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc2dhr",
                    description: Some(
                        "DAC channel2 12-bit right-aligned data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "R8bdhr1",
            extends: None,
            description: Some(
                "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc1dhr",
                    description: Some(
                        "DAC channel1 8-bit right-aligned data.",
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
            name: "R8bdhr2",
            extends: None,
            description: Some(
                "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc2dhr",
                    description: Some(
                        "DAC channel2 8-bit right-aligned data.",
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
            name: "Rd12bdhr",
            extends: None,
            description: Some(
                "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc1dhr",
                    description: Some(
                        "DAC channel1 12-bit right-aligned data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dacc2dhr",
                    description: Some(
                        "DAC channel2 12-bit right-aligned data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rd8bdhr",
            extends: None,
            description: Some(
                "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dacc1dhr",
                    description: Some(
                        "DAC channel1 8-bit right-aligned data.",
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
                Field {
                    name: "dacc2dhr",
                    description: Some(
                        "DAC channel2 8-bit right-aligned data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Swtr",
            extends: None,
            description: Some(
                "DAC software trigger register (DAC_SWTRIGR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swtrig1",
                    description: Some(
                        "DAC channel1 software trigger.",
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
                    name: "swtrig2",
                    description: Some(
                        "DAC channel2 software trigger.",
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
            ],
        },
    ],
    enums: &[],
};
