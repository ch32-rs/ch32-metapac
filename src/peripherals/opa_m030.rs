#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "OPA_CMP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa {
    ptr: *mut u8,
}
unsafe impl Send for Opa {}
unsafe impl Sync for Opa {}
impl Opa {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "OPA_CMP_QII configuration register."]
    #[inline(always)]
    pub const fn qii_cfgr(self) -> crate::common::Reg<regs::QiiCfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ISP control register."]
    #[inline(always)]
    pub const fn isp_ctlr(self) -> crate::common::Reg<regs::IspCtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CMP1_2_3 control register."]
    #[inline(always)]
    pub const fn cmp_ctlr(self) -> crate::common::Reg<regs::CmpCtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CMP3 configur register."]
    #[inline(always)]
    pub const fn cmp3_cfgr(self) -> crate::common::Reg<regs::Cmp3Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CMP3 switch register."]
    #[inline(always)]
    pub const fn cmp3_sw(self) -> crate::common::Reg<regs::Cmp3Sw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CMP state register."]
    #[inline(always)]
    pub const fn cmp_statr(self) -> crate::common::Reg<regs::CmpStatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "CMP3 configur register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmp3Cfgr(pub u32);
    impl Cmp3Cfgr {
        #[doc = "CMP3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CMP3 positive channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "CMP3 positive channel selection."]
        #[inline(always)]
        pub const fn set_psel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "CMP3 nagetive channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "CMP3 nagetive channel selection."]
        #[inline(always)]
        pub const fn set_nsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "CMP3 hysteresis voltage selection."]
        #[must_use]
        #[inline(always)]
        pub const fn hys(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "CMP3 hysteresis voltage selection."]
        #[inline(always)]
        pub const fn set_hys(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "CMP3 virtual center point was enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rmid_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 virtual center point was enabled."]
        #[inline(always)]
        pub const fn set_rmid_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CMP3 module DAC output selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dacdat(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "CMP3 module DAC output selection."]
        #[inline(always)]
        pub const fn set_dacdat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "enable CMP3 analog output to IO."]
        #[must_use]
        #[inline(always)]
        pub const fn oen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "enable CMP3 analog output to IO."]
        #[inline(always)]
        pub const fn set_oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "IO output is enabled when CMP3 push-pull output is connected to IO."]
        #[must_use]
        #[inline(always)]
        pub const fn oe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "IO output is enabled when CMP3 push-pull output is connected to IO."]
        #[inline(always)]
        pub const fn set_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "pin selection when CMP3 push-pull output is connected to IO."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_iosel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "pin selection when CMP3 push-pull output is connected to IO."]
        #[inline(always)]
        pub const fn set_mode_iosel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cmp3Cfgr {
        #[inline(always)]
        fn default() -> Cmp3Cfgr {
            Cmp3Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cmp3Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmp3Cfgr")
                .field("en", &self.en())
                .field("psel", &self.psel())
                .field("nsel", &self.nsel())
                .field("hys", &self.hys())
                .field("rmid_en", &self.rmid_en())
                .field("dacdat", &self.dacdat())
                .field("oen", &self.oen())
                .field("oe", &self.oe())
                .field("mode_iosel", &self.mode_iosel())
                .finish()
        }
    }
    #[doc = "CMP3 switch register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmp3Sw(pub u32);
    impl Cmp3Sw {
        #[doc = "CMP3 channel number of TIM2 channels to switch."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CMP3 channel number of TIM2 channels to switch."]
        #[inline(always)]
        pub const fn set_ch_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "the first channel to compare the output. polarity configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn out_pol_ch1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "the first channel to compare the output. polarity configuration bit."]
        #[inline(always)]
        pub const fn set_out_pol_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "the comparison output. polarity configuration bit for the second channel."]
        #[must_use]
        #[inline(always)]
        pub const fn out_pol_ch2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "the comparison output. polarity configuration bit for the second channel."]
        #[inline(always)]
        pub const fn set_out_pol_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "the comparison output. polarity configuration bit for the third channel."]
        #[must_use]
        #[inline(always)]
        pub const fn out_pol_ch3(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "the comparison output. polarity configuration bit for the third channel."]
        #[inline(always)]
        pub const fn set_out_pol_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "the comparison output. polarity configuration bit for the channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn out_pol_ch4(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "the comparison output. polarity configuration bit for the channel 4."]
        #[inline(always)]
        pub const fn set_out_pol_ch4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Cmp3Sw {
        #[inline(always)]
        fn default() -> Cmp3Sw {
            Cmp3Sw(0)
        }
    }
    impl core::fmt::Debug for Cmp3Sw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmp3Sw")
                .field("ch_num", &self.ch_num())
                .field("out_pol_ch1", &self.out_pol_ch1())
                .field("out_pol_ch2", &self.out_pol_ch2())
                .field("out_pol_ch3", &self.out_pol_ch3())
                .field("out_pol_ch4", &self.out_pol_ch4())
                .finish()
        }
    }
    #[doc = "CMP1_2_3 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpCtlr(pub u32);
    impl CmpCtlr {
        #[doc = "CMP1 output digital filtering function was enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1_filt_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 output digital filtering function was enabled."]
        #[inline(always)]
        pub const fn set_cmp1_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CMP1 output digital filter length configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1_filt_cfg(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[doc = "CMP1 output digital filter length configuration."]
        #[inline(always)]
        pub const fn set_cmp1_filt_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "CMP2 output digital filtering function was enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2_filt_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 output digital filtering function was enabled."]
        #[inline(always)]
        pub const fn set_cmp2_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CMP2 output digital filter length configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2_filt_cfg(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x1f;
            val as u8
        }
        #[doc = "CMP2 output digital filter length configuration."]
        #[inline(always)]
        pub const fn set_cmp2_filt_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
        }
        #[doc = "Enable CMP2 output connect to timer 1 brake input."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2_bk_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable CMP2 output connect to timer 1 brake input."]
        #[inline(always)]
        pub const fn set_cmp2_bk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable CMP2 output interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2_int_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable CMP2 output interrupt."]
        #[inline(always)]
        pub const fn set_cmp2_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CMP3 hardware trigger source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_trg_src(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "CMP3 hardware trigger source selection."]
        #[inline(always)]
        pub const fn set_cmp3_trg_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "CMP3 hardware trigger channel switching."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_trg_gate(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 hardware trigger channel switching."]
        #[inline(always)]
        pub const fn set_cmp3_trg_gate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "The QII output is sent internally directly to the TIM3 capture enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn qii_cap_en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "The QII output is sent internally directly to the TIM3 capture enable bit."]
        #[inline(always)]
        pub const fn set_qii_cap_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "CMP3 continues comparison mode is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_com_mode(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 continues comparison mode is enabled."]
        #[inline(always)]
        pub const fn set_cmp3_com_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "CMP3 output digital filtering function was enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_filt_en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 output digital filtering function was enabled."]
        #[inline(always)]
        pub const fn set_cmp3_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CMP3 output digital filter length configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_filt_cfg(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[doc = "CMP3 output digital filter length configuration."]
        #[inline(always)]
        pub const fn set_cmp3_filt_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[doc = "Enable CMP3 output connect to timer 1 brake input."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_bk_en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Enable CMP3 output connect to timer 1 brake input."]
        #[inline(always)]
        pub const fn set_cmp3_bk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Enable CMP3 output interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_int_en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Enable CMP3 output interrupt."]
        #[inline(always)]
        pub const fn set_cmp3_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "The CMP2 push-pull output is internally connected to the IO function."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2_to_io(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "The CMP2 push-pull output is internally connected to the IO function."]
        #[inline(always)]
        pub const fn set_cmp2_to_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "The CMP1 push-pull output is internally connected to the IO function."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1_to_io(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "The CMP1 push-pull output is internally connected to the IO function."]
        #[inline(always)]
        pub const fn set_cmp1_to_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "CMP3 compares the output of the 4 channels internally connected to TIMER2."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_cap(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 compares the output of the 4 channels internally connected to TIMER2."]
        #[inline(always)]
        pub const fn set_cmp3_cap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for CmpCtlr {
        #[inline(always)]
        fn default() -> CmpCtlr {
            CmpCtlr(0)
        }
    }
    impl core::fmt::Debug for CmpCtlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmpCtlr")
                .field("cmp1_filt_en", &self.cmp1_filt_en())
                .field("cmp1_filt_cfg", &self.cmp1_filt_cfg())
                .field("cmp2_filt_en", &self.cmp2_filt_en())
                .field("cmp2_filt_cfg", &self.cmp2_filt_cfg())
                .field("cmp2_bk_en", &self.cmp2_bk_en())
                .field("cmp2_int_en", &self.cmp2_int_en())
                .field("cmp3_trg_src", &self.cmp3_trg_src())
                .field("cmp3_trg_gate", &self.cmp3_trg_gate())
                .field("qii_cap_en", &self.qii_cap_en())
                .field("cmp3_com_mode", &self.cmp3_com_mode())
                .field("cmp3_filt_en", &self.cmp3_filt_en())
                .field("cmp3_filt_cfg", &self.cmp3_filt_cfg())
                .field("cmp3_bk_en", &self.cmp3_bk_en())
                .field("cmp3_int_en", &self.cmp3_int_en())
                .field("cmp2_to_io", &self.cmp2_to_io())
                .field("cmp1_to_io", &self.cmp1_to_io())
                .field("cmp3_cap", &self.cmp3_cap())
                .finish()
        }
    }
    #[doc = "CMP state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpStatr(pub u32);
    impl CmpStatr {
        #[doc = "Turn off the output of CMP1 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1_out(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Turn off the output of CMP1 filter."]
        #[inline(always)]
        pub const fn set_cmp1_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Open the output of CMP1 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1_out_filt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Open the output of CMP1 filter."]
        #[inline(always)]
        pub const fn set_cmp1_out_filt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Turn off the output of CMP2 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2_out(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Turn off the output of CMP2 filter."]
        #[inline(always)]
        pub const fn set_cmp2_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Open the output of CMP2 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2_out_filt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Open the output of CMP2 filter."]
        #[inline(always)]
        pub const fn set_cmp2_out_filt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "The CMP2 output is a high level interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2_comif(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "The CMP2 output is a high level interrupt flag bit."]
        #[inline(always)]
        pub const fn set_cmp2_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Turn off the output of CMP3 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_out(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Turn off the output of CMP3 filter."]
        #[inline(always)]
        pub const fn set_cmp3_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Open the output of CMP3 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_out_filt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Open the output of CMP3 filter."]
        #[inline(always)]
        pub const fn set_cmp3_out_filt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "CMP3 channel switch the comparison output of the channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_chout0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 channel switch the comparison output of the channel 0."]
        #[inline(always)]
        pub const fn set_cmp3_chout0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "CMP3 channel switch the comparison output of the first channel."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_chout1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 channel switch the comparison output of the first channel."]
        #[inline(always)]
        pub const fn set_cmp3_chout1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "CMP3 channel switch the comparison output of the second channel."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_chout2(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 channel switch the comparison output of the second channel."]
        #[inline(always)]
        pub const fn set_cmp3_chout2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CMP3 channel switch the comparison output of the third channel."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3_chout3(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 channel switch the comparison output of the third channel."]
        #[inline(always)]
        pub const fn set_cmp3_chout3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for CmpStatr {
        #[inline(always)]
        fn default() -> CmpStatr {
            CmpStatr(0)
        }
    }
    impl core::fmt::Debug for CmpStatr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmpStatr")
                .field("cmp1_out", &self.cmp1_out())
                .field("cmp1_out_filt", &self.cmp1_out_filt())
                .field("cmp2_out", &self.cmp2_out())
                .field("cmp2_out_filt", &self.cmp2_out_filt())
                .field("cmp2_comif", &self.cmp2_comif())
                .field("cmp3_out", &self.cmp3_out())
                .field("cmp3_out_filt", &self.cmp3_out_filt())
                .field("cmp3_chout0", &self.cmp3_chout0())
                .field("cmp3_chout1", &self.cmp3_chout1())
                .field("cmp3_chout2", &self.cmp3_chout2())
                .field("cmp3_chout3", &self.cmp3_chout3())
                .finish()
        }
    }
    #[doc = "ISP control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IspCtlr(pub u32);
    impl IspCtlr {
        #[doc = "ISP1 module enable."]
        #[must_use]
        #[inline(always)]
        pub const fn isp1_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ISP1 module enable."]
        #[inline(always)]
        pub const fn set_isp1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ISP1_OPA gain selection."]
        #[must_use]
        #[inline(always)]
        pub const fn isp1_gain(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "ISP1_OPA gain selection."]
        #[inline(always)]
        pub const fn set_isp1_gain(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "ISP1 module reference voltage selection."]
        #[must_use]
        #[inline(always)]
        pub const fn isp1_vfbias(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ISP1 module reference voltage selection."]
        #[inline(always)]
        pub const fn set_isp1_vfbias(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Input selection at the negative end of the ISP1."]
        #[must_use]
        #[inline(always)]
        pub const fn isp1_nsel(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Input selection at the negative end of the ISP1."]
        #[inline(always)]
        pub const fn set_isp1_nsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "The switch ISP1 module outputs to the ADC."]
        #[must_use]
        #[inline(always)]
        pub const fn isp1_sel_io(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "The switch ISP1 module outputs to the ADC."]
        #[inline(always)]
        pub const fn set_isp1_sel_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ISP1 module Q value detection enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn qdet1_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ISP1 module Q value detection enabled."]
        #[inline(always)]
        pub const fn set_qdet1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "The Q value of the ISP1 module detects the 30K resistance enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn qdet1_pd30k(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The Q value of the ISP1 module detects the 30K resistance enabled."]
        #[inline(always)]
        pub const fn set_qdet1_pd30k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "The ISP1 module Q value detects the reference voltage enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn qdet1_vbsel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "The ISP1 module Q value detects the reference voltage enabled."]
        #[inline(always)]
        pub const fn set_qdet1_vbsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ISP2 module enable."]
        #[must_use]
        #[inline(always)]
        pub const fn isp2_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ISP2 module enable."]
        #[inline(always)]
        pub const fn set_isp2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ISP2_OPA gain selection."]
        #[must_use]
        #[inline(always)]
        pub const fn isp2_gain(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "ISP2_OPA gain selection."]
        #[inline(always)]
        pub const fn set_isp2_gain(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "ISP2 module reference voltage selection."]
        #[must_use]
        #[inline(always)]
        pub const fn isp2_vfbias(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ISP2 module reference voltage selection."]
        #[inline(always)]
        pub const fn set_isp2_vfbias(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Input selection at the negative end of the ISP2."]
        #[must_use]
        #[inline(always)]
        pub const fn isp2_nsel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Input selection at the negative end of the ISP2."]
        #[inline(always)]
        pub const fn set_isp2_nsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "The switch ISP2 module outputs to the ADC."]
        #[must_use]
        #[inline(always)]
        pub const fn isp2_sel_io(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "The switch ISP2 module outputs to the ADC."]
        #[inline(always)]
        pub const fn set_isp2_sel_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ISP2 module Q value detection enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn qdet2_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ISP2 module Q value detection enabled."]
        #[inline(always)]
        pub const fn set_qdet2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "The Q value of the ISP2 module detects the 30K resistance enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn qdet2_pd30k(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "The Q value of the ISP2 module detects the 30K resistance enabled."]
        #[inline(always)]
        pub const fn set_qdet2_pd30k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "The ISP2 module Q value detects the reference voltage enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn qdet2_vbsel(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "The ISP2 module Q value detects the reference voltage enabled."]
        #[inline(always)]
        pub const fn set_qdet2_vbsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IspCtlr {
        #[inline(always)]
        fn default() -> IspCtlr {
            IspCtlr(0)
        }
    }
    impl core::fmt::Debug for IspCtlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IspCtlr")
                .field("isp1_en", &self.isp1_en())
                .field("isp1_gain", &self.isp1_gain())
                .field("isp1_vfbias", &self.isp1_vfbias())
                .field("isp1_nsel", &self.isp1_nsel())
                .field("isp1_sel_io", &self.isp1_sel_io())
                .field("qdet1_en", &self.qdet1_en())
                .field("qdet1_pd30k", &self.qdet1_pd30k())
                .field("qdet1_vbsel", &self.qdet1_vbsel())
                .field("isp2_en", &self.isp2_en())
                .field("isp2_gain", &self.isp2_gain())
                .field("isp2_vfbias", &self.isp2_vfbias())
                .field("isp2_nsel", &self.isp2_nsel())
                .field("isp2_sel_io", &self.isp2_sel_io())
                .field("qdet2_en", &self.qdet2_en())
                .field("qdet2_pd30k", &self.qdet2_pd30k())
                .field("qdet2_vbsel", &self.qdet2_vbsel())
                .finish()
        }
    }
    #[doc = "OPA_CMP_QII configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QiiCfgr(pub u32);
    impl QiiCfgr {
        #[doc = "QII1_CMP enable."]
        #[must_use]
        #[inline(always)]
        pub const fn qii1_ae(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "QII1_CMP enable."]
        #[inline(always)]
        pub const fn set_qii1_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "QII1_CMP hysteresis selection."]
        #[must_use]
        #[inline(always)]
        pub const fn qii1_hypsel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "QII1_CMP hysteresis selection."]
        #[inline(always)]
        pub const fn set_qii1_hypsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "QII1_OPA gain selection."]
        #[must_use]
        #[inline(always)]
        pub const fn qii1_avsel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "QII1_OPA gain selection."]
        #[inline(always)]
        pub const fn set_qii1_avsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "QII2_CMP enable."]
        #[must_use]
        #[inline(always)]
        pub const fn qii2_ae(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "QII2_CMP enable."]
        #[inline(always)]
        pub const fn set_qii2_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "QII2_OPA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn qii2_opaen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "QII2_OPA enable."]
        #[inline(always)]
        pub const fn set_qii2_opaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "QII2 single end amplification mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn qii2_sesel(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "QII2 single end amplification mode selection."]
        #[inline(always)]
        pub const fn set_qii2_sesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "QII2_OPA gain selection."]
        #[must_use]
        #[inline(always)]
        pub const fn qii2_avsel(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "QII2_OPA gain selection."]
        #[inline(always)]
        pub const fn set_qii2_avsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "QII2_CMP hysteresis selection."]
        #[must_use]
        #[inline(always)]
        pub const fn qii2_hypsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "QII2_CMP hysteresis selection."]
        #[inline(always)]
        pub const fn set_qii2_hypsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "QII2_DAC enable."]
        #[must_use]
        #[inline(always)]
        pub const fn qii2_dacen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "QII2_DAC enable."]
        #[inline(always)]
        pub const fn set_qii2_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QII2_DAC output voltage selection."]
        #[must_use]
        #[inline(always)]
        pub const fn qii2_dac(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "QII2_DAC output voltage selection."]
        #[inline(always)]
        pub const fn set_qii2_dac(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "Input selection at the negative end of the QII2_CMP."]
        #[must_use]
        #[inline(always)]
        pub const fn qii2_chnsel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Input selection at the negative end of the QII2_CMP."]
        #[inline(always)]
        pub const fn set_qii2_chnsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Input selection at the positive end of the QII2_CMP."]
        #[must_use]
        #[inline(always)]
        pub const fn qii2_chpsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Input selection at the positive end of the QII2_CMP."]
        #[inline(always)]
        pub const fn set_qii2_chpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for QiiCfgr {
        #[inline(always)]
        fn default() -> QiiCfgr {
            QiiCfgr(0)
        }
    }
    impl core::fmt::Debug for QiiCfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QiiCfgr")
                .field("qii1_ae", &self.qii1_ae())
                .field("qii1_hypsel", &self.qii1_hypsel())
                .field("qii1_avsel", &self.qii1_avsel())
                .field("qii2_ae", &self.qii2_ae())
                .field("qii2_opaen", &self.qii2_opaen())
                .field("qii2_sesel", &self.qii2_sesel())
                .field("qii2_avsel", &self.qii2_avsel())
                .field("qii2_hypsel", &self.qii2_hypsel())
                .field("qii2_dacen", &self.qii2_dacen())
                .field("qii2_dac", &self.qii2_dac())
                .field("qii2_chnsel", &self.qii2_chnsel())
                .field("qii2_chpsel", &self.qii2_chpsel())
                .finish()
        }
    }
}
