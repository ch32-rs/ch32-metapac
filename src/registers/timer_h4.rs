use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adtm",
            extends: None,
            description: Some(
                "Advanced timer.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcfgr",
                    description: Some(
                        "slave mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaintenr",
                    description: Some(
                        "DMA/Interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaintenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intfr",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                BlockItem {
                    name: "swevgr",
                    description: Some(
                        "event generation register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Swevgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctlr_input",
                    description: Some(
                        "capture/compare mode register 1 (input mode).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChctlrInput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctlr_output",
                    description: Some(
                        "capture/compare mode register (output mode).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChctlrOutput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccer",
                    description: Some(
                        "capture/compare enable register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "atrlr",
                    description: Some(
                        "auto-reload register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rptcr",
                    description: Some(
                        "repetition counter register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rptcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chcvr",
                    description: Some(
                        "capture/compare register 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "bdtr",
                    description: Some(
                        "break and dead-time register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacfgr",
                    description: Some(
                        "DMA control register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaadr",
                    description: Some(
                        "DMA address for full transfer.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaadr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aux",
                    description: Some(
                        "Auxiliary register (dead-time / brake select / dual-edge capture).",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Aux",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Bctm",
            extends: None,
            description: Some(
                "Basic timer.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaintenr",
                    description: Some(
                        "DMA/Interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaintenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intfr",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                BlockItem {
                    name: "swevgr",
                    description: Some(
                        "event generation register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Swevgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "atrlr",
                    description: Some(
                        "auto-reload register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Gptm",
            extends: None,
            description: Some(
                "General purpose timer, 16bit.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcfgr",
                    description: Some(
                        "slave mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaintenr",
                    description: Some(
                        "DMA/Interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaintenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intfr",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                BlockItem {
                    name: "swevgr",
                    description: Some(
                        "event generation register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Swevgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctlr_input",
                    description: Some(
                        "capture/compare mode register 1 (input mode).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChctlrInput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctlr_output",
                    description: Some(
                        "capture/compare mode register 1 (output mode).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChctlrOutput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccer",
                    description: Some(
                        "capture/compare enable register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "atrlr",
                    description: Some(
                        "auto-reload register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "chcvr",
                    description: Some(
                        "capture/compare register 1.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dmacfgr",
                    description: Some(
                        "DMA control register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaadr",
                    description: Some(
                        "DMA address for full transfer.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaadr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Gptm32",
            extends: None,
            description: Some(
                "General purpose timer, 32bit.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcfgr",
                    description: Some(
                        "slave mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaintenr",
                    description: Some(
                        "DMA/Interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaintenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intfr",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                BlockItem {
                    name: "swevgr",
                    description: Some(
                        "event generation register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Swevgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctlr_input",
                    description: Some(
                        "capture/compare mode register 1 (input mode).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChctlrInput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctlr_output",
                    description: Some(
                        "capture/compare mode register 1 (output mode).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChctlrOutput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccer",
                    description: Some(
                        "capture/compare enable register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter, 32bit.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "atrlr",
                    description: Some(
                        "auto-reload register, 32bit.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "chcvr",
                    description: Some(
                        "capture/compare register, 32bit.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dmacfgr",
                    description: Some(
                        "DMA control register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaadr",
                    description: Some(
                        "DMA address for full transfer.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaadr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Aux",
            extends: None,
            description: Some(
                "Double-side edge capture register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "cap_ed_ch2",
                    description: Some(
                        "Double-side edge capture is enable for channel2.",
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
                    name: "cap_ed_ch3",
                    description: Some(
                        "Double-side edge capture is enable for channel3.",
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
                    name: "cap_ed_ch4",
                    description: Some(
                        "Double-side edge capture is enable for channel4.",
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
                    name: "bk_sel",
                    description: Some(
                        "Break source selection.",
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
                    name: "dtp_mode",
                    description: Some(
                        "The dead-time duration of the DT_VLU2 enabled setting occurs.  on the rising edge of OCXREF.",
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
                    name: "dtn_mode",
                    description: Some(
                        "The dead-time duration of the DT_VLU2 enabled setting occurs.  on the falling edge of OCXREF.",
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
                Field {
                    name: "dt_vlu2",
                    description: Some(
                        "set the duration of the dead zone.",
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
            name: "Bdtr",
            extends: None,
            description: Some(
                "break and dead-time register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtg",
                    description: Some(
                        "Dead-time generator setup.",
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
                    name: "lock",
                    description: Some(
                        "Lock configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ossi",
                    description: Some(
                        "Off-state selection for Idle mode.",
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
                    name: "ossr",
                    description: Some(
                        "Off-state selection for Run mode.",
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
                    name: "bke",
                    description: Some(
                        "Break enable.",
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
                    name: "bkp",
                    description: Some(
                        "Break polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoe",
                    description: Some(
                        "Automatic output enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "moe",
                    description: Some(
                        "Main output enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccer",
            extends: None,
            description: Some(
                "capture/compare enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cce",
                    description: Some(
                        "Capture/Compare 1 output enable.",
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
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccp",
                    description: Some(
                        "Capture/Compare 1 output Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccne",
                    description: Some(
                        "Capture/Compare 1 complementary output enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ccnp",
                    description: Some(
                        "Capture/Compare 1 output Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ChctlrInput",
            extends: None,
            description: Some(
                "capture/compare mode register 1 (input mode).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccs",
                    description: Some(
                        "Capture/Compare 1 selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "CcmrInputCcs",
                    ),
                },
                Field {
                    name: "icpsc",
                    description: Some(
                        "Input capture 1 prescaler.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "icf",
                    description: Some(
                        "Input capture 1 filter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "FilterValue",
                    ),
                },
            ],
        },
        FieldSet {
            name: "ChctlrOutput",
            extends: None,
            description: Some(
                "capture/compare mode register (output mode).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccs",
                    description: Some(
                        "Capture/Compare 1 selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "CcmrOutputCcs",
                    ),
                },
                Field {
                    name: "iocfe",
                    description: Some(
                        "Output Compare 1 fast enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocpe",
                    description: Some(
                        "Output Compare 1 preload enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocm",
                    description: Some(
                        "Output Compare 1 mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Ocm",
                    ),
                },
                Field {
                    name: "occe",
                    description: Some(
                        "Output Compare 1 clear enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlr1",
            extends: None,
            description: Some(
                "control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cen",
                    description: Some(
                        "Counter enable.",
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
                    name: "udis",
                    description: Some(
                        "Update disable.",
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
                    name: "urs",
                    description: Some(
                        "Update request source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Urs",
                    ),
                },
                Field {
                    name: "opm",
                    description: Some(
                        "One-pulse mode.",
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
                    name: "dir",
                    description: Some(
                        "Direction.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "cms",
                    description: Some(
                        "Center-aligned mode selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Cms",
                    ),
                },
                Field {
                    name: "arpe",
                    description: Some(
                        "Auto-reload preload enable.",
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
                Field {
                    name: "ckd",
                    description: Some(
                        "Clock division.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ckd",
                    ),
                },
                Field {
                    name: "capov",
                    description: Some(
                        "Timer capture value configuration enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "caplvl",
                    description: Some(
                        "Timer capture level indication enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlr2",
            extends: None,
            description: Some(
                "control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccpc",
                    description: Some(
                        "Capture/compare preloaded control.",
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
                    name: "ccus",
                    description: Some(
                        "Capture/compare control update selection.",
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
                    name: "ccds",
                    description: Some(
                        "Capture/compare DMA selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ccds",
                    ),
                },
                Field {
                    name: "mms",
                    description: Some(
                        "Master mode selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mms",
                    ),
                },
                Field {
                    name: "ti1s",
                    description: Some(
                        "TI1 selection.",
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
                Field {
                    name: "ois",
                    description: Some(
                        "Output Idle state 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "oisn",
                    description: Some(
                        "Output Idle state 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmaadr",
            extends: None,
            description: Some(
                "DMA address for full transfer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaadr",
                    description: Some(
                        "DMA register for burst accesses.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmacfgr",
            extends: None,
            description: Some(
                "DMA control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dba",
                    description: Some(
                        "DMA base address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbl",
                    description: Some(
                        "DMA burst length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmaintenr",
            extends: None,
            description: Some(
                "DMA/Interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uie",
                    description: Some(
                        "Update interrupt enable.",
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
                    name: "ccie",
                    description: Some(
                        "Capture/Compare 1 interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comie",
                    description: Some(
                        "COM interrupt enable.",
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
                    name: "tie",
                    description: Some(
                        "Trigger interrupt enable.",
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
                    name: "bie",
                    description: Some(
                        "Break interrupt enable.",
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
                Field {
                    name: "ude",
                    description: Some(
                        "Update DMA request enable.",
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
                    name: "ccde",
                    description: Some(
                        "Capture/Compare 1 DMA request enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comde",
                    description: Some(
                        "COM DMA request enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tde",
                    description: Some(
                        "Trigger DMA request enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intfr",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uif",
                    description: Some(
                        "Update interrupt flag.",
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
                    name: "ccif",
                    description: Some(
                        "Capture/compare 1 interrupt flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comif",
                    description: Some(
                        "COM interrupt flag.",
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
                    name: "tif",
                    description: Some(
                        "Trigger interrupt flag.",
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
                    name: "bif",
                    description: Some(
                        "Break interrupt flag.",
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
                Field {
                    name: "ccof",
                    description: Some(
                        "Capture/Compare 1 overcapture flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rptcr",
            extends: None,
            description: Some(
                "repetition counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rptcr",
                    description: Some(
                        "Repetition counter value.",
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
            name: "Smcfgr",
            extends: None,
            description: Some(
                "slave mode control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sms",
                    description: Some(
                        "Slave mode selection.",
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
                    name: "ts",
                    description: Some(
                        "Trigger selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msm",
                    description: Some(
                        "Master/Slave mode.",
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
                Field {
                    name: "etf",
                    description: Some(
                        "External trigger filter.",
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
                    name: "etps",
                    description: Some(
                        "External trigger prescaler.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Etps",
                    ),
                },
                Field {
                    name: "ece",
                    description: Some(
                        "External clock enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "etp",
                    description: Some(
                        "External trigger polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Swevgr",
            extends: None,
            description: Some(
                "event generation register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ug",
                    description: Some(
                        "Update generation.",
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
                    name: "ccg",
                    description: Some(
                        "Capture/compare 1 generation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comg",
                    description: Some(
                        "Capture/Compare control update generation.",
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
                    name: "tg",
                    description: Some(
                        "Trigger generation.",
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
                    name: "bg",
                    description: Some(
                        "Break generation.",
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
            name: "Ccds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ONCOMPARE",
                    description: Some(
                        "CCx DMA request sent when CCx event occurs",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ONUPDATE",
                    description: Some(
                        "CCx DMA request sent when update event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CcmrInputCcs",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "TI4",
                    description: Some(
                        "CCx channel is configured as input, normal mapping: ICx mapped to TIx",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TI3",
                    description: Some(
                        "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TRC",
                    description: Some(
                        "CCx channel is configured as input, ICx is mapped on TRC",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "CcmrOutputCcs",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "OUTPUT",
                    description: Some(
                        "CCx channel is configured as output",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ckd",
            description: Some(
                "division ratio between the timer clock (CK_INT) frequency, the dead time and the sampling clock used by the dead time generator and the digital filter (ETR,TIx)",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV_1",
                    description: Some(
                        "Tdts=Tck_int",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV_2",
                    description: Some(
                        "Tdts=2*Tck_int",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV_4",
                    description: Some(
                        "Tdts=4*Tck_int",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Cms",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "EDGEALIGNED",
                    description: Some(
                        "The counter counts up or down depending on the direction bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CENTERALIGNED1",
                    description: Some(
                        "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CENTERALIGNED2",
                    description: Some(
                        "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CENTERALIGNED3",
                    description: Some(
                        "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UP",
                    description: Some(
                        "Counter used as upcounter",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DOWN",
                    description: Some(
                        "Counter used as downcounter",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Etps",
            description: Some(
                "External trigger prescaler.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV_1",
                    description: Some(
                        "ETRP frequency divided by 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV_2",
                    description: Some(
                        "ETRP frequency divided by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV_4",
                    description: Some(
                        "ETRP frequency divided by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV_8",
                    description: Some(
                        "ETRP frequency divided by 8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "FilterValue",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NOFILTER",
                    description: Some(
                        "No filter, sampling is done at fDTS",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FCK_INT_N2",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FCK_INT_N4",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FCK_INT_N8",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=8",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FDTS_DIV2_N6",
                    description: Some(
                        "fSAMPLING=fDTS/2, N=6",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FDTS_DIV2_N8",
                    description: Some(
                        "fSAMPLING=fDTS/2, N=8",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "FDTS_DIV4_N6",
                    description: Some(
                        "fSAMPLING=fDTS/4, N=6",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "FDTS_DIV4_N8",
                    description: Some(
                        "fSAMPLING=fDTS/4, N=8",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "FDTS_DIV8_N6",
                    description: Some(
                        "fSAMPLING=fDTS/8, N=6",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "FDTS_DIV8_N8",
                    description: Some(
                        "fSAMPLING=fDTS/8, N=8",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N5",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=5",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N6",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=6",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N8",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=8",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N5",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=5",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N6",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=6",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N8",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=8",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Mms",
            description: Some(
                "Master mode selection.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLE",
                    description: Some(
                        "Enable",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "UPDATE",
                    description: Some(
                        "Update",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "COMPARE_PULSE",
                    description: Some(
                        "Compare Pulse",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "COMPARE_OC1REF",
                    description: Some(
                        "Compare OC1REF",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "COMPARE_OC2REF",
                    description: Some(
                        "Compare OC2REF",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "COMPARE_OC3REF",
                    description: Some(
                        "Compare OC3REF",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "COMPARE_OC4REF",
                    description: Some(
                        "Compare OC4REF",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Ocm",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "FROZEN",
                    description: Some(
                        "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVEONMATCH",
                    description: Some(
                        "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "INACTIVEONMATCH",
                    description: Some(
                        "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TOGGLE",
                    description: Some(
                        "OCyREF toggles when TIMx_CNT=TIMx_CCRy",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FORCEINACTIVE",
                    description: Some(
                        "OCyREF is forced low",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FORCEACTIVE",
                    description: Some(
                        "OCyREF is forced high",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "PWMMODE1",
                    description: Some(
                        "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "PWMMODE2",
                    description: Some(
                        "Inversely to PwmMode1",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Urs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANYEVENT",
                    description: Some(
                        "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "COUNTERONLY",
                    description: Some(
                        "Only counter overflow/underflow generates an update interrupt or DMA request",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
