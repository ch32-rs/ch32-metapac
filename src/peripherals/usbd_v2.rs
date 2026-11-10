#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Universal serial bus full-speed device interface."]
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
    #[doc = "endpoint register. 0 to 7."]
    #[inline(always)]
    pub const fn epr(self, n: usize) -> crate::common::Reg<regs::Epr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "control register."]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "interrupt status register."]
    #[inline(always)]
    pub const fn istr(self) -> crate::common::Reg<regs::Istr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "frame number register."]
    #[inline(always)]
    pub const fn fnr(self) -> crate::common::Reg<regs::Fnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "device address."]
    #[inline(always)]
    pub const fn daddr(self) -> crate::common::Reg<regs::Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Buffer table address."]
    #[inline(always)]
    pub const fn btable(self) -> crate::common::Reg<regs::Btable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
}
pub mod regs {
    #[doc = "Buffer table address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btable(pub u32);
    impl Btable {
        #[doc = "Buffer table."]
        #[must_use]
        #[inline(always)]
        pub const fn btable(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "Buffer table."]
        #[inline(always)]
        pub const fn set_btable(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Btable {
        #[inline(always)]
        fn default() -> Btable {
            Btable(0)
        }
    }
    impl core::fmt::Debug for Btable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Btable")
                .field("btable", &self.btable())
                .finish()
        }
    }
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntr(pub u16);
    impl Cntr {
        #[doc = "Force USB Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn fres(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force USB Reset."]
        #[inline(always)]
        pub const fn set_fres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Power down."]
        #[must_use]
        #[inline(always)]
        pub const fn pdwn(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Power down."]
        #[inline(always)]
        pub const fn set_pdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power mode."]
        #[inline(always)]
        pub const fn set_lpmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Force suspend."]
        #[must_use]
        #[inline(always)]
        pub const fn fsusp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force suspend."]
        #[inline(always)]
        pub const fn set_fsusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Resume request."]
        #[must_use]
        #[inline(always)]
        pub const fn resume(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Resume request."]
        #[inline(always)]
        pub const fn set_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Expected start of frame interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn esofm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Expected start of frame interrupt mask."]
        #[inline(always)]
        pub const fn set_esofm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Start of frame interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn sofm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Start of frame interrupt mask."]
        #[inline(always)]
        pub const fn set_sofm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "USB reset interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn resetm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset interrupt mask."]
        #[inline(always)]
        pub const fn set_resetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Suspend mode interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn suspm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend mode interrupt mask."]
        #[inline(always)]
        pub const fn set_suspm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Wakeup interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt mask."]
        #[inline(always)]
        pub const fn set_wkupm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Error interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn errm(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt mask."]
        #[inline(always)]
        pub const fn set_errm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Packet memory area over / underrun interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn pmaovrm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Packet memory area over / underrun interrupt mask."]
        #[inline(always)]
        pub const fn set_pmaovrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Correct transfer interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn ctrm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Correct transfer interrupt mask."]
        #[inline(always)]
        pub const fn set_ctrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Cntr {
        #[inline(always)]
        fn default() -> Cntr {
            Cntr(0)
        }
    }
    impl core::fmt::Debug for Cntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntr")
                .field("fres", &self.fres())
                .field("pdwn", &self.pdwn())
                .field("lpmode", &self.lpmode())
                .field("fsusp", &self.fsusp())
                .field("resume", &self.resume())
                .field("esofm", &self.esofm())
                .field("sofm", &self.sofm())
                .field("resetm", &self.resetm())
                .field("suspm", &self.suspm())
                .field("wkupm", &self.wkupm())
                .field("errm", &self.errm())
                .field("pmaovrm", &self.pmaovrm())
                .field("ctrm", &self.ctrm())
                .finish()
        }
    }
    #[doc = "device address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Daddr(pub u16);
    impl Daddr {
        #[doc = "Device address."]
        #[must_use]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Device address."]
        #[inline(always)]
        pub const fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
        }
        #[doc = "Enable function."]
        #[must_use]
        #[inline(always)]
        pub const fn ef(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable function."]
        #[inline(always)]
        pub const fn set_ef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
    }
    impl Default for Daddr {
        #[inline(always)]
        fn default() -> Daddr {
            Daddr(0)
        }
    }
    impl core::fmt::Debug for Daddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Daddr")
                .field("add", &self.add())
                .field("ef", &self.ef())
                .finish()
        }
    }
    #[doc = "endpoint register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epr(pub u16);
    impl Epr {
        #[doc = "Endpoint address."]
        #[must_use]
        #[inline(always)]
        pub const fn ea(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Endpoint address."]
        #[inline(always)]
        pub const fn set_ea(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "Status bits, for transmission transfers."]
        #[must_use]
        #[inline(always)]
        pub const fn stat_tx(&self) -> super::vals::Stat {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Stat::from_bits(val as u8)
        }
        #[doc = "Status bits, for transmission transfers."]
        #[inline(always)]
        pub const fn set_stat_tx(&mut self, val: super::vals::Stat) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
        }
        #[doc = "Data Toggle, for transmission transfers."]
        #[must_use]
        #[inline(always)]
        pub const fn dtog_tx(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Data Toggle, for transmission transfers."]
        #[inline(always)]
        pub const fn set_dtog_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Correct Transfer for transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn ctr_tx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Correct Transfer for transmission."]
        #[inline(always)]
        pub const fn set_ctr_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Endpoint kind."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_kind(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint kind."]
        #[inline(always)]
        pub const fn set_ep_kind(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Endpoint type."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_type(&self) -> super::vals::EpType {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::EpType::from_bits(val as u8)
        }
        #[doc = "Endpoint type."]
        #[inline(always)]
        pub const fn set_ep_type(&mut self, val: super::vals::EpType) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u16) & 0x03) << 9usize);
        }
        #[doc = "Setup transaction completed."]
        #[must_use]
        #[inline(always)]
        pub const fn setup(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Setup transaction completed."]
        #[inline(always)]
        pub const fn set_setup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Status bits, for reception transfers."]
        #[must_use]
        #[inline(always)]
        pub const fn stat_rx(&self) -> super::vals::Stat {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Stat::from_bits(val as u8)
        }
        #[doc = "Status bits, for reception transfers."]
        #[inline(always)]
        pub const fn set_stat_rx(&mut self, val: super::vals::Stat) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
        }
        #[doc = "Data Toggle, for reception transfers."]
        #[must_use]
        #[inline(always)]
        pub const fn dtog_rx(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Data Toggle, for reception transfers."]
        #[inline(always)]
        pub const fn set_dtog_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Correct transfer for reception."]
        #[must_use]
        #[inline(always)]
        pub const fn ctr_rx(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Correct transfer for reception."]
        #[inline(always)]
        pub const fn set_ctr_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Epr {
        #[inline(always)]
        fn default() -> Epr {
            Epr(0)
        }
    }
    impl core::fmt::Debug for Epr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epr")
                .field("ea", &self.ea())
                .field("stat_tx", &self.stat_tx())
                .field("dtog_tx", &self.dtog_tx())
                .field("ctr_tx", &self.ctr_tx())
                .field("ep_kind", &self.ep_kind())
                .field("ep_type", &self.ep_type())
                .field("setup", &self.setup())
                .field("stat_rx", &self.stat_rx())
                .field("dtog_rx", &self.dtog_rx())
                .field("ctr_rx", &self.ctr_rx())
                .finish()
        }
    }
    #[doc = "frame number register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fnr(pub u16);
    impl Fnr {
        #[doc = "Frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn fn_(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Frame number."]
        #[inline(always)]
        pub const fn set_fn_(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
        #[doc = "Lost SOF."]
        #[must_use]
        #[inline(always)]
        pub const fn lsof(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Lost SOF."]
        #[inline(always)]
        pub const fn set_lsof(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u16) & 0x03) << 11usize);
        }
        #[doc = "Locked."]
        #[must_use]
        #[inline(always)]
        pub const fn lck(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Locked."]
        #[inline(always)]
        pub const fn set_lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Receive data - line status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Receive data - line status."]
        #[inline(always)]
        pub const fn set_rxdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Receive data + line status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive data + line status."]
        #[inline(always)]
        pub const fn set_rxdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Fnr {
        #[inline(always)]
        fn default() -> Fnr {
            Fnr(0)
        }
    }
    impl core::fmt::Debug for Fnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fnr")
                .field("fn_", &self.fn_())
                .field("lsof", &self.lsof())
                .field("lck", &self.lck())
                .field("rxdm", &self.rxdm())
                .field("rxdp", &self.rxdp())
                .finish()
        }
    }
    #[doc = "interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Istr(pub u16);
    impl Istr {
        #[doc = "Endpoint Identifier."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Endpoint Identifier."]
        #[inline(always)]
        pub const fn set_ep_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "Direction of transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Direction of transaction."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
        }
        #[doc = "Expected start frame."]
        #[must_use]
        #[inline(always)]
        pub const fn esof(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Expected start frame."]
        #[inline(always)]
        pub const fn set_esof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "start of frame."]
        #[must_use]
        #[inline(always)]
        pub const fn sof(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "start of frame."]
        #[inline(always)]
        pub const fn set_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "reset request."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "reset request."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Suspend mode request."]
        #[must_use]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend mode request."]
        #[inline(always)]
        pub const fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn wkup(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup."]
        #[inline(always)]
        pub const fn set_wkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Error."]
        #[must_use]
        #[inline(always)]
        pub const fn err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Error."]
        #[inline(always)]
        pub const fn set_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Packet memory area over / underrun."]
        #[must_use]
        #[inline(always)]
        pub const fn pmaovr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Packet memory area over / underrun."]
        #[inline(always)]
        pub const fn set_pmaovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Correct transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn ctr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Correct transfer."]
        #[inline(always)]
        pub const fn set_ctr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Istr {
        #[inline(always)]
        fn default() -> Istr {
            Istr(0)
        }
    }
    impl core::fmt::Debug for Istr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Istr")
                .field("ep_id", &self.ep_id())
                .field("dir", &self.dir())
                .field("esof", &self.esof())
                .field("sof", &self.sof())
                .field("reset", &self.reset())
                .field("susp", &self.susp())
                .field("wkup", &self.wkup())
                .field("err", &self.err())
                .field("pmaovr", &self.pmaovr())
                .field("ctr", &self.ctr())
                .finish()
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dir {
        #[doc = "data transmitted by the USB peripheral to the host PC"]
        TO = 0x0,
        #[doc = "data received by the USB peripheral from the host PC"]
        FROM = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum EpType {
        #[doc = "Bulk endpoint"]
        BULK = 0x0,
        #[doc = "Control endpoint"]
        CONTROL = 0x01,
        #[doc = "Iso endpoint"]
        ISO = 0x02,
        #[doc = "Interrupt endpoint"]
        INTERRUPT = 0x03,
    }
    impl EpType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EpType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EpType {
        #[inline(always)]
        fn from(val: u8) -> EpType {
            EpType::from_bits(val)
        }
    }
    impl From<EpType> for u8 {
        #[inline(always)]
        fn from(val: EpType) -> u8 {
            EpType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stat {
        #[doc = "all requests addressed to this endpoint are ignored"]
        DISABLED = 0x0,
        #[doc = "the endpoint is stalled and all requests result in a STALL handshake"]
        STALL = 0x01,
        #[doc = "the endpoint is naked and all requests result in a NAK handshake"]
        NAK = 0x02,
        #[doc = "this endpoint is enabled, requests are ACKed"]
        VALID = 0x03,
    }
    impl Stat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stat {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stat {
        #[inline(always)]
        fn from(val: u8) -> Stat {
            Stat::from_bits(val)
        }
    }
    impl From<Stat> for u8 {
        #[inline(always)]
        fn from(val: Stat) -> u8 {
            Stat::to_bits(val)
        }
    }
}
