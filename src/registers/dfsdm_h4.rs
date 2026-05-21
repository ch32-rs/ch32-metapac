use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dfsdm",
            extends: None,
            description: Some(
                "Digital filter for sigma delta modulators.",
            ),
            items: &[
                BlockItem {
                    name: "ch0cfgr1",
                    description: Some(
                        "channel configuration 0 register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1cfgr1",
                    description: Some(
                        "channel configuration 1 register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0cfgr2",
                    description: Some(
                        "channel configuration 0 register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1cfgr2",
                    description: Some(
                        "channel configuration 1 register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0awscdr",
                    description: Some(
                        "analog watchdog and short-circuit detector register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0awscdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1awscdr",
                    description: Some(
                        "analog watchdog and short-circuit detector register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1awscdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0wdatr",
                    description: Some(
                        "channel watchdog filter data register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0wdatr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1wdatr",
                    description: Some(
                        "channel watchdog filter data register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1wdatr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0datinr",
                    description: Some(
                        "channel data input register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0datinr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1datinr",
                    description: Some(
                        "channel data input register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1datinr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0cr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1cr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0cr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1cr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0isr",
                    description: Some(
                        "interrupt and status register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1isr",
                    description: Some(
                        "interrupt and status register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0icr",
                    description: Some(
                        "interrupt flag clear register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1icr",
                    description: Some(
                        "interrupt flag clear register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0jchgr",
                    description: Some(
                        "injected channel group selection register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0jchgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1jchgr",
                    description: Some(
                        "injected channel group selection register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1jchgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0fcr3",
                    description: Some(
                        "control register 3.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0fcr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1fcr3",
                    description: Some(
                        "control register 3.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1fcr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0jdatar",
                    description: Some(
                        "data register for injected group.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0jdatar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1jdatar",
                    description: Some(
                        "data register for injected group.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1jdatar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0rdatar",
                    description: Some(
                        "data register for the regular channel.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0rdatar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1rdatar",
                    description: Some(
                        "data register for the regular channel.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1rdatar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0awhtr",
                    description: Some(
                        "analog watchdog high threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0awhtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1awhtr",
                    description: Some(
                        "analog watchdog high threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1awhtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0awltr",
                    description: Some(
                        "analog watchdog low threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0awltr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1awltr",
                    description: Some(
                        "analog watchdog low threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1awltr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0awsr",
                    description: Some(
                        "analog watchdog status register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0awsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1awsr",
                    description: Some(
                        "analog watchdog status register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1awsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0awcfr",
                    description: Some(
                        "analog watchdog clear flag register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0awcfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1awcfr",
                    description: Some(
                        "analog watchdog clear flag register.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1awcfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0exmax",
                    description: Some(
                        "Extremes detector maximum register.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0exmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1exmax",
                    description: Some(
                        "Extremes detector maximum register.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1exmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0exmin",
                    description: Some(
                        "Extremes detector minimum register.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0exmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1exmin",
                    description: Some(
                        "Extremes detector minimum register.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1exmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt0cnvtimr",
                    description: Some(
                        "conversion timer register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt0cnvtimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfsdm_flt1cnvtimr",
                    description: Some(
                        "conversion timer register.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DfsdmFlt1cnvtimr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ch0awscdr",
            extends: None,
            description: Some(
                "analog watchdog and short-circuit detector register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scdt",
                    description: Some(
                        "Short Circuit Detector Threshold for Channel 0.",
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
                    name: "bkscd",
                    description: Some(
                        "Channel 0 Short Circuit Detector Open Signal Distribution.",
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
                Field {
                    name: "awfosr",
                    description: Some(
                        "Channel 0 analog watchdog filter oversampling rate.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awford",
                    description: Some(
                        "Channel 0 analog watchdog Sinc filter order.",
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
            ],
        },
        FieldSet {
            name: "Ch0cfgr1",
            extends: None,
            description: Some(
                "channel configuration 0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sitp",
                    description: Some(
                        "Channel 0 Serial Interface Type.",
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
                    name: "spicksel",
                    description: Some(
                        "Channel 0 SPI Clock Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scden",
                    description: Some(
                        "Channel 0 Short Circuit Detector Enables.",
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
                    name: "ckaben",
                    description: Some(
                        "Channel 0 Clock Missing Detector Enables.",
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
                    name: "chen",
                    description: Some(
                        "Channel 0 enables.",
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
                    name: "chinsel",
                    description: Some(
                        "Channel Input Selection.",
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
                    name: "datmpx",
                    description: Some(
                        "Channel 0 Input Data Multiplexer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datpack",
                    description: Some(
                        "R32_DFSDM_CHyDATINR register data encapsulation mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckoutdiv",
                    description: Some(
                        "Output Serial Clock Divider.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckoutsrc",
                    description: Some(
                        "Output Serial Clock Source Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfsdmen",
                    description: Some(
                        "The DFSDM interface is globally enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch0cfgr2",
            extends: None,
            description: Some(
                "channel configuration 0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtrbs",
                    description: Some(
                        "Channel 0 data right shift.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset",
                    description: Some(
                        "Channel 0 24-bit calibration offset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch0datinr",
            extends: None,
            description: Some(
                "channel data input register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indat0",
                    description: Some(
                        "Channel Y Input Data.",
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
                Field {
                    name: "indat1",
                    description: Some(
                        "Input data for channel y or channel y+1.",
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
            name: "Ch0wdatr",
            extends: None,
            description: Some(
                "channel watchdog filter data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdata",
                    description: Some(
                        "Enter channel 0 watchdog data.",
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
            name: "Ch1awscdr",
            extends: None,
            description: Some(
                "analog watchdog and short-circuit detector register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scdt",
                    description: Some(
                        "Short Circuit Detector Threshold for Channel 1.",
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
                    name: "bkscd",
                    description: Some(
                        "Channel 1 Short Circuit Detector Open Signal Distribution.",
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
                Field {
                    name: "awfosr",
                    description: Some(
                        "Channel 1 analog watchdog filter oversampling rate.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awford",
                    description: Some(
                        "Channel 1 analog watchdog Sinc filter order.",
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
            ],
        },
        FieldSet {
            name: "Ch1cfgr1",
            extends: None,
            description: Some(
                "channel configuration 1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sitp",
                    description: Some(
                        "Channel 1 Serial Interface Type.",
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
                    name: "spicksel",
                    description: Some(
                        "Channel 1 SPI Clock Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scden",
                    description: Some(
                        "Channel 1 Short Circuit Detector Enables.",
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
                    name: "ckaben",
                    description: Some(
                        "Channel 1 Clock Missing Detector Enables.",
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
                    name: "chen",
                    description: Some(
                        "Channel 1 enables.",
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
                    name: "chinsel",
                    description: Some(
                        "Channel Input Selection.",
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
                    name: "datmpx",
                    description: Some(
                        "Channel 1 Input Data Multiplexer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datpack",
                    description: Some(
                        "R32_DFSDM_CHyDATINR register data encapsulation mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckoutdiv",
                    description: Some(
                        "Output Serial Clock Divider.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckoutsrc",
                    description: Some(
                        "Output Serial Clock Source Selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfsdmen",
                    description: Some(
                        "The DFSDM interface is globally enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1cfgr2",
            extends: None,
            description: Some(
                "channel configuration 1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtrbs",
                    description: Some(
                        "Channel 1 data right shift.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset",
                    description: Some(
                        "Channel 1 24-bit calibration offset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1datinr",
            extends: None,
            description: Some(
                "channel data input register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indat0",
                    description: Some(
                        "Channel 1 Input Data.",
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
                Field {
                    name: "indat1",
                    description: Some(
                        "Input data for channel 1 or channel 2.",
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
            name: "Ch1wdatr",
            extends: None,
            description: Some(
                "channel watchdog filter data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdata",
                    description: Some(
                        "Enter channel 1 watchdog data.",
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
            name: "DfsdmFlt0awcfr",
            extends: None,
            description: Some(
                "analog watchdog clear flag register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clrawltf",
                    description: Some(
                        "Clear the analog watchdog low threshold flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrawhtf",
                    description: Some(
                        "Clear the analog watchdog high threshold flag.",
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
            ],
        },
        FieldSet {
            name: "DfsdmFlt0awhtr",
            extends: None,
            description: Some(
                "analog watchdog high threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkawh",
                    description: Some(
                        "Break signal assignment to analog watchdog high threshold event.",
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
                    name: "awht",
                    description: Some(
                        "Analog watchdog high threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0awltr",
            extends: None,
            description: Some(
                "analog watchdog low threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkawl",
                    description: Some(
                        "Break signal assignment to analog watchdog low threshold event.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awlt",
                    description: Some(
                        "Analog watchdog low threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0awsr",
            extends: None,
            description: Some(
                "analog watchdog status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awltf",
                    description: Some(
                        "Analog watchdog low threshold flag.",
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
                    name: "awhtf",
                    description: Some(
                        "Analog watchdog high threshold flag.",
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
            name: "DfsdmFlt0cnvtimr",
            extends: None,
            description: Some(
                "conversion timer register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnvcnt",
                    description: Some(
                        "28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0cr1",
            extends: None,
            description: Some(
                "control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfen",
                    description: Some(
                        "DFSDM enable.",
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
                    name: "jswstart",
                    description: Some(
                        "Start a conversion of the injected group of channels.",
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
                    name: "jsync",
                    description: Some(
                        "Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger.",
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
                    name: "jscan",
                    description: Some(
                        "Scanning conversion mode for injected conversions.",
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
                    name: "jdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the injected channel group.",
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
                    name: "jextsel",
                    description: Some(
                        "Trigger signal selection for launching injected conversions.",
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
                    name: "jexten",
                    description: Some(
                        "Trigger enable and trigger edge selection for injected conversions.",
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
                Field {
                    name: "rswstart",
                    description: Some(
                        "Software start of a conversion on the regular channel.",
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
                    name: "rcont",
                    description: Some(
                        "Continuous mode selection for regular conversions.",
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
                    name: "rsync",
                    description: Some(
                        "Launch regular conversion synchronously with DFSDM0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the regular conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rch",
                    description: Some(
                        "Regular channel selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fast",
                    description: Some(
                        "Fast conversion mode selection for regular conversions.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awfsel",
                    description: Some(
                        "Analog watchdog fast mode select.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0cr2",
            extends: None,
            description: Some(
                "control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jeocie",
                    description: Some(
                        "Injected end of conversion interrupt enable.",
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
                    name: "reocie",
                    description: Some(
                        "Regular end of conversion interrupt enable.",
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
                    name: "jovrie",
                    description: Some(
                        "Injected data overrun interrupt enable.",
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
                    name: "rovrie",
                    description: Some(
                        "Regular data overrun interrupt enable.",
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
                    name: "awdie",
                    description: Some(
                        "Analog watchdog interrupt enable.",
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
                    name: "scdie",
                    description: Some(
                        "Short-circuit detector interrupt enable.",
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
                    name: "ckabie",
                    description: Some(
                        "Clock absence interrupt enable.",
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
                    name: "exch",
                    description: Some(
                        "Extremes detector channel selection.",
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
                Field {
                    name: "awdch",
                    description: Some(
                        "Analog watchdog channel selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0exmax",
            extends: None,
            description: Some(
                "Extremes detector maximum register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exmaxch",
                    description: Some(
                        "Extremes detector maximum data channel.",
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
                    name: "exmax",
                    description: Some(
                        "Extremes detector maximum value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0exmin",
            extends: None,
            description: Some(
                "Extremes detector minimum register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exminch",
                    description: Some(
                        "Extremes detector minimum data channel.",
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
                    name: "exmin",
                    description: Some(
                        "EXMIN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0fcr3",
            extends: None,
            description: Some(
                "control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iosr",
                    description: Some(
                        "The integrator oversampling rate is 2 to the power of IOSR.",
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
                    name: "fosr",
                    description: Some(
                        "Sinc filter oversampling rate.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ford",
                    description: Some(
                        "Sinc Filter Order.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0icr",
            extends: None,
            description: Some(
                "interrupt flag clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clrjovrf",
                    description: Some(
                        "Clear the injected conversion overrun flag.",
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
                    name: "clrrovrf",
                    description: Some(
                        "Clear the regular conversion overrun flag.",
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
                    name: "clrckabf",
                    description: Some(
                        "Clear the clock absence flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrscdf",
                    description: Some(
                        "Clear the short-circuit detector flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0isr",
            extends: None,
            description: Some(
                "interrupt and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jeocf",
                    description: Some(
                        "End of injected conversion flag.",
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
                    name: "reocf",
                    description: Some(
                        "End of regular conversion flag.",
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
                    name: "jovrf",
                    description: Some(
                        "Injected conversion overrun flag.",
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
                    name: "rovrf",
                    description: Some(
                        "Regular conversion overrun flag.",
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
                    name: "awdf",
                    description: Some(
                        "Analog watchdog.",
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
                    name: "jcip",
                    description: Some(
                        "Injected conversion in progress status.",
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
                    name: "rcip",
                    description: Some(
                        "Regular conversion in progress status.",
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
                    name: "ckabf",
                    description: Some(
                        "Clock absence flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scdf",
                    description: Some(
                        "short-circuit detector flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0jchgr",
            extends: None,
            description: Some(
                "injected channel group selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jchg",
                    description: Some(
                        "Injected channel group selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0jdatar",
            extends: None,
            description: Some(
                "data register for injected group.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdatach",
                    description: Some(
                        "Injected channel most recently converted.",
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
                    name: "jdata",
                    description: Some(
                        "Injected group conversion data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt0rdatar",
            extends: None,
            description: Some(
                "data register for the regular channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdatach",
                    description: Some(
                        "Regular channel most recently converted.",
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
                    name: "rpend",
                    description: Some(
                        "Regular channel pending data.",
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
                    name: "rdata",
                    description: Some(
                        "Regular channel conversion data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1awcfr",
            extends: None,
            description: Some(
                "analog watchdog clear flag register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clrawltf",
                    description: Some(
                        "Clear the analog watchdog low threshold flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrawhtf",
                    description: Some(
                        "Clear the analog watchdog high threshold flag.",
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
            ],
        },
        FieldSet {
            name: "DfsdmFlt1awhtr",
            extends: None,
            description: Some(
                "analog watchdog high threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkawh",
                    description: Some(
                        "Break signal assignment to analog watchdog high threshold event.",
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
                    name: "awht",
                    description: Some(
                        "Analog watchdog high threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1awltr",
            extends: None,
            description: Some(
                "analog watchdog low threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkawl",
                    description: Some(
                        "Break signal assignment to analog watchdog low threshold event.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awlt",
                    description: Some(
                        "Analog watchdog low threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1awsr",
            extends: None,
            description: Some(
                "analog watchdog status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awltf",
                    description: Some(
                        "Analog watchdog low threshold flag.",
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
                    name: "awhtf",
                    description: Some(
                        "Analog watchdog high threshold flag.",
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
            name: "DfsdmFlt1cnvtimr",
            extends: None,
            description: Some(
                "conversion timer register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnvcnt",
                    description: Some(
                        "28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1cr1",
            extends: None,
            description: Some(
                "control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfen",
                    description: Some(
                        "DFSDM enable.",
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
                    name: "jswstart",
                    description: Some(
                        "Start a conversion of the injected group of channels.",
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
                    name: "jsync",
                    description: Some(
                        "Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger.",
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
                    name: "jscan",
                    description: Some(
                        "Scanning conversion mode for injected conversions.",
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
                    name: "jdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the injected channel group.",
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
                    name: "jextsel",
                    description: Some(
                        "Trigger signal selection for launching injected conversions.",
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
                    name: "jexten",
                    description: Some(
                        "Trigger enable and trigger edge selection for injected conversions.",
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
                Field {
                    name: "rswstart",
                    description: Some(
                        "Software start of a conversion on the regular channel.",
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
                    name: "rcont",
                    description: Some(
                        "Continuous mode selection for regular conversions.",
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
                    name: "rsync",
                    description: Some(
                        "Launch regular conversion synchronously with DFSDM0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the regular conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rch",
                    description: Some(
                        "Regular channel selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fast",
                    description: Some(
                        "Fast conversion mode selection for regular conversions.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awfsel",
                    description: Some(
                        "Analog watchdog fast mode select.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1cr2",
            extends: None,
            description: Some(
                "control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jeocie",
                    description: Some(
                        "Injected end of conversion interrupt enable.",
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
                    name: "reocie",
                    description: Some(
                        "Regular end of conversion interrupt enable.",
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
                    name: "jovrie",
                    description: Some(
                        "Injected data overrun interrupt enable.",
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
                    name: "rovrie",
                    description: Some(
                        "Regular data overrun interrupt enable.",
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
                    name: "awdie",
                    description: Some(
                        "Analog watchdog interrupt enable.",
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
                    name: "scdie",
                    description: Some(
                        "Short-circuit detector interrupt enable.",
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
                    name: "ckabie",
                    description: Some(
                        "Clock absence interrupt enable.",
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
                    name: "exch",
                    description: Some(
                        "Extremes detector channel selection.",
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
                Field {
                    name: "awdch",
                    description: Some(
                        "Analog watchdog channel selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1exmax",
            extends: None,
            description: Some(
                "Extremes detector maximum register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exmaxch",
                    description: Some(
                        "Extremes detector maximum data channel.",
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
                    name: "exmax",
                    description: Some(
                        "Extremes detector maximum value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1exmin",
            extends: None,
            description: Some(
                "Extremes detector minimum register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exminch",
                    description: Some(
                        "Extremes detector minimum data channel.",
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
                    name: "exmin",
                    description: Some(
                        "EXMIN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1fcr3",
            extends: None,
            description: Some(
                "control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iosr",
                    description: Some(
                        "The integrator oversampling rate is 2 to the power of IOSR.",
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
                    name: "fosr",
                    description: Some(
                        "Sinc filter oversampling rate.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ford",
                    description: Some(
                        "Sinc Filter Order.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1icr",
            extends: None,
            description: Some(
                "interrupt flag clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clrjovrf",
                    description: Some(
                        "Clear the injected conversion overrun flag.",
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
                    name: "clrrovrf",
                    description: Some(
                        "Clear the regular conversion overrun flag.",
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
                    name: "clrckabf",
                    description: Some(
                        "Clear the clock absence flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrscdf",
                    description: Some(
                        "Clear the short-circuit detector flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1isr",
            extends: None,
            description: Some(
                "interrupt and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jeocf",
                    description: Some(
                        "End of injected conversion flag.",
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
                    name: "reocf",
                    description: Some(
                        "End of regular conversion flag.",
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
                    name: "jovrf",
                    description: Some(
                        "Injected conversion overrun flag.",
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
                    name: "rovrf",
                    description: Some(
                        "Regular conversion overrun flag.",
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
                    name: "awdf",
                    description: Some(
                        "Analog watchdog.",
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
                    name: "jcip",
                    description: Some(
                        "Injected conversion in progress status.",
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
                    name: "rcip",
                    description: Some(
                        "Regular conversion in progress status.",
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
                    name: "ckabf",
                    description: Some(
                        "Clock absence flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scdf",
                    description: Some(
                        "short-circuit detector flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1jchgr",
            extends: None,
            description: Some(
                "injected channel group selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jchg",
                    description: Some(
                        "Injected channel group selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1jdatar",
            extends: None,
            description: Some(
                "data register for injected group.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdatach",
                    description: Some(
                        "Injected channel most recently converted.",
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
                    name: "jdata",
                    description: Some(
                        "Injected group conversion data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DfsdmFlt1rdatar",
            extends: None,
            description: Some(
                "data register for the regular channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdatach",
                    description: Some(
                        "Regular channel most recently converted.",
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
                    name: "rpend",
                    description: Some(
                        "Regular channel pending data.",
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
                    name: "rdata",
                    description: Some(
                        "Regular channel conversion data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
