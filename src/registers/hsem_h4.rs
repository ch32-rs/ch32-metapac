use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Hsem",
            extends: None,
            description: Some(
                "Hardware semaphore (HSEM). 32 lock channels with PID/CID-keyed write-lock and read-lock acquisition, batch unlock by key, and per-channel interrupt on remote unlock.",
            ),
            items: &[
                BlockItem {
                    name: "rx",
                    description: Some(
                        "Lock channel register (write-lock / 2-step acquisition).",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
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
                                "Lock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rlrx",
                    description: Some(
                        "Read-lock channel register (read-lock / 1-step acquisition). Same field layout as RX but read-only.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Lock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lse",
                    description: Some(
                        "Lock status register. Bit n indicates whether channel n is currently locked.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Chbits",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clr",
                    description: Some(
                        "Batch unlock register. Writing the unlock key together with match PID/CID and masks unlocks all channels matching the rule.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Clr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key",
                    description: Some(
                        "Unlock key and auto-clear control register.",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Key",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "Per-channel interrupt enable register (hart-private). Bit n enables interrupt when channel n is unlocked by the other hart.",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chbits",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr",
                    description: Some(
                        "Per-channel interrupt status register (hart-private). Bit n is set when channel n is unlocked by the other hart. Write 1 to clear.",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chbits",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ism",
                    description: Some(
                        "Per-channel interrupt mask status register (hart-private). Read-only AND of IER and ISR — bit n set means channel n is currently raising an interrupt.",
                    ),
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Chbits",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lsm",
                    description: Some(
                        "Lock status mask register (hart-private). Bit n indicates channel n is locked by THIS hart.",
                    ),
                    array: None,
                    byte_offset: 0x318,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Chbits",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Chbits",
            extends: None,
            description: Some(
                "32-bit per-channel bitmap (used by LSE / IER / ISR / ISM / LSM). Bit n maps to HSEM channel n.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "Per-channel flag.",
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
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Clr",
            extends: None,
            description: Some(
                "Batch unlock register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "match_pid",
                    description: Some(
                        "PID value to match against each locked channel's PID for batch unlock.",
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
                    name: "match_cid",
                    description: Some(
                        "CID value to match against each locked channel's CID for batch unlock.",
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
                    name: "pid_mask",
                    description: Some(
                        "1=mask PID match (unlock regardless of PID); 0=require PID equality.",
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
                    name: "cid_mask",
                    description: Some(
                        "1=mask CID match (unlock regardless of CID); 0=require CID equality.",
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
                    name: "clr_key",
                    description: Some(
                        "Unlock key. Must equal HSEM_KEY.KEY_VALUE (default 0x5AA5) for the write to take effect.",
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
            name: "Key",
            extends: None,
            description: Some(
                "Unlock key and auto-clear control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "auto_1step_clr",
                    description: Some(
                        "When set, a 1-step lock failure auto-clears the corresponding interrupt status bit.",
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
                    name: "auto_2step_clr",
                    description: Some(
                        "When set, a 2-step lock failure auto-clears the corresponding interrupt status bit.",
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
                    name: "key_value",
                    description: Some(
                        "Unlock key value used for batch-unlock matching (read-only, default 0x5AA5).",
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
            name: "Lock",
            extends: None,
            description: Some(
                "Lock channel register layout used by RX (RW) and RLRX (RO).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pid",
                    description: Some(
                        "Process ID. Reset/release value is 0; on write-lock takes the bus write data.",
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
                    name: "cid",
                    description: Some(
                        "Core ID of the hart that locked the channel. 0=hart 0 (V3F), 1=hart 1 (V5F).",
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
                    name: "lock",
                    description: Some(
                        "Channel lock state. 1=locked, 0=idle.",
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
    ],
    enums: &[],
};
