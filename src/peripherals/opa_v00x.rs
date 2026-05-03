#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "OPA and CMP configuration."]
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
    #[doc = "OPA Configuration register 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "OPA Control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "OPA Configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "OPA Control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "OPA unlock key register."]
    #[inline(always)]
    pub const fn opa_key(self) -> crate::common::Reg<regs::OpaKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "CMP unlock key register."]
    #[inline(always)]
    pub const fn cmp_key(self) -> crate::common::Reg<regs::CmpKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "POLL lock key register."]
    #[inline(always)]
    pub const fn poll_key(self) -> crate::common::Reg<regs::PollKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "OPA Configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "OPA1 front-end polling enable."]
        #[inline(always)]
        pub const fn poll_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 front-end polling enable."]
        #[inline(always)]
        pub fn set_poll_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Number of OPA1 polling positive ends. 00 = 1 channel, 01 = 2 channels, 10 = 3 channels."]
        #[inline(always)]
        pub const fn poll1_num(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Number of OPA1 polling positive ends. 00 = 1 channel, 01 = 2 channels, 10 = 3 channels."]
        #[inline(always)]
        pub fn set_poll1_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "OPA1 reset system enable. When set, an OPA1 polling result of high will reset the system."]
        #[inline(always)]
        pub const fn rst_en1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 reset system enable. When set, an OPA1 polling result of high will reset the system."]
        #[inline(always)]
        pub fn set_rst_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OPA establishment time configuration. 00/10 = 0.5us, 01 = 0.312us, 11 = 0.77us."]
        #[inline(always)]
        pub const fn setup_cfg(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "OPA establishment time configuration. 00/10 = 0.5us, 01 = 0.312us, 11 = 0.77us."]
        #[inline(always)]
        pub fn set_setup_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "OPA polling automatic ADC trigger configuration."]
        #[inline(always)]
        pub const fn auto_adc_cfg(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "OPA polling automatic ADC trigger configuration."]
        #[inline(always)]
        pub fn set_auto_adc_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "OPA1 output interrupt enable."]
        #[inline(always)]
        pub const fn ie_out1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 output interrupt enable."]
        #[inline(always)]
        pub fn set_ie_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "OPA NMI interrupt enable."]
        #[inline(always)]
        pub const fn nmi_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OPA NMI interrupt enable."]
        #[inline(always)]
        pub fn set_nmi_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "1st polling channel OPA1 output high interrupt flag."]
        #[inline(always)]
        pub const fn if_out_poll_ch1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1st polling channel OPA1 output high interrupt flag."]
        #[inline(always)]
        pub fn set_if_out_poll_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "2nd polling channel OPA1 output high interrupt flag."]
        #[inline(always)]
        pub const fn if_out_poll_ch2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "2nd polling channel OPA1 output high interrupt flag."]
        #[inline(always)]
        pub fn set_if_out_poll_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "3rd polling channel OPA1 output high interrupt flag."]
        #[inline(always)]
        pub const fn if_out_poll_ch3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "3rd polling channel OPA1 output high interrupt flag."]
        #[inline(always)]
        pub fn set_if_out_poll_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OPA polling order, 1st polling channel."]
        #[inline(always)]
        pub const fn poll_ch1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "OPA polling order, 1st polling channel."]
        #[inline(always)]
        pub fn set_poll_ch1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "OPA polling order, 2nd polling channel."]
        #[inline(always)]
        pub const fn poll_ch2(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "OPA polling order, 2nd polling channel."]
        #[inline(always)]
        pub fn set_poll_ch2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "OPA polling order, 3rd polling channel."]
        #[inline(always)]
        pub const fn poll_ch3(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "OPA polling order, 3rd polling channel."]
        #[inline(always)]
        pub fn set_poll_ch3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "OPA polling software trigger. Set by software, cleared by hardware once polling starts."]
        #[inline(always)]
        pub const fn poll_swstrt(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OPA polling software trigger. Set by software, cleared by hardware once polling starts."]
        #[inline(always)]
        pub fn set_poll_swstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "OPA polling trigger event selection. 000 = software, 001 = TIM1_CH4, 010 = TIM2_CH4, 011 = TIM3_CH1, 100 = TIM3_CH2."]
        #[inline(always)]
        pub const fn poll_sel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x07;
            val as u8
        }
        #[doc = "OPA polling trigger event selection. 000 = software, 001 = TIM1_CH4, 010 = TIM2_CH4, 011 = TIM3_CH1, 100 = TIM3_CH2."]
        #[inline(always)]
        pub fn set_poll_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
        }
        #[doc = "POLL lock status. Cleared only by module reset."]
        #[inline(always)]
        pub const fn poll_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "POLL lock status. Cleared only by module reset."]
        #[inline(always)]
        pub fn set_poll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    #[doc = "OPA Configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "CMP1 front-end polling enable."]
        #[inline(always)]
        pub const fn poll_en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 front-end polling enable."]
        #[inline(always)]
        pub fn set_poll_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Number of CMP1 polling positive ends. 00 = 1, 01 = 2, 10 = 3."]
        #[inline(always)]
        pub const fn poll1_num(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Number of CMP1 polling positive ends. 00 = 1, 01 = 2, 10 = 3."]
        #[inline(always)]
        pub fn set_poll1_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "CMP1 reset system enable."]
        #[inline(always)]
        pub const fn rst_en1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 reset system enable."]
        #[inline(always)]
        pub fn set_rst_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CMP2 reset system enable."]
        #[inline(always)]
        pub const fn rst_en2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 reset system enable."]
        #[inline(always)]
        pub fn set_rst_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CMP1 interrupt enable."]
        #[inline(always)]
        pub const fn ie_out1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 interrupt enable."]
        #[inline(always)]
        pub fn set_ie_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CMP1 polling interval end interrupt enable."]
        #[inline(always)]
        pub const fn ie_cnt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 polling interval end interrupt enable."]
        #[inline(always)]
        pub fn set_ie_cnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1st CMP1 polling channel output-high flag."]
        #[inline(always)]
        pub const fn if_out_poll_ch1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1st CMP1 polling channel output-high flag."]
        #[inline(always)]
        pub fn set_if_out_poll_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "2nd CMP1 polling channel output-high flag."]
        #[inline(always)]
        pub const fn if_out_poll_ch2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "2nd CMP1 polling channel output-high flag."]
        #[inline(always)]
        pub fn set_if_out_poll_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "3rd CMP1 polling channel output-high flag."]
        #[inline(always)]
        pub const fn if_out_poll_ch3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "3rd CMP1 polling channel output-high flag."]
        #[inline(always)]
        pub fn set_if_out_poll_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "CMP1 polling interval end flag."]
        #[inline(always)]
        pub const fn if_cnt(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 polling interval end flag."]
        #[inline(always)]
        pub fn set_if_cnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CMP1 front-end polling interval. Polling interval = (POLL_VLU + 1) * 1us."]
        #[inline(always)]
        pub const fn poll_vlu(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "CMP1 front-end polling interval. Polling interval = (POLL_VLU + 1) * 1us."]
        #[inline(always)]
        pub fn set_poll_vlu(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "CMP1 polling order, 1st polling channel."]
        #[inline(always)]
        pub const fn poll_ch1(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "CMP1 polling order, 1st polling channel."]
        #[inline(always)]
        pub fn set_poll_ch1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "CMP1 polling order, 2nd polling channel."]
        #[inline(always)]
        pub const fn poll_ch2(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x03;
            val as u8
        }
        #[doc = "CMP1 polling order, 2nd polling channel."]
        #[inline(always)]
        pub fn set_poll_ch2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
        }
        #[doc = "CMP1 polling order, 3rd polling channel."]
        #[inline(always)]
        pub const fn poll_ch3(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "CMP1 polling order, 3rd polling channel."]
        #[inline(always)]
        pub fn set_poll_ch3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "CMP unlock key register. Write KEY1 = 0x45670123, then KEY2 = 0xCDEF89AB to unlock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpKey(pub u32);
    impl CmpKey {
        #[doc = "CMP unlock key."]
        #[inline(always)]
        pub const fn cmp_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CMP unlock key."]
        #[inline(always)]
        pub fn set_cmp_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmpKey {
        #[inline(always)]
        fn default() -> CmpKey {
            CmpKey(0)
        }
    }
    #[doc = "OPA Control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr1(pub u32);
    impl Ctlr1 {
        #[doc = "OPA1 enable."]
        #[inline(always)]
        pub const fn opa_en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 enable."]
        #[inline(always)]
        pub fn set_opa_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OPA1 output channel selection. 00 = PD4 plus internal CMP2 input, 01 = PA5 plus internal CMP2 input, 1x = internal CMP2 input only."]
        #[inline(always)]
        pub const fn mode1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 output channel selection. 00 = PD4 plus internal CMP2 input, 01 = PA5 plus internal CMP2 input, 1x = internal CMP2 input only."]
        #[inline(always)]
        pub fn set_mode1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "OPA1 positive (P) input channel selection. 00 = PA2, 01 = PD7, 10 = PD3, 11 = PD1."]
        #[inline(always)]
        pub const fn psel1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 positive (P) input channel selection. 00 = PA2, 01 = PD7, 10 = PD3, 11 = PD1."]
        #[inline(always)]
        pub fn set_psel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "OPA1 negative input channel and PGA gain selection. 000 = PA1, 001 = PD0, 011 = PGA gain 4, 100 = PGA gain 8, 101 = PGA gain 16, 110 = PGA gain 32."]
        #[inline(always)]
        pub const fn nsel1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "OPA1 negative input channel and PGA gain selection. 000 = PA1, 001 = PD0, 011 = PGA gain 4, 100 = PGA gain 8, 101 = PGA gain 16, 110 = PGA gain 32."]
        #[inline(always)]
        pub fn set_nsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "OPA1 internal feedback resistor enable. Must be set when NSEL1 is in PGA mode."]
        #[inline(always)]
        pub const fn fb_en1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 internal feedback resistor enable. Must be set when NSEL1 is in PGA mode."]
        #[inline(always)]
        pub fn set_fb_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Differential input PGA mode enable. The negative end is connected to OPA_CHN2 (PA4)."]
        #[inline(always)]
        pub const fn pgadif(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Differential input PGA mode enable. The negative end is connected to OPA_CHN2 (PA4)."]
        #[inline(always)]
        pub fn set_pgadif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PGA mode positive reference voltage enable."]
        #[inline(always)]
        pub const fn vben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PGA mode positive reference voltage enable."]
        #[inline(always)]
        pub fn set_vben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PGA mode positive reference voltage selection. 0 = VDD/2, 1 = VDD/4."]
        #[inline(always)]
        pub const fn vbsel(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PGA mode positive reference voltage selection. 0 = VDD/2, 1 = VDD/4."]
        #[inline(always)]
        pub fn set_vbsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "CMP2 negative reference voltage selection. Only valid when VBEN = 1. 11 = off."]
        #[inline(always)]
        pub const fn vbcmpsel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "CMP2 negative reference voltage selection. Only valid when VBEN = 1. 11 = off."]
        #[inline(always)]
        pub fn set_vbcmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "OPA1 high-speed mode enable. Increases the slew rate to 40V/us."]
        #[inline(always)]
        pub const fn opa_hs1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 high-speed mode enable. Increases the slew rate to 40V/us."]
        #[inline(always)]
        pub fn set_opa_hs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OPA lock. Write 1 to lock, write 0 has no effect."]
        #[inline(always)]
        pub const fn opa_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "OPA lock. Write 1 to lock, write 0 has no effect."]
        #[inline(always)]
        pub fn set_opa_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctlr1 {
        #[inline(always)]
        fn default() -> Ctlr1 {
            Ctlr1(0)
        }
    }
    #[doc = "OPA Control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "CMP1 enable."]
        #[inline(always)]
        pub const fn cmp_en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 enable."]
        #[inline(always)]
        pub fn set_cmp_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CMP1 output mode selection. 00 = output to PC0, 01 = TIM1_CH4 internal, 10 = TIM2_CH4 internal."]
        #[inline(always)]
        pub const fn mode1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "CMP1 output mode selection. 00 = output to PC0, 01 = TIM1_CH4 internal, 10 = TIM2_CH4 internal."]
        #[inline(always)]
        pub fn set_mode1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "CMP1 negative input channel selection. 00 = PC2, 01 = PD5, 10 = PA6, 11 = invalid."]
        #[inline(always)]
        pub const fn nsel1(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "CMP1 negative input channel selection. 00 = PC2, 01 = PD5, 10 = PA6, 11 = invalid."]
        #[inline(always)]
        pub fn set_nsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "CMP1 positive input channel selection. 00 = PC5, 01 = PB3, 10 = PD2, 11 = invalid."]
        #[inline(always)]
        pub const fn psel1(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "CMP1 positive input channel selection. 00 = PC5, 01 = PB3, 10 = PD2, 11 = invalid."]
        #[inline(always)]
        pub fn set_psel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "CMP1 hysteresis function enable (+/- 24mV)."]
        #[inline(always)]
        pub const fn hyen1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 hysteresis function enable (+/- 24mV)."]
        #[inline(always)]
        pub fn set_hyen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CMP1 positive input channel virtual center point enable."]
        #[inline(always)]
        pub const fn rmid1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 positive input channel virtual center point enable."]
        #[inline(always)]
        pub fn set_rmid1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CMP2 enable."]
        #[inline(always)]
        pub const fn cmp_en2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 enable."]
        #[inline(always)]
        pub fn set_cmp_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CMP digital filter enable."]
        #[inline(always)]
        pub const fn filt_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CMP digital filter enable."]
        #[inline(always)]
        pub fn set_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CMP output digital filter length selection. 0 = 0.33us, 1 = 0.5us."]
        #[inline(always)]
        pub const fn filt_sel(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CMP output digital filter length selection. 0 = 0.33us, 1 = 0.5us."]
        #[inline(always)]
        pub fn set_filt_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "TIM1 brake source configuration. 00 = IO, 01 = CMP1, 10 = CMP2, 11 = OPA."]
        #[inline(always)]
        pub const fn bkin_cfg(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "TIM1 brake source configuration. 00 = IO, 01 = CMP1, 10 = CMP2, 11 = OPA."]
        #[inline(always)]
        pub fn set_bkin_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "CMP lock. Write 1 to lock, write 0 has no effect."]
        #[inline(always)]
        pub const fn cmp_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CMP lock. Write 1 to lock, write 0 has no effect."]
        #[inline(always)]
        pub fn set_cmp_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctlr2 {
        #[inline(always)]
        fn default() -> Ctlr2 {
            Ctlr2(0)
        }
    }
    #[doc = "OPA unlock key register. Write KEY1 = 0x45670123, then KEY2 = 0xCDEF89AB to unlock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpaKey(pub u32);
    impl OpaKey {
        #[doc = "OPA unlock key."]
        #[inline(always)]
        pub const fn opa_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OPA unlock key."]
        #[inline(always)]
        pub fn set_opa_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpaKey {
        #[inline(always)]
        fn default() -> OpaKey {
            OpaKey(0)
        }
    }
    #[doc = "POLL lock key register. Write KEY1 = 0x45670123, then KEY2 = 0xCDEF89AB; once locked the module must be reset to unlock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PollKey(pub u32);
    impl PollKey {
        #[doc = "POLL lock key."]
        #[inline(always)]
        pub const fn poll_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "POLL lock key."]
        #[inline(always)]
        pub fn set_poll_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PollKey {
        #[inline(always)]
        fn default() -> PollKey {
            PollKey(0)
        }
    }
}
