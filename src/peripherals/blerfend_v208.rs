#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BLE RF Frontend — analog config (PLL/VCO/filter/bias) + calibration tables."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRfend {
    ptr: *mut u8,
}
unsafe impl Send for BleRfend {}
unsafe impl Sync for BleRfend {}
impl BleRfend {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Calibration trigger/path bits (TX_tune_trigger bit0, TX_cal_mode bit4, TXF_enable bit8, RX_filter_mode bit12, RX_ADC_config bit16)."]
    #[inline(always)]
    pub const fn cal_trig(self) -> crate::common::Reg<regs::CalTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "TX/RX path enable (RX_ADC bit16, TX cal bit17, TX/RX PLL pre-enable bits\\[20:21\\])."]
    #[inline(always)]
    pub const fn path_en(self) -> crate::common::Reg<regs::PathEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RFEND control + reset (reset sequence writes 0x1101 → 0 → 0x1101; RX_filter_strobe bit4, ADC_ref_strobe bit8)."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RFEND CFG0 (default 0x480, hw-confirmed)."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "PLL/VCO config (post_cal_enable bit4, channel-lock release bit1)."]
    #[inline(always)]
    pub const fn pll_vco(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "PLL loop filter config (bits\\[3:0\\]/\\[7:4\\]/\\[10:8\\]/\\[22:20\\]/\\[30:28\\])."]
    #[inline(always)]
    pub const fn loop_filter(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "PLL enable (bits\\[25:19\\]
cleared, bit20 set, bit31 set)."]
    #[inline(always)]
    pub const fn cfg4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "CFG5 + frequency code (bits\\[15:12\\]=8, bits\\[26:24\\], bits\\[31:30\\]; freq_code at bits\\[15:8\\]
BF00=2401MHz, D300=2440MHz, E700=2480MHz; nCO2440 bits\\[5:0\\], nGA2440 bits\\[30:24\\])."]
    #[inline(always)]
    pub const fn cfg5_freq(self) -> crate::common::Reg<regs::Cfg5Freq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "PLL integer/frac divider (int_div bits\\[24:20\\], frac_div bits\\[13:0\\])."]
    #[inline(always)]
    pub const fn pll_div(self) -> crate::common::Reg<regs::PllDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Analog RF0 config (RF analog enable bit31, bits\\[3:0\\]=9, bits\\[26:24\\]=0b100, bit29)."]
    #[inline(always)]
    pub const fn rf0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Analog RF1 bias (bits\\[2:0\\]=3, bits\\[6:4\\]=3, bits\\[10:8\\]=3, bit24=0, bit25=1)."]
    #[inline(always)]
    pub const fn rf1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Analog RF2 (bit14 = 1)."]
    #[inline(always)]
    pub const fn rf2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Analog RF2b (bits\\[3:0\\]=12, bit7=1, bit12=0)."]
    #[inline(always)]
    pub const fn rf2b(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RX ADC reference enable (bit16)."]
    #[inline(always)]
    pub const fn adc_ref(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "PLL tune result (CO bits\\[5:0\\], tune_done bit25, tune_active bit26). Reading also latches GA into CO_RESULT2."]
    #[inline(always)]
    pub const fn tune_result(self) -> crate::common::Reg<regs::TuneResult, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "GA result (bits\\[16:10\\], latched by TUNE_RESULT read)."]
    #[inline(always)]
    pub const fn ga_result(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "RX filter calibration result (bits\\[4:0\\]) + done bit8."]
    #[inline(always)]
    pub const fn rx_filter_result(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "CO calibration table 1 — nibble-packed `delta_low * (39-ch)/39` for ch0..ch39 (5×u32)."]
    #[inline(always)]
    pub const fn co_table1(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "CO calibration table 2 — nibble-packed `delta_high * (ch+1)/40` for ch0..ch39 (5×u32)."]
    #[inline(always)]
    pub const fn co_table2(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize + n * 4usize) as _) }
    }
    #[doc = "GA calibration table + CO2 overflow guard (3×u32 covering nibbles for ch40..41 + GA1\\[0..9\\]
