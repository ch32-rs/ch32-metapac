#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Hardware semaphore (HSEM). 32 lock channels with PID/CID-keyed write-lock and read-lock acquisition, batch unlock by key, and per-channel interrupt on remote unlock."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsem {
    ptr: *mut u8,
}
unsafe impl Send for Hsem {}
unsafe impl Sync for Hsem {}
impl Hsem {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Lock channel register (write-lock / 2-step acquisition)."]
    #[inline(always)]
    pub const fn rx(self, n: usize) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Read-lock channel register (read-lock / 1-step acquisition). Same field layout as RX but read-only."]
    #[inline(always)]
    pub const fn rlrx(self, n: usize) -> crate::common::Reg<regs::Lock, crate::common::R> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "Lock status register. Bit n indicates whether channel n is currently locked."]
    #[inline(always)]
    pub const fn lse(self) -> crate::common::Reg<regs::Chbits, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Batch unlock register. Writing the unlock key together with match PID/CID and masks unlocks all channels matching the rule."]
    #[inline(always)]
    pub const fn clr(self) -> crate::common::Reg<regs::Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Unlock key and auto-clear control register."]
    #[inline(always)]
    pub const fn key(self) -> crate::common::Reg<regs::Key, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "Per-channel interrupt enable register (hart-private). Bit n enables interrupt when channel n is unlocked by the other hart."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Chbits, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Per-channel interrupt status register (hart-private). Bit n is set when channel n is unlocked by the other hart. Write 1 to clear."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Chbits, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Per-channel interrupt mask status register (hart-private). Read-only AND of IER and ISR — bit n set means channel n is currently raising an interrupt."]
    #[inline(always)]
    pub const fn ism(self) -> crate::common::Reg<regs::Chbits, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "Lock status mask register (hart-private). Bit n indicates channel n is locked by THIS hart."]
    #[inline(always)]
    pub const fn lsm(self) -> crate::common::Reg<regs::Chbits, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
}
pub mod regs {
    #[doc = "32-bit per-channel bitmap (used by LSE / IER / ISR / ISM / LSM). Bit n maps to HSEM channel n."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chbits(pub u32);
    impl Chbits {
        #[doc = "Per-channel flag."]
        #[inline(always)]
        pub const fn ch(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Per-channel flag."]
        #[inline(always)]
        pub fn set_ch(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Chbits {
        #[inline(always)]
        fn default() -> Chbits {
            Chbits(0)
        }
    }
    #[doc = "Batch unlock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clr(pub u32);
    impl Clr {
        #[doc = "PID value to match against each locked channel's PID for batch unlock."]
        #[inline(always)]
        pub const fn match_pid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PID value to match against each locked channel's PID for batch unlock."]
        #[inline(always)]
        pub fn set_match_pid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "CID value to match against each locked channel's CID for batch unlock."]
        #[inline(always)]
        pub const fn match_cid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CID value to match against each locked channel's CID for batch unlock."]
        #[inline(always)]
        pub fn set_match_cid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "1=mask PID match (unlock regardless of PID); 0=require PID equality."]
        #[inline(always)]
        pub const fn pid_mask(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1=mask PID match (unlock regardless of PID); 0=require PID equality."]
        #[inline(always)]
        pub fn set_pid_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1=mask CID match (unlock regardless of CID); 0=require CID equality."]
        #[inline(always)]
        pub const fn cid_mask(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1=mask CID match (unlock regardless of CID); 0=require CID equality."]
        #[inline(always)]
        pub fn set_cid_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Unlock key. Must equal HSEM_KEY.KEY_VALUE (default 0x5AA5) for the write to take effect."]
        #[inline(always)]
        pub const fn clr_key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Unlock key. Must equal HSEM_KEY.KEY_VALUE (default 0x5AA5) for the write to take effect."]
        #[inline(always)]
        pub fn set_clr_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Clr {
        #[inline(always)]
        fn default() -> Clr {
            Clr(0)
        }
    }
    #[doc = "Unlock key and auto-clear control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key(pub u32);
    impl Key {
        #[doc = "When set, a 1-step lock failure auto-clears the corresponding interrupt status bit."]
        #[inline(always)]
        pub const fn auto_1step_clr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set, a 1-step lock failure auto-clears the corresponding interrupt status bit."]
        #[inline(always)]
        pub fn set_auto_1step_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set, a 2-step lock failure auto-clears the corresponding interrupt status bit."]
        #[inline(always)]
        pub const fn auto_2step_clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set, a 2-step lock failure auto-clears the corresponding interrupt status bit."]
        #[inline(always)]
        pub fn set_auto_2step_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Unlock key value used for batch-unlock matching (read-only, default 0x5AA5)."]
        #[inline(always)]
        pub const fn key_value(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Unlock key value used for batch-unlock matching (read-only, default 0x5AA5)."]
        #[inline(always)]
        pub fn set_key_value(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Key {
        #[inline(always)]
        fn default() -> Key {
            Key(0)
        }
    }
    #[doc = "Lock channel register layout used by RX (RW) and RLRX (RO)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lock(pub u32);
    impl Lock {
        #[doc = "Process ID. Reset/release value is 0; on write-lock takes the bus write data."]
        #[inline(always)]
        pub const fn pid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Process ID. Reset/release value is 0; on write-lock takes the bus write data."]
        #[inline(always)]
        pub fn set_pid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Core ID of the hart that locked the channel. 0=hart 0 (V3F), 1=hart 1 (V5F)."]
        #[inline(always)]
        pub const fn cid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Core ID of the hart that locked the channel. 0=hart 0 (V3F), 1=hart 1 (V5F)."]
        #[inline(always)]
        pub fn set_cid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Channel lock state. 1=locked, 0=idle."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Channel lock state. 1=locked, 0=idle."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Lock {
        #[inline(always)]
        fn default() -> Lock {
            Lock(0)
        }
    }
}
