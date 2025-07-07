#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB register. USBHS, host/device interface, for V305, and V307"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB base control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn dev_ad(self) -> crate::common::Reg<regs::DevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "FRAME_NO."]
    #[inline(always)]
    pub const fn frame_no(self) -> crate::common::Reg<regs::FrameNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "indicate USB suspend status."]
    #[inline(always)]
    pub const fn suspend(self) -> crate::common::Reg<regs::Suspend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB current speed type."]
    #[inline(always)]
    pub const fn speed_type(self) -> crate::common::Reg<regs::SpeedType, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::MisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::IntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::IntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn rx_len(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "USB device endpoint register block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbd {
    ptr: *mut u8,
}
unsafe impl Send for Usbd {}
unsafe impl Sync for Usbd {}
impl Usbd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB base control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn dev_ad(self) -> crate::common::Reg<regs::DevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "FRAME_NO."]
    #[inline(always)]
    pub const fn frame_no(self) -> crate::common::Reg<regs::FrameNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "indicate USB suspend status."]
    #[inline(always)]
    pub const fn suspend(self) -> crate::common::Reg<regs::Suspend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB current speed type."]
    #[inline(always)]
    pub const fn speed_type(self) -> crate::common::Reg<regs::SpeedType, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::MisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::IntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::IntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn rx_len(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "USB endpoint configuration."]
    #[inline(always)]
    pub const fn ep_config(self) -> crate::common::Reg<regs::EpConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Endpoint type configuration register"]
    #[inline(always)]
    pub const fn ep_type(self) -> crate::common::Reg<regs::EpType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "USB endpoint buffer mode."]
    #[inline(always)]
    pub const fn ep_buf_mod(self) -> crate::common::Reg<regs::EpBufMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "endpoint 0 DMA buffer address."]
    #[inline(always)]
    pub const fn ep0_dma(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "endpoint n (n=1-15) DMA RX buffer address."]
    #[inline(always)]
    pub const fn ep_rx_dma(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "endpoint n (n=1-15) DMA TX buffer address."]
    #[inline(always)]
    pub const fn ep_tx_dma(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize + n * 4usize) as _) }
    }
    #[doc = "endpoint n (n=0-15) max acceptable length."]
    #[inline(always)]
    pub const fn ep_max_len(self, n: usize) -> crate::common::Reg<regs::EpLen, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize + n * 4usize) as _) }
    }
    #[doc = "endpoint n (n=0-15) send length."]
    #[inline(always)]
    pub const fn ep_t_len(self, n: usize) -> crate::common::Reg<regs::EpLen, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize + n * 4usize) as _) }
    }
    #[doc = "endpoint n (n=0-15) send control."]
    #[inline(always)]
    pub const fn ep_tx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::EpTxCtrl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdausize + n * 4usize) as _) }
    }
    #[doc = "endpoint n (n=0-15) receive control."]
    #[inline(always)]
    pub const fn ep_rx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::EpRxCtrl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdbusize + n * 4usize) as _) }
    }
}
#[doc = "USB host register block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbh {
    ptr: *mut u8,
}
unsafe impl Send for Usbh {}
unsafe impl Sync for Usbh {}
impl Usbh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB HOST control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::UhCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn dev_ad(self) -> crate::common::Reg<regs::DevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "FRAME_NO."]
    #[inline(always)]
    pub const fn frame_no(self) -> crate::common::Reg<regs::FrameNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "indicate USB suspend status."]
    #[inline(always)]
    pub const fn suspend(self) -> crate::common::Reg<regs::Suspend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB current speed type."]
    #[inline(always)]
    pub const fn speed_type(self) -> crate::common::Reg<regs::SpeedType, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::MisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::IntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::IntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn rx_len(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "USB endpoint configuration."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::UhConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "USB endpoint type."]
    #[inline(always)]
    pub const fn ep_type(self) -> crate::common::Reg<regs::UhEpType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "USB host receive buffer start address"]
    #[inline(always)]
    pub const fn rx_dma(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "USB host transmit buffer start address"]
    #[inline(always)]
    pub const fn tx_dma(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "USB host receive maximum length packet register"]
    #[inline(always)]
    pub const fn rx_max_len(self) -> crate::common::Reg<regs::EpLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "USB host token setup register"]
    #[inline(always)]
    pub const fn ep_pid(self) -> crate::common::Reg<regs::UhEpPid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "USB host receive control register"]
    #[inline(always)]
    pub const fn rx_ctrl(self) -> crate::common::Reg<regs::UhRxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe3usize) as _) }
    }
    #[doc = "USB host transmit length register"]
    #[inline(always)]
    pub const fn tx_len(self) -> crate::common::Reg<regs::EpLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "USB host transmit control register"]
    #[inline(always)]
    pub const fn tx_ctrl(self) -> crate::common::Reg<regs::UhTxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe6usize) as _) }
    }
    #[doc = "USB host transmit data of the SPLIT packet"]
    #[inline(always)]
    pub const fn split_data(self) -> crate::common::Reg<regs::UhSplitData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
}
pub mod regs {
    #[doc = "USB base control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u8);
    impl Ctrl {
        #[doc = "DMA enable and DMA interrupt enable for USB."]
        #[inline(always)]
        pub const fn dma_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable and DMA interrupt enable for USB."]
        #[inline(always)]
        pub fn set_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "force clear FIFO and count of USB."]
        #[inline(always)]
        pub const fn clr_all(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear FIFO and count of USB."]
        #[inline(always)]
        pub fn set_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "force reset USB SIE, need software clear."]
        #[inline(always)]
        pub const fn reset_sie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "force reset USB SIE, need software clear."]
        #[inline(always)]
        pub fn set_reset_sie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid."]
        #[inline(always)]
        pub const fn int_busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid."]
        #[inline(always)]
        pub fn set_int_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub const fn dev_pu_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub fn set_dev_pu_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB low speed: 00=full speed, 01=high speed, 10 =low speed."]
        #[inline(always)]
        pub const fn speed_type(&self) -> super::vals::SpeedType {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::SpeedType::from_bits(val as u8)
        }
        #[doc = "enable USB low speed: 00=full speed, 01=high speed, 10 =low speed."]
        #[inline(always)]
        pub fn set_speed_type(&mut self, val: super::vals::SpeedType) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u8) & 0x03) << 5usize);
        }
        #[doc = "enable USB host mode: 0=device mode, 1=host mode."]
        #[inline(always)]
        pub const fn host_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB host mode: 0=device mode, 1=host mode."]
        #[inline(always)]
        pub fn set_host_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "USB device address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DevAd(pub u8);
    impl DevAd {
        #[doc = "USB device address."]
        #[inline(always)]
        pub const fn addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "USB device address."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for DevAd {
        #[inline(always)]
        fn default() -> DevAd {
            DevAd(0)
        }
    }
    #[doc = "USB endpoint buffer mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpBufMod(pub u32);
    impl EpBufMod {
        #[doc = "buffer mode of USB endpoint."]
        #[inline(always)]
        pub const fn buf_mod(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint."]
        #[inline(always)]
        pub fn set_buf_mod(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "buffer mode of USB endpoint."]
        #[inline(always)]
        pub const fn iso_buf_mod(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint."]
        #[inline(always)]
        pub fn set_iso_buf_mod(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for EpBufMod {
        #[inline(always)]
        fn default() -> EpBufMod {
            EpBufMod(0)
        }
    }
    #[doc = "USB endpoint configuration register (R32_UEP_CONFIG)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpConfig(pub u32);
    impl EpConfig {
        #[doc = "Endpoint 1 to 15 transmit enable"]
        #[inline(always)]
        pub const fn t_en(&self, n: usize) -> bool {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 1 to 15 transmit enable"]
        #[inline(always)]
        pub fn set_t_en(&mut self, n: usize, val: bool) {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Endpoint 1 to 15 receive enable"]
        #[inline(always)]
        pub const fn r_en(&self, n: usize) -> bool {
            assert!(n < 15usize);
            let offs = 17usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 1 to 15 receive enable"]
        #[inline(always)]
        pub fn set_r_en(&mut self, n: usize, val: bool) {
            assert!(n < 15usize);
            let offs = 17usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for EpConfig {
        #[inline(always)]
        fn default() -> EpConfig {
            EpConfig(0)
        }
    }
    #[doc = "endpoint n acceptable length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpLen(pub u16);
    impl EpLen {
        #[doc = "endpoint n acceptable length."]
        #[inline(always)]
        pub const fn len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "endpoint n acceptable length."]
        #[inline(always)]
        pub fn set_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for EpLen {
        #[inline(always)]
        fn default() -> EpLen {
            EpLen(0)
        }
    }
    #[doc = "endpoint n receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpRxCtrl(pub u8);
    impl EpRxCtrl {
        #[doc = "MASK_UEP_R_RES"]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> super::vals::EpRxResponse {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::EpRxResponse::from_bits(val as u8)
        }
        #[doc = "MASK_UEP_R_RES"]
        #[inline(always)]
        pub fn set_mask_uep_r_res(&mut self, val: super::vals::EpRxResponse) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
        }
        #[doc = "MASK_UEP_R_TOG"]
        #[inline(always)]
        pub const fn mask_uep_r_tog(&self) -> super::vals::EpTog {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::EpTog::from_bits(val as u8)
        }
        #[doc = "MASK_UEP_R_TOG"]
        #[inline(always)]
        pub fn set_mask_uep_r_tog(&mut self, val: super::vals::EpTog) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u8) & 0x03) << 3usize);
        }
        #[doc = "endpoint n synchronous trigger bit automatic filp enables the control bit."]
        #[inline(always)]
        pub const fn r_tog_auto(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint n synchronous trigger bit automatic filp enables the control bit."]
        #[inline(always)]
        pub fn set_r_tog_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for EpRxCtrl {
        #[inline(always)]
        fn default() -> EpRxCtrl {
            EpRxCtrl(0)
        }
    }
    #[doc = "endpoint n send control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpTxCtrl(pub u8);
    impl EpTxCtrl {
        #[doc = "MASK_UEP_T_RES"]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> super::vals::EpTxResponse {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::EpTxResponse::from_bits(val as u8)
        }
        #[doc = "MASK_UEP_T_RES"]
        #[inline(always)]
        pub fn set_mask_uep_t_res(&mut self, val: super::vals::EpTxResponse) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
        }
        #[doc = "MASK_UEP_T_TOG"]
        #[inline(always)]
        pub const fn mask_uep_t_tog(&self) -> super::vals::EpTog {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::EpTog::from_bits(val as u8)
        }
        #[doc = "MASK_UEP_T_TOG"]
        #[inline(always)]
        pub fn set_mask_uep_t_tog(&mut self, val: super::vals::EpTog) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u8) & 0x03) << 3usize);
        }
        #[doc = "endpoint n synchronous trigger bit automatic filp enables the control bit."]
        #[inline(always)]
        pub const fn t_tog_auto(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint n synchronous trigger bit automatic filp enables the control bit."]
        #[inline(always)]
        pub fn set_t_tog_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for EpTxCtrl {
        #[inline(always)]
        fn default() -> EpTxCtrl {
            EpTxCtrl(0)
        }
    }
    #[doc = "USB endpoint type control register (R32_UEP_TYPE)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpType(pub u32);
    impl EpType {
        #[doc = "Endpoint 1 to 15 transmit type, 1 means synchronous transmission"]
        #[inline(always)]
        pub const fn t_type(&self, n: usize) -> super::vals::EndpointType {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::EndpointType::from_bits(val as u8)
        }
        #[doc = "Endpoint 1 to 15 transmit type, 1 means synchronous transmission"]
        #[inline(always)]
        pub fn set_t_type(&mut self, n: usize, val: super::vals::EndpointType) {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Endpoint 1 to 15 receive type, 1 means synchronous transmission"]
        #[inline(always)]
        pub const fn r_type(&self, n: usize) -> super::vals::EndpointType {
            assert!(n < 15usize);
            let offs = 17usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::EndpointType::from_bits(val as u8)
        }
        #[doc = "Endpoint 1 to 15 receive type, 1 means synchronous transmission"]
        #[inline(always)]
        pub fn set_r_type(&mut self, n: usize, val: super::vals::EndpointType) {
            assert!(n < 15usize);
            let offs = 17usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for EpType {
        #[inline(always)]
        fn default() -> EpType {
            EpType(0)
        }
    }
    #[doc = "FRAME_NO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FrameNo(pub u16);
    impl FrameNo {
        #[doc = "FRAME_NO."]
        #[inline(always)]
        pub const fn frame_no(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "FRAME_NO."]
        #[inline(always)]
        pub fn set_frame_no(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for FrameNo {
        #[inline(always)]
        fn default() -> FrameNo {
            FrameNo(0)
        }
    }
    #[doc = "USB interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u8);
    impl IntEn {
        #[doc = "enable interrupt for USB bus reset event for USB device mode."]
        #[inline(always)]
        pub const fn bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB bus reset event for USB device mode."]
        #[inline(always)]
        pub fn set_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for USB device detected event for USB host mode."]
        #[inline(always)]
        pub const fn detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB device detected event for USB host mode."]
        #[inline(always)]
        pub fn set_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for USB transfer completion."]
        #[inline(always)]
        pub const fn transfer(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB transfer completion."]
        #[inline(always)]
        pub fn set_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable interrupt for USB suspend or resume event."]
        #[inline(always)]
        pub const fn suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB suspend or resume event."]
        #[inline(always)]
        pub fn set_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "indicate host SOF timer action status for USB host."]
        #[inline(always)]
        pub const fn sof_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "indicate host SOF timer action status for USB host."]
        #[inline(always)]
        pub fn set_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable interrupt for FIFO overflow."]
        #[inline(always)]
        pub const fn fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for FIFO overflow."]
        #[inline(always)]
        pub fn set_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "indicate host SETUP timer action status for USB host."]
        #[inline(always)]
        pub const fn setup_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "indicate host SETUP timer action status for USB host."]
        #[inline(always)]
        pub fn set_setup_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub const fn iso_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub fn set_iso_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub const fn dev_nak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub fn set_dev_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "USB interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFg(pub u8);
    impl IntFg {
        #[doc = "in USB device mode, USB bus reset event interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub const fn bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "in USB device mode, USB bus reset event interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub fn set_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "in USB host mode, USB device connect or disconnect event interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub const fn detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "in USB host mode, USB device connect or disconnect event interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub fn set_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB transfer completion interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn transfer(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer completion interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn hst_sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_hst_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "SETUP transaction completion interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub const fn setup_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SETUP transaction completion interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub fn set_setup_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "isochronous transmission starts to send/receive data interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub const fn iso_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "isochronous transmission starts to send/receive data interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub fn set_iso_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate current USB transfer is NAK received."]
        #[inline(always)]
        pub const fn is_nak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer is NAK received."]
        #[inline(always)]
        pub fn set_is_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntFg {
        #[inline(always)]
        fn default() -> IntFg {
            IntFg(0)
        }
    }
    #[doc = "USB interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSt(pub u8);
    impl IntSt {
        #[doc = "RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub const fn endp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub fn set_endp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received."]
        #[inline(always)]
        pub const fn h_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received."]
        #[inline(always)]
        pub fn set_h_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[inline(always)]
        pub const fn token(&self) -> super::vals::UsbToken {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::UsbToken::from_bits(val as u8)
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[inline(always)]
        pub fn set_token(&mut self, val: super::vals::UsbToken) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[inline(always)]
        pub const fn tog_ok(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[inline(always)]
        pub fn set_tog_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate current USB transfer is NAK received for USB device mode."]
        #[inline(always)]
        pub const fn is_nak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer is NAK received for USB device mode."]
        #[inline(always)]
        pub fn set_is_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntSt {
        #[inline(always)]
        fn default() -> IntSt {
            IntSt(0)
        }
    }
    #[doc = "USB miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MisSt(pub u8);
    impl MisSt {
        #[doc = "RO, in USB host mode, SPLIT packet transmit enable."]
        #[inline(always)]
        pub const fn split_can(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, in USB host mode, SPLIT packet transmit enable."]
        #[inline(always)]
        pub fn set_split_can(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, USB device attach status for the port in USB host mode."]
        #[inline(always)]
        pub const fn dev_attach(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, USB device attach status for the port in USB host mode."]
        #[inline(always)]
        pub fn set_dev_attach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RO, indicate USB suspend status."]
        #[inline(always)]
        pub const fn suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB suspend status."]
        #[inline(always)]
        pub fn set_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RO, indicate USB bus reset status."]
        #[inline(always)]
        pub const fn bus_reset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB bus reset status."]
        #[inline(always)]
        pub fn set_bus_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RO, indicate USB receiving FIFO ready status (not empty)."]
        #[inline(always)]
        pub const fn r_fifo_rdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB receiving FIFO ready status (not empty)."]
        #[inline(always)]
        pub fn set_r_fifo_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[inline(always)]
        pub const fn sie_free(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[inline(always)]
        pub fn set_sie_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, indicate host SOF timer action status for USB host."]
        #[inline(always)]
        pub const fn sof_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate host SOF timer action status for USB host."]
        #[inline(always)]
        pub fn set_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate host SOF timer presage status."]
        #[inline(always)]
        pub const fn sof_pres(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate host SOF timer presage status."]
        #[inline(always)]
        pub fn set_sof_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for MisSt {
        #[inline(always)]
        fn default() -> MisSt {
            MisSt(0)
        }
    }
    #[doc = "USB current speed type."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpeedType(pub u8);
    impl SpeedType {
        #[doc = "in host mode, it indicates the speed type of the currently connected device; in device mode, it indicates the speed type of the current device."]
        #[inline(always)]
        pub const fn speed_type(&self) -> super::vals::SpeedType {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::SpeedType::from_bits(val as u8)
        }
        #[doc = "in host mode, it indicates the speed type of the currently connected device; in device mode, it indicates the speed type of the current device."]
        #[inline(always)]
        pub fn set_speed_type(&mut self, val: super::vals::SpeedType) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
        }
    }
    impl Default for SpeedType {
        #[inline(always)]
        fn default() -> SpeedType {
            SpeedType(0)
        }
    }
    #[doc = "indicate USB suspend status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Suspend(pub u8);
    impl Suspend {
        #[doc = "SYS_MOD."]
        #[inline(always)]
        pub const fn sys_mod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SYS_MOD."]
        #[inline(always)]
        pub fn set_sys_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "remote resume."]
        #[inline(always)]
        pub const fn wakeup(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "remote resume."]
        #[inline(always)]
        pub fn set_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LINESTATE."]
        #[inline(always)]
        pub const fn linestate(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "LINESTATE."]
        #[inline(always)]
        pub fn set_linestate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
    }
    impl Default for Suspend {
        #[inline(always)]
        fn default() -> Suspend {
            Suspend(0)
        }
    }
    #[doc = "USB endpoint configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhConfig(pub u32);
    impl UhConfig {
        #[doc = "host TX enable."]
        #[inline(always)]
        pub const fn h_tx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "host TX enable."]
        #[inline(always)]
        pub fn set_h_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "host RX enable."]
        #[inline(always)]
        pub const fn h_rx_en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "host RX enable."]
        #[inline(always)]
        pub fn set_h_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for UhConfig {
        #[inline(always)]
        fn default() -> UhConfig {
            UhConfig(0)
        }
    }
    #[doc = "USB HOST control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhCtrl(pub u8);
    impl UhCtrl {
        #[doc = "USB host bus reset status."]
        #[inline(always)]
        pub const fn tx_bus_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB host bus reset status."]
        #[inline(always)]
        pub fn set_tx_bus_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "the host sends hang sigal."]
        #[inline(always)]
        pub const fn tx_bus_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the host sends hang sigal."]
        #[inline(always)]
        pub fn set_tx_bus_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "host wake up device."]
        #[inline(always)]
        pub const fn tx_bus_resume(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "host wake up device."]
        #[inline(always)]
        pub fn set_tx_bus_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "the remoke wake-up."]
        #[inline(always)]
        pub const fn remote_wkup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "the remoke wake-up."]
        #[inline(always)]
        pub fn set_remote_wkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB-PHY thesuspended state the internal USB-PLL is turned off."]
        #[inline(always)]
        pub const fn phy_suspendm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB-PHY thesuspended state the internal USB-PLL is turned off."]
        #[inline(always)]
        pub fn set_phy_suspendm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "the bus is idle."]
        #[inline(always)]
        pub const fn sof_free(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "the bus is idle."]
        #[inline(always)]
        pub fn set_sof_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "automatically generate the SOF packet enabling control bit."]
        #[inline(always)]
        pub const fn sof_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "automatically generate the SOF packet enabling control bit."]
        #[inline(always)]
        pub fn set_sof_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhCtrl {
        #[inline(always)]
        fn default() -> UhCtrl {
            UhCtrl(0)
        }
    }
    #[doc = "host token setup register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhEpPid(pub u16);
    impl UhEpPid {
        #[doc = "set the endpoint number of the target device."]
        #[inline(always)]
        pub const fn endp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "set the endpoint number of the target device."]
        #[inline(always)]
        pub fn set_endp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "set the token PID of this usb transaction."]
        #[inline(always)]
        pub const fn token(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "set the token PID of this usb transaction."]
        #[inline(always)]
        pub fn set_token(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
    }
    impl Default for UhEpPid {
        #[inline(always)]
        fn default() -> UhEpPid {
            UhEpPid(0)
        }
    }
    #[doc = "USB endpoint type."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhEpType(pub u32);
    impl UhEpType {
        #[doc = "host TX type."]
        #[inline(always)]
        pub const fn h_tx_type(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "host TX type."]
        #[inline(always)]
        pub fn set_h_tx_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "host RX type."]
        #[inline(always)]
        pub const fn h_rx_type(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "host RX type."]
        #[inline(always)]
        pub fn set_h_rx_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for UhEpType {
        #[inline(always)]
        fn default() -> UhEpType {
            UhEpType(0)
        }
    }
    #[doc = "USB host receive control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxCtrl(pub u8);
    impl UhRxCtrl {
        #[doc = "host control of the accept response to IN transactions."]
        #[inline(always)]
        pub const fn r_res(&self) -> super::vals::HostRxResponse {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::HostRxResponse::from_bits(val as u8)
        }
        #[doc = "host control of the accept response to IN transactions."]
        #[inline(always)]
        pub fn set_r_res(&mut self, val: super::vals::HostRxResponse) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
        }
        #[doc = "H_R_RES_NO"]
        #[inline(always)]
        pub const fn r_res_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "H_R_RES_NO"]
        #[inline(always)]
        pub fn set_r_res_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "host synchronous trigger bit for the accept to prepare."]
        #[inline(always)]
        pub const fn r_tog(&self) -> super::vals::Tog {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Tog::from_bits(val as u8)
        }
        #[doc = "host synchronous trigger bit for the accept to prepare."]
        #[inline(always)]
        pub fn set_r_tog(&mut self, val: super::vals::Tog) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u8) & 0x03) << 3usize);
        }
        #[doc = "host synchronization trigger bit auto toggle enable."]
        #[inline(always)]
        pub const fn r_auto_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "host synchronization trigger bit auto toggle enable."]
        #[inline(always)]
        pub fn set_r_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "expect data packet (IN)"]
        #[inline(always)]
        pub const fn r_data_no(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "expect data packet (IN)"]
        #[inline(always)]
        pub fn set_r_data_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for UhRxCtrl {
        #[inline(always)]
        fn default() -> UhRxCtrl {
            UhRxCtrl(0)
        }
    }
    #[doc = "data content of the split packet sent by the host."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhSplitData(pub u16);
    impl UhSplitData {
        #[doc = "data content of the split packet sent by the host."]
        #[inline(always)]
        pub const fn split_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "data content of the split packet sent by the host."]
        #[inline(always)]
        pub fn set_split_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for UhSplitData {
        #[inline(always)]
        fn default() -> UhSplitData {
            UhSplitData(0)
        }
    }
    #[doc = "USB host transmit control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxCtrl(pub u8);
    impl UhTxCtrl {
        #[doc = "USB host transmitter response control bits to SETUP/OUT transactions"]
        #[inline(always)]
        pub const fn t_res(&self) -> super::vals::HostTxResponse {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::HostTxResponse::from_bits(val as u8)
        }
        #[doc = "USB host transmitter response control bits to SETUP/OUT transactions"]
        #[inline(always)]
        pub fn set_t_res(&mut self, val: super::vals::HostTxResponse) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
        }
        #[doc = "expect a response after sending data successfully."]
        #[inline(always)]
        pub const fn t_res_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expect a response after sending data successfully."]
        #[inline(always)]
        pub fn set_t_res_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "sync trigger bit prepared by USB host transmitter (handling SETUP/OUT transactions)"]
        #[inline(always)]
        pub const fn t_tog(&self) -> super::vals::Tog {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Tog::from_bits(val as u8)
        }
        #[doc = "sync trigger bit prepared by USB host transmitter (handling SETUP/OUT transactions)"]
        #[inline(always)]
        pub fn set_t_tog(&mut self, val: super::vals::Tog) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u8) & 0x03) << 3usize);
        }
        #[doc = "host synchronization trigger bit auto toggle enable."]
        #[inline(always)]
        pub const fn t_auto_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "host synchronization trigger bit auto toggle enable."]
        #[inline(always)]
        pub fn set_t_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "send data packets (OUT/SETUP)."]
        #[inline(always)]
        pub const fn t_data_no(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "send data packets (OUT/SETUP)."]
        #[inline(always)]
        pub fn set_t_data_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for UhTxCtrl {
        #[inline(always)]
        fn default() -> UhTxCtrl {
            UhTxCtrl(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum EndpointType {
        #[doc = "Non Isochronous (Interrupt/Bulk)"]
        NISO = 0x0,
        #[doc = "Isochronous Transfer"]
        ISO = 0x01,
    }
    impl EndpointType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EndpointType {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EndpointType {
        #[inline(always)]
        fn from(val: u8) -> EndpointType {
            EndpointType::from_bits(val)
        }
    }
    impl From<EndpointType> for u8 {
        #[inline(always)]
        fn from(val: EndpointType) -> u8 {
            EndpointType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum EpRxResponse {
        #[doc = "Respond with ACK"]
        ACK = 0x0,
        #[doc = "Respond NYET"]
        NYET = 0x01,
        #[doc = "Respond with NAK(Busy)"]
        NAK = 0x02,
        #[doc = "Respond with STALL(Error)"]
        STALL = 0x03,
    }
    impl EpRxResponse {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EpRxResponse {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EpRxResponse {
        #[inline(always)]
        fn from(val: u8) -> EpRxResponse {
            EpRxResponse::from_bits(val)
        }
    }
    impl From<EpRxResponse> for u8 {
        #[inline(always)]
        fn from(val: EpRxResponse) -> u8 {
            EpRxResponse::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum EpTog {
        DATA0 = 0x0,
        DATA1 = 0x01,
        DATA2 = 0x02,
        MDATA = 0x03,
    }
    impl EpTog {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EpTog {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EpTog {
        #[inline(always)]
        fn from(val: u8) -> EpTog {
            EpTog::from_bits(val)
        }
    }
    impl From<EpTog> for u8 {
        #[inline(always)]
        fn from(val: EpTog) -> u8 {
            EpTog::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum EpTxResponse {
        #[doc = "Respond with DATA0/DATA1 and expect ACK"]
        ACK = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Respond with NAK or Busy"]
        NAK = 0x02,
        #[doc = "Respond with STALL or Error"]
        STALL = 0x03,
    }
    impl EpTxResponse {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EpTxResponse {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EpTxResponse {
        #[inline(always)]
        fn from(val: u8) -> EpTxResponse {
            EpTxResponse::from_bits(val)
        }
    }
    impl From<EpTxResponse> for u8 {
        #[inline(always)]
        fn from(val: EpTxResponse) -> u8 {
            EpTxResponse::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum HostRxResponse {
        #[doc = "Respond with ACK"]
        ACK = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl HostRxResponse {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HostRxResponse {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HostRxResponse {
        #[inline(always)]
        fn from(val: u8) -> HostRxResponse {
            HostRxResponse::from_bits(val)
        }
    }
    impl From<HostRxResponse> for u8 {
        #[inline(always)]
        fn from(val: HostRxResponse) -> u8 {
            HostRxResponse::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum HostTxResponse {
        #[doc = "Expect ACK"]
        ACK = 0x0,
        #[doc = "Expect NYET"]
        NYET = 0x01,
        #[doc = "Expect NAK(Busy)"]
        NAK = 0x02,
        #[doc = "Expect STALL(Error)"]
        STALL = 0x03,
    }
    impl HostTxResponse {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HostTxResponse {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HostTxResponse {
        #[inline(always)]
        fn from(val: u8) -> HostTxResponse {
            HostTxResponse::from_bits(val)
        }
    }
    impl From<HostTxResponse> for u8 {
        #[inline(always)]
        fn from(val: HostTxResponse) -> u8 {
            HostTxResponse::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SpeedType {
        #[doc = "USB Full Speed (12Mbps)"]
        FULLSPEED = 0x0,
        #[doc = "USB High Speed (480Mbps)"]
        HIGHSPEED = 0x01,
        #[doc = "USB Low Speed (1.5Mbps)"]
        LOWSPEED = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl SpeedType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SpeedType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SpeedType {
        #[inline(always)]
        fn from(val: u8) -> SpeedType {
            SpeedType::from_bits(val)
        }
    }
    impl From<SpeedType> for u8 {
        #[inline(always)]
        fn from(val: SpeedType) -> u8 {
            SpeedType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tog {
        DATA0 = 0x0,
        DATA1 = 0x01,
        DATA2 = 0x02,
        MDATA = 0x03,
    }
    impl Tog {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tog {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tog {
        #[inline(always)]
        fn from(val: u8) -> Tog {
            Tog::from_bits(val)
        }
    }
    impl From<Tog> for u8 {
        #[inline(always)]
        fn from(val: Tog) -> u8 {
            Tog::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum UsbToken {
        #[doc = "OUT Packet"]
        OUT = 0x0,
        #[doc = "Start of Frame"]
        SOF = 0x01,
        #[doc = "IN Packet"]
        IN = 0x02,
        #[doc = "SETUP Packet"]
        SETUP = 0x03,
    }
    impl UsbToken {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> UsbToken {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for UsbToken {
        #[inline(always)]
        fn from(val: u8) -> UsbToken {
            UsbToken::from_bits(val)
        }
    }
    impl From<UsbToken> for u8 {
        #[inline(always)]
        fn from(val: UsbToken) -> u8 {
            UsbToken::to_bits(val)
        }
    }
}
