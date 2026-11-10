#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Low-power timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptim {
    ptr: *mut u8,
}
unsafe impl Send for Lptim {}
unsafe impl Sync for Lptim {}
impl Lptim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "compare register."]
    #[inline(always)]
    pub const fn cmp(self) -> crate::common::Reg<regs::Cmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "aoto-reload register."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "COUNT register."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "aoto-reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr(pub u32);
    impl Arr {
        #[doc = "aoto-reload count value."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "aoto-reload count value."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Arr {
        #[inline(always)]
        fn default() -> Arr {
            Arr(0)
        }
    }
    impl core::fmt::Debug for Arr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Arr").field("arr", &self.arr()).finish()
        }
    }
    #[doc = "configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "effective edge configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn cksel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "effective edge configuration."]
        #[inline(always)]
        pub const fn set_cksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "configure effective edges."]
        #[must_use]
        #[inline(always)]
        pub const fn ckpol(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "configure effective edges."]
        #[inline(always)]
        pub const fn set_ckpol(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "digital filter for ex-clock."]
        #[must_use]
        #[inline(always)]
        pub const fn ckflt(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "digital filter for ex-clock."]
        #[inline(always)]
        pub const fn set_ckflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "digital filter for flip-flops."]
        #[must_use]
        #[inline(always)]
        pub const fn trgflt(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "digital filter for flip-flops."]
        #[inline(always)]
        pub const fn set_trgflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "prescaler configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn presc(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "prescaler configuration."]
        #[inline(always)]
        pub const fn set_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "trigger source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn trigsel(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "trigger source selection."]
        #[inline(always)]
        pub const fn set_trigsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "trigger configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn trigen(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "trigger configuration."]
        #[inline(always)]
        pub const fn set_trigen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "TIMEOUT control."]
        #[must_use]
        #[inline(always)]
        pub const fn timout(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIMEOUT control."]
        #[inline(always)]
        pub const fn set_timout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PWM."]
        #[must_use]
        #[inline(always)]
        pub const fn wave(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PWM."]
        #[inline(always)]
        pub const fn set_wave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PWM polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn wavpol(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PWM polarity."]
        #[inline(always)]
        pub const fn set_wavpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "update mode control."]
        #[must_use]
        #[inline(always)]
        pub const fn preload(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "update mode control."]
        #[inline(always)]
        pub const fn set_preload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "count clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn countmode(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "count clock selection."]
        #[inline(always)]
        pub const fn set_countmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "coder mode."]
        #[must_use]
        #[inline(always)]
        pub const fn enc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "coder mode."]
        #[inline(always)]
        pub const fn set_enc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn clkmx_sel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "clock selection."]
        #[inline(always)]
        pub const fn set_clkmx_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "Forcing pwm output."]
        #[must_use]
        #[inline(always)]
        pub const fn force_pwm(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Forcing pwm output."]
        #[inline(always)]
        pub const fn set_force_pwm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("cksel", &self.cksel())
                .field("ckpol", &self.ckpol())
                .field("ckflt", &self.ckflt())
                .field("trgflt", &self.trgflt())
                .field("presc", &self.presc())
                .field("trigsel", &self.trigsel())
                .field("trigen", &self.trigen())
                .field("timout", &self.timout())
                .field("wave", &self.wave())
                .field("wavpol", &self.wavpol())
                .field("preload", &self.preload())
                .field("countmode", &self.countmode())
                .field("enc", &self.enc())
                .field("clkmx_sel", &self.clkmx_sel())
                .field("force_pwm", &self.force_pwm())
                .finish()
        }
    }
    #[doc = "compare register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmp(pub u32);
    impl Cmp {
        #[doc = "compare value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "compare value."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cmp {
        #[inline(always)]
        fn default() -> Cmp {
            Cmp(0)
        }
    }
    impl core::fmt::Debug for Cmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmp").field("cmp", &self.cmp()).finish()
        }
    }
    #[doc = "COUNT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "Timer count value."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timer count value."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt").field("count", &self.count()).finish()
        }
    }
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "timer enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "timer enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "start in one trigger mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sngstrt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "start in one trigger mode."]
        #[inline(always)]
        pub const fn set_sngstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "start in continuous mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cntstrt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "start in continuous mode."]
        #[inline(always)]
        pub const fn set_cntstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "pwm output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn outen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "pwm output enable."]
        #[inline(always)]
        pub const fn set_outen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "externally trigger count direction enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dir_exten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "externally trigger count direction enable."]
        #[inline(always)]
        pub const fn set_dir_exten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("enable", &self.enable())
                .field("sngstrt", &self.sngstrt())
                .field("cntstrt", &self.cntstrt())
                .field("outen", &self.outen())
                .field("dir_exten", &self.dir_exten())
                .finish()
        }
    }
    #[doc = "interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "clear compare register match flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpmcf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "clear compare register match flag."]
        #[inline(always)]
        pub const fn set_cmpmcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "clear Aoto-reload register match flag."]
        #[must_use]
        #[inline(always)]
        pub const fn arrmcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clear Aoto-reload register match flag."]
        #[inline(always)]
        pub const fn set_arrmcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "clear Edge event are triggerd externally flag."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrigcf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear Edge event are triggerd externally flag."]
        #[inline(always)]
        pub const fn set_exttrigcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "clear compare register data update flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpokcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "clear compare register data update flag."]
        #[inline(always)]
        pub const fn set_cmpokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "clear Aoto-reload register data update flag."]
        #[must_use]
        #[inline(always)]
        pub const fn arrokcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "clear Aoto-reload register data update flag."]
        #[inline(always)]
        pub const fn set_arrokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "clear up flag."]
        #[must_use]
        #[inline(always)]
        pub const fn upcf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "clear up flag."]
        #[inline(always)]
        pub const fn set_upcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "clear down flag."]
        #[must_use]
        #[inline(always)]
        pub const fn downcf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "clear down flag."]
        #[inline(always)]
        pub const fn set_downcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("cmpmcf", &self.cmpmcf())
                .field("arrmcf", &self.arrmcf())
                .field("exttrigcf", &self.exttrigcf())
                .field("cmpokcf", &self.cmpokcf())
                .field("arrokcf", &self.arrokcf())
                .field("upcf", &self.upcf())
                .field("downcf", &self.downcf())
                .finish()
        }
    }
    #[doc = "interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "compare register match successfully interrupts enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpmie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "compare register match successfully interrupts enable."]
        #[inline(always)]
        pub const fn set_cmpmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Aoto-reload register match successfully interrupts enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arrmie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Aoto-reload register match successfully interrupts enable."]
        #[inline(always)]
        pub const fn set_arrmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Edge event are triggerd externally successfully interrupts enable."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrigie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Edge event are triggerd externally successfully interrupts enable."]
        #[inline(always)]
        pub const fn set_exttrigie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "compare register data update successfully interrupts enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpokie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "compare register data update successfully interrupts enable."]
        #[inline(always)]
        pub const fn set_cmpokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Aoto-reload register data update successfully interrupts enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arrokie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Aoto-reload register data update successfully interrupts enable."]
        #[inline(always)]
        pub const fn set_arrokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "UP interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn upie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UP interrupt enable."]
        #[inline(always)]
        pub const fn set_upie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Down interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn downie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Down interrupt enable."]
        #[inline(always)]
        pub const fn set_downie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("cmpmie", &self.cmpmie())
                .field("arrmie", &self.arrmie())
                .field("exttrigie", &self.exttrigie())
                .field("cmpokie", &self.cmpokie())
                .field("arrokie", &self.arrokie())
                .field("upie", &self.upie())
                .field("downie", &self.downie())
                .finish()
        }
    }
    #[doc = "interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "DATA of compare register and LPTIM_CNT match."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DATA of compare register and LPTIM_CNT match."]
        #[inline(always)]
        pub const fn set_cmpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DATA of Aoto-reload register and LPTIM_CNT match."]
        #[must_use]
        #[inline(always)]
        pub const fn arrm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DATA of Aoto-reload register and LPTIM_CNT match."]
        #[inline(always)]
        pub const fn set_arrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Edge event are triggerd externally."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Edge event are triggerd externally."]
        #[inline(always)]
        pub const fn set_exttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "compare register data update successfully."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpok(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "compare register data update successfully."]
        #[inline(always)]
        pub const fn set_cmpok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Aoto-reload register data update successfully."]
        #[must_use]
        #[inline(always)]
        pub const fn arrok(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Aoto-reload register data update successfully."]
        #[inline(always)]
        pub const fn set_arrok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "count-up."]
        #[must_use]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "count-up."]
        #[inline(always)]
        pub const fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "count down."]
        #[must_use]
        #[inline(always)]
        pub const fn down(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "count down."]
        #[inline(always)]
        pub const fn set_down(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "direction of count."]
        #[must_use]
        #[inline(always)]
        pub const fn dir_sync(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "direction of count."]
        #[inline(always)]
        pub const fn set_dir_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("cmpm", &self.cmpm())
                .field("arrm", &self.arrm())
                .field("exttrig", &self.exttrig())
                .field("cmpok", &self.cmpok())
                .field("arrok", &self.arrok())
                .field("up", &self.up())
                .field("down", &self.down())
                .field("dir_sync", &self.dir_sync())
                .finish()
        }
    }
}
