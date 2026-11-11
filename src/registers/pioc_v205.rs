use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pioc",
            extends: None,
            description: Some(
                "Programmable protocol I/O microcontroller.",
            ),
            items: &[
                BlockItem {
                    name: "sfr_indir_port",
                    description: Some(
                        "Indirectly addressed data read/write ports.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rsfr_indir_port2",
                    description: Some(
                        "Indirectly addresses the data read/write port of 2.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_prg_count",
                    description: Some(
                        "Program counters for the low bytes of the PC.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_status_reg",
                    description: Some(
                        "Status registers.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SfrStatusReg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_indir_addr",
                    description: Some(
                        "Indirectly addressed address registers.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_tmr0_count",
                    description: Some(
                        "The counter register of timer 0.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_timer_ctrl",
                    description: Some(
                        "The control register of the timer.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_tmr0_init",
                    description: Some(
                        "The initial register of timer 0.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_bit_cycle",
                    description: Some(
                        "Periodic registers for coded bits.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SfrBitCycle",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_indir_addr2",
                    description: Some(
                        "Address registers for indirect address 2.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_port_dir",
                    description: Some(
                        "The port direction sets the register.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "SfrPortDir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_port_io",
                    description: Some(
                        "Port input and output registers.",
                    ),
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_bit_config",
                    description: Some(
                        "Coded bit configuration registers.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "SfrBitConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_sys_cfg",
                    description: Some(
                        "System configuration registers.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SfrSysCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_ctrl_rd",
                    description: Some(
                        "The eMCU reads and writes and the host reads only the registers.",
                    ),
                    array: None,
                    byte_offset: 0x1d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_ctrl_wr",
                    description: Some(
                        "The host reads and writes and the eMCU reads only registers.",
                    ),
                    array: None,
                    byte_offset: 0x1e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_data_exch",
                    description: Some(
                        "Data exchange registers.",
                    ),
                    array: None,
                    byte_offset: 0x1f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sfr_data_reg",
                    description: Some(
                        "PIOC data registers 0-31.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "SfrBitConfig",
            extends: None,
            description: Some(
                "Coded bit configuration registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sb_bit_cyc_cnt3",
                    description: Some(
                        "The periodic timing state of the encoded bits.",
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
                    name: "sb_bit_cyc_cnt4",
                    description: Some(
                        "The periodic timing state of the encoded bits.",
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
                    name: "sb_bit_cyc_cnt5",
                    description: Some(
                        "The periodic timing state of the encoded bits.",
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
                    name: "sb_bit_cyc_cnt6",
                    description: Some(
                        "The periodic timing state of the encoded bits.",
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
                    name: "sb_bit_cyc_tail",
                    description: Some(
                        "Periodic state of the encoded bits.",
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
                    name: "sb_port_in_edge",
                    description: Some(
                        "Pin input level sampling point selection.",
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
                    name: "sb_bit_code_mod",
                    description: Some(
                        "How the coded bits are modulated.",
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
                    name: "sb_bit_tx_en",
                    description: Some(
                        "The transmission of the coded bits is enabled.",
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
        FieldSet {
            name: "SfrBitCycle",
            extends: None,
            description: Some(
                "Periodic registers for coded bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sb_bit_cycle",
                    description: Some(
                        "Sets the width of the encoded bits in clock cycles, which is the actual number of bits minus 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sb_bit_tx_o0",
                    description: Some(
                        "The raw bit data of the encoded bit to be sent, which is a double-buffered structure.",
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
        FieldSet {
            name: "SfrPortDir",
            extends: None,
            description: Some(
                "The port direction sets the register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sb_port_dir0",
                    description: Some(
                        "IO0 Pin Direction Control.",
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
                    name: "sb_port_dir1",
                    description: Some(
                        "IO1 Pin Direction Control.",
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
                    name: "sb_port_pu0",
                    description: Some(
                        "IO0 Pin Pull-Up Enables.",
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
                    name: "sb_port_pu1",
                    description: Some(
                        "IO1 Pin Pull-Up Enables.",
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
                    name: "sb_port_mod0",
                    description: Some(
                        "Pin mode control, host side defines the purpose.",
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
                    name: "sb_port_mod1",
                    description: Some(
                        "Pin mode control, host side defines the purpose.",
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
                    name: "sb_port_mod2",
                    description: Some(
                        "Pin mode control, host side defines the purpose.",
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
                    name: "sb_port_mod3",
                    description: Some(
                        "Pin mode control, host side defines the purpose.",
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
        FieldSet {
            name: "SfrStatusReg",
            extends: None,
            description: Some(
                "Status registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sb_flag_c",
                    description: Some(
                        "ALU carry flag, whether the result is carried or produced by displacement.",
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
                    name: "sb_gp_bit_x",
                    description: Some(
                        "A generic bit variable X, defined and used by the application.",
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
                    name: "sb_flag_z",
                    description: Some(
                        "ALU zero flag, whether the result is 00H.",
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
                    name: "sb_gp_bit_y",
                    description: Some(
                        "a generic bit variable Y, defined and used by the application.",
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
                    name: "sb_en_tout_rst",
                    description: Some(
                        "Timer Timeout Reset Enabled.",
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
                    name: "sb_stack_used",
                    description: Some(
                        "Current stack usage flags.",
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
            ],
        },
        FieldSet {
            name: "SfrSysCfg",
            extends: None,
            description: Some(
                "System configuration registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sb_mst_clk_gate",
                    description: Some(
                        "Global Clock Control for eMCUs.",
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
                    name: "sb_mst_reset",
                    description: Some(
                        "Force eMCU Reset.",
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
                    name: "sb_mst_io_en0",
                    description: Some(
                        "Mode and output control switches for IO0 pins.",
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
                    name: "sb_mst_io_en1",
                    description: Some(
                        "Mode and output control switches for IO1 pins.",
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
                    name: "sb_mst_cfg_b4",
                    description: Some(
                        "Configure information bits, software-defined uses.",
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
                    name: "sb_data_mw_sr",
                    description: Some(
                        "SFR_CTRL_WR waiting to read the status bit.",
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
                    name: "sb_data_sw_mr",
                    description: Some(
                        "SFR_CTRL_RD wait to read the status bit.",
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
                    name: "sb_int_req",
                    description: Some(
                        "Interrupt Request Activation Bits.",
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
    enums: &[],
};
