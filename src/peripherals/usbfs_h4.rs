#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB FS OTG register. Common registers; device-only and host-only registers live in USBFS_DEVICE and USBFS_HOST."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfs {
    ptr: *mut u8,
}
unsafe impl Send for Usbfs {}
unsafe impl Sync for Usbfs {}
impl Usbfs {
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
    pub const fn usb_ctrl(self) -> crate::common::Reg<regs::UsbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn usb_int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn usb_dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn usb_mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn usb_int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn usb_int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn usb_rx_len(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "usb otg control."]
    #[inline(always)]
    pub const fn usb_otg_cr(self) -> crate::common::Reg<regs::UsbOtgCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "usb otg status."]
    #[inline(always)]
    pub const fn usb_otg_sr(self) -> crate::common::Reg<regs::UsbOtgSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
}
#[doc = "USB FS OTG register in device mode. Endpoint registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbfsDevice {
    ptr: *mut u8,
}
unsafe impl Send for UsbfsDevice {}
unsafe impl Sync for UsbfsDevice {}
impl UsbfsDevice {
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
    pub const fn usb_ctrl(self) -> crate::common::Reg<regs::UsbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB device physical port control register."]
    #[inline(always)]
    pub const fn udev_ctrl(self) -> crate::common::Reg<regs::UdevCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn usb_int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn usb_dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn usb_mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn usb_int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn usb_int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn usb_rx_len(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "endpoint 4/1 mode."]
    #[inline(always)]
    pub const fn uep4_1_mod(self) -> crate::common::Reg<regs::Uep41Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "endpoint 2/3 mode."]
    #[inline(always)]
    pub const fn uep2_3_mod(self) -> crate::common::Reg<regs::Uep23Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0dusize) as _) }
    }
    #[doc = "endpoint 5/6 mode."]
    #[inline(always)]
    pub const fn uep5_6_mod(self) -> crate::common::Reg<regs::Uep56Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "endpoint 7 mode."]
    #[inline(always)]
    pub const fn uep7_mod(self) -> crate::common::Reg<regs::Uep7Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fusize) as _) }
    }
    #[doc = "endpoint DMA buffer address."]
    #[inline(always)]
    pub const fn uep_dma(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "endpoint transmittal length. Endpoint 3 uses 10 bits; the other endpoints ignore the upper bits."]
    #[inline(always)]
    pub const fn uep_t_len(self, n: usize) -> crate::common::Reg<regs::UepTLen, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "endpoint control."]
    #[inline(always)]
    pub const fn uep_tx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxCtrl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x32usize + n * 4usize) as _) }
    }
    #[doc = "endpoint control."]
    #[inline(always)]
    pub const fn uep_rx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxCtrl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x33usize + n * 4usize) as _) }
    }
    #[doc = "usb otg control."]
    #[inline(always)]
    pub const fn usb_otg_cr(self) -> crate::common::Reg<regs::UsbOtgCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "usb otg status."]
    #[inline(always)]
    pub const fn usb_otg_sr(self) -> crate::common::Reg<regs::UsbOtgSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
}
#[doc = "USB FS OTG register in host mode. UH_* registers; offsets overlap with USBFS_DEVICE."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbfsHost {
    ptr: *mut u8,
}
unsafe impl Send for UsbfsHost {}
unsafe impl Sync for UsbfsHost {}
impl UsbfsHost {
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
    pub const fn usb_ctrl(self) -> crate::common::Reg<regs::UsbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB host physical port control register."]
    #[inline(always)]
    pub const fn uhost_ctrl(self) -> crate::common::Reg<regs::UhostCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn usb_int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn usb_dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn usb_mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn usb_int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn usb_int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn usb_rx_len(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "USB host endpoint mode control register."]
    #[inline(always)]
    pub const fn uh_ep_mod(self) -> crate::common::Reg<regs::UhEpMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0dusize) as _) }
    }
    #[doc = "USB host receiving DMA buffer address."]
    #[inline(always)]
    pub const fn uh_rx_dma(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "USB host transmittal DMA buffer address."]
    #[inline(always)]
    pub const fn uh_tx_dma(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "USB host setup."]
    #[inline(always)]
    pub const fn uh_setup(self) -> crate::common::Reg<regs::UhSetup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x36usize) as _) }
    }
    #[doc = "USB host endpoint PID."]
    #[inline(always)]
    pub const fn uh_ep_pid(self) -> crate::common::Reg<regs::UhEpPid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "USB host receiving control."]
    #[inline(always)]
    pub const fn uh_rx_ctrl(self) -> crate::common::Reg<regs::UhRxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3busize) as _) }
    }
    #[doc = "USB host transmittal length."]
    #[inline(always)]
    pub const fn uh_tx_len(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "USB host transmittal control."]
    #[inline(always)]
    pub const fn uh_tx_ctrl(self) -> crate::common::Reg<regs::UhTxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3eusize) as _) }
    }
    #[doc = "usb otg control."]
    #[inline(always)]
    pub const fn usb_otg_cr(self) -> crate::common::Reg<regs::UsbOtgCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "usb otg status."]
    #[inline(always)]
    pub const fn usb_otg_sr(self) -> crate::common::Reg<regs::UsbOtgSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
}
pub mod regs {
    #[doc = "USB device physical port control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UdevCtrl(pub u8);
    impl UdevCtrl {
        #[doc = "enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached."]
        #[inline(always)]
        pub const fn ud_port_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached."]
        #[inline(always)]
        pub fn set_ud_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB device port general purpose bit."]
        #[inline(always)]
        pub const fn ud_gp_bit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB device port general purpose bit."]
        #[inline(always)]
        pub fn set_ud_gp_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable USB port low speed: 0=full speed, 1=low speed."]
        #[inline(always)]
        pub const fn ud_low_speed(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB port low speed: 0=full speed, 1=low speed."]
        #[inline(always)]
        pub fn set_ud_low_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "ReadOnly: indicate current UDM pin level."]
        #[inline(always)]
        pub const fn ud_dm_pin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ReadOnly: indicate current UDM pin level."]
        #[inline(always)]
        pub fn set_ud_dm_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub const fn ud_dp_pin(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub fn set_ud_dp_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "disable USB UDP/UDM pulldown resistance: 0=enable pulldown, 1=disable."]
        #[inline(always)]
        pub const fn ud_pd_dis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "disable USB UDP/UDM pulldown resistance: 0=enable pulldown, 1=disable."]
        #[inline(always)]
        pub fn set_ud_pd_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UdevCtrl {
        #[inline(always)]
        fn default() -> UdevCtrl {
            UdevCtrl(0)
        }
    }
    #[doc = "endpoint 2/3 mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep23Mod(pub u8);
    impl Uep23Mod {
        #[doc = "buffer mode of USB endpoint 2."]
        #[inline(always)]
        pub const fn uep2_buf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 2."]
        #[inline(always)]
        pub fn set_uep2_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 2 transmittal (IN)."]
        #[inline(always)]
        pub const fn uep2_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 2 transmittal (IN)."]
        #[inline(always)]
        pub fn set_uep2_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 2 receiving (OUT)."]
        #[inline(always)]
        pub const fn uep2_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 2 receiving (OUT)."]
        #[inline(always)]
        pub fn set_uep2_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB endpoint 3."]
        #[inline(always)]
        pub const fn uep3_buf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 3."]
        #[inline(always)]
        pub fn set_uep3_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB endpoint 3 transmittal (IN)."]
        #[inline(always)]
        pub const fn uep3_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 3 transmittal (IN)."]
        #[inline(always)]
        pub fn set_uep3_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB endpoint 3 receiving (OUT)."]
        #[inline(always)]
        pub const fn uep3_rx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 3 receiving (OUT)."]
        #[inline(always)]
        pub fn set_uep3_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep23Mod {
        #[inline(always)]
        fn default() -> Uep23Mod {
            Uep23Mod(0)
        }
    }
    #[doc = "endpoint 4/1 mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep41Mod(pub u8);
    impl Uep41Mod {
        #[doc = "buffer mode of USB endpoint 4."]
        #[inline(always)]
        pub const fn uep4_buf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 4."]
        #[inline(always)]
        pub fn set_uep4_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 4 transmittal (IN)."]
        #[inline(always)]
        pub const fn uep4_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 4 transmittal (IN)."]
        #[inline(always)]
        pub fn set_uep4_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 4 receiving (OUT)."]
        #[inline(always)]
        pub const fn uep4_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 4 receiving (OUT)."]
        #[inline(always)]
        pub fn set_uep4_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB endpoint 1."]
        #[inline(always)]
        pub const fn uep1_buf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 1."]
        #[inline(always)]
        pub fn set_uep1_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB endpoint 1 transmittal (IN)."]
        #[inline(always)]
        pub const fn uep1_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 1 transmittal (IN)."]
        #[inline(always)]
        pub fn set_uep1_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB endpoint 1 receiving (OUT)."]
        #[inline(always)]
        pub const fn uep1_rx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 1 receiving (OUT)."]
        #[inline(always)]
        pub fn set_uep1_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep41Mod {
        #[inline(always)]
        fn default() -> Uep41Mod {
            Uep41Mod(0)
        }
    }
    #[doc = "endpoint 5/6 mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep56Mod(pub u8);
    impl Uep56Mod {
        #[doc = "buffer mode of USB endpoint 5."]
        #[inline(always)]
        pub const fn uep5_buf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 5."]
        #[inline(always)]
        pub fn set_uep5_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 5 transmittal (IN)."]
        #[inline(always)]
        pub const fn uep5_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 5 transmittal (IN)."]
        #[inline(always)]
        pub fn set_uep5_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 5 receiving (OUT)."]
        #[inline(always)]
        pub const fn uep5_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 5 receiving (OUT)."]
        #[inline(always)]
        pub fn set_uep5_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB endpoint 6."]
        #[inline(always)]
        pub const fn uep6_buf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 6."]
        #[inline(always)]
        pub fn set_uep6_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB endpoint 6 transmittal (IN)."]
        #[inline(always)]
        pub const fn uep6_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 6 transmittal (IN)."]
        #[inline(always)]
        pub fn set_uep6_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB endpoint 6 receiving (OUT)."]
        #[inline(always)]
        pub const fn uep6_rx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 6 receiving (OUT)."]
        #[inline(always)]
        pub fn set_uep6_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep56Mod {
        #[inline(always)]
        fn default() -> Uep56Mod {
            Uep56Mod(0)
        }
    }
    #[doc = "endpoint 7 mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7Mod(pub u8);
    impl Uep7Mod {
        #[doc = "buffer mode of USB endpoint 7."]
        #[inline(always)]
        pub const fn uep7_buf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 7."]
        #[inline(always)]
        pub fn set_uep7_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 7 transmittal (IN)."]
        #[inline(always)]
        pub const fn uep7_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 7 transmittal (IN)."]
        #[inline(always)]
        pub fn set_uep7_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 7 receiving (OUT)."]
        #[inline(always)]
        pub const fn uep7_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 7 receiving (OUT)."]
        #[inline(always)]
        pub fn set_uep7_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for Uep7Mod {
        #[inline(always)]
        fn default() -> Uep7Mod {
            Uep7Mod(0)
        }
    }
    #[doc = "endpoint control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxCtrl(pub u8);
    impl UepRxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> super::vals::EpRxResponse {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::EpRxResponse::from_bits(val as u8)
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub fn set_mask_uep_r_res(&mut self, val: super::vals::EpRxResponse) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn uep_r_tog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub fn set_uep_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for UepRxCtrl {
        #[inline(always)]
        fn default() -> UepRxCtrl {
            UepRxCtrl(0)
        }
    }
    #[doc = "endpoint transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTLen(pub u16);
    impl UepTLen {
        #[doc = "endpoint 3 transmittal length, 10 bits wide; endpoints 0-2 use the low 8 bits only."]
        #[inline(always)]
        pub const fn uep_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "endpoint 3 transmittal length, 10 bits wide; endpoints 0-2 use the low 8 bits only."]
        #[inline(always)]
        pub fn set_uep_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u16) & 0x03ff) << 0usize);
        }
    }
    impl Default for UepTLen {
        #[inline(always)]
        fn default() -> UepTLen {
            UepTLen(0)
        }
    }
    #[doc = "endpoint control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxCtrl(pub u8);
    impl UepTxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> super::vals::EpTxResponse {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::EpTxResponse::from_bits(val as u8)
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub fn set_mask_uep_t_res(&mut self, val: super::vals::EpTxResponse) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn uep_t_tog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub fn set_uep_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for UepTxCtrl {
        #[inline(always)]
        fn default() -> UepTxCtrl {
            UepTxCtrl(0)
        }
    }
    #[doc = "USB host endpoint mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhEpMod(pub u8);
    impl UhEpMod {
        #[doc = "buffer mode of USB host IN endpoint."]
        #[inline(always)]
        pub const fn uh_ep_rbuf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB host IN endpoint."]
        #[inline(always)]
        pub fn set_uh_ep_rbuf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB host IN endpoint receiving."]
        #[inline(always)]
        pub const fn uh_ep_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB host IN endpoint receiving."]
        #[inline(always)]
        pub fn set_uh_ep_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB host OUT endpoint."]
        #[inline(always)]
        pub const fn uh_ep_tbuf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB host OUT endpoint."]
        #[inline(always)]
        pub fn set_uh_ep_tbuf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB host OUT endpoint transmittal."]
        #[inline(always)]
        pub const fn uh_ep_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB host OUT endpoint transmittal."]
        #[inline(always)]
        pub fn set_uh_ep_tx_en(&mut self, val: bool) {
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
        #[doc = "bit mask of endpoint number for USB host transfer."]
        #[inline(always)]
        pub const fn uh_endp_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "bit mask of endpoint number for USB host transfer."]
        #[inline(always)]
        pub fn set_uh_endp_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "bit mask of token PID for USB host transfer."]
        #[inline(always)]
        pub const fn uh_token_mask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "bit mask of token PID for USB host transfer."]
        #[inline(always)]
        pub fn set_uh_token_mask(&mut self, val: u8) {
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
        #[doc = "USB host receiving response control. Bit 1 is reserved."]
        #[inline(always)]
        pub const fn uh_r_res(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB host receiving response control. Bit 1 is reserved."]
        #[inline(always)]
        pub fn set_uh_r_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "expected data toggle flag of USB host receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn uh_r_tog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB host receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub fn set_uh_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub const fn uh_r_auto_tog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub fn set_uh_r_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
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
    pub struct UhSetup(pub u16);
    impl UhSetup {
        #[doc = "USB host automatic SOF enable."]
        #[inline(always)]
        pub const fn uh_sof_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB host automatic SOF enable."]
        #[inline(always)]
        pub fn set_uh_sof_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "USB host PRE PID enable for low speed device via hub."]
        #[inline(always)]
        pub const fn uh_pre_pid_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "USB host PRE PID enable for low speed device via hub."]
        #[inline(always)]
        pub fn set_uh_pre_pid_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
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
        #[doc = "USB host transmittal response control. Bit 1 is reserved."]
        #[inline(always)]
        pub const fn uh_t_res(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB host transmittal response control. Bit 1 is reserved."]
        #[inline(always)]
        pub fn set_uh_t_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "prepared data toggle flag of USB host transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn uh_t_tog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB host transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub fn set_uh_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub const fn uh_t_auto_tog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub fn set_uh_t_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
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
        #[doc = "enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached."]
        #[inline(always)]
        pub const fn uh_port_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached."]
        #[inline(always)]
        pub fn set_uh_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "force clear FIFO and count of USB."]
        #[inline(always)]
        pub const fn uh_bus_reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear FIFO and count of USB."]
        #[inline(always)]
        pub fn set_uh_bus_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable USB port low speed: 0=full speed, 1=low speed."]
        #[inline(always)]
        pub const fn uh_low_speed(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB port low speed: 0=full speed, 1=low speed."]
        #[inline(always)]
        pub fn set_uh_low_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "ReadOnly: indicate current UDM pin level."]
        #[inline(always)]
        pub const fn uh_dm_pin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ReadOnly: indicate current UDM pin level."]
        #[inline(always)]
        pub fn set_uh_dm_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub const fn uh_dp_pin(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub fn set_uh_dp_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "disable USB UDP/UDM pulldown resistance: 0=enable pulldown, 1=disable."]
        #[inline(always)]
        pub const fn uh_pd_dis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "disable USB UDP/UDM pulldown resistance: 0=enable pulldown, 1=disable."]
        #[inline(always)]
        pub fn set_uh_pd_dis(&mut self, val: bool) {
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
    pub struct UsbCtrl(pub u8);
    impl UsbCtrl {
        #[doc = "DMA enable and DMA interrupt enable for USB."]
        #[inline(always)]
        pub const fn uc_dma_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable and DMA interrupt enable for USB."]
        #[inline(always)]
        pub fn set_uc_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "force clear FIFO and count of USB."]
        #[inline(always)]
        pub const fn uc_clr_all(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear FIFO and count of USB."]
        #[inline(always)]
        pub fn set_uc_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "force reset USB SIE, need software clear."]
        #[inline(always)]
        pub const fn uc_rst_sie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "force reset USB SIE, need software clear."]
        #[inline(always)]
        pub fn set_uc_rst_sie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid."]
        #[inline(always)]
        pub const fn uc_int_busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid."]
        #[inline(always)]
        pub fn set_uc_int_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB system control, low speed and full speed."]
        #[inline(always)]
        pub const fn uc_sys_ctrl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USB system control, low speed and full speed."]
        #[inline(always)]
        pub fn set_uc_sys_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub const fn uc_dev_pu_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub fn set_uc_dev_pu_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "enable USB low speed: 0=12Mbps, 1=1.5Mbps."]
        #[inline(always)]
        pub const fn uc_low_speed(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB low speed: 0=12Mbps, 1=1.5Mbps."]
        #[inline(always)]
        pub fn set_uc_low_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB host mode: 0=device mode, 1=host mode."]
        #[inline(always)]
        pub const fn uc_host_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB host mode: 0=device mode, 1=host mode."]
        #[inline(always)]
        pub fn set_uc_host_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbCtrl {
        #[inline(always)]
        fn default() -> UsbCtrl {
            UsbCtrl(0)
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
        pub const fn uie_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB bus reset event for USB device mode."]
        #[inline(always)]
        pub fn set_uie_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for USB device detected event for USB host mode."]
        #[inline(always)]
        pub const fn uie_detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB device detected event for USB host mode."]
        #[inline(always)]
        pub fn set_uie_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for USB transfer completion."]
        #[inline(always)]
        pub const fn uie_transfer(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB transfer completion."]
        #[inline(always)]
        pub fn set_uie_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable interrupt for USB suspend or resume event."]
        #[inline(always)]
        pub const fn uie_suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB suspend or resume event."]
        #[inline(always)]
        pub fn set_uie_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable interrupt for host SOF timer action for USB host mode."]
        #[inline(always)]
        pub const fn uie_hst_sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for host SOF timer action for USB host mode."]
        #[inline(always)]
        pub fn set_uie_hst_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable interrupt for FIFO overflow."]
        #[inline(always)]
        pub const fn uie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for FIFO overflow."]
        #[inline(always)]
        pub fn set_uie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub const fn uie_dev_nak(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub fn set_uie_dev_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable interrupt for SOF received for USB device mode."]
        #[inline(always)]
        pub const fn uie_dev_sof(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SOF received for USB device mode."]
        #[inline(always)]
        pub fn set_uie_dev_sof(&mut self, val: bool) {
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
        #[doc = "bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn uif_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_uif_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn uif_detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_uif_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB transfer completion interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn uif_transfer(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer completion interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_uif_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn uif_suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_uif_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn uif_hst_sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_uif_hst_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn uif_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub fn set_uif_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[inline(always)]
        pub const fn u_sie_free(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[inline(always)]
        pub fn set_u_sie_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[inline(always)]
        pub const fn u_tog_ok(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[inline(always)]
        pub fn set_u_tog_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate current USB transfer is NAK received."]
        #[inline(always)]
        pub const fn u_is_nak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer is NAK received."]
        #[inline(always)]
        pub fn set_u_is_nak(&mut self, val: bool) {
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
        #[doc = "RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub const fn mask_uis_endp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub fn set_mask_uis_endp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received."]
        #[inline(always)]
        pub const fn mask_uis_h_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received."]
        #[inline(always)]
        pub fn set_mask_uis_h_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[inline(always)]
        pub const fn mask_uis_token(&self) -> super::vals::UsbToken {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::UsbToken::from_bits(val as u8)
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[inline(always)]
        pub fn set_mask_uis_token(&mut self, val: super::vals::UsbToken) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[inline(always)]
        pub const fn uis_tog_ok(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[inline(always)]
        pub fn set_uis_tog_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate current USB transfer is NAK received for USB device mode."]
        #[inline(always)]
        pub const fn uis_is_nak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer is NAK received for USB device mode."]
        #[inline(always)]
        pub fn set_uis_is_nak(&mut self, val: bool) {
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
        pub const fn ums_dev_attach(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate device attached status on USB host."]
        #[inline(always)]
        pub fn set_ums_dev_attach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, indicate UDM level saved at device attached to USB host."]
        #[inline(always)]
        pub const fn ums_dm_level(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate UDM level saved at device attached to USB host."]
        #[inline(always)]
        pub fn set_ums_dm_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RO, indicate USB suspend status."]
        #[inline(always)]
        pub const fn ums_suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB suspend status."]
        #[inline(always)]
        pub fn set_ums_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RO, indicate USB bus reset status."]
        #[inline(always)]
        pub const fn ums_bus_reset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB bus reset status."]
        #[inline(always)]
        pub fn set_ums_bus_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RO, indicate USB receiving FIFO ready status (not empty)."]
        #[inline(always)]
        pub const fn ums_r_fifo_rdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB receiving FIFO ready status (not empty)."]
        #[inline(always)]
        pub fn set_ums_r_fifo_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[inline(always)]
        pub const fn ums_sie_free(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[inline(always)]
        pub fn set_ums_sie_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, indicate host SOF timer action status for USB host."]
        #[inline(always)]
        pub const fn ums_sof_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate host SOF timer action status for USB host."]
        #[inline(always)]
        pub fn set_ums_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate host SOF timer presage status."]
        #[inline(always)]
        pub const fn ums_sof_pres(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate host SOF timer presage status."]
        #[inline(always)]
        pub fn set_ums_sof_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbMisSt {
        #[inline(always)]
        fn default() -> UsbMisSt {
            UsbMisSt(0)
        }
    }
    #[doc = "usb otg control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbOtgCr(pub u32);
    impl UsbOtgCr {
        #[doc = "usb otg control."]
        #[inline(always)]
        pub const fn usb_otg_cr_dischargevbus(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub fn set_usb_otg_cr_dischargevbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub const fn usb_otg_cr_chargevbus(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub fn set_usb_otg_cr_chargevbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub const fn usb_otg_cr_idpu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub fn set_usb_otg_cr_idpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub const fn usb_otg_cr_otg_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub fn set_usb_otg_cr_otg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub const fn usb_otg_cr_vbus(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub fn set_usb_otg_cr_vbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub const fn usb_otg_cr_sess(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg control."]
        #[inline(always)]
        pub fn set_usb_otg_cr_sess(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for UsbOtgCr {
        #[inline(always)]
        fn default() -> UsbOtgCr {
            UsbOtgCr(0)
        }
    }
    #[doc = "usb otg status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbOtgSr(pub u32);
    impl UsbOtgSr {
        #[doc = "usb otg status."]
        #[inline(always)]
        pub const fn usb_otg_sr_vbus_vld(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg status."]
        #[inline(always)]
        pub fn set_usb_otg_sr_vbus_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "usb otg status."]
        #[inline(always)]
        pub const fn usb_otg_sr_sess_vld(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg status."]
        #[inline(always)]
        pub fn set_usb_otg_sr_sess_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "usb otg status."]
        #[inline(always)]
        pub const fn usb_otg_sr_sess_end(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg status."]
        #[inline(always)]
        pub fn set_usb_otg_sr_sess_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "usb otg status."]
        #[inline(always)]
        pub const fn usb_otg_sr_id_dig(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "usb otg status."]
        #[inline(always)]
        pub fn set_usb_otg_sr_id_dig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for UsbOtgSr {
        #[inline(always)]
        fn default() -> UsbOtgSr {
            UsbOtgSr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum EpRxResponse {
        #[doc = "Respond with ACK."]
        ACK = 0x0,
        #[doc = "Timeout / No Response, used for non-EP0 iso/sync transfer."]
        NONE = 0x01,
        #[doc = "Respond with NAK or Busy."]
        NAK = 0x02,
        #[doc = "Respond with STALL or Error."]
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
        #[doc = "Respond with DATA0/DATA1 and expect ACK."]
        ACK = 0x0,
        #[doc = "Respond with DATA0/DATA1 and not expect ACK."]
        NONE = 0x01,
        #[doc = "Respond with NAK or Busy."]
        NAK = 0x02,
        #[doc = "Respond with STALL or Error."]
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
