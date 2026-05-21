use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pfic",
            extends: None,
            description: Some(
                "Programmable Fast Interrupt Controller.",
            ),
            items: &[
                BlockItem {
                    name: "isr1",
                    description: Some(
                        "Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr2",
                    description: Some(
                        "Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr3",
                    description: Some(
                        "Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr4",
                    description: Some(
                        "Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr5",
                    description: Some(
                        "Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipr1",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipr2",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipr3",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipr4",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipr5",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ithresdr",
                    description: Some(
                        "Interrupt Priority Register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ithresdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "Interrupt configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
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
                BlockItem {
                    name: "gisr",
                    description: Some(
                        "Interrupt Global Register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Gisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vtfidr",
                    description: Some(
                        "ID Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vtfidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vtfaddrr0",
                    description: Some(
                        "Interrupt 0 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vtfaddrr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vtfaddrr1",
                    description: Some(
                        "Interrupt 1 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vtfaddrr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vtfaddrr2",
                    description: Some(
                        "Interrupt 2 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vtfaddrr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vtfaddrr3",
                    description: Some(
                        "Interrupt 3 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vtfaddrr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ienr1",
                    description: Some(
                        "Interrupt Setting Register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ienr2",
                    description: Some(
                        "Interrupt Setting Register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ienr3",
                    description: Some(
                        "Interrupt Setting Register.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ienr4",
                    description: Some(
                        "Interrupt Setting Register.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ienr5",
                    description: Some(
                        "Interrupt Setting Register.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irer1",
                    description: Some(
                        "Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Irer1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irer2",
                    description: Some(
                        "Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Irer2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irer3",
                    description: Some(
                        "Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Irer3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irer4",
                    description: Some(
                        "Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Irer4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irer5",
                    description: Some(
                        "Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Irer5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipsr1",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipsr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipsr2",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipsr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipsr3",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipsr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipsr4",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipsr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipsr5",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipsr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprr1",
                    description: Some(
                        "Interrupt Pending Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprr2",
                    description: Some(
                        "Interrupt Pending Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprr3",
                    description: Some(
                        "Interrupt Pending Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprr4",
                    description: Some(
                        "Interrupt Pending Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x28c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprr5",
                    description: Some(
                        "Interrupt Pending Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iactr1",
                    description: Some(
                        "Interrupt ACTIVE Register.",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iactr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iactr2",
                    description: Some(
                        "Interrupt ACTIVE Register.",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iactr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iactr3",
                    description: Some(
                        "Interrupt ACTIVE Register.",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iactr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iactr4",
                    description: Some(
                        "Interrupt ACTIVE Register.",
                    ),
                    array: None,
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iactr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iactr5",
                    description: Some(
                        "Interrupt ACTIVE Register.",
                    ),
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iactr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior",
                    description: Some(
                        "Interrupt Priority Register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 64,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr0",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x600,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr1",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x604,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr2",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x608,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr3",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x60c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr4",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x610,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr5",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x614,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr6",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x618,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr7",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x61c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr8",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x620,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr9",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x624,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr10",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x628,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr11",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x62c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr12",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x630,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr13",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x634,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr14",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x638,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr15",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x63c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr16",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x640,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr17",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x644,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr18",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x648,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr19",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x64c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr20",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x650,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr21",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x654,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr22",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x658,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr23",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x65c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr24",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x660,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr25",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x664,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr26",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x668,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr27",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x66c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr28",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x670,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr29",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x674,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr30",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x678,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr31",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x67c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr32",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x680,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr33",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x684,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr34",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x688,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr35",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x68c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr36",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x690,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr37",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x694,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr38",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x698,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr39",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x69c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr40",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr41",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr42",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr43",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr44",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr45",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr46",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr47",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr48",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr49",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr50",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr51",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr52",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr53",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr54",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr55",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr56",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr57",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr58",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr59",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr60",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr61",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr62",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iallocr63",
                    description: Some(
                        "Interrupt Allocation Register.",
                    ),
                    array: None,
                    byte_offset: 0x6fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "iautr1",
                    description: Some(
                        "interrupts authority register 1.",
                    ),
                    array: None,
                    byte_offset: 0x700,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iautr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iautr2",
                    description: Some(
                        "interrupts authority register 2.",
                    ),
                    array: None,
                    byte_offset: 0x704,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iautr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iautr3",
                    description: Some(
                        "interrupts authority register 3.",
                    ),
                    array: None,
                    byte_offset: 0x708,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iautr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iautr4",
                    description: Some(
                        "interrupts authority register 4.",
                    ),
                    array: None,
                    byte_offset: 0x70c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iautr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iautr5",
                    description: Some(
                        "interrupts authority register 5.",
                    ),
                    array: None,
                    byte_offset: 0x710,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iautr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wakeip0",
                    description: Some(
                        "PFIC wake-up instruction pointer register 0.",
                    ),
                    array: None,
                    byte_offset: 0x720,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wakeip0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wakeip1",
                    description: Some(
                        "PFIC wake-up instruction pointer register 1.",
                    ),
                    array: None,
                    byte_offset: 0x724,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wakeip1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cstar0",
                    description: Some(
                        "PFIC kernel status register 0.",
                    ),
                    array: None,
                    byte_offset: 0x780,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cstar0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cstar1",
                    description: Some(
                        "PFIC kernel status register 1.",
                    ),
                    array: None,
                    byte_offset: 0x784,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cstar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eenr",
                    description: Some(
                        "PFIC Event Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0xc80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "epr",
                    description: Some(
                        "PFIC Event Suspend Register.",
                    ),
                    array: None,
                    byte_offset: 0xc84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Epr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ewupr",
                    description: Some(
                        "PFIC Event Wake Register.",
                    ),
                    array: None,
                    byte_offset: 0xc88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ewupr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sctlr",
                    description: Some(
                        "System Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xd10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sctlr",
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
                "Interrupt configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sysrst",
                    description: Some(
                        "System reset register.",
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
                    name: "keycode",
                    description: Some(
                        "KEYCODE.",
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
            name: "Cstar0",
            extends: None,
            description: Some(
                "PFIC kernel status register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cpu_nest_sta_0",
                    description: Some(
                        "The nested status register of kernel C0 interrupts is used to.  query the nested state of kernel C0 interrupts. nest_sta bits from the INEST_CTLR register of the CSR register.",
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
                    name: "cpu_irq_active_0",
                    description: Some(
                        "Kernel C0 interrupt active flag register,.  which is used to query whether kernel C0 is processing interrupts.",
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
                    name: "cpu_irq_pend_0",
                    description: Some(
                        "Kernel C0 interrupt pending flag register, which is used to query kernel C1 for unhandled interrupts.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpu_globl_ie_0",
                    description: Some(
                        "The kernel C0 global interrupt enable register is used to.  query whether the kernel C0 global interrupt is enabled.",
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
                    name: "cpu_dbg_mode_0",
                    description: Some(
                        "Kernel C0 debug mode register, which is used to query whether kernel C0 is in debug mode.",
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
                    name: "cpu_lock_up_0",
                    description: Some(
                        "The kernel C0 lock status register is used to query whether kernel C0 is in the locked state.",
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
                    name: "cpu_ex_state_0",
                    description: Some(
                        "The kernel C0 status register is used to obtain the running state of kernel C0.",
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
            ],
        },
        FieldSet {
            name: "Cstar1",
            extends: None,
            description: Some(
                "PFIC kernel status register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cpu_nest_sta_1",
                    description: Some(
                        "The nested status register of kernel C1 interrupts is used to.  query the nested state of kernel C1 interrupts. nest_sta bits from the INEST_CTLR register of the CSR register.",
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
                    name: "cpu_irq_active_1",
                    description: Some(
                        "Kernel C1 interrupt active flag register,.  which is used to query whether kernel C1 is processing interrupts.",
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
                    name: "cpu_irq_pend_1",
                    description: Some(
                        "Kernel C1 interrupt pending flag register, which is used to query kernel C1 for unhandled interrupts.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpu_globl_ie_1",
                    description: Some(
                        "The kernel C1 global interrupt enable register is used to.  query whether the kernel C1 global interrupt is enabled.",
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
                    name: "cpu_dbg_mode_1",
                    description: Some(
                        "Kernel C1 debug mode register, which is used to query whether kernel C1 is in debug mode.",
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
                    name: "cpu_lock_up_1",
                    description: Some(
                        "The kernel C1 lock status register is used to query whether kernel C1 is in the locked state.",
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
                    name: "cpu_ex_state_1",
                    description: Some(
                        "The kernel C1 status register is used to obtain the running state of kernel C1.",
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
            ],
        },
        FieldSet {
            name: "Eenr",
            extends: None,
            description: Some(
                "PFIC Event Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eventen",
                    description: Some(
                        "31-0 Event wake-up enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Epr",
            extends: None,
            description: Some(
                "PFIC Event Suspend Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "event_pend7_0",
                    description: Some(
                        "7-0 Event Pending Status, Not Clearable.",
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
                    name: "event_pend31_8",
                    description: Some(
                        "31-8 event is suspended, write 1 to clear zero.",
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
            name: "Ewupr",
            extends: None,
            description: Some(
                "PFIC Event Wake Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "event_wup",
                    description: Some(
                        "31-0 Event Wake Register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gisr",
            extends: None,
            description: Some(
                "Interrupt Global Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "neststa",
                    description: Some(
                        "interrupt nesting.",
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
                    name: "gactsta",
                    description: Some(
                        "interrupt execution.",
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
                    name: "gpendsta",
                    description: Some(
                        "interrupt pending.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "globl_ie",
                    description: Some(
                        "global interrupt enable.",
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
                    name: "dbg_mode",
                    description: Some(
                        "debug mode.",
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
                    name: "lock_up",
                    description: Some(
                        "lock-out state.",
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
                    name: "ex_state",
                    description: Some(
                        "The kernel status register, which is used to obtain the running state of the kernel.",
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
            ],
        },
        FieldSet {
            name: "Iactr1",
            extends: None,
            description: Some(
                "Interrupt ACTIVE Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacts2_3",
                    description: Some(
                        "IACTS.",
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
                    name: "iacts5",
                    description: Some(
                        "IACTS.",
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
                    name: "iacts8_9",
                    description: Some(
                        "IACTS.",
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
                    name: "iacts12_31",
                    description: Some(
                        "IACTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iactr2",
            extends: None,
            description: Some(
                "Interrupt ACTIVE Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacts",
                    description: Some(
                        "IACTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iactr3",
            extends: None,
            description: Some(
                "Interrupt ACTIVE Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacts",
                    description: Some(
                        "IACTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iactr4",
            extends: None,
            description: Some(
                "Interrupt ACTIVE Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacts",
                    description: Some(
                        "IACTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iactr5",
            extends: None,
            description: Some(
                "Interrupt ACTIVE Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacts",
                    description: Some(
                        "IACTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iautr1",
            extends: None,
            description: Some(
                "interrupts authority register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iaut",
                    description: Some(
                        "interrupt allocates all registers.  and is used to query whether the interrupt responds to each kernel allocation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iautr2",
            extends: None,
            description: Some(
                "interrupts authority register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iaut",
                    description: Some(
                        "interrupt allocates all registers.  and is used to query whether the interrupt responds to each kernel allocation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iautr3",
            extends: None,
            description: Some(
                "interrupts authority register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iaut",
                    description: Some(
                        "interrupt allocates all registers.  and is used to query whether the interrupt responds to each kernel allocation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iautr4",
            extends: None,
            description: Some(
                "interrupts authority register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iaut",
                    description: Some(
                        "interrupt allocates all registers.  and is used to query whether the interrupt responds to each kernel allocation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iautr5",
            extends: None,
            description: Some(
                "interrupts authority register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iaut",
                    description: Some(
                        "interrupt allocates all registers.  and is used to query whether the interrupt responds to each kernel allocation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ienr1",
            extends: None,
            description: Some(
                "Interrupt Setting Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten12_31",
                    description: Some(
                        "INTEN12_31.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ienr2",
            extends: None,
            description: Some(
                "Interrupt Setting Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "INTEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ienr3",
            extends: None,
            description: Some(
                "Interrupt Setting Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "INTEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ienr4",
            extends: None,
            description: Some(
                "Interrupt Setting Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "INTEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ienr5",
            extends: None,
            description: Some(
                "Interrupt Setting Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "INTEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipr1",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendsta2_3",
                    description: Some(
                        "PENDSTA.",
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
                    name: "pendsta5",
                    description: Some(
                        "PENDSTA.",
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
                    name: "intensta8_9",
                    description: Some(
                        "PENDSTA.",
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
                    name: "intensta12_31",
                    description: Some(
                        "PENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipr2",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendsta",
                    description: Some(
                        "PENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipr3",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendsta",
                    description: Some(
                        "PENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipr4",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendsta",
                    description: Some(
                        "PENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipr5",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendsta",
                    description: Some(
                        "PENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprr1",
            extends: None,
            description: Some(
                "Interrupt Pending Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendrst2_3",
                    description: Some(
                        "PENDRESET.",
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
                    name: "pendrst5",
                    description: Some(
                        "PENDRESET.",
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
                    name: "pendrst8",
                    description: Some(
                        "PENDRESET.",
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
                    name: "pendrst12_31",
                    description: Some(
                        "PENDRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprr2",
            extends: None,
            description: Some(
                "Interrupt Pending Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendrst",
                    description: Some(
                        "PENDRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprr3",
            extends: None,
            description: Some(
                "Interrupt Pending Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendrst",
                    description: Some(
                        "PENDRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprr4",
            extends: None,
            description: Some(
                "Interrupt Pending Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendrst",
                    description: Some(
                        "PENDRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprr5",
            extends: None,
            description: Some(
                "Interrupt Pending Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendrst",
                    description: Some(
                        "PENDRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipsr1",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendset2_3",
                    description: Some(
                        "PENDSET.",
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
                    name: "pendset5",
                    description: Some(
                        "PENDSET.",
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
                    name: "pendset8_9",
                    description: Some(
                        "PENDSET.",
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
                    name: "pendset12_31",
                    description: Some(
                        "PENDSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipsr2",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendset",
                    description: Some(
                        "PENDSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipsr3",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendset",
                    description: Some(
                        "PENDSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipsr4",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendset",
                    description: Some(
                        "PENDSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipsr5",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendset",
                    description: Some(
                        "PENDSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irer1",
            extends: None,
            description: Some(
                "Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intrset12_31",
                    description: Some(
                        "INTRSET12_31.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irer2",
            extends: None,
            description: Some(
                "Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intrset",
                    description: Some(
                        "INTRSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irer3",
            extends: None,
            description: Some(
                "Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intrset",
                    description: Some(
                        "INTRSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irer4",
            extends: None,
            description: Some(
                "Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intrset",
                    description: Some(
                        "INTRSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irer5",
            extends: None,
            description: Some(
                "Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intrset",
                    description: Some(
                        "INTRSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr1",
            extends: None,
            description: Some(
                "Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intensta2_3",
                    description: Some(
                        "Interrupt ID Status.",
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
                    name: "intensta5",
                    description: Some(
                        "Interrupt ID Status.",
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
                    name: "intensta8_9",
                    description: Some(
                        "Interrupt ID Status.",
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
                    name: "intensta12_31",
                    description: Some(
                        "Interrupt ID Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr2",
            extends: None,
            description: Some(
                "Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intensta",
                    description: Some(
                        "Interrupt ID Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr3",
            extends: None,
            description: Some(
                "Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intensta",
                    description: Some(
                        "Interrupt ID Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr4",
            extends: None,
            description: Some(
                "Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intensta",
                    description: Some(
                        "Interrupt ID Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr5",
            extends: None,
            description: Some(
                "Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intensta",
                    description: Some(
                        "Interrupt ID Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ithresdr",
            extends: None,
            description: Some(
                "Interrupt Priority Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "threshold",
                    description: Some(
                        "THRESHOLD.",
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
            name: "Sctlr",
            extends: None,
            description: Some(
                "System Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sleeponexit",
                    description: Some(
                        "system leaves the state after an interruption.",
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
                    name: "sleepdeep",
                    description: Some(
                        "low power mode selection.",
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
                    name: "wfitowfe",
                    description: Some(
                        "Treat WFI as WFE.",
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
                    name: "sevonpend",
                    description: Some(
                        "Send-event-on-pend. When set, any pended interrupt (even masked) wakes the core from WFE.",
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
                    name: "sendevent",
                    description: Some(
                        "Send a one-shot event. Used to wake the other hart from WFE (paired with PFIC_WAKEIPx for dual-core boot).",
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
                    name: "hart_id",
                    description: Some(
                        "Hart ID of the core reading this register. 0 = hart 0 (V3F on CH32H417), 1 = hart 1 (V5F). Reserved upper bits read as 0.",
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
                    name: "sysrst",
                    description: Some(
                        "System reset.",
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
            name: "Vtfaddrr0",
            extends: None,
            description: Some(
                "Interrupt 0 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vtf0en",
                    description: Some(
                        "VTF0EN.",
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
                    name: "addr0",
                    description: Some(
                        "ADDR0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vtfaddrr1",
            extends: None,
            description: Some(
                "Interrupt 1 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vtf1en",
                    description: Some(
                        "VTF1EN.",
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
                    name: "addr1",
                    description: Some(
                        "ADDR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vtfaddrr2",
            extends: None,
            description: Some(
                "Interrupt 2 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vtf2en",
                    description: Some(
                        "VTF2EN.",
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
                    name: "addr2",
                    description: Some(
                        "ADDR2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vtfaddrr3",
            extends: None,
            description: Some(
                "Interrupt 3 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vtf3en",
                    description: Some(
                        "VTF3EN.",
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
                    name: "addr3",
                    description: Some(
                        "ADDR3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vtfidr",
            extends: None,
            description: Some(
                "ID Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vtfid0",
                    description: Some(
                        "VTFID0.",
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
                    name: "vtfid1",
                    description: Some(
                        "VTFID1.",
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
                    name: "vtfid2",
                    description: Some(
                        "VTFID2.",
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
                    name: "vtfid3",
                    description: Some(
                        "VTFID3.",
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
            name: "Wakeip0",
            extends: None,
            description: Some(
                "PFIC wake-up instruction pointer register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shutdown0",
                    description: Some(
                        "Kernel C0 Deep Sleep (Locked) Cancellation Register.",
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
                    name: "ip_reload0",
                    description: Some(
                        "The PC address register on kernel C0 wake-up,.  which is used to reload the PC value after power-on wake-up and reset wake-up.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wakeip1",
            extends: None,
            description: Some(
                "PFIC wake-up instruction pointer register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shutdown1",
                    description: Some(
                        "Kernel C1 Deep Sleep (Locked) Cancellation Register.",
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
                    name: "ip_reload1",
                    description: Some(
                        "The PC address register on kernel C1 wake-up,.  which is used to reload the PC value after power-on wake-up and reset wake-up.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
