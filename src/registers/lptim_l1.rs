use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Lptim",
        extends: None,
        description: Some("Low-power timer."),
        items: &[
            BlockItem {
                name: "isr",
                description: Some("interrupt status register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Isr"),
                }),
            },
            BlockItem {
                name: "icr",
                description: Some("interrupt clear register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Icr"),
                }),
            },
            BlockItem {
                name: "ier",
                description: Some("interrupt enable register."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ier"),
                }),
            },
            BlockItem {
                name: "cfgr",
                description: Some("configuration register."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfgr"),
                }),
            },
            BlockItem {
                name: "cr",
                description: Some("control register."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "cmp",
                description: Some("compare register."),
                array: None,
                byte_offset: 0x14,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cmp"),
                }),
            },
            BlockItem {
                name: "arr",
                description: Some("aoto-reload register."),
                array: None,
                byte_offset: 0x18,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Arr"),
                }),
            },
            BlockItem {
                name: "cnt",
                description: Some("COUNT register."),
                array: None,
                byte_offset: 0x1c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cnt"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Arr",
            extends: None,
            description: Some("aoto-reload register."),
            bit_size: 32,
            fields: &[Field {
                name: "arr",
                description: Some("aoto-reload count value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some("configuration register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cksel",
                    description: Some("effective edge configuration."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckpol",
                    description: Some("configure effective edges."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckflt",
                    description: Some("digital filter for ex-clock."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trgflt",
                    description: Some("digital filter for flip-flops."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "presc",
                    description: Some("prescaler configuration."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigsel",
                    description: Some("trigger source selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigen",
                    description: Some("trigger configuration."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timout",
                    description: Some("TIMEOUT control."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wave",
                    description: Some("PWM."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wavpol",
                    description: Some("PWM polarity."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "preload",
                    description: Some("update mode control."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "countmode",
                    description: Some("count clock selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enc",
                    description: Some("coder mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clkmx_sel",
                    description: Some("clock selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 25 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "force_pwm",
                    description: Some("Forcing pwm output."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 27 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cmp",
            extends: None,
            description: Some("compare register."),
            bit_size: 32,
            fields: &[Field {
                name: "cmp",
                description: Some("compare value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cnt",
            extends: None,
            description: Some("COUNT register."),
            bit_size: 32,
            fields: &[Field {
                name: "count",
                description: Some("Timer count value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("control register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some("timer enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sngstrt",
                    description: Some("start in one trigger mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cntstrt",
                    description: Some("start in continuous mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "outen",
                    description: Some("pwm output enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dir_exten",
                    description: Some("externally trigger count direction enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: Some("interrupt clear register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpmcf",
                    description: Some("clear compare register match flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arrmcf",
                    description: Some("clear Aoto-reload register match flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exttrigcf",
                    description: Some("clear Edge event are triggerd externally flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmpokcf",
                    description: Some("clear compare register data update flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arrokcf",
                    description: Some("clear Aoto-reload register data update flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "upcf",
                    description: Some("clear up flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "downcf",
                    description: Some("clear down flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some("interrupt enable register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpmie",
                    description: Some("compare register match successfully interrupts enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arrmie",
                    description: Some("Aoto-reload register match successfully interrupts enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exttrigie",
                    description: Some(
                        "Edge event are triggerd externally successfully interrupts enable.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmpokie",
                    description: Some(
                        "compare register data update successfully interrupts enable.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arrokie",
                    description: Some(
                        "Aoto-reload register data update successfully interrupts enable.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "upie",
                    description: Some("UP interrupt enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "downie",
                    description: Some("Down interrupt enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some("interrupt status register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpm",
                    description: Some("DATA of compare register and LPTIM_CNT match."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arrm",
                    description: Some("DATA of Aoto-reload register and LPTIM_CNT match."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exttrig",
                    description: Some("Edge event are triggerd externally."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmpok",
                    description: Some("compare register data update successfully."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arrok",
                    description: Some("Aoto-reload register data update successfully."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "up",
                    description: Some("count-up."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "down",
                    description: Some("count down."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dir_sync",
                    description: Some("direction of count."),
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