+ GA2\\[0..9\\])."]
    #[inline(always)]
    pub const fn ga_table(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "RFEND calibration trigger/path bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalTrig(pub u32);
    impl CalTrig {
        #[doc = "TX tune trigger pulse for PLL measurement."]
        #[inline(always)]
        pub const fn tx_tune_trigger(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX tune trigger pulse for PLL measurement."]
        #[inline(always)]
        pub fn set_tx_tune_trigger(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX calibration mode (set during cal, cleared post)."]
        #[inline(always)]
        pub const fn tx_cal_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX calibration mode (set during cal, cleared post)."]
        #[inline(always)]
        pub fn set_tx_cal_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TX filter enable (set in RFEND_TXFtune; cleared post-cal Bug"]
        #[inline(always)]
        pub const fn txf_enable(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TX filter enable (set in RFEND_TXFtune; cleared post-cal Bug"]
        #[inline(always)]
        pub fn set_txf_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RX filter calibration mode (cleared post Bug"]
        #[inline(always)]
        pub const fn rx_filter_mode(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RX filter calibration mode (cleared post Bug"]
        #[inline(always)]
        pub fn set_rx_filter_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RX ADC config (cleared then set in RFEND_RXAdc)."]
        #[inline(always)]
        pub const fn rx_adc_config(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RX ADC config (cleared then set in RFEND_RXAdc)."]
        #[inline(always)]
        pub fn set_rx_adc_config(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for CalTrig {
        #[inline(always)]
        fn default() -> CalTrig {
            CalTrig(0)
        }
    }
    #[doc = "CFG5 + frequency-code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg5Freq(pub u32);
    impl Cfg5Freq {
        #[doc = "Stored CO anchor for default-channel comp (bits\\[5:0\\])."]
        #[inline(always)]
        pub const fn nco2440(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Stored CO anchor for default-channel comp (bits\\[5:0\\])."]
        #[inline(always)]
        pub fn set_nco2440(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Frequency code bits\\[15:8\\]
(BF=2401MHz, D3=2440MHz, E7=2480MHz)."]
        #[inline(always)]
        pub const fn freq_code(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Frequency code bits\\[15:8\\]
(BF=2401MHz, D3=2440MHz, E7=2480MHz)."]
        #[inline(always)]
        pub fn set_freq_code(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Stored GA anchor for default-channel comp (bits\\[30:24\\])."]
        #[inline(always)]
        pub const fn nga2440(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Stored GA anchor for default-channel comp (bits\\[30:24\\])."]
        #[inline(always)]
        pub fn set_nga2440(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Cfg5Freq {
        #[inline(always)]
        fn default() -> Cfg5Freq {
            Cfg5Freq(0)
        }
    }
    #[doc = "RFEND TX/RX path enable mask (pre-init = 0x00330000 enables bits 16/17/20/21)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PathEn(pub u32);
    impl PathEn {
        #[doc = "RX ADC path enable."]
        #[inline(always)]
        pub const fn rx_adc_path(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RX ADC path enable."]
        #[inline(always)]
        pub fn set_rx_adc_path(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TX calibration path enable."]
        #[inline(always)]
        pub const fn tx_cal_path(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TX calibration path enable."]
        #[inline(always)]
        pub fn set_tx_cal_path(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TX PLL pre-enable."]
        #[inline(always)]
        pub const fn tx_pll_pre(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TX PLL pre-enable."]
        #[inline(always)]
        pub fn set_tx_pll_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RX filter path enable."]
        #[inline(always)]
        pub const fn rx_filter_path(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RX filter path enable."]
        #[inline(always)]
        pub fn set_rx_filter_path(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for PathEn {
        #[inline(always)]
        fn default() -> PathEn {
            PathEn(0)
        }
    }
    #[doc = "PLL channel divider (programmed by listener.set_channel_freq)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PllDiv(pub u32);
    impl PllDiv {
        #[doc = "Fractional divider (`((freq_khz % 64000) << 10) / 250` masked to 14 bits)."]
        #[inline(always)]
        pub const fn frac_div(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Fractional divider (`((freq_khz % 64000) << 10) / 250` masked to 14 bits)."]
        #[inline(always)]
        pub fn set_frac_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "Integer divider (`freq_khz / 64000` masked to 5 bits)."]
        #[inline(always)]
        pub const fn int_div(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[doc = "Integer divider (`freq_khz / 64000` masked to 5 bits)."]
        #[inline(always)]
        pub fn set_int_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
    }
    impl Default for PllDiv {
        #[inline(always)]
        fn default() -> PllDiv {
            PllDiv(0)
        }
    }
    #[doc = "PLL tune result (CO, tune_done, tune_active)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TuneResult(pub u32);
    impl TuneResult {
        #[doc = "CO calibration result (read after PLL lock)."]
        #[inline(always)]
        pub const fn co(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "CO calibration result (read after PLL lock)."]
        #[inline(always)]
        pub fn set_co(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Tune done flag — second-check by RFEND_WaitTune."]
        #[inline(always)]
        pub const fn tune_done(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Tune done flag — second-check by RFEND_WaitTune."]
        #[inline(always)]
        pub fn set_tune_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Tune active flag — set first; double-check semantic."]
        #[inline(always)]
        pub const fn tune_active(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Tune active flag — set first; double-check semantic."]
        #[inline(always)]
        pub fn set_tune_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for TuneResult {
        #[inline(always)]
        fn default() -> TuneResult {
            TuneResult(0)
        }
    }
}
