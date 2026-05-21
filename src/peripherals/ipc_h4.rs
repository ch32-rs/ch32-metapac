#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Inter-Processor Communication. Four bidirectional channels, each carrying 8 status bits with independent enable/mask/set/clear, plus four 32-bit shared message slots for short messages."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc {
    ptr: *mut u8,
}
unsafe impl Send for Ipc {}
unsafe impl Sync for Ipc {}
impl Ipc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IPC control register. Per-channel TX/RX core ID routing, interrupt enables, auto-update mode and configuration lock."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Per-channel interrupt status (read-only summary; bits set when channel state matches the configured trigger)."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Channels, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Per-channel interrupt mask status (read-only; bit set means channel is currently raising an interrupt request to its routed core)."]
    #[inline(always)]
    pub const fn ism(self) -> crate::common::Reg<regs::Channels, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Per-channel status-bit interrupt enable. Each enabled bit produces TX or RX interrupts based on the value in STS."]
    #[inline(always)]
    pub const fn ena(self) -> crate::common::Reg<regs::Channels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Per-channel status register. Bit value 1 may raise an RX interrupt; bit value 0 may raise a TX interrupt (subject to ENA / IER configuration)."]
    #[inline(always)]
    pub const fn sts(self) -> crate::common::Reg<regs::Channels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Per-channel status set register. Writing 1 sets the corresponding bit in STS (and in ENA when AUTOEN is on)."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::Channels, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Per-channel status clear register. Writing 1 clears the corresponding bit in STS."]
    #[inline(always)]
    pub const fn clr(self) -> crate::common::Reg<regs::Channels, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "32-bit inline message slot. Four slots are available to carry short messages without touching shared SRAM."]
    #[inline(always)]
    pub const fn msg(self, n: usize) -> crate::common::Reg<regs::Msg, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Per-channel 8-bit value (used by ISR / ISM / ENA / STS / SET / CLR). Channel n occupies bits 8n..8n+7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Channels(pub u32);
    impl Channels {
        #[doc = "Channel byte. Each of the 8 bits is an independent status / enable / set / clear flag."]
        #[inline(always)]
        pub const fn ch(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "Channel byte. Each of the 8 bits is an independent status / enable / set / clear flag."]
        #[inline(always)]
        pub fn set_ch(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Channels {
        #[inline(always)]
        fn default() -> Channels {
            Channels(0)
        }
    }
    #[doc = "IPC control register. Each of the four channels occupies 8 bits (channel n at bits 8n..8n+7)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Sender core ID. 0=route TX interrupt to hart 0 (V3F), 1=route to hart 1 (V5F)."]
        #[inline(always)]
        pub const fn tx_cid(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Sender core ID. 0=route TX interrupt to hart 0 (V3F), 1=route to hart 1 (V5F)."]
        #[inline(always)]
        pub fn set_tx_cid(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Receiver core ID. 0=route RX interrupt to hart 0, 1=route to hart 1."]
        #[inline(always)]
        pub const fn rx_cid(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Receiver core ID. 0=route RX interrupt to hart 0, 1=route to hart 1."]
        #[inline(always)]
        pub fn set_rx_cid(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "TX interrupt enable."]
        #[inline(always)]
        pub const fn tx_ier(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TX interrupt enable."]
        #[inline(always)]
        pub fn set_tx_ier(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "RX interrupt enable."]
        #[inline(always)]
        pub const fn rx_ier(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 5usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "RX interrupt enable."]
        #[inline(always)]
        pub fn set_rx_ier(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 5usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Status-bit auto-update. When set, writing to STS / SET also updates ENA to the same value so the interrupt fires immediately."]
        #[inline(always)]
        pub const fn autoen(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Status-bit auto-update. When set, writing to STS / SET also updates ENA to the same value so the interrupt fires immediately."]
        #[inline(always)]
        pub fn set_autoen(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 6usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel configuration lock. Write 1 to freeze the channel's other control bits until reset."]
        #[inline(always)]
        pub const fn lock(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 7usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel configuration lock. Write 1 to freeze the channel's other control bits until reset."]
        #[inline(always)]
        pub fn set_lock(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 7usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "32-bit inline message register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msg(pub u32);
    impl Msg {
        #[doc = "32-bit message value."]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "32-bit message value."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Msg {
        #[inline(always)]
        fn default() -> Msg {
            Msg(0)
        }
    }
}
