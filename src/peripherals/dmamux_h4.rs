#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DMA request multiplexer. Routes peripheral DMA request lines to DMA1 channels 1-8 (DMAMUX channels 1-8) and DMA2 channels 1-8 (DMAMUX channels 9-16)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmamux {
    ptr: *mut u8,
}
unsafe impl Send for Dmamux {}
unsafe impl Sync for Dmamux {}
impl Dmamux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA request multiplexer channel configuration register. Each register holds 4 channels (7-bit MUX selector per channel)."]
    #[inline(always)]
    pub const fn cfgr(self, n: usize) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "DMAMUX channel configuration. Four 7-bit MUX selectors packed into a 32-bit register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "DMA request multiplexer input source for the channel (peripheral request ID, see RM table for assignments)."]
        #[must_use]
        #[inline(always)]
        pub const fn channel_mux(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x7f;
            val as u8
        }
        #[doc = "DMA request multiplexer input source for the channel (peripheral request ID, see RM table for assignments)."]
        #[inline(always)]
        pub const fn set_channel_mux(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x7f << offs)) | (((val as u32) & 0x7f) << offs);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("channel_mux[0]", &self.channel_mux(0usize))
                .field("channel_mux[1]", &self.channel_mux(1usize))
                .field("channel_mux[2]", &self.channel_mux(2usize))
                .field("channel_mux[3]", &self.channel_mux(3usize))
                .finish()
        }
    }
}
