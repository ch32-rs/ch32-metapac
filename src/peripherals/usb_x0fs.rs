#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Universal serial bus FS register"]
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::UsbBaseCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn rx_len(self) -> crate::common::Reg<regs::UsbRxLen, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Universal serial bus FS device register"]
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::UsbBaseCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB device physical port control register."]
    #[inline(always)]
    pub const fn udev_ctrl(self) -> crate::common::Reg<regs::UdevCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn rx_len(self) -> crate::common::Reg<regs::UsbRxLen, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "endpoint 4/1 mode."]
    #[inline(always)]
    pub const fn uep4_1_mod(self) -> crate::common::Reg<regs::UepMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Endpoint 2/3 mode control register."]
    #[inline(always)]
    pub const fn uep2_3_mod(self) -> crate::common::Reg<regs::UepMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0dusize) as _) }
    }
    #[doc = "endpoint 5/6/7 mode."]
    #[inline(always)]
    pub const fn uep567_mod(self) -> crate::common::Reg<regs::Uep567Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "endpoint DMA buffer address."]
    #[inline(always)]
    pub const fn uep0123_dma(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepDma, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "endpoint transmit length."]
    #[inline(always)]
    pub const fn uep01234_t_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTLen, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "endpoint control."]
    #[inline(always)]
    pub const fn uep01234_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepCtrl, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x22usize + n * 4usize) as _) }
    }
    #[doc = "endpoint DMA buffer address."]
    #[inline(always)]
    pub const fn uep567_dma(self, n: usize) -> crate::common::Reg<regs::UepDma, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize + n * 4usize) as _) }
    }
    #[doc = "endpoint transmit length."]
    #[inline(always)]
    pub const fn uep567_t_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTLen, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize + n * 4usize) as _) }
    }
    #[doc = "endpoint control."]
    #[inline(always)]
    pub const fn uep567_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepCtrl, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x66usize + n * 4usize) as _) }
    }
    #[doc = "UEP endpoint X mode control register."]
    #[inline(always)]
    pub const fn uepx_mod(self) -> crate::common::Reg<regs::UepxMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
}
#[doc = "Universal serial bus FS host register"]
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
    #[doc = "USB base control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::UsbBaseCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB host physical port control register."]
    #[inline(always)]
    pub const fn uhost_ctrl(self) -> crate::common::Reg<regs::UhostCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn rx_len(self) -> crate::common::Reg<regs::UsbRxLen, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "USB host endpoint mode control register."]
    #[inline(always)]
    pub const fn ep_mod(self) -> crate::common::Reg<regs::UhEpMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0dusize) as _) }
    }
    #[doc = "USB host receiving DMA buffer address."]
    #[inline(always)]
    pub const fn rx_dma(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "USB host transmittal DMA buffer address."]
    #[inline(always)]
    pub const fn tx_dma(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "USB host setup."]
    #[inline(always)]
    pub const fn setup(self) -> crate::common::Reg<regs::UhSetup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x26usize) as _) }
    }
    #[doc = "USB host endpoint PID."]
    #[inline(always)]
    pub const fn ep_pid(self) -> crate::common::Reg<regs::UhEpPid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "USB host receiving control."]
    #[inline(always)]
    pub const fn rx_ctrl(self) -> crate::common::Reg<regs::UhRxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2ausize) as _) }
    }
    #[doc = "USB host transmittal length."]
    #[inline(always)]
    pub const fn tx_len(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "USB host transmittal control."]
    #[inline(always)]
    pub const fn tx_ctrl(self) -> crate::common::Reg<regs::UhTxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2eusize) as _) }
    }
}
pub mod regs {
    #[doc = "USB device physical port control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UdevCtrl(pub u8);
    impl UdevCtrl {
        #[doc = "USB device port enable."]
        #[inline(always)]
        pub const fn port_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB device port enable."]
        #[inline(always)]
        pub fn set_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB device port general purpose bit."]
        #[inline(always)]
        pub const fn gp_bit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB device port general purpose bit."]
        #[inline(always)]
        pub fn set_gp_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB device port low speed enable."]
        #[inline(always)]
        pub const fn low_speed(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB device port low speed enable."]
        #[inline(always)]
        pub fn set_low_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB device port UD- pin status."]
        #[inline(always)]
        pub const fn dm_pin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB device port UD- pin status."]
        #[inline(always)]
        pub fn set_dm_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "USB device port UD+ pin status."]
        #[inline(always)]
        pub const fn dp_pin(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device port UD+ pin status."]
        #[inline(always)]
        pub fn set_dp_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "USB device port UD+/UD- pin internal pull-down resistor control."]
        #[inline(always)]
        pub const fn pd_dis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB device port UD+/UD- pin internal pull-down resistor control."]
        #[inline(always)]
        pub fn set_pd_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UdevCtrl {
        #[inline(always)]
        fn default() -> UdevCtrl {
            UdevCtrl(0)
        }
    }
    #[doc = "endpoint 5/6/7 mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep567Mod(pub u8);
    impl Uep567Mod {
        #[inline(always)]
        pub const fn tx_en(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_tx_en(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u8) & 0x01) << offs);
        }
        #[inline(always)]
        pub const fn rx_en(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 1usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_rx_en(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 1usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u8) & 0x01) << offs);
        }
    }
    impl Default for Uep567Mod {
        #[inline(always)]
        fn default() -> Uep567Mod {
            Uep567Mod(0)
        }
    }
    #[doc = "USB endpoint control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepCtrl(pub u8);
    impl UepCtrl {
        #[inline(always)]
        pub const fn t_res(&self) -> super::vals::EpTxResponse {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::EpTxResponse::from_bits(val as u8)
        }
        #[inline(always)]
        pub fn set_t_res(&mut self, val: super::vals::EpTxResponse) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn r_res(&self) -> super::vals::EpRxResponse {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::EpRxResponse::from_bits(val as u8)
        }
        #[inline(always)]
        pub fn set_r_res(&mut self, val: super::vals::EpRxResponse) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u8) & 0x03) << 2usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub fn set_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub fn set_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepCtrl {
        #[inline(always)]
        fn default() -> UepCtrl {
            UepCtrl(0)
        }
    }
    #[doc = "USB endpoint DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepDma(pub u32);
    impl UepDma {
        #[doc = "USB endpoint DMA buffer address."]
        #[inline(always)]
        pub const fn buf_addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "USB endpoint DMA buffer address."]
        #[inline(always)]
        pub fn set_buf_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for UepDma {
        #[inline(always)]
        fn default() -> UepDma {
            UepDma(0)
        }
    }
    #[doc = "endpoint a/b mode. lower bits comes first"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepMod(pub u8);
    impl UepMod {
        #[doc = "buffer mode of USB endpoint"]
        #[inline(always)]
        pub const fn buf_mod(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint"]
        #[inline(always)]
        pub fn set_buf_mod(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u8) & 0x01) << offs);
        }
        #[doc = "enable USB endpoint 1 transmittal (IN)."]
        #[inline(always)]
        pub const fn tx_en(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 1 transmittal (IN)."]
        #[inline(always)]
        pub fn set_tx_en(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u8) & 0x01) << offs);
        }
        #[doc = "enable USB endpoint 4 receiving (OUT)."]
        #[inline(always)]
        pub const fn rx_en(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 4 receiving (OUT)."]
        #[inline(always)]
        pub fn set_rx_en(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u8) & 0x01) << offs);
        }
    }
    impl Default for UepMod {
        #[inline(always)]
        fn default() -> UepMod {
            UepMod(0)
        }
    }
    #[doc = "USB endpoint transmit length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTLen(pub u8);
    impl UepTLen {
        #[doc = "transmit length."]
        #[inline(always)]
        pub const fn t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "transmit length."]
        #[inline(always)]
        pub fn set_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for UepTLen {
        #[inline(always)]
        fn default() -> UepTLen {
            UepTLen(0)
        }
    }
    #[doc = "UEP endpoint X mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepxMod(pub u32);
    impl UepxMod {
        #[doc = "enable USB endpoint (8-15) transmittal (IN)."]
        #[inline(always)]
        pub const fn t_en(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint (8-15) transmittal (IN)."]
        #[inline(always)]
        pub fn set_t_en(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "enable USB endpoint (8-15) receiving (OUT)."]
        #[inline(always)]
        pub const fn r_en(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint (8-15) receiving (OUT)."]
        #[inline(always)]
        pub fn set_r_en(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "enable USB endpoint (8-15) transmittal alternate function."]
        #[inline(always)]
        pub const fn t_af(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 17usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint (8-15) transmittal alternate function."]
        #[inline(always)]
        pub fn set_t_af(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 17usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for UepxMod {
        #[inline(always)]
        fn default() -> UepxMod {
            UepxMod(0)
        }
    }
    #[doc = "USB host endpoint mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhEpMod(pub u8);
    impl UhEpMod {
        #[inline(always)]
        pub const fn rbuf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_rbuf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn tbuf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_tbuf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for UhEpMod {
        #[inline(always)]
        fn default() -> UhEpMod {
            UhEpMod(0)
        }
    }
    #[doc = "USB host endpoint PID."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhEpPid(pub u8);
    impl UhEpPid {
        #[doc = "endpoint PID"]
        #[inline(always)]
        pub const fn mask_endp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "endpoint PID"]
        #[inline(always)]
        pub fn set_mask_endp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "token PID"]
        #[inline(always)]
        pub const fn mask_token(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "token PID"]
        #[inline(always)]
        pub fn set_mask_token(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for UhEpPid {
        #[inline(always)]
        fn default() -> UhEpPid {
            UhEpPid(0)
        }
    }
    #[doc = "USB host receiving control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxCtrl(pub u8);
    impl UhRxCtrl {
        #[inline(always)]
        pub const fn r_res(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_r_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub const fn r_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub fn set_r_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub fn set_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhRxCtrl {
        #[inline(always)]
        fn default() -> UhRxCtrl {
            UhRxCtrl(0)
        }
    }
    #[doc = "USB host setup."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhSetup(pub u8);
    impl UhSetup {
        #[doc = "SOF packet en"]
        #[inline(always)]
        pub const fn sof_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SOF packet en"]
        #[inline(always)]
        pub fn set_sof_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "pre pid en"]
        #[inline(always)]
        pub const fn pre_pid_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "pre pid en"]
        #[inline(always)]
        pub fn set_pre_pid_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for UhSetup {
        #[inline(always)]
        fn default() -> UhSetup {
            UhSetup(0)
        }
    }
    #[doc = "USB host transmittal control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxCtrl(pub u8);
    impl UhTxCtrl {
        #[inline(always)]
        pub const fn t_res(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_t_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn t_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_t_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for UhTxCtrl {
        #[inline(always)]
        fn default() -> UhTxCtrl {
            UhTxCtrl(0)
        }
    }
    #[doc = "USB host physical port control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhostCtrl(pub u8);
    impl UhostCtrl {
        #[doc = "USB host port enable."]
        #[inline(always)]
        pub const fn port_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB host port enable."]
        #[inline(always)]
        pub fn set_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB host port bus reset."]
        #[inline(always)]
        pub const fn bus_rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB host port bus reset."]
        #[inline(always)]
        pub fn set_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB host port low speed enable."]
        #[inline(always)]
        pub const fn low_speed(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB host port low speed enable."]
        #[inline(always)]
        pub fn set_low_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Current UD- pin status."]
        #[inline(always)]
        pub const fn dm_pin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Current UD- pin status."]
        #[inline(always)]
        pub fn set_dm_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Current UD+ pin status."]
        #[inline(always)]
        pub const fn dp_pin(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Current UD+ pin status."]
        #[inline(always)]
        pub fn set_dp_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Internal pull-down resistor control for USB host port UD+/UD- pins."]
        #[inline(always)]
        pub const fn pd_dis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Internal pull-down resistor control for USB host port UD+/UD- pins."]
        #[inline(always)]
        pub fn set_pd_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhostCtrl {
        #[inline(always)]
        fn default() -> UhostCtrl {
            UhostCtrl(0)
        }
    }
    #[doc = "USB base control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbBaseCtrl(pub u8);
    impl UsbBaseCtrl {
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
        pub const fn sys_ctrl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub fn set_sys_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "USB device internal pullup resistance enable."]
        #[inline(always)]
        pub const fn dev_pu_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device internal pullup resistance enable."]
        #[inline(always)]
        pub fn set_dev_pu_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "enable USB low speed: 0=12Mbps, 1=1.5Mbps."]
        #[inline(always)]
        pub const fn low_speed(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB low speed: 0=12Mbps, 1=1.5Mbps."]
        #[inline(always)]
        pub fn set_low_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
    impl Default for UsbBaseCtrl {
        #[inline(always)]
        fn default() -> UsbBaseCtrl {
            UsbBaseCtrl(0)
        }
    }
    #[doc = "USB device address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbDevAd(pub u8);
    impl UsbDevAd {
        #[doc = "bit mask for USB device address."]
        #[inline(always)]
        pub const fn mask_usb_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "bit mask for USB device address."]
        #[inline(always)]
        pub fn set_mask_usb_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
        #[doc = "general purpose bit."]
        #[inline(always)]
        pub const fn uda_gp_bit(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "general purpose bit."]
        #[inline(always)]
        pub fn set_uda_gp_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbDevAd {
        #[inline(always)]
        fn default() -> UsbDevAd {
            UsbDevAd(0)
        }
    }
    #[doc = "USB interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbIntEn(pub u8);
    impl UsbIntEn {
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
        #[doc = "enable interrupt for host SOF timer action for USB host mode."]
        #[inline(always)]
        pub const fn hst_sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for host SOF timer action for USB host mode."]
        #[inline(always)]
        pub fn set_hst_sof(&mut self, val: bool) {
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
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub const fn dev_nak(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub fn set_dev_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable interrupt for SOF received for USB device mode."]
        #[inline(always)]
        pub const fn dev_sof(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SOF received for USB device mode."]
        #[inline(always)]
        pub fn set_dev_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbIntEn {
        #[inline(always)]
        fn default() -> UsbIntEn {
            UsbIntEn(0)
        }
    }
    #[doc = "USB interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbIntFg(pub u8);
    impl UsbIntFg {
        #[doc = "bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear"]
        #[inline(always)]
        pub const fn bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear"]
        #[inline(always)]
        pub fn set_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear."]
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
    impl Default for UsbIntFg {
        #[inline(always)]
        fn default() -> UsbIntFg {
            UsbIntFg(0)
        }
    }
    #[doc = "USB interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbIntSt(pub u8);
    impl UsbIntSt {
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub const fn mask_h_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub fn set_mask_h_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub const fn mask_uis_endp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub fn set_mask_uis_endp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[inline(always)]
        pub const fn mask_token(&self) -> super::vals::UsbToken {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::UsbToken::from_bits(val as u8)
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[inline(always)]
        pub fn set_mask_token(&mut self, val: super::vals::UsbToken) {
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
    impl Default for UsbIntSt {
        #[inline(always)]
        fn default() -> UsbIntSt {
            UsbIntSt(0)
        }
    }
    #[doc = "USB miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbMisSt(pub u8);
    impl UsbMisSt {
        #[doc = "RO, indicate device attached status on USB host."]
        #[inline(always)]
        pub const fn dev_attach(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate device attached status on USB host."]
        #[inline(always)]
        pub fn set_dev_attach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, indicate UDM level saved at device attached to USB host."]
        #[inline(always)]
        pub const fn dm_level(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate UDM level saved at device attached to USB host."]
        #[inline(always)]
        pub fn set_dm_level(&mut self, val: bool) {
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
    impl Default for UsbMisSt {
        #[inline(always)]
        fn default() -> UsbMisSt {
            UsbMisSt(0)
        }
    }
    #[doc = "USB receiving length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbRxLen(pub u16);
    impl UsbRxLen {
        #[doc = "receiving length."]
        #[inline(always)]
        pub const fn rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "receiving length."]
        #[inline(always)]
        pub fn set_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u16) & 0x03ff) << 0usize);
        }
    }
    impl Default for UsbRxLen {
        #[inline(always)]
        fn default() -> UsbRxLen {
            UsbRxLen(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum EpRxResponse {
        #[doc = "Data ready and respond with ACK."]
        ACK = 0x0,
        #[doc = "Respond with NYET."]
        NYET = 0x01,
        #[doc = "Respond with NAK or busy."]
        NAK = 0x02,
        #[doc = "Respond with STALL or error."]
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
    pub enum EpTxResponse {
        #[doc = "Expect an ACK response."]
        ACK = 0x0,
        #[doc = "No response expected."]
        NONE = 0x01,
        #[doc = "Expect a NAK or busy response."]
        NAK = 0x02,
        #[doc = "Expect a STALL or error response."]
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
    pub enum UsbToken {
        #[doc = "OUT packet."]
        OUT = 0x0,
        #[doc = "Reserved."]
        RSVD = 0x01,
        #[doc = "IN packet."]
        IN = 0x02,
        #[doc = "SETUP packet."]
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
