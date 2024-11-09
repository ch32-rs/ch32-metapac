#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Digital Video Port."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvp {
    ptr: *mut u8,
}
unsafe impl Send for Dvp {}
unsafe impl Sync for Dvp {}
impl Dvp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Digital Video control register. (DVP_CR0)."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Digital Video control register. (DVP_CR1)."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "Digital Video Interrupt register. (DVP_IER)."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "Image line count configuration register. (DVP_ROW_NUM)."]
    #[inline(always)]
    pub const fn row_num(self) -> crate::common::Reg<regs::RowNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Image column number configuration register. (DVP_COL_NUM)."]
    #[inline(always)]
    pub const fn col_num(self) -> crate::common::Reg<regs::ColNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "Digital Video DMA address register. (DVP_DMA_BUF0)."]
    #[inline(always)]
    pub const fn dma_buf0(self) -> crate::common::Reg<regs::DmaBuf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Digital Video DMA address register. (DVP_DMA_BUF1)."]
    #[inline(always)]
    pub const fn dma_buf1(self) -> crate::common::Reg<regs::DmaBuf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Digital Video Flag register. (DVP_IFR)."]
    #[inline(always)]
    pub const fn ifr(self) -> crate::common::Reg<regs::Ifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Digital Video STATUS register. (DVP_STATUS)."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x11usize) as _) }
    }
    #[doc = "Digital Video line counter register. (DVP_ROW_CNT)."]
    #[inline(always)]
    pub const fn row_cnt(self) -> crate::common::Reg<regs::RowCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Digital Video horizontal displacement register. (DVP_HOFFCNT)."]
    #[inline(always)]
    pub const fn hoffcnt(self) -> crate::common::Reg<regs::Hoffcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Digital Video line number register. (DVP_VST)."]
    #[inline(always)]
    pub const fn vst(self) -> crate::common::Reg<regs::Vst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[doc = "Digital Video Capture count register. (DVP_CAPCNT)."]
    #[inline(always)]
    pub const fn capcnt(self) -> crate::common::Reg<regs::Capcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Digital Video Vertical line count register. (DVP_VLINE)."]
    #[inline(always)]
    pub const fn vline(self) -> crate::common::Reg<regs::Vline, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
    }
    #[doc = "Digital Video Data register. (DVP_DR)."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Digital Video Capture count register. (DVP_CAPCNT)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Capcnt(pub u16);
    impl Capcnt {
        #[doc = "Number of PCLK cycles captured by clipping window."]
        #[inline(always)]
        pub const fn capcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of PCLK cycles captured by clipping window."]
        #[inline(always)]
        pub fn set_capcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Capcnt {
        #[inline(always)]
        fn default() -> Capcnt {
            Capcnt(0)
        }
    }
    #[doc = "Image column number configuration register. (DVP_COL_NUM)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ColNum(pub u16);
    impl ColNum {
        #[doc = "Number of PCLK cycles for row data."]
        #[inline(always)]
        pub const fn col_num(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of PCLK cycles for row data."]
        #[inline(always)]
        pub fn set_col_num(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for ColNum {
        #[inline(always)]
        fn default() -> ColNum {
            ColNum(0)
        }
    }
    #[doc = "Digital Video control register. (DVP_CR0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u8);
    impl Cr0 {
        #[doc = "DVP enable."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DVP enable."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DVP VSYNC polarity control."]
        #[inline(always)]
        pub const fn v_polar(&self) -> super::vals::VsyncPolarity {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::VsyncPolarity::from_bits(val as u8)
        }
        #[doc = "DVP VSYNC polarity control."]
        #[inline(always)]
        pub fn set_v_polar(&mut self, val: super::vals::VsyncPolarity) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
        }
        #[doc = "DVP HSYNC polarity control."]
        #[inline(always)]
        pub const fn h_polar(&self) -> super::vals::HsyncPolarity {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::HsyncPolarity::from_bits(val as u8)
        }
        #[doc = "DVP HSYNC polarity control."]
        #[inline(always)]
        pub fn set_h_polar(&mut self, val: super::vals::HsyncPolarity) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
        }
        #[doc = "DVP PCLK polarity control."]
        #[inline(always)]
        pub const fn p_polar(&self) -> super::vals::PclkPolarity {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::PclkPolarity::from_bits(val as u8)
        }
        #[doc = "DVP PCLK polarity control."]
        #[inline(always)]
        pub fn set_p_polar(&mut self, val: super::vals::PclkPolarity) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
        }
        #[doc = "DVP data mode."]
        #[inline(always)]
        pub const fn msk_dat_mod(&self) -> super::vals::DataMode {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::DataMode::from_bits(val as u8)
        }
        #[doc = "DVP data mode."]
        #[inline(always)]
        pub fn set_msk_dat_mod(&mut self, val: super::vals::DataMode) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
        }
        #[doc = "DVP JPEG mode."]
        #[inline(always)]
        pub const fn jpeg(&self) -> super::vals::CaptureMode {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::CaptureMode::from_bits(val as u8)
        }
        #[doc = "DVP JPEG mode."]
        #[inline(always)]
        pub fn set_jpeg(&mut self, val: super::vals::CaptureMode) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Cr0 {
        #[inline(always)]
        fn default() -> Cr0 {
            Cr0(0)
        }
    }
    #[doc = "Digital Video control register. (DVP_CR1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u8);
    impl Cr1 {
        #[doc = "DVP dma enable."]
        #[inline(always)]
        pub const fn dma_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DVP dma enable."]
        #[inline(always)]
        pub fn set_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DVP all clear."]
        #[inline(always)]
        pub const fn all_clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DVP all clear."]
        #[inline(always)]
        pub fn set_all_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "DVP receive logic clear."]
        #[inline(always)]
        pub const fn rcv_clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DVP receive logic clear."]
        #[inline(always)]
        pub fn set_rcv_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "DVP bug toggle by software."]
        #[inline(always)]
        pub const fn buf_tog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DVP bug toggle by software."]
        #[inline(always)]
        pub fn set_buf_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DVP capture mode."]
        #[inline(always)]
        pub const fn cm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DVP capture mode."]
        #[inline(always)]
        pub fn set_cm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "DVP Crop feature enable."]
        #[inline(always)]
        pub const fn crop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DVP Crop feature enable."]
        #[inline(always)]
        pub fn set_crop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "DVP frame capture rate control."]
        #[inline(always)]
        pub const fn fcrc(&self) -> super::vals::CaptureRate {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::CaptureRate::from_bits(val as u8)
        }
        #[doc = "DVP frame capture rate control."]
        #[inline(always)]
        pub fn set_fcrc(&mut self, val: super::vals::CaptureRate) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    #[doc = "Digital Video DMA address register. (DVP_DMA_BUF0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBuf0(pub u32);
    impl DmaBuf0 {
        #[doc = "DMA receive address 0."]
        #[inline(always)]
        pub const fn dma_buf0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "DMA receive address 0."]
        #[inline(always)]
        pub fn set_dma_buf0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for DmaBuf0 {
        #[inline(always)]
        fn default() -> DmaBuf0 {
            DmaBuf0(0)
        }
    }
    #[doc = "Digital Video DMA address register. (DVP_DMA_BUF1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBuf1(pub u32);
    impl DmaBuf1 {
        #[doc = "DMA receive address 1."]
        #[inline(always)]
        pub const fn dma_buf1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "DMA receive address 1."]
        #[inline(always)]
        pub fn set_dma_buf1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for DmaBuf1 {
        #[inline(always)]
        fn default() -> DmaBuf1 {
            DmaBuf1(0)
        }
    }
    #[doc = "Digital Video Data register. (DVP_DR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Prevent DMA overflow."]
        #[inline(always)]
        pub const fn dr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Prevent DMA overflow."]
        #[inline(always)]
        pub fn set_dr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    #[doc = "Digital Video horizontal displacement register. (DVP_HOFFCNT)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hoffcnt(pub u16);
    impl Hoffcnt {
        #[doc = "Number of PCLK cycles for row data."]
        #[inline(always)]
        pub const fn hoffcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of PCLK cycles for row data."]
        #[inline(always)]
        pub fn set_hoffcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Hoffcnt {
        #[inline(always)]
        fn default() -> Hoffcnt {
            Hoffcnt(0)
        }
    }
    #[doc = "Digital Video Interrupt register. (DVP_IER)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u8);
    impl Ier {
        #[doc = "DVP frame start interrupt enable."]
        #[inline(always)]
        pub const fn ie_str_frm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DVP frame start interrupt enable."]
        #[inline(always)]
        pub fn set_ie_str_frm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DVP row received done interrupt enable."]
        #[inline(always)]
        pub const fn ie_row_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DVP row received done interrupt enable."]
        #[inline(always)]
        pub fn set_ie_row_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "DVP frame received done interrupt enable."]
        #[inline(always)]
        pub const fn ie_frm_done(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DVP frame received done interrupt enable."]
        #[inline(always)]
        pub fn set_ie_frm_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "DVP receive fifo overflow interrupt enable."]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DVP receive fifo overflow interrupt enable."]
        #[inline(always)]
        pub fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DVP frame stop interrupt enable."]
        #[inline(always)]
        pub const fn ie_stp_frm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DVP frame stop interrupt enable."]
        #[inline(always)]
        pub fn set_ie_stp_frm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "Digital Video Flag register. (DVP_IFR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifr(pub u8);
    impl Ifr {
        #[doc = "DVP frame start interrupt enable."]
        #[inline(always)]
        pub const fn if_str_frm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DVP frame start interrupt enable."]
        #[inline(always)]
        pub fn set_if_str_frm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DVP row received done interrupt enable."]
        #[inline(always)]
        pub const fn if_row_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DVP row received done interrupt enable."]
        #[inline(always)]
        pub fn set_if_row_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "DVP frame received done interrupt enable."]
        #[inline(always)]
        pub const fn if_frm_done(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DVP frame received done interrupt enable."]
        #[inline(always)]
        pub fn set_if_frm_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "DVP receive fifo overflow interrupt enable."]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DVP receive fifo overflow interrupt enable."]
        #[inline(always)]
        pub fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DVP frame stop interrupt enable."]
        #[inline(always)]
        pub const fn if_stp_frm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DVP frame stop interrupt enable."]
        #[inline(always)]
        pub fn set_if_stp_frm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Ifr {
        #[inline(always)]
        fn default() -> Ifr {
            Ifr(0)
        }
    }
    #[doc = "Digital Video line counter register. (DVP_ROW_CNT)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RowCnt(pub u16);
    impl RowCnt {
        #[doc = "The number of rows of frame image data."]
        #[inline(always)]
        pub const fn row_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The number of rows of frame image data."]
        #[inline(always)]
        pub fn set_row_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for RowCnt {
        #[inline(always)]
        fn default() -> RowCnt {
            RowCnt(0)
        }
    }
    #[doc = "Image line count configuration register. (DVP_ROW_NUM)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RowNum(pub u16);
    impl RowNum {
        #[doc = "The number of rows of frame image data."]
        #[inline(always)]
        pub const fn row_num(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The number of rows of frame image data."]
        #[inline(always)]
        pub fn set_row_num(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for RowNum {
        #[inline(always)]
        fn default() -> RowNum {
            RowNum(0)
        }
    }
    #[doc = "Digital Video STATUS register. (DVP_STATUS)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u8);
    impl Status {
        #[doc = "DVP frame start interrupt enable."]
        #[inline(always)]
        pub const fn fifo_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DVP frame start interrupt enable."]
        #[inline(always)]
        pub fn set_fifo_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DVP row received done interrupt enable."]
        #[inline(always)]
        pub const fn fifo_full(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DVP row received done interrupt enable."]
        #[inline(always)]
        pub fn set_fifo_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "DVP frame received done interrupt enable."]
        #[inline(always)]
        pub const fn fifo_ov(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DVP frame received done interrupt enable."]
        #[inline(always)]
        pub fn set_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "DVP receive fifo overflow interrupt enable."]
        #[inline(always)]
        pub const fn msk_fifo_cnt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "DVP receive fifo overflow interrupt enable."]
        #[inline(always)]
        pub fn set_msk_fifo_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    #[doc = "Digital Video Vertical line count register. (DVP_VLINE)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vline(pub u16);
    impl Vline {
        #[doc = "Crop the number of rows captured by window."]
        #[inline(always)]
        pub const fn vline(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Crop the number of rows captured by window."]
        #[inline(always)]
        pub fn set_vline(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Vline {
        #[inline(always)]
        fn default() -> Vline {
            Vline(0)
        }
    }
    #[doc = "Digital Video line number register. (DVP_VST)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vst(pub u16);
    impl Vst {
        #[doc = "The number of lines captured by the image."]
        #[inline(always)]
        pub const fn vst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The number of lines captured by the image."]
        #[inline(always)]
        pub fn set_vst(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Vst {
        #[inline(always)]
        fn default() -> Vst {
            Vst(0)
        }
    }
}
pub mod vals {
    #[doc = "Capture mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CaptureMode {
        #[doc = "Raw data capture mode."]
        RAW = 0x0,
        #[doc = "JPEG data capture mode."]
        JPEG = 0x01,
    }
    impl CaptureMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CaptureMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CaptureMode {
        #[inline(always)]
        fn from(val: u8) -> CaptureMode {
            CaptureMode::from_bits(val)
        }
    }
    impl From<CaptureMode> for u8 {
        #[inline(always)]
        fn from(val: CaptureMode) -> u8 {
            CaptureMode::to_bits(val)
        }
    }
    #[doc = "Capture rate"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CaptureRate {
        #[doc = "Capture all frames."]
        ALL = 0x0,
        #[doc = "One frame per Two Captures."]
        HALF = 0x01,
        #[doc = "One frame per Four Captures."]
        QUARTER = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CaptureRate {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CaptureRate {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CaptureRate {
        #[inline(always)]
        fn from(val: u8) -> CaptureRate {
            CaptureRate::from_bits(val)
        }
    }
    impl From<CaptureRate> for u8 {
        #[inline(always)]
        fn from(val: CaptureRate) -> u8 {
            CaptureRate::to_bits(val)
        }
    }
    #[doc = "Data mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum DataMode {
        #[doc = "8-bit data mode."]
        _8BIT = 0x0,
        #[doc = "10-bit data mode."]
        _10BIT = 0x01,
        #[doc = "12-bit data mode."]
        _12BIT = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl DataMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DataMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DataMode {
        #[inline(always)]
        fn from(val: u8) -> DataMode {
            DataMode::from_bits(val)
        }
    }
    impl From<DataMode> for u8 {
        #[inline(always)]
        fn from(val: DataMode) -> u8 {
            DataMode::to_bits(val)
        }
    }
    #[doc = "HSYNC polarity control."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum HsyncPolarity {
        #[doc = "HSYNC active high."]
        ACTIVE_HIGH = 0x0,
        #[doc = "HSYNC active low."]
        ACTIVE_LOW = 0x01,
    }
    impl HsyncPolarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HsyncPolarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HsyncPolarity {
        #[inline(always)]
        fn from(val: u8) -> HsyncPolarity {
            HsyncPolarity::from_bits(val)
        }
    }
    impl From<HsyncPolarity> for u8 {
        #[inline(always)]
        fn from(val: HsyncPolarity) -> u8 {
            HsyncPolarity::to_bits(val)
        }
    }
    #[doc = "PCLK polarity control."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PclkPolarity {
        #[doc = "PCLK rising edge."]
        RISING_EDGE = 0x0,
        #[doc = "PCLK falling edge."]
        FALLING_EDGE = 0x01,
    }
    impl PclkPolarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PclkPolarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PclkPolarity {
        #[inline(always)]
        fn from(val: u8) -> PclkPolarity {
            PclkPolarity::from_bits(val)
        }
    }
    impl From<PclkPolarity> for u8 {
        #[inline(always)]
        fn from(val: PclkPolarity) -> u8 {
            PclkPolarity::to_bits(val)
        }
    }
    #[doc = "VSYNC polarity control."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum VsyncPolarity {
        #[doc = "VSYNC active low."]
        ACTIVE_LOW = 0x0,
        #[doc = "VSYNC active high."]
        ACTIVE_HIGH = 0x01,
    }
    impl VsyncPolarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VsyncPolarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VsyncPolarity {
        #[inline(always)]
        fn from(val: u8) -> VsyncPolarity {
            VsyncPolarity::from_bits(val)
        }
    }
    impl From<VsyncPolarity> for u8 {
        #[inline(always)]
        fn from(val: VsyncPolarity) -> u8 {
            VsyncPolarity::to_bits(val)
        }
    }
}
