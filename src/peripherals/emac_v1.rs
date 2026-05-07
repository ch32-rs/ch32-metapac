#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Ethernet MAC (Synopsys DWC 10/100M + 10M PHY)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eth {
    ptr: *mut u8,
}
unsafe impl Send for Eth {}
unsafe impl Sync for Eth {}
impl Eth {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ethernet MAC configuration register"]
    #[inline(always)]
    pub const fn maccr(self) -> crate::common::Reg<regs::Maccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Ethernet MAC frame filter register"]
    #[inline(always)]
    pub const fn macffr(self) -> crate::common::Reg<regs::Macffr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Ethernet MAC hash table high register"]
    #[inline(always)]
    pub const fn machthr(self) -> crate::common::Reg<regs::Machthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Ethernet MAC hash table low register"]
    #[inline(always)]
    pub const fn machtlr(self) -> crate::common::Reg<regs::Machtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Ethernet MAC MII address register"]
    #[inline(always)]
    pub const fn macmiiar(self) -> crate::common::Reg<regs::Macmiiar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Ethernet MAC MII data register"]
    #[inline(always)]
    pub const fn macmiidr(self) -> crate::common::Reg<regs::Macmiidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Ethernet MAC flow control register"]
    #[inline(always)]
    pub const fn macfcr(self) -> crate::common::Reg<regs::Macfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Ethernet MAC VLAN tag register"]
    #[inline(always)]
    pub const fn macvlantr(self) -> crate::common::Reg<regs::Macvlantr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Ethernet MAC remote wakeup frame filter register"]
    #[inline(always)]
    pub const fn macrwuffr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Ethernet MAC PMT control and status register"]
    #[inline(always)]
    pub const fn macpmtcsr(self) -> crate::common::Reg<regs::Macpmtcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Ethernet MAC interrupt status register"]
    #[inline(always)]
    pub const fn macsr(self) -> crate::common::Reg<regs::Macsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Ethernet MAC interrupt mask register"]
    #[inline(always)]
    pub const fn macimr(self) -> crate::common::Reg<regs::Macimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Ethernet MAC address 0 high register"]
    #[inline(always)]
    pub const fn maca0hr(self) -> crate::common::Reg<regs::Maca0hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Ethernet MAC address 0 low register"]
    #[inline(always)]
    pub const fn maca0lr(self) -> crate::common::Reg<regs::Maca0lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Ethernet MAC address 1 high register"]
    #[inline(always)]
    pub const fn maca1hr(self) -> crate::common::Reg<regs::Maca1hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Ethernet MAC address 1 low register"]
    #[inline(always)]
    pub const fn maca1lr(self) -> crate::common::Reg<regs::Maca1lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Ethernet MAC address 2 high register"]
    #[inline(always)]
    pub const fn maca2hr(self) -> crate::common::Reg<regs::Maca2hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Ethernet MAC address 2 low register"]
    #[inline(always)]
    pub const fn maca2lr(self) -> crate::common::Reg<regs::Maca2lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Ethernet MAC address 3 high register"]
    #[inline(always)]
    pub const fn maca3hr(self) -> crate::common::Reg<regs::Maca3hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Ethernet MAC address 3 low register"]
    #[inline(always)]
    pub const fn maca3lr(self) -> crate::common::Reg<regs::Maca3lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Ethernet DMA bus mode register"]
    #[inline(always)]
    pub const fn dmabmr(self) -> crate::common::Reg<regs::Dmabmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
    #[doc = "Ethernet DMA transmit poll demand register"]
    #[inline(always)]
    pub const fn dmatpdr(self) -> crate::common::Reg<regs::Dmatpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1004usize) as _) }
    }
    #[doc = "Ethernet DMA receive poll demand register"]
    #[inline(always)]
    pub const fn dmarpdr(self) -> crate::common::Reg<regs::Dmarpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1008usize) as _) }
    }
    #[doc = "Ethernet DMA receive descriptor list address register"]
    #[inline(always)]
    pub const fn dmardlar(self) -> crate::common::Reg<regs::Dmardlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x100cusize) as _) }
    }
    #[doc = "Ethernet DMA transmit descriptor list address register"]
    #[inline(always)]
    pub const fn dmatdlar(self) -> crate::common::Reg<regs::Dmatdlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1010usize) as _) }
    }
    #[doc = "Ethernet DMA status register"]
    #[inline(always)]
    pub const fn dmasr(self) -> crate::common::Reg<regs::Dmasr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1014usize) as _) }
    }
    #[doc = "Ethernet DMA operation mode register"]
    #[inline(always)]
    pub const fn dmaomr(self) -> crate::common::Reg<regs::Dmaomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1018usize) as _) }
    }
    #[doc = "Ethernet DMA interrupt enable register"]
    #[inline(always)]
    pub const fn dmaier(self) -> crate::common::Reg<regs::Dmaier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x101cusize) as _) }
    }
    #[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
    #[inline(always)]
    pub const fn dmamfbocr(self) -> crate::common::Reg<regs::Dmamfbocr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1020usize) as _) }
    }
    #[doc = "Ethernet DMA current host transmit descriptor register"]
    #[inline(always)]
    pub const fn dmachtdr(self) -> crate::common::Reg<regs::Dmachtdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1048usize) as _) }
    }
    #[doc = "Ethernet DMA current host receive descriptor register"]
    #[inline(always)]
    pub const fn dmachrdr(self) -> crate::common::Reg<regs::Dmachrdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x104cusize) as _) }
    }
    #[doc = "Ethernet DMA current host transmit buffer address register"]
    #[inline(always)]
    pub const fn dmachtbar(self) -> crate::common::Reg<regs::Dmachtbar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1050usize) as _) }
    }
    #[doc = "Ethernet DMA current host receive buffer address register"]
    #[inline(always)]
    pub const fn dmachrbar(self) -> crate::common::Reg<regs::Dmachrbar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1054usize) as _) }
    }
}
pub mod regs {
    #[doc = "Ethernet DMA bus mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmabmr(pub u32);
    impl Dmabmr {
        #[doc = "Software reset"]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset"]
        #[inline(always)]
        pub fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA arbitration"]
        #[inline(always)]
        pub const fn da(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA arbitration"]
        #[inline(always)]
        pub fn set_da(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Descriptor skip length"]
        #[inline(always)]
        pub const fn dsl(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "Descriptor skip length"]
        #[inline(always)]
        pub fn set_dsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "Programmable burst length"]
        #[inline(always)]
        pub const fn pbl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Programmable burst length"]
        #[inline(always)]
        pub fn set_pbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Rx Tx priority ratio"]
        #[inline(always)]
        pub const fn rtpr(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Rx Tx priority ratio"]
        #[inline(always)]
        pub fn set_rtpr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Fixed burst"]
        #[inline(always)]
        pub const fn fb(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fixed burst"]
        #[inline(always)]
        pub fn set_fb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Rx DMA PBL"]
        #[inline(always)]
        pub const fn rdp(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx DMA PBL"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
        }
        #[doc = "Use separate PBL"]
        #[inline(always)]
        pub const fn usp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Use separate PBL"]
        #[inline(always)]
        pub fn set_usp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "4xPBL mode"]
        #[inline(always)]
        pub const fn fpm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "4xPBL mode"]
        #[inline(always)]
        pub fn set_fpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Address-aligned beats"]
        #[inline(always)]
        pub const fn aab(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Address-aligned beats"]
        #[inline(always)]
        pub fn set_aab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Dmabmr {
        #[inline(always)]
        fn default() -> Dmabmr {
            Dmabmr(0)
        }
    }
    #[doc = "Ethernet DMA current host receive buffer address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachrbar(pub u32);
    impl Dmachrbar {
        #[doc = "Host receive buffer address pointer"]
        #[inline(always)]
        pub const fn hrbap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host receive buffer address pointer"]
        #[inline(always)]
        pub fn set_hrbap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachrbar {
        #[inline(always)]
        fn default() -> Dmachrbar {
            Dmachrbar(0)
        }
    }
    #[doc = "Ethernet DMA current host receive descriptor register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachrdr(pub u32);
    impl Dmachrdr {
        #[doc = "Host receive descriptor address pointer"]
        #[inline(always)]
        pub const fn hrdap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host receive descriptor address pointer"]
        #[inline(always)]
        pub fn set_hrdap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachrdr {
        #[inline(always)]
        fn default() -> Dmachrdr {
            Dmachrdr(0)
        }
    }
    #[doc = "Ethernet DMA current host transmit buffer address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachtbar(pub u32);
    impl Dmachtbar {
        #[doc = "Host transmit buffer address pointer"]
        #[inline(always)]
        pub const fn htbap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host transmit buffer address pointer"]
        #[inline(always)]
        pub fn set_htbap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachtbar {
        #[inline(always)]
        fn default() -> Dmachtbar {
            Dmachtbar(0)
        }
    }
    #[doc = "Ethernet DMA current host transmit descriptor register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachtdr(pub u32);
    impl Dmachtdr {
        #[doc = "Host transmit descriptor address pointer"]
        #[inline(always)]
        pub const fn htdap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host transmit descriptor address pointer"]
        #[inline(always)]
        pub fn set_htdap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachtdr {
        #[inline(always)]
        fn default() -> Dmachtdr {
            Dmachtdr(0)
        }
    }
    #[doc = "Ethernet DMA interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaier(pub u32);
    impl Dmaier {
        #[doc = "Transmit interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit process stopped interrupt enable"]
        #[inline(always)]
        pub const fn tpsie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit process stopped interrupt enable"]
        #[inline(always)]
        pub fn set_tpsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit buffer unavailable interrupt enable"]
        #[inline(always)]
        pub const fn tbuie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer unavailable interrupt enable"]
        #[inline(always)]
        pub fn set_tbuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit jabber timeout interrupt enable"]
        #[inline(always)]
        pub const fn tjtie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit jabber timeout interrupt enable"]
        #[inline(always)]
        pub fn set_tjtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Overflow interrupt enable"]
        #[inline(always)]
        pub const fn roie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow interrupt enable"]
        #[inline(always)]
        pub fn set_roie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Underflow interrupt enable"]
        #[inline(always)]
        pub const fn tuie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Underflow interrupt enable"]
        #[inline(always)]
        pub fn set_tuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive interrupt enable"]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive interrupt enable"]
        #[inline(always)]
        pub fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive buffer unavailable interrupt enable"]
        #[inline(always)]
        pub const fn rbuie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer unavailable interrupt enable"]
        #[inline(always)]
        pub fn set_rbuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive process stopped interrupt enable"]
        #[inline(always)]
        pub const fn rpsie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive process stopped interrupt enable"]
        #[inline(always)]
        pub fn set_rpsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive watchdog timeout interrupt enable"]
        #[inline(always)]
        pub const fn rwtie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive watchdog timeout interrupt enable"]
        #[inline(always)]
        pub fn set_rwtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early transmit interrupt enable"]
        #[inline(always)]
        pub const fn etie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early transmit interrupt enable"]
        #[inline(always)]
        pub fn set_etie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fatal bus error interrupt enable"]
        #[inline(always)]
        pub const fn fbeie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal bus error interrupt enable"]
        #[inline(always)]
        pub fn set_fbeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Early receive interrupt enable"]
        #[inline(always)]
        pub const fn erie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early receive interrupt enable"]
        #[inline(always)]
        pub fn set_erie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal interrupt summary enable"]
        #[inline(always)]
        pub const fn aise(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal interrupt summary enable"]
        #[inline(always)]
        pub fn set_aise(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal interrupt summary enable"]
        #[inline(always)]
        pub const fn nise(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal interrupt summary enable"]
        #[inline(always)]
        pub fn set_nise(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "10M PHY link status change interrupt enable"]
        #[inline(always)]
        pub const fn iple(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "10M PHY link status change interrupt enable"]
        #[inline(always)]
        pub fn set_iple(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmaier {
        #[inline(always)]
        fn default() -> Dmaier {
            Dmaier(0)
        }
    }
    #[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmamfbocr(pub u32);
    impl Dmamfbocr {
        #[doc = "Missed frames by the controller"]
        #[inline(always)]
        pub const fn mfc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Missed frames by the controller"]
        #[inline(always)]
        pub fn set_mfc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Overflow bit for missed frame counter"]
        #[inline(always)]
        pub const fn omfc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow bit for missed frame counter"]
        #[inline(always)]
        pub fn set_omfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Missed frames by the application"]
        #[inline(always)]
        pub const fn mfa(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x07ff;
            val as u16
        }
        #[doc = "Missed frames by the application"]
        #[inline(always)]
        pub fn set_mfa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 17usize)) | (((val as u32) & 0x07ff) << 17usize);
        }
        #[doc = "Overflow bit for FIFO overflow counter"]
        #[inline(always)]
        pub const fn ofoc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow bit for FIFO overflow counter"]
        #[inline(always)]
        pub fn set_ofoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Dmamfbocr {
        #[inline(always)]
        fn default() -> Dmamfbocr {
            Dmamfbocr(0)
        }
    }
    #[doc = "Ethernet DMA operation mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaomr(pub u32);
    impl Dmaomr {
        #[doc = "Start/stop receive"]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start/stop receive"]
        #[inline(always)]
        pub fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Operate on second frame"]
        #[inline(always)]
        pub const fn osf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Operate on second frame"]
        #[inline(always)]
        pub fn set_osf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive threshold control"]
        #[inline(always)]
        pub const fn rtc(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Receive threshold control"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "Forward undersized good frames"]
        #[inline(always)]
        pub const fn fugf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Forward undersized good frames"]
        #[inline(always)]
        pub fn set_fugf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Forward error frames"]
        #[inline(always)]
        pub const fn fef(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Forward error frames"]
        #[inline(always)]
        pub fn set_fef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Start/stop transmission"]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Start/stop transmission"]
        #[inline(always)]
        pub fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit threshold control"]
        #[inline(always)]
        pub const fn ttc(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit threshold control"]
        #[inline(always)]
        pub fn set_ttc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Flush transmit FIFO"]
        #[inline(always)]
        pub const fn ftf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Flush transmit FIFO"]
        #[inline(always)]
        pub fn set_ftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Transmit store and forward"]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit store and forward"]
        #[inline(always)]
        pub fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Disable flushing of received frames"]
        #[inline(always)]
        pub const fn dfrf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Disable flushing of received frames"]
        #[inline(always)]
        pub fn set_dfrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Receive store and forward"]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Receive store and forward"]
        #[inline(always)]
        pub fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Dropping of TCP/IP checksum error frames disable"]
        #[inline(always)]
        pub const fn dtcefd(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Dropping of TCP/IP checksum error frames disable"]
        #[inline(always)]
        pub fn set_dtcefd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Dmaomr {
        #[inline(always)]
        fn default() -> Dmaomr {
            Dmaomr(0)
        }
    }
    #[doc = "Ethernet DMA receive descriptor list address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmardlar(pub u32);
    impl Dmardlar {
        #[doc = "Start of receive list"]
        #[inline(always)]
        pub const fn srl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of receive list"]
        #[inline(always)]
        pub fn set_srl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmardlar {
        #[inline(always)]
        fn default() -> Dmardlar {
            Dmardlar(0)
        }
    }
    #[doc = "Ethernet DMA receive poll demand register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmarpdr(pub u32);
    impl Dmarpdr {
        #[doc = "Receive poll demand"]
        #[inline(always)]
        pub const fn rpd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive poll demand"]
        #[inline(always)]
        pub fn set_rpd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmarpdr {
        #[inline(always)]
        fn default() -> Dmarpdr {
            Dmarpdr(0)
        }
    }
    #[doc = "Ethernet DMA status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmasr(pub u32);
    impl Dmasr {
        #[doc = "Transmit status"]
        #[inline(always)]
        pub const fn ts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit status"]
        #[inline(always)]
        pub fn set_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit process stopped status"]
        #[inline(always)]
        pub const fn tpss(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit process stopped status"]
        #[inline(always)]
        pub fn set_tpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit buffer unavailable status"]
        #[inline(always)]
        pub const fn tbus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer unavailable status"]
        #[inline(always)]
        pub fn set_tbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit jabber timeout status"]
        #[inline(always)]
        pub const fn tjts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit jabber timeout status"]
        #[inline(always)]
        pub fn set_tjts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive overflow status"]
        #[inline(always)]
        pub const fn ros(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive overflow status"]
        #[inline(always)]
        pub fn set_ros(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit underflow status"]
        #[inline(always)]
        pub const fn tus(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit underflow status"]
        #[inline(always)]
        pub fn set_tus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive status"]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive status"]
        #[inline(always)]
        pub fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive buffer unavailable status"]
        #[inline(always)]
        pub const fn rbus(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer unavailable status"]
        #[inline(always)]
        pub fn set_rbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive process stopped status"]
        #[inline(always)]
        pub const fn rpss(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive process stopped status"]
        #[inline(always)]
        pub fn set_rpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive watchdog timeout status"]
        #[inline(always)]
        pub const fn rwts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive watchdog timeout status"]
        #[inline(always)]
        pub fn set_rwts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early transmit status"]
        #[inline(always)]
        pub const fn ets(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early transmit status"]
        #[inline(always)]
        pub fn set_ets(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fatal bus error status"]
        #[inline(always)]
        pub const fn fbes(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal bus error status"]
        #[inline(always)]
        pub fn set_fbes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Early receive status"]
        #[inline(always)]
        pub const fn ers(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early receive status"]
        #[inline(always)]
        pub fn set_ers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal interrupt summary"]
        #[inline(always)]
        pub const fn ais(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal interrupt summary"]
        #[inline(always)]
        pub fn set_ais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal interrupt summary"]
        #[inline(always)]
        pub const fn nis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal interrupt summary"]
        #[inline(always)]
        pub fn set_nis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive process state"]
        #[inline(always)]
        pub const fn rps(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Receive process state"]
        #[inline(always)]
        pub fn set_rps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Transmit process state"]
        #[inline(always)]
        pub const fn tps(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit process state"]
        #[inline(always)]
        pub fn set_tps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Error bits status"]
        #[inline(always)]
        pub const fn ebs(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[doc = "Error bits status"]
        #[inline(always)]
        pub fn set_ebs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "MMC status"]
        #[inline(always)]
        pub const fn mmcs(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC status"]
        #[inline(always)]
        pub fn set_mmcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PMT status"]
        #[inline(always)]
        pub const fn pmts(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PMT status"]
        #[inline(always)]
        pub fn set_pmts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Time stamp trigger status"]
        #[inline(always)]
        pub const fn tsts(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp trigger status"]
        #[inline(always)]
        pub fn set_tsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "10M PHY physical layer status (write 1 to clear)"]
        #[inline(always)]
        pub const fn ipls(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "10M PHY physical layer status (write 1 to clear)"]
        #[inline(always)]
        pub fn set_ipls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmasr {
        #[inline(always)]
        fn default() -> Dmasr {
            Dmasr(0)
        }
    }
    #[doc = "Ethernet DMA transmit descriptor list address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmatdlar(pub u32);
    impl Dmatdlar {
        #[doc = "Start of transmit list"]
        #[inline(always)]
        pub const fn stl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of transmit list"]
        #[inline(always)]
        pub fn set_stl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmatdlar {
        #[inline(always)]
        fn default() -> Dmatdlar {
            Dmatdlar(0)
        }
    }
    #[doc = "Ethernet DMA transmit poll demand register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmatpdr(pub u32);
    impl Dmatpdr {
        #[doc = "Transmit poll demand"]
        #[inline(always)]
        pub const fn tpd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmit poll demand"]
        #[inline(always)]
        pub fn set_tpd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmatpdr {
        #[inline(always)]
        fn default() -> Dmatpdr {
            Dmatpdr(0)
        }
    }
    #[doc = "Ethernet MAC address 0 high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0hr(pub u32);
    impl Maca0hr {
        #[doc = "MAC address0 high"]
        #[inline(always)]
        pub const fn maca0h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address0 high"]
        #[inline(always)]
        pub fn set_maca0h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Always 1"]
        #[inline(always)]
        pub const fn mo(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Always 1"]
        #[inline(always)]
        pub fn set_mo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca0hr {
        #[inline(always)]
        fn default() -> Maca0hr {
            Maca0hr(0)
        }
    }
    #[doc = "Ethernet MAC address 0 low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0lr(pub u32);
    impl Maca0lr {
        #[doc = "MAC address0 low"]
        #[inline(always)]
        pub const fn maca0l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address0 low"]
        #[inline(always)]
        pub fn set_maca0l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca0lr {
        #[inline(always)]
        fn default() -> Maca0lr {
            Maca0lr(0)
        }
    }
    #[doc = "Ethernet MAC address 1 high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca1hr(pub u32);
    impl Maca1hr {
        #[doc = "MAC address1 high"]
        #[inline(always)]
        pub const fn maca1h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address1 high"]
        #[inline(always)]
        pub fn set_maca1h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask byte control"]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask byte control"]
        #[inline(always)]
        pub fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Source address"]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Source address"]
        #[inline(always)]
        pub fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Address enable"]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address enable"]
        #[inline(always)]
        pub fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca1hr {
        #[inline(always)]
        fn default() -> Maca1hr {
            Maca1hr(0)
        }
    }
    #[doc = "Ethernet MAC address 1 low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca1lr(pub u32);
    impl Maca1lr {
        #[doc = "MAC address1 low"]
        #[inline(always)]
        pub const fn maca1l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address1 low"]
        #[inline(always)]
        pub fn set_maca1l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca1lr {
        #[inline(always)]
        fn default() -> Maca1lr {
            Maca1lr(0)
        }
    }
    #[doc = "Ethernet MAC address 2 high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca2hr(pub u32);
    impl Maca2hr {
        #[doc = "MAC address2 high"]
        #[inline(always)]
        pub const fn maca2h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address2 high"]
        #[inline(always)]
        pub fn set_maca2h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask byte control"]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask byte control"]
        #[inline(always)]
        pub fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Source address"]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Source address"]
        #[inline(always)]
        pub fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Address enable"]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address enable"]
        #[inline(always)]
        pub fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca2hr {
        #[inline(always)]
        fn default() -> Maca2hr {
            Maca2hr(0)
        }
    }
    #[doc = "Ethernet MAC address 2 low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca2lr(pub u32);
    impl Maca2lr {
        #[doc = "MAC address2 low"]
        #[inline(always)]
        pub const fn maca2l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address2 low"]
        #[inline(always)]
        pub fn set_maca2l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca2lr {
        #[inline(always)]
        fn default() -> Maca2lr {
            Maca2lr(0)
        }
    }
    #[doc = "Ethernet MAC address 3 high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca3hr(pub u32);
    impl Maca3hr {
        #[doc = "MAC address3 high"]
        #[inline(always)]
        pub const fn maca3h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address3 high"]
        #[inline(always)]
        pub fn set_maca3h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask byte control"]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask byte control"]
        #[inline(always)]
        pub fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Source address"]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Source address"]
        #[inline(always)]
        pub fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Address enable"]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address enable"]
        #[inline(always)]
        pub fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca3hr {
        #[inline(always)]
        fn default() -> Maca3hr {
            Maca3hr(0)
        }
    }
    #[doc = "Ethernet MAC address 3 low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca3lr(pub u32);
    impl Maca3lr {
        #[doc = "MAC address3 low"]
        #[inline(always)]
        pub const fn maca3l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address3 low"]
        #[inline(always)]
        pub fn set_maca3l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca3lr {
        #[inline(always)]
        fn default() -> Maca3lr {
            Maca3lr(0)
        }
    }
    #[doc = "Ethernet MAC configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maccr(pub u32);
    impl Maccr {
        #[doc = "Send clock selection bit (10M PHY)"]
        #[inline(always)]
        pub const fn tces(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Send clock selection bit (10M PHY)"]
        #[inline(always)]
        pub fn set_tces(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Send clock reversal (10M PHY)"]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Send clock reversal (10M PHY)"]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receiver enable"]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver enable"]
        #[inline(always)]
        pub fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter enable"]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter enable"]
        #[inline(always)]
        pub fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Deferral check"]
        #[inline(always)]
        pub const fn dc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Deferral check"]
        #[inline(always)]
        pub fn set_dc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Back-off limit"]
        #[inline(always)]
        pub const fn bl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Back-off limit"]
        #[inline(always)]
        pub fn set_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Automatic pad/CRC stripping"]
        #[inline(always)]
        pub const fn apcs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic pad/CRC stripping"]
        #[inline(always)]
        pub fn set_apcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Retry disable"]
        #[inline(always)]
        pub const fn rd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Retry disable"]
        #[inline(always)]
        pub fn set_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IPv4 checksum offload"]
        #[inline(always)]
        pub const fn ipco(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IPv4 checksum offload"]
        #[inline(always)]
        pub fn set_ipco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Duplex mode"]
        #[inline(always)]
        pub const fn dm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Duplex mode"]
        #[inline(always)]
        pub fn set_dm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loopback mode"]
        #[inline(always)]
        pub const fn lm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback mode"]
        #[inline(always)]
        pub fn set_lm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Receive own disable"]
        #[inline(always)]
        pub const fn rod(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Receive own disable"]
        #[inline(always)]
        pub fn set_rod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Ethernet speed: 00=10M, 01=100M, 10=1G, 11=reserved"]
        #[inline(always)]
        pub const fn fes(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Ethernet speed: 00=10M, 01=100M, 10=1G, 11=reserved"]
        #[inline(always)]
        pub fn set_fes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Carrier sense disable"]
        #[inline(always)]
        pub const fn csd(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Carrier sense disable"]
        #[inline(always)]
        pub fn set_csd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Interframe gap"]
        #[inline(always)]
        pub const fn ifg(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Interframe gap"]
        #[inline(always)]
        pub fn set_ifg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "10M PHY 50 ohm set"]
        #[inline(always)]
        pub const fn ire(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "10M PHY 50 ohm set"]
        #[inline(always)]
        pub fn set_ire(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "10M PHY TX driver bias current"]
        #[inline(always)]
        pub const fn pdi(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "10M PHY TX driver bias current"]
        #[inline(always)]
        pub fn set_pdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Jabber disable"]
        #[inline(always)]
        pub const fn jd(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Jabber disable"]
        #[inline(always)]
        pub fn set_jd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Watchdog disable"]
        #[inline(always)]
        pub const fn wd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog disable"]
        #[inline(always)]
        pub fn set_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Send clock delay (10M PHY)"]
        #[inline(always)]
        pub const fn tcd(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "Send clock delay (10M PHY)"]
        #[inline(always)]
        pub fn set_tcd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Maccr {
        #[inline(always)]
        fn default() -> Maccr {
            Maccr(0)
        }
    }
    #[doc = "Ethernet MAC flow control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macfcr(pub u32);
    impl Macfcr {
        #[doc = "Flow control busy/back pressure activate"]
        #[inline(always)]
        pub const fn fcb_bpa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flow control busy/back pressure activate"]
        #[inline(always)]
        pub fn set_fcb_bpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit flow control enable"]
        #[inline(always)]
        pub const fn tfce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit flow control enable"]
        #[inline(always)]
        pub fn set_tfce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive flow control enable"]
        #[inline(always)]
        pub const fn rfce(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive flow control enable"]
        #[inline(always)]
        pub fn set_rfce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Unicast pause frame detect"]
        #[inline(always)]
        pub const fn upfd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast pause frame detect"]
        #[inline(always)]
        pub fn set_upfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pause low threshold"]
        #[inline(always)]
        pub const fn plt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Pause low threshold"]
        #[inline(always)]
        pub fn set_plt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Zero-quanta pause disable"]
        #[inline(always)]
        pub const fn zqpd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Zero-quanta pause disable"]
        #[inline(always)]
        pub fn set_zqpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Pause time"]
        #[inline(always)]
        pub const fn pt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Pause time"]
        #[inline(always)]
        pub fn set_pt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macfcr {
        #[inline(always)]
        fn default() -> Macfcr {
            Macfcr(0)
        }
    }
    #[doc = "Ethernet MAC frame filter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macffr(pub u32);
    impl Macffr {
        #[doc = "Promiscuous mode"]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Promiscuous mode"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Hash unicast"]
        #[inline(always)]
        pub const fn hu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Hash unicast"]
        #[inline(always)]
        pub fn set_hu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Hash multicast"]
        #[inline(always)]
        pub const fn hm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Hash multicast"]
        #[inline(always)]
        pub fn set_hm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Destination address inverse filtering"]
        #[inline(always)]
        pub const fn daif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Destination address inverse filtering"]
        #[inline(always)]
        pub fn set_daif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pass all multicast"]
        #[inline(always)]
        pub const fn pam(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pass all multicast"]
        #[inline(always)]
        pub fn set_pam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Broadcast frames disable"]
        #[inline(always)]
        pub const fn bfd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Broadcast frames disable"]
        #[inline(always)]
        pub fn set_bfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Pass control frames"]
        #[inline(always)]
        pub const fn pcf(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Pass control frames"]
        #[inline(always)]
        pub fn set_pcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Source address inverse filtering"]
        #[inline(always)]
        pub const fn saif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Source address inverse filtering"]
        #[inline(always)]
        pub fn set_saif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Source address filter"]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Source address filter"]
        #[inline(always)]
        pub fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hash or perfect filter"]
        #[inline(always)]
        pub const fn hpf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hash or perfect filter"]
        #[inline(always)]
        pub fn set_hpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receive all"]
        #[inline(always)]
        pub const fn ra(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Receive all"]
        #[inline(always)]
        pub fn set_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macffr {
        #[inline(always)]
        fn default() -> Macffr {
            Macffr(0)
        }
    }
    #[doc = "Ethernet MAC hash table high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machthr(pub u32);
    impl Machthr {
        #[doc = "Hash table high"]
        #[inline(always)]
        pub const fn hth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hash table high"]
        #[inline(always)]
        pub fn set_hth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machthr {
        #[inline(always)]
        fn default() -> Machthr {
            Machthr(0)
        }
    }
    #[doc = "Ethernet MAC hash table low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machtlr(pub u32);
    impl Machtlr {
        #[doc = "Hash table low"]
        #[inline(always)]
        pub const fn htl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hash table low"]
        #[inline(always)]
        pub fn set_htl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machtlr {
        #[inline(always)]
        fn default() -> Machtlr {
            Machtlr(0)
        }
    }
    #[doc = "Ethernet MAC interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macimr(pub u32);
    impl Macimr {
        #[doc = "PMT interrupt mask"]
        #[inline(always)]
        pub const fn pmtim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMT interrupt mask"]
        #[inline(always)]
        pub fn set_pmtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Time stamp trigger interrupt mask"]
        #[inline(always)]
        pub const fn tstim(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp trigger interrupt mask"]
        #[inline(always)]
        pub fn set_tstim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Macimr {
        #[inline(always)]
        fn default() -> Macimr {
            Macimr(0)
        }
    }
    #[doc = "Ethernet MAC MII address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmiiar(pub u32);
    impl Macmiiar {
        #[doc = "MII busy"]
        #[inline(always)]
        pub const fn mb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MII busy"]
        #[inline(always)]
        pub fn set_mb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MII write"]
        #[inline(always)]
        pub const fn mw(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MII write"]
        #[inline(always)]
        pub fn set_mw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clock range"]
        #[inline(always)]
        pub const fn cr(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "Clock range"]
        #[inline(always)]
        pub fn set_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "MII register"]
        #[inline(always)]
        pub const fn mr(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "MII register"]
        #[inline(always)]
        pub fn set_mr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "PHY address"]
        #[inline(always)]
        pub const fn pa(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "PHY address"]
        #[inline(always)]
        pub fn set_pa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
    }
    impl Default for Macmiiar {
        #[inline(always)]
        fn default() -> Macmiiar {
            Macmiiar(0)
        }
    }
    #[doc = "Ethernet MAC MII data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmiidr(pub u32);
    impl Macmiidr {
        #[doc = "MII data"]
        #[inline(always)]
        pub const fn md(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MII data"]
        #[inline(always)]
        pub fn set_md(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Macmiidr {
        #[inline(always)]
        fn default() -> Macmiidr {
            Macmiidr(0)
        }
    }
    #[doc = "Ethernet MAC PMT control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpmtcsr(pub u32);
    impl Macpmtcsr {
        #[doc = "Power down"]
        #[inline(always)]
        pub const fn pd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power down"]
        #[inline(always)]
        pub fn set_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Magic packet enable"]
        #[inline(always)]
        pub const fn mpe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Magic packet enable"]
        #[inline(always)]
        pub fn set_mpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup frame enable"]
        #[inline(always)]
        pub const fn wfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup frame enable"]
        #[inline(always)]
        pub fn set_wfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Magic packet received"]
        #[inline(always)]
        pub const fn mpr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Magic packet received"]
        #[inline(always)]
        pub fn set_mpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Wakeup frame received"]
        #[inline(always)]
        pub const fn wfr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup frame received"]
        #[inline(always)]
        pub fn set_wfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global unicast"]
        #[inline(always)]
        pub const fn gu(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global unicast"]
        #[inline(always)]
        pub fn set_gu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Wakeup frame filter register pointer reset"]
        #[inline(always)]
        pub const fn wffrpr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup frame filter register pointer reset"]
        #[inline(always)]
        pub fn set_wffrpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macpmtcsr {
        #[inline(always)]
        fn default() -> Macpmtcsr {
            Macpmtcsr(0)
        }
    }
    #[doc = "Ethernet MAC interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macsr(pub u32);
    impl Macsr {
        #[doc = "PMT status"]
        #[inline(always)]
        pub const fn pmts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMT status"]
        #[inline(always)]
        pub fn set_pmts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC status"]
        #[inline(always)]
        pub const fn mmcs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC status"]
        #[inline(always)]
        pub fn set_mmcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC receive status"]
        #[inline(always)]
        pub const fn mmcrs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC receive status"]
        #[inline(always)]
        pub fn set_mmcrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC transmit status"]
        #[inline(always)]
        pub const fn mmcts(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC transmit status"]
        #[inline(always)]
        pub fn set_mmcts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Time stamp trigger status"]
        #[inline(always)]
        pub const fn tsts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp trigger status"]
        #[inline(always)]
        pub fn set_tsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Macsr {
        #[inline(always)]
        fn default() -> Macsr {
            Macsr(0)
        }
    }
    #[doc = "Ethernet MAC VLAN tag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvlantr(pub u32);
    impl Macvlantr {
        #[doc = "VLAN tag identifier (for receive frames)"]
        #[inline(always)]
        pub const fn vlanti(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN tag identifier (for receive frames)"]
        #[inline(always)]
        pub fn set_vlanti(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "12-bit VLAN tag comparison"]
        #[inline(always)]
        pub const fn vlantc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "12-bit VLAN tag comparison"]
        #[inline(always)]
        pub fn set_vlantc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Macvlantr {
        #[inline(always)]
        fn default() -> Macvlantr {
            Macvlantr(0)
        }
    }
}
