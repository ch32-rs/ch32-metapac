use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "BleLle",
            extends: None,
            description: Some(
                "BLE Link-Layer Engine — timing, state machine, IRQ, DMA buffer ptr.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "Link-layer CTRL — channel field bits[5:0], whitening bit6, mode bits[8:7], rate bits[13:12], BLE GO bit23.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crc_init",
                    description: Some(
                        "CRC seed (BLE adv default 0x555555).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "access_addr",
                    description: Some(
                        "Access address / IRQ status (W1C). Read=live status; write 1 to clear, or write ADV AA 0x8E89BED6 for advertising.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "irq_mask",
                    description: Some(
                        "IRQ mask (default 0xF00F = bits[15:12] + bits[3:0]).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "timing0",
                    description: Some(
                        "Timing slot 0 (default 140).",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "state_machine",
                    description: Some(
                        "Link-layer state machine (108=Sleep, 93=ConnRxWait, 97=ConnTxPrep, 101=ConnAckWait, 105=ConnEventClosing, 107=SleepPrep).",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "StateMachine",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timing2",
                    description: Some(
                        "Timing slot 2 (default 140).",
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
                    name: "timing3",
                    description: Some(
                        "Timing slot 3 (default 60).",
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
                    name: "timing4",
                    description: Some(
                        "Timing slot 4 (default 140).",
                    ),
                    array: None,
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
                    name: "timing5",
                    description: Some(
                        "Timing slot 5 (default 60).",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "timing6",
                    description: Some(
                        "Timing slot 6 (default 140).",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "timing7",
                    description: Some(
                        "Timing slot 7 (default 108).",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "settle",
                    description: Some(
                        "Settle timer used during BLE_RegInit (93 during cal, 0 post).",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "timer",
                    description: Some(
                        "Countdown timer for RFEND_WaitTune; written from gBleIPPara[16..19] (=776 for ADV TX).",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "scan_offset",
                    description: Some(
                        "Active-scan offset (= gBleIPPara[20..23] << 1, set in `.L6` bit5 path).",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "tx_buf_ptr",
                    description: Some(
                        "TX buffer base (BB+0x70 in adv.rs; written before ADV GO).",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_buf",
                    description: Some(
                        "DMA buffer base address (= gBleIPPara[36] MEMAddr; required non-zero for LLE state machine to fire).",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "BLE_LLE CTRL — channel, whitening, mode, rate, GO.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "channel",
                    description: Some(
                        "BLE logical channel bits[5:0] (37/38/39 for ADV, 0..36 for data).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "whiten",
                    description: Some(
                        "Whitening enable bit6 (DTM uses 0; ADV may use 1).",
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
                    name: "mode",
                    description: Some(
                        "TX/RX mode select bits[8:7] (TX path sets bit8; RX sets bit8+other).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rate",
                    description: Some(
                        "PHY rate select bits[13:12] (00=1Mbps; non-zero=2M/Coded).",
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
                    name: "ble_go",
                    description: Some(
                        "BLE GO strobe bit23 (lui 0x800; set to fire TX/RX).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "StateMachine",
            extends: None,
            description: Some(
                "BLE_LLE state machine value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "state",
                    description: Some(
                        "8-bit state (108=Sleep default; see register description).",
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
    ],
    enums: &[],
};
