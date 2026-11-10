#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Ethernet MAC, integrated 10/100M PHY control, MMC and IEEE 1588."]
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
    #[doc = "MAC control register. `FES=0b10` selects gigabit; `TCD` delays the RGMII transmit clock by 0.5 ns per step."]
    #[inline(always)]
    pub const fn maccr(self) -> crate::common::Reg<regs::Maccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MAC frame filter register."]
    #[inline(always)]
    pub const fn macffr(self) -> crate::common::Reg<regs::Macffr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MAC hash table high register."]
    #[inline(always)]
    pub const fn machthr(self) -> crate::common::Reg<regs::Machthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MAC hash table low register."]
    #[inline(always)]
    pub const fn machtlr(self) -> crate::common::Reg<regs::Machtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MII address register."]
    #[inline(always)]
    pub const fn macmiiar(self) -> crate::common::Reg<regs::Macmiiar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "MII data register."]
    #[inline(always)]
    pub const fn macmiidr(self) -> crate::common::Reg<regs::Macmiidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "MAC flow control register."]
    #[inline(always)]
    pub const fn macfcr(self) -> crate::common::Reg<regs::Macfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "VLAN tag register."]
    #[inline(always)]
    pub const fn macvlan(self) -> crate::common::Reg<regs::Macvlan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Remote wake-up frame filter register. Eight consecutive accesses reach all eight filters."]
    #[inline(always)]
    pub const fn macrwuffr(self) -> crate::common::Reg<regs::Macrwuffr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PMT control and status register."]
    #[inline(always)]
    pub const fn macpmtcsr(self) -> crate::common::Reg<regs::Macpmtcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "MAC interrupt status register."]
    #[inline(always)]
    pub const fn macsr(self) -> crate::common::Reg<regs::Macsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "MAC interrupt mask register."]
    #[inline(always)]
    pub const fn macimr(self) -> crate::common::Reg<regs::Macimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "MAC address 0 high register."]
    #[inline(always)]
    pub const fn maca0hr(self) -> crate::common::Reg<regs::Maca0hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "MAC address 0 low register."]
    #[inline(always)]
    pub const fn maca0lr(self) -> crate::common::Reg<regs::Maca0lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "MAC address 1 high register."]
    #[inline(always)]
    pub const fn maca1hr(self) -> crate::common::Reg<regs::Maca1hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "MAC address 1 low register."]
    #[inline(always)]
    pub const fn maca1lr(self) -> crate::common::Reg<regs::Maca1lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "MAC address 2 high register."]
    #[inline(always)]
    pub const fn maca2hr(self) -> crate::common::Reg<regs::Maca2hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "MAC address 2 low register."]
    #[inline(always)]
    pub const fn maca2lr(self) -> crate::common::Reg<regs::Maca2lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "MAC address 3 high register."]
    #[inline(always)]
    pub const fn maca3hr(self) -> crate::common::Reg<regs::Maca3hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "MAC address 3 low register."]
    #[inline(always)]
    pub const fn maca3lr(self) -> crate::common::Reg<regs::Maca3lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Integrated PHY control and status (the SDK calls this MACPHYCR)."]
    #[inline(always)]
    pub const fn phy_cfgr(self) -> crate::common::Reg<regs::PhyCfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "MMC control register."]
    #[inline(always)]
    pub const fn mmccr(self) -> crate::common::Reg<regs::Mmccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "MMC receive interrupt register."]
    #[inline(always)]
    pub const fn mmcrir(self) -> crate::common::Reg<regs::Mmcrir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "MMC transmit interrupt register."]
    #[inline(always)]
    pub const fn mmctir(self) -> crate::common::Reg<regs::Mmctir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "MMC receive interrupt mask register."]
    #[inline(always)]
    pub const fn mmcrimr(self) -> crate::common::Reg<regs::Mmcrimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "MMC transmit interrupt mask register."]
    #[inline(always)]
    pub const fn mmctimr(self) -> crate::common::Reg<regs::Mmctimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Frames transmitted after a single collision, half duplex."]
    #[inline(always)]
    pub const fn mmctgfsccr(self) -> crate::common::Reg<regs::Mmctgfsccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Frames transmitted after more than one collision, half duplex."]
    #[inline(always)]
    pub const fn mmctgfmsccr(self) -> crate::common::Reg<regs::Mmctgfmsccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Good frames transmitted."]
    #[inline(always)]
    pub const fn mmctgfcr(self) -> crate::common::Reg<regs::Mmctgfcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Frames received with a CRC error."]
    #[inline(always)]
    pub const fn mmcrfcecr(self) -> crate::common::Reg<regs::Mmcrfcecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Frames received with an alignment error."]
    #[inline(always)]
    pub const fn mmcrfaecr(self) -> crate::common::Reg<regs::Mmcrfaecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "All frames received, errored or not."]
    #[inline(always)]
    pub const fn mmcrafcr(self) -> crate::common::Reg<regs::Mmcrafcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Good unicast frames received."]
    #[inline(always)]
    pub const fn mmcrgufcr(self) -> crate::common::Reg<regs::Mmcrgufcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "PTP timestamp control register."]
    #[inline(always)]
    pub const fn ptptscr(self) -> crate::common::Reg<regs::Ptptscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "PTP sub-second increment register."]
    #[inline(always)]
    pub const fn ptpssir(self) -> crate::common::Reg<regs::Ptpssir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "PTP system time, seconds."]
    #[inline(always)]
    pub const fn ptptshr(self) -> crate::common::Reg<regs::Ptptshr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "PTP system time, sub-seconds."]
    #[inline(always)]
    pub const fn ptptslr(self) -> crate::common::Reg<regs::Ptptslr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "PTP timestamp update, seconds."]
    #[inline(always)]
    pub const fn ptptshur(self) -> crate::common::Reg<regs::Ptptshur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "PTP timestamp update, sub-seconds."]
    #[inline(always)]
    pub const fn ptptslur(self) -> crate::common::Reg<regs::Ptptslur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0714usize) as _) }
    }
    #[doc = "PTP timestamp addend."]
    #[inline(always)]
    pub const fn ptptsar(self) -> crate::common::Reg<regs::Ptptsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0718usize) as _) }
    }
    #[doc = "PTP target time, seconds."]
    #[inline(always)]
    pub const fn ptptthr(self) -> crate::common::Reg<regs::Ptptthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x071cusize) as _) }
    }
    #[doc = "PTP target time, sub-seconds."]
    #[inline(always)]
    pub const fn ptpttlr(self) -> crate::common::Reg<regs::Ptpttlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "DMA bus mode register."]
    #[inline(always)]
    pub const fn dmabmr(self) -> crate::common::Reg<regs::Dmabmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "DMA transmit poll demand (the SDK spells the field `TPD`)."]
    #[inline(always)]
    pub const fn dmatpdr(self) -> crate::common::Reg<regs::Dmatpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "DMA receive poll demand (the SDK spells the field `RPD`)."]
    #[inline(always)]
    pub const fn dmarpdr(self) -> crate::common::Reg<regs::Dmarpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "DMA receive descriptor list address. Descriptors are 32-byte aligned."]
    #[inline(always)]
    pub const fn dmardlar(self) -> crate::common::Reg<regs::Dmardlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "DMA transmit descriptor list address. Descriptors are 32-byte aligned."]
    #[inline(always)]
    pub const fn dmatdlar(self) -> crate::common::Reg<regs::Dmatdlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "DMA status register. Status bits are write-1-to-clear."]
    #[inline(always)]
    pub const fn dmasr(self) -> crate::common::Reg<regs::Dmasr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "DMA operation mode register."]
    #[inline(always)]
    pub const fn dmaomr(self) -> crate::common::Reg<regs::Dmaomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "DMA interrupt enable register."]
    #[inline(always)]
    pub const fn dmaier(self) -> crate::common::Reg<regs::Dmaier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "DMA missed frame and buffer overflow counter."]
    #[inline(always)]
    pub const fn dmamfbocr(self) -> crate::common::Reg<regs::Dmamfbocr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "DMA current transmit descriptor address (the SDK spells the field `HTDAP`)."]
    #[inline(always)]
    pub const fn dmachtdr(self) -> crate::common::Reg<regs::Dmachtdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "DMA current receive descriptor address (the SDK spells the field `HRDAP`)."]
    #[inline(always)]
    pub const fn dmachrdr(self) -> crate::common::Reg<regs::Dmachrdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "DMA current transmit buffer address (the SDK spells the field `HTBAP`)."]
    #[inline(always)]
    pub const fn dmachtbar(self) -> crate::common::Reg<regs::Dmachtbar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "DMA current receive buffer address (the SDK spells the field `HRBAP`)."]
    #[inline(always)]
    pub const fn dmachrbar(self) -> crate::common::Reg<regs::Dmachrbar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
}
pub mod regs {
    #[doc = "DMA bus mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmabmr(pub u32);
    impl Dmabmr {
        #[doc = "Software reset; self-clears when the reset completes."]
        #[must_use]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset; self-clears when the reset completes."]
        #[inline(always)]
        pub const fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Descriptor skip length in 32-bit words."]
        #[must_use]
        #[inline(always)]
        pub const fn dsl(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "Descriptor skip length in 32-bit words."]
        #[inline(always)]
        pub const fn set_dsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "1 = enhanced descriptors, 0 = normal descriptors."]
        #[must_use]
        #[inline(always)]
        pub const fn dfm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1 = enhanced descriptors, 0 = normal descriptors."]
        #[inline(always)]
        pub const fn set_dfm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Reset the DMA transmit datapath."]
        #[must_use]
        #[inline(always)]
        pub const fn dmatx_rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Reset the DMA transmit datapath."]
        #[inline(always)]
        pub const fn set_dmatx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset the DMA receive datapath."]
        #[must_use]
        #[inline(always)]
        pub const fn dmarx_rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset the DMA receive datapath."]
        #[inline(always)]
        pub const fn set_dmarx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reset the MAC transmitter."]
        #[must_use]
        #[inline(always)]
        pub const fn etx_rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset the MAC transmitter."]
        #[inline(always)]
        pub const fn set_etx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Reset the MAC receiver."]
        #[must_use]
        #[inline(always)]
        pub const fn erx_rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Reset the MAC receiver."]
        #[inline(always)]
        pub const fn set_erx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmabmr {
        #[inline(always)]
        fn default() -> Dmabmr {
            Dmabmr(0)
        }
    }
    impl core::fmt::Debug for Dmabmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmabmr")
                .field("sr", &self.sr())
                .field("dsl", &self.dsl())
                .field("dfm", &self.dfm())
                .field("dmatx_rst", &self.dmatx_rst())
                .field("dmarx_rst", &self.dmarx_rst())
                .field("etx_rst", &self.etx_rst())
                .field("erx_rst", &self.erx_rst())
                .finish()
        }
    }
    #[doc = "DMA current receive buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachrbar(pub u32);
    impl Dmachrbar {
        #[doc = "Buffer being processed."]
        #[must_use]
        #[inline(always)]
        pub const fn hrbar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer being processed."]
        #[inline(always)]
        pub const fn set_hrbar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachrbar {
        #[inline(always)]
        fn default() -> Dmachrbar {
            Dmachrbar(0)
        }
    }
    impl core::fmt::Debug for Dmachrbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachrbar")
                .field("hrbar", &self.hrbar())
                .finish()
        }
    }
    #[doc = "DMA current receive descriptor address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachrdr(pub u32);
    impl Dmachrdr {
        #[doc = "Descriptor being processed."]
        #[must_use]
        #[inline(always)]
        pub const fn hrdar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Descriptor being processed."]
        #[inline(always)]
        pub const fn set_hrdar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachrdr {
        #[inline(always)]
        fn default() -> Dmachrdr {
            Dmachrdr(0)
        }
    }
    impl core::fmt::Debug for Dmachrdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachrdr")
                .field("hrdar", &self.hrdar())
                .finish()
        }
    }
    #[doc = "DMA current transmit buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachtbar(pub u32);
    impl Dmachtbar {
        #[doc = "Buffer being processed."]
        #[must_use]
        #[inline(always)]
        pub const fn htbar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer being processed."]
        #[inline(always)]
        pub const fn set_htbar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachtbar {
        #[inline(always)]
        fn default() -> Dmachtbar {
            Dmachtbar(0)
        }
    }
    impl core::fmt::Debug for Dmachtbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachtbar")
                .field("htbar", &self.htbar())
                .finish()
        }
    }
    #[doc = "DMA current transmit descriptor address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachtdr(pub u32);
    impl Dmachtdr {
        #[doc = "Descriptor being processed."]
        #[must_use]
        #[inline(always)]
        pub const fn htdar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Descriptor being processed."]
        #[inline(always)]
        pub const fn set_htdar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachtdr {
        #[inline(always)]
        fn default() -> Dmachtdr {
            Dmachtdr(0)
        }
    }
    impl core::fmt::Debug for Dmachtdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachtdr")
                .field("htdar", &self.htdar())
                .finish()
        }
    }
    #[doc = "DMA interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaier(pub u32);
    impl Dmaier {
        #[doc = "Transmit interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit interrupt."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit process stopped interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn tpsie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit process stopped interrupt."]
        #[inline(always)]
        pub const fn set_tpsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit buffer unavailable interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn tbuie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer unavailable interrupt."]
        #[inline(always)]
        pub const fn set_tbuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit jabber timeout interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn tjtie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit jabber timeout interrupt."]
        #[inline(always)]
        pub const fn set_tjtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive overflow interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn roie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive overflow interrupt."]
        #[inline(always)]
        pub const fn set_roie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit underflow interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn tuie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit underflow interrupt."]
        #[inline(always)]
        pub const fn set_tuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive interrupt."]
        #[inline(always)]
        pub const fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive buffer unavailable interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rbuie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer unavailable interrupt."]
        #[inline(always)]
        pub const fn set_rbuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive process stopped interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rpsie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive process stopped interrupt."]
        #[inline(always)]
        pub const fn set_rpsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive watchdog timeout interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rwtie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive watchdog timeout interrupt."]
        #[inline(always)]
        pub const fn set_rwtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early transmit interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn etie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early transmit interrupt."]
        #[inline(always)]
        pub const fn set_etie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Integrated PHY link status change interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ple(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Integrated PHY link status change interrupt."]
        #[inline(always)]
        pub const fn set_ple(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Early receive interrupt (the SDK calls this `ERIE`)."]
        #[must_use]
        #[inline(always)]
        pub const fn ers(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early receive interrupt (the SDK calls this `ERIE`)."]
        #[inline(always)]
        pub const fn set_ers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal interrupt summary."]
        #[must_use]
        #[inline(always)]
        pub const fn aise(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal interrupt summary."]
        #[inline(always)]
        pub const fn set_aise(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal interrupt summary."]
        #[must_use]
        #[inline(always)]
        pub const fn nise(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal interrupt summary."]
        #[inline(always)]
        pub const fn set_nise(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Dmaier {
        #[inline(always)]
        fn default() -> Dmaier {
            Dmaier(0)
        }
    }
    impl core::fmt::Debug for Dmaier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaier")
                .field("tie", &self.tie())
                .field("tpsie", &self.tpsie())
                .field("tbuie", &self.tbuie())
                .field("tjtie", &self.tjtie())
                .field("roie", &self.roie())
                .field("tuie", &self.tuie())
                .field("rie", &self.rie())
                .field("rbuie", &self.rbuie())
                .field("rpsie", &self.rpsie())
                .field("rwtie", &self.rwtie())
                .field("etie", &self.etie())
                .field("ple", &self.ple())
                .field("ers", &self.ers())
                .field("aise", &self.aise())
                .field("nise", &self.nise())
                .finish()
        }
    }
    #[doc = "DMA missed frame and buffer overflow counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmamfbocr(pub u32);
    impl Dmamfbocr {
        #[doc = "Frames missed because the receive buffer was unavailable."]
        #[must_use]
        #[inline(always)]
        pub const fn mfc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Frames missed because the receive buffer was unavailable."]
        #[inline(always)]
        pub const fn set_mfc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The MFC counter overflowed."]
        #[must_use]
        #[inline(always)]
        pub const fn omfc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "The MFC counter overflowed."]
        #[inline(always)]
        pub const fn set_omfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Frames missed by the application."]
        #[must_use]
        #[inline(always)]
        pub const fn mfa(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x07ff;
            val as u16
        }
        #[doc = "Frames missed by the application."]
        #[inline(always)]
        pub const fn set_mfa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 17usize)) | (((val as u32) & 0x07ff) << 17usize);
        }
        #[doc = "FIFO overflow counter overflowed."]
        #[must_use]
        #[inline(always)]
        pub const fn ofoc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow counter overflowed."]
        #[inline(always)]
        pub const fn set_ofoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Dmamfbocr {
        #[inline(always)]
        fn default() -> Dmamfbocr {
            Dmamfbocr(0)
        }
    }
    impl core::fmt::Debug for Dmamfbocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmamfbocr")
                .field("mfc", &self.mfc())
                .field("omfc", &self.omfc())
                .field("mfa", &self.mfa())
                .field("ofoc", &self.ofoc())
                .finish()
        }
    }
    #[doc = "DMA operation mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaomr(pub u32);
    impl Dmaomr {
        #[doc = "Start or stop the receive process."]
        #[must_use]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start or stop the receive process."]
        #[inline(always)]
        pub const fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Forward undersized good frames."]
        #[must_use]
        #[inline(always)]
        pub const fn fugf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Forward undersized good frames."]
        #[inline(always)]
        pub const fn set_fugf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Forward errored frames."]
        #[must_use]
        #[inline(always)]
        pub const fn fef(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Forward errored frames."]
        #[inline(always)]
        pub const fn set_fef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Start or stop the transmit process."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Start or stop the transmit process."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Flush the transmit FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn ftf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Flush the transmit FIFO."]
        #[inline(always)]
        pub const fn set_ftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Transmit store-and-forward."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit store-and-forward."]
        #[inline(always)]
        pub const fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Do not drop frames with a TCP/IP checksum error."]
        #[must_use]
        #[inline(always)]
        pub const fn dtcefd(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Do not drop frames with a TCP/IP checksum error."]
        #[inline(always)]
        pub const fn set_dtcefd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Dmaomr {
        #[inline(always)]
        fn default() -> Dmaomr {
            Dmaomr(0)
        }
    }
    impl core::fmt::Debug for Dmaomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaomr")
                .field("sr", &self.sr())
                .field("fugf", &self.fugf())
                .field("fef", &self.fef())
                .field("st", &self.st())
                .field("ftf", &self.ftf())
                .field("tsf", &self.tsf())
                .field("dtcefd", &self.dtcefd())
                .finish()
        }
    }
    #[doc = "DMA receive descriptor list address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmardlar(pub u32);
    impl Dmardlar {
        #[doc = "Address of the first receive descriptor."]
        #[must_use]
        #[inline(always)]
        pub const fn rdlar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Address of the first receive descriptor."]
        #[inline(always)]
        pub const fn set_rdlar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmardlar {
        #[inline(always)]
        fn default() -> Dmardlar {
            Dmardlar(0)
        }
    }
    impl core::fmt::Debug for Dmardlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmardlar")
                .field("rdlar", &self.rdlar())
                .finish()
        }
    }
    #[doc = "DMA receive poll demand."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmarpdr(pub u32);
    impl Dmarpdr {
        #[doc = "Write any value to resume a suspended receive process."]
        #[must_use]
        #[inline(always)]
        pub const fn rpdr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write any value to resume a suspended receive process."]
        #[inline(always)]
        pub const fn set_rpdr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmarpdr {
        #[inline(always)]
        fn default() -> Dmarpdr {
            Dmarpdr(0)
        }
    }
    impl core::fmt::Debug for Dmarpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmarpdr")
                .field("rpdr", &self.rpdr())
                .finish()
        }
    }
    #[doc = "DMA status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmasr(pub u32);
    impl Dmasr {
        #[doc = "Transmit finished."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit finished."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit process stopped."]
        #[must_use]
        #[inline(always)]
        pub const fn tpss(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit process stopped."]
        #[inline(always)]
        pub const fn set_tpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit buffer unavailable."]
        #[must_use]
        #[inline(always)]
        pub const fn tbus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer unavailable."]
        #[inline(always)]
        pub const fn set_tbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit jabber timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn tjts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit jabber timeout."]
        #[inline(always)]
        pub const fn set_tjts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ros(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive overflow."]
        #[inline(always)]
        pub const fn set_ros(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn tus(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit underflow."]
        #[inline(always)]
        pub const fn set_tus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive finished."]
        #[must_use]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive finished."]
        #[inline(always)]
        pub const fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive buffer unavailable."]
        #[must_use]
        #[inline(always)]
        pub const fn rbus(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer unavailable."]
        #[inline(always)]
        pub const fn set_rbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive process stopped."]
        #[must_use]
        #[inline(always)]
        pub const fn rpss(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive process stopped."]
        #[inline(always)]
        pub const fn set_rpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive watchdog timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn rwts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive watchdog timeout."]
        #[inline(always)]
        pub const fn set_rwts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early transmit."]
        #[must_use]
        #[inline(always)]
        pub const fn ets(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early transmit."]
        #[inline(always)]
        pub const fn set_ets(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Integrated PHY link status changed."]
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Integrated PHY link status changed."]
        #[inline(always)]
        pub const fn set_pls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Early receive."]
        #[must_use]
        #[inline(always)]
        pub const fn ers(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early receive."]
        #[inline(always)]
        pub const fn set_ers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal interrupt summary."]
        #[must_use]
        #[inline(always)]
        pub const fn ais(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal interrupt summary."]
        #[inline(always)]
        pub const fn set_ais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal interrupt summary."]
        #[must_use]
        #[inline(always)]
        pub const fn nis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal interrupt summary."]
        #[inline(always)]
        pub const fn set_nis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive process state."]
        #[must_use]
        #[inline(always)]
        pub const fn rps(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Receive process state."]
        #[inline(always)]
        pub const fn set_rps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Transmit process state."]
        #[must_use]
        #[inline(always)]
        pub const fn tps(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit process state."]
        #[inline(always)]
        pub const fn set_tps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Bus error type."]
        #[must_use]
        #[inline(always)]
        pub const fn ebs(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[doc = "Bus error type."]
        #[inline(always)]
        pub const fn set_ebs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "MMC event occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcs(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC event occurred."]
        #[inline(always)]
        pub const fn set_mmcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PMT event occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn pmts(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PMT event occurred."]
        #[inline(always)]
        pub const fn set_pmts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Timestamp trigger event occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn tsts(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp trigger event occurred."]
        #[inline(always)]
        pub const fn set_tsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Dmasr {
        #[inline(always)]
        fn default() -> Dmasr {
            Dmasr(0)
        }
    }
    impl core::fmt::Debug for Dmasr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmasr")
                .field("ts", &self.ts())
                .field("tpss", &self.tpss())
                .field("tbus", &self.tbus())
                .field("tjts", &self.tjts())
                .field("ros", &self.ros())
                .field("tus", &self.tus())
                .field("rs", &self.rs())
                .field("rbus", &self.rbus())
                .field("rpss", &self.rpss())
                .field("rwts", &self.rwts())
                .field("ets", &self.ets())
                .field("pls", &self.pls())
                .field("ers", &self.ers())
                .field("ais", &self.ais())
                .field("nis", &self.nis())
                .field("rps", &self.rps())
                .field("tps", &self.tps())
                .field("ebs", &self.ebs())
                .field("mmcs", &self.mmcs())
                .field("pmts", &self.pmts())
                .field("tsts", &self.tsts())
                .finish()
        }
    }
    #[doc = "DMA transmit descriptor list address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmatdlar(pub u32);
    impl Dmatdlar {
        #[doc = "Address of the first transmit descriptor."]
        #[must_use]
        #[inline(always)]
        pub const fn tdlar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Address of the first transmit descriptor."]
        #[inline(always)]
        pub const fn set_tdlar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmatdlar {
        #[inline(always)]
        fn default() -> Dmatdlar {
            Dmatdlar(0)
        }
    }
    impl core::fmt::Debug for Dmatdlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmatdlar")
                .field("tdlar", &self.tdlar())
                .finish()
        }
    }
    #[doc = "DMA transmit poll demand."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmatpdr(pub u32);
    impl Dmatpdr {
        #[doc = "Write any value to resume a suspended transmit process."]
        #[must_use]
        #[inline(always)]
        pub const fn tpdr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write any value to resume a suspended transmit process."]
        #[inline(always)]
        pub const fn set_tpdr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmatpdr {
        #[inline(always)]
        fn default() -> Dmatpdr {
            Dmatpdr(0)
        }
    }
    impl core::fmt::Debug for Dmatpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmatpdr")
                .field("tpdr", &self.tpdr())
                .finish()
        }
    }
    #[doc = "MAC address 0 high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0hr(pub u32);
    impl Maca0hr {
        #[doc = "MAC address bits 47:32."]
        #[must_use]
        #[inline(always)]
        pub const fn maca0h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address bits 47:32."]
        #[inline(always)]
        pub const fn set_maca0h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Always reads 1."]
        #[must_use]
        #[inline(always)]
        pub const fn mo(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Always reads 1."]
        #[inline(always)]
        pub const fn set_mo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca0hr {
        #[inline(always)]
        fn default() -> Maca0hr {
            Maca0hr(0)
        }
    }
    impl core::fmt::Debug for Maca0hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca0hr")
                .field("maca0h", &self.maca0h())
                .field("mo", &self.mo())
                .finish()
        }
    }
    #[doc = "MAC address 0 low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0lr(pub u32);
    impl Maca0lr {
        #[doc = "MAC address bits 31:0."]
        #[must_use]
        #[inline(always)]
        pub const fn maca0l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address bits 31:0."]
        #[inline(always)]
        pub const fn set_maca0l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca0lr {
        #[inline(always)]
        fn default() -> Maca0lr {
            Maca0lr(0)
        }
    }
    impl core::fmt::Debug for Maca0lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca0lr")
                .field("maca0l", &self.maca0l())
                .finish()
        }
    }
    #[doc = "MAC address 1 high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca1hr(pub u32);
    impl Maca1hr {
        #[doc = "MAC address bits 47:32."]
        #[must_use]
        #[inline(always)]
        pub const fn maca1h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address bits 47:32."]
        #[inline(always)]
        pub const fn set_maca1h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask byte control; one bit per address byte."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask byte control; one bit per address byte."]
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Compare against the source address instead of the destination."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Compare against the source address instead of the destination."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Use this address in the perfect filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Use this address in the perfect filter."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca1hr {
        #[inline(always)]
        fn default() -> Maca1hr {
            Maca1hr(0)
        }
    }
    impl core::fmt::Debug for Maca1hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca1hr")
                .field("maca1h", &self.maca1h())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[doc = "MAC address 1 low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca1lr(pub u32);
    impl Maca1lr {
        #[doc = "MAC address bits 31:0."]
        #[must_use]
        #[inline(always)]
        pub const fn maca1l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address bits 31:0."]
        #[inline(always)]
        pub const fn set_maca1l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca1lr {
        #[inline(always)]
        fn default() -> Maca1lr {
            Maca1lr(0)
        }
    }
    impl core::fmt::Debug for Maca1lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca1lr")
                .field("maca1l", &self.maca1l())
                .finish()
        }
    }
    #[doc = "MAC address 2 high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca2hr(pub u32);
    impl Maca2hr {
        #[doc = "MAC address bits 47:32."]
        #[must_use]
        #[inline(always)]
        pub const fn maca2h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address bits 47:32."]
        #[inline(always)]
        pub const fn set_maca2h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask byte control; one bit per address byte."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask byte control; one bit per address byte."]
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Compare against the source address instead of the destination."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Compare against the source address instead of the destination."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Use this address in the perfect filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Use this address in the perfect filter."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca2hr {
        #[inline(always)]
        fn default() -> Maca2hr {
            Maca2hr(0)
        }
    }
    impl core::fmt::Debug for Maca2hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca2hr")
                .field("maca2h", &self.maca2h())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[doc = "MAC address 2 low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca2lr(pub u32);
    impl Maca2lr {
        #[doc = "MAC address bits 31:0."]
        #[must_use]
        #[inline(always)]
        pub const fn maca2l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address bits 31:0."]
        #[inline(always)]
        pub const fn set_maca2l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca2lr {
        #[inline(always)]
        fn default() -> Maca2lr {
            Maca2lr(0)
        }
    }
    impl core::fmt::Debug for Maca2lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca2lr")
                .field("maca2l", &self.maca2l())
                .finish()
        }
    }
    #[doc = "MAC address 3 high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca3hr(pub u32);
    impl Maca3hr {
        #[doc = "MAC address bits 47:32."]
        #[must_use]
        #[inline(always)]
        pub const fn maca3h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address bits 47:32."]
        #[inline(always)]
        pub const fn set_maca3h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask byte control; one bit per address byte."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask byte control; one bit per address byte."]
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Compare against the source address instead of the destination."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Compare against the source address instead of the destination."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Use this address in the perfect filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Use this address in the perfect filter."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca3hr {
        #[inline(always)]
        fn default() -> Maca3hr {
            Maca3hr(0)
        }
    }
    impl core::fmt::Debug for Maca3hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca3hr")
                .field("maca3h", &self.maca3h())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[doc = "MAC address 3 low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca3lr(pub u32);
    impl Maca3lr {
        #[doc = "MAC address bits 31:0."]
        #[must_use]
        #[inline(always)]
        pub const fn maca3l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address bits 31:0."]
        #[inline(always)]
        pub const fn set_maca3l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca3lr {
        #[inline(always)]
        fn default() -> Maca3lr {
            Maca3lr(0)
        }
    }
    impl core::fmt::Debug for Maca3lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca3lr")
                .field("maca3l", &self.maca3l())
                .finish()
        }
    }
    #[doc = "MAC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maccr(pub u32);
    impl Maccr {
        #[doc = "Transmit clock inversion."]
        #[must_use]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit clock inversion."]
        #[inline(always)]
        pub const fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receiver enable."]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver enable."]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter enable."]
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Automatic pad/CRC stripping."]
        #[must_use]
        #[inline(always)]
        pub const fn apcs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic pad/CRC stripping."]
        #[inline(always)]
        pub const fn set_apcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IPv4 checksum offload."]
        #[must_use]
        #[inline(always)]
        pub const fn ipco(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IPv4 checksum offload."]
        #[inline(always)]
        pub const fn set_ipco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Full-duplex mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Full-duplex mode."]
        #[inline(always)]
        pub const fn set_dm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loopback mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback mode."]
        #[inline(always)]
        pub const fn set_lm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Speed. 0b00 = 10M, 0b01 = 100M, 0b10 = 1G, 0b11 reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn fes(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Speed. 0b00 = 10M, 0b01 = 100M, 0b10 = 1G, 0b11 reserved."]
        #[inline(always)]
        pub const fn set_fes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Minimum inter-frame gap, from 96 bits (0) down to 40 bits (7)."]
        #[must_use]
        #[inline(always)]
        pub const fn ifg(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Minimum inter-frame gap, from 96 bits (0) down to 40 bits (7)."]
        #[inline(always)]
        pub const fn set_ifg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Jabber disable."]
        #[must_use]
        #[inline(always)]
        pub const fn jd(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Jabber disable."]
        #[inline(always)]
        pub const fn set_jd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Watchdog disable."]
        #[must_use]
        #[inline(always)]
        pub const fn wd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog disable."]
        #[inline(always)]
        pub const fn set_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Transmit clock delay, 0.5 ns per step."]
        #[must_use]
        #[inline(always)]
        pub const fn tcd(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit clock delay, 0.5 ns per step."]
        #[inline(always)]
        pub const fn set_tcd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Maccr {
        #[inline(always)]
        fn default() -> Maccr {
            Maccr(0)
        }
    }
    impl core::fmt::Debug for Maccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maccr")
                .field("tcf", &self.tcf())
                .field("re", &self.re())
                .field("te", &self.te())
                .field("apcs", &self.apcs())
                .field("ipco", &self.ipco())
                .field("dm", &self.dm())
                .field("lm", &self.lm())
                .field("fes", &self.fes())
                .field("ifg", &self.ifg())
                .field("jd", &self.jd())
                .field("wd", &self.wd())
                .field("tcd", &self.tcd())
                .finish()
        }
    }
    #[doc = "MAC flow control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macfcr(pub u32);
    impl Macfcr {
        #[doc = "Flow control busy; writing 1 sends one pause frame."]
        #[must_use]
        #[inline(always)]
        pub const fn fcb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flow control busy; writing 1 sends one pause frame."]
        #[inline(always)]
        pub const fn set_fcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit flow control enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tfce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit flow control enable."]
        #[inline(always)]
        pub const fn set_tfce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive flow control enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rfce(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive flow control enable."]
        #[inline(always)]
        pub const fn set_rfce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Unicast pause frame detect."]
        #[must_use]
        #[inline(always)]
        pub const fn upfd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast pause frame detect."]
        #[inline(always)]
        pub const fn set_upfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pause time, in units of a 64-byte slot."]
        #[must_use]
        #[inline(always)]
        pub const fn pt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Pause time, in units of a 64-byte slot."]
        #[inline(always)]
        pub const fn set_pt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macfcr {
        #[inline(always)]
        fn default() -> Macfcr {
            Macfcr(0)
        }
    }
    impl core::fmt::Debug for Macfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macfcr")
                .field("fcb", &self.fcb())
                .field("tfce", &self.tfce())
                .field("rfce", &self.rfce())
                .field("upfd", &self.upfd())
                .field("pt", &self.pt())
                .finish()
        }
    }
    #[doc = "MAC frame filter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macffr(pub u32);
    impl Macffr {
        #[doc = "Promiscuous mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Promiscuous mode."]
        #[inline(always)]
        pub const fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Hash unicast."]
        #[must_use]
        #[inline(always)]
        pub const fn hu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Hash unicast."]
        #[inline(always)]
        pub const fn set_hu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Hash multicast."]
        #[must_use]
        #[inline(always)]
        pub const fn hm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Hash multicast."]
        #[inline(always)]
        pub const fn set_hm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Destination address inverse filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn daif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Destination address inverse filtering."]
        #[inline(always)]
        pub const fn set_daif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pass all multicast."]
        #[must_use]
        #[inline(always)]
        pub const fn pam(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pass all multicast."]
        #[inline(always)]
        pub const fn set_pam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Broadcast frame disable."]
        #[must_use]
        #[inline(always)]
        pub const fn bfd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Broadcast frame disable."]
        #[inline(always)]
        pub const fn set_bfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Pass control frames."]
        #[must_use]
        #[inline(always)]
        pub const fn pcf(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Pass control frames."]
        #[inline(always)]
        pub const fn set_pcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Source address inverse filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn saif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Source address inverse filtering."]
        #[inline(always)]
        pub const fn set_saif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Source address filter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Source address filter enable."]
        #[inline(always)]
        pub const fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hash or perfect filter."]
        #[must_use]
        #[inline(always)]
        pub const fn hpf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hash or perfect filter."]
        #[inline(always)]
        pub const fn set_hpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receive all frames."]
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Receive all frames."]
        #[inline(always)]
        pub const fn set_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macffr {
        #[inline(always)]
        fn default() -> Macffr {
            Macffr(0)
        }
    }
    impl core::fmt::Debug for Macffr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macffr")
                .field("pm", &self.pm())
                .field("hu", &self.hu())
                .field("hm", &self.hm())
                .field("daif", &self.daif())
                .field("pam", &self.pam())
                .field("bfd", &self.bfd())
                .field("pcf", &self.pcf())
                .field("saif", &self.saif())
                .field("saf", &self.saf())
                .field("hpf", &self.hpf())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[doc = "MAC hash table high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machthr(pub u32);
    impl Machthr {
        #[doc = "Hash table bits 63:32."]
        #[must_use]
        #[inline(always)]
        pub const fn hth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hash table bits 63:32."]
        #[inline(always)]
        pub const fn set_hth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machthr {
        #[inline(always)]
        fn default() -> Machthr {
            Machthr(0)
        }
    }
    impl core::fmt::Debug for Machthr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machthr").field("hth", &self.hth()).finish()
        }
    }
    #[doc = "MAC hash table low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machtlr(pub u32);
    impl Machtlr {
        #[doc = "Hash table bits 31:0."]
        #[must_use]
        #[inline(always)]
        pub const fn htl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hash table bits 31:0."]
        #[inline(always)]
        pub const fn set_htl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machtlr {
        #[inline(always)]
        fn default() -> Machtlr {
            Machtlr(0)
        }
    }
    impl core::fmt::Debug for Machtlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machtlr").field("htl", &self.htl()).finish()
        }
    }
    #[doc = "MAC interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macimr(pub u32);
    impl Macimr {
        #[doc = "PMT interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn pmtim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMT interrupt mask."]
        #[inline(always)]
        pub const fn set_pmtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timestamp trigger interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn tstim(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp trigger interrupt mask."]
        #[inline(always)]
        pub const fn set_tstim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Macimr {
        #[inline(always)]
        fn default() -> Macimr {
            Macimr(0)
        }
    }
    impl core::fmt::Debug for Macimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macimr")
                .field("pmtim", &self.pmtim())
                .field("tstim", &self.tstim())
                .finish()
        }
    }
    #[doc = "MII address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmiiar(pub u32);
    impl Macmiiar {
        #[doc = "MII busy."]
        #[must_use]
        #[inline(always)]
        pub const fn mb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MII busy."]
        #[inline(always)]
        pub const fn set_mb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MII write; clear for a read."]
        #[must_use]
        #[inline(always)]
        pub const fn mw(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MII write; clear for a read."]
        #[inline(always)]
        pub const fn set_mw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MDC clock range. 0 = HCLK/42, 2 = HCLK/16, 3 = HCLK/26."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "MDC clock range. 0 = HCLK/42, 2 = HCLK/16, 3 = HCLK/26."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "PHY register address."]
        #[must_use]
        #[inline(always)]
        pub const fn mr(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "PHY register address."]
        #[inline(always)]
        pub const fn set_mr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "PHY address."]
        #[must_use]
        #[inline(always)]
        pub const fn pa(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "PHY address."]
        #[inline(always)]
        pub const fn set_pa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
    }
    impl Default for Macmiiar {
        #[inline(always)]
        fn default() -> Macmiiar {
            Macmiiar(0)
        }
    }
    impl core::fmt::Debug for Macmiiar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmiiar")
                .field("mb", &self.mb())
                .field("mw", &self.mw())
                .field("cr", &self.cr())
                .field("mr", &self.mr())
                .field("pa", &self.pa())
                .finish()
        }
    }
    #[doc = "MII data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmiidr(pub u32);
    impl Macmiidr {
        #[doc = "Data read from or written to the PHY."]
        #[must_use]
        #[inline(always)]
        pub const fn md(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Data read from or written to the PHY."]
        #[inline(always)]
        pub const fn set_md(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Macmiidr {
        #[inline(always)]
        fn default() -> Macmiidr {
            Macmiidr(0)
        }
    }
    impl core::fmt::Debug for Macmiidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmiidr").field("md", &self.md()).finish()
        }
    }
    #[doc = "PMT control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpmtcsr(pub u32);
    impl Macpmtcsr {
        #[doc = "Power down; self-clears when a wake-up frame or magic packet is received."]
        #[must_use]
        #[inline(always)]
        pub const fn pd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power down; self-clears when a wake-up frame or magic packet is received."]
        #[inline(always)]
        pub const fn set_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Magic packet enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mpe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Magic packet enable."]
        #[inline(always)]
        pub const fn set_mpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wake-up frame enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up frame enable."]
        #[inline(always)]
        pub const fn set_wfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Let a wake-up frame from the integrated PHY raise a PMT event."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_pmeb_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Let a wake-up frame from the integrated PHY raise a PMT event."]
        #[inline(always)]
        pub const fn set_phy_pmeb_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Magic packet received; clears on read."]
        #[must_use]
        #[inline(always)]
        pub const fn mpr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Magic packet received; clears on read."]
        #[inline(always)]
        pub const fn set_mpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Wake-up frame received; clears on read."]
        #[must_use]
        #[inline(always)]
        pub const fn wfr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up frame received; clears on read."]
        #[inline(always)]
        pub const fn set_wfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Integrated PHY wake-up frame received; clears on read."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_pmebr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Integrated PHY wake-up frame received; clears on read."]
        #[inline(always)]
        pub const fn set_phy_pmebr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Treat every filtered unicast frame as a wake-up frame."]
        #[must_use]
        #[inline(always)]
        pub const fn gu(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Treat every filtered unicast frame as a wake-up frame."]
        #[inline(always)]
        pub const fn set_gu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Reset the wake-up frame filter pointer; self-clears."]
        #[must_use]
        #[inline(always)]
        pub const fn wffrpr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Reset the wake-up frame filter pointer; self-clears."]
        #[inline(always)]
        pub const fn set_wffrpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macpmtcsr {
        #[inline(always)]
        fn default() -> Macpmtcsr {
            Macpmtcsr(0)
        }
    }
    impl core::fmt::Debug for Macpmtcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macpmtcsr")
                .field("pd", &self.pd())
                .field("mpe", &self.mpe())
                .field("wfe", &self.wfe())
                .field("phy_pmeb_en", &self.phy_pmeb_en())
                .field("mpr", &self.mpr())
                .field("wfr", &self.wfr())
                .field("phy_pmebr", &self.phy_pmebr())
                .field("gu", &self.gu())
                .field("wffrpr", &self.wffrpr())
                .finish()
        }
    }
    #[doc = "Remote wake-up frame filter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macrwuffr(pub u32);
    impl Macrwuffr {
        #[doc = "Wake-up frame filter data; eight sequential accesses reach all eight filters."]
        #[must_use]
        #[inline(always)]
        pub const fn rwuffr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Wake-up frame filter data; eight sequential accesses reach all eight filters."]
        #[inline(always)]
        pub const fn set_rwuffr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macrwuffr {
        #[inline(always)]
        fn default() -> Macrwuffr {
            Macrwuffr(0)
        }
    }
    impl core::fmt::Debug for Macrwuffr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macrwuffr")
                .field("rwuffr", &self.rwuffr())
                .finish()
        }
    }
    #[doc = "MAC interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macsr(pub u32);
    impl Macsr {
        #[doc = "PMT status; clears when WFR and MPR are both cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn pmts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMT status; clears when WFR and MPR are both cleared."]
        #[inline(always)]
        pub const fn set_pmts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC status."]
        #[inline(always)]
        pub const fn set_mmcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC receive status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcrs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC receive status."]
        #[inline(always)]
        pub const fn set_mmcrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC transmit status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcts(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC transmit status."]
        #[inline(always)]
        pub const fn set_mmcts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Timestamp trigger status; clears on read."]
        #[must_use]
        #[inline(always)]
        pub const fn tsts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp trigger status; clears on read."]
        #[inline(always)]
        pub const fn set_tsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Macsr {
        #[inline(always)]
        fn default() -> Macsr {
            Macsr(0)
        }
    }
    impl core::fmt::Debug for Macsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macsr")
                .field("pmts", &self.pmts())
                .field("mmcs", &self.mmcs())
                .field("mmcrs", &self.mmcrs())
                .field("mmcts", &self.mmcts())
                .field("tsts", &self.tsts())
                .finish()
        }
    }
    #[doc = "VLAN tag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvlan(pub u32);
    impl Macvlan {
        #[doc = "VLAN tag identifier to compare against."]
        #[must_use]
        #[inline(always)]
        pub const fn vlanti(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN tag identifier to compare against."]
        #[inline(always)]
        pub const fn set_vlanti(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Compare only the 12 identifier bits (the SDK calls this `VLANTC`)."]
        #[must_use]
        #[inline(always)]
        pub const fn vlant(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Compare only the 12 identifier bits (the SDK calls this `VLANTC`)."]
        #[inline(always)]
        pub const fn set_vlant(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Macvlan {
        #[inline(always)]
        fn default() -> Macvlan {
            Macvlan(0)
        }
    }
    impl core::fmt::Debug for Macvlan {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvlan")
                .field("vlanti", &self.vlanti())
                .field("vlant", &self.vlant())
                .finish()
        }
    }
    #[doc = "MMC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmccr(pub u32);
    impl Mmccr {
        #[doc = "Reset all counters; self-clears."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reset all counters; self-clears."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Stop counters at their maximum instead of rolling over."]
        #[must_use]
        #[inline(always)]
        pub const fn csr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Stop counters at their maximum instead of rolling over."]
        #[inline(always)]
        pub const fn set_csr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset a counter when it is read."]
        #[must_use]
        #[inline(always)]
        pub const fn ror(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset a counter when it is read."]
        #[inline(always)]
        pub const fn set_ror(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Freeze all counters."]
        #[must_use]
        #[inline(always)]
        pub const fn mcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Freeze all counters."]
        #[inline(always)]
        pub const fn set_mcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Mmccr {
        #[inline(always)]
        fn default() -> Mmccr {
            Mmccr(0)
        }
    }
    impl core::fmt::Debug for Mmccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmccr")
                .field("cr", &self.cr())
                .field("csr", &self.csr())
                .field("ror", &self.ror())
                .field("mcf", &self.mcf())
                .finish()
        }
    }
    #[doc = "All frames received."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrafcr(pub u32);
    impl Mmcrafcr {
        #[doc = "Frame count."]
        #[must_use]
        #[inline(always)]
        pub const fn rafcr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame count."]
        #[inline(always)]
        pub const fn set_rafcr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrafcr {
        #[inline(always)]
        fn default() -> Mmcrafcr {
            Mmcrafcr(0)
        }
    }
    impl core::fmt::Debug for Mmcrafcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrafcr")
                .field("rafcr", &self.rafcr())
                .finish()
        }
    }
    #[doc = "Frames received with an alignment error."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrfaecr(pub u32);
    impl Mmcrfaecr {
        #[doc = "Frame count."]
        #[must_use]
        #[inline(always)]
        pub const fn rfaecr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame count."]
        #[inline(always)]
        pub const fn set_rfaecr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrfaecr {
        #[inline(always)]
        fn default() -> Mmcrfaecr {
            Mmcrfaecr(0)
        }
    }
    impl core::fmt::Debug for Mmcrfaecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrfaecr")
                .field("rfaecr", &self.rfaecr())
                .finish()
        }
    }
    #[doc = "Frames received with a CRC error."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrfcecr(pub u32);
    impl Mmcrfcecr {
        #[doc = "Frame count."]
        #[must_use]
        #[inline(always)]
        pub const fn rfcecr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame count."]
        #[inline(always)]
        pub const fn set_rfcecr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrfcecr {
        #[inline(always)]
        fn default() -> Mmcrfcecr {
            Mmcrfcecr(0)
        }
    }
    impl core::fmt::Debug for Mmcrfcecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrfcecr")
                .field("rfcecr", &self.rfcecr())
                .finish()
        }
    }
    #[doc = "Good unicast frames received."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrgufcr(pub u32);
    impl Mmcrgufcr {
        #[doc = "Frame count."]
        #[must_use]
        #[inline(always)]
        pub const fn rgufcr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame count."]
        #[inline(always)]
        pub const fn set_rgufcr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrgufcr {
        #[inline(always)]
        fn default() -> Mmcrgufcr {
            Mmcrgufcr(0)
        }
    }
    impl core::fmt::Debug for Mmcrgufcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrgufcr")
                .field("rgufcr", &self.rgufcr())
                .finish()
        }
    }
    #[doc = "MMC receive interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrimr(pub u32);
    impl Mmcrimr {
        #[doc = "Mask the CRC-error counter interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rfcem(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Mask the CRC-error counter interrupt."]
        #[inline(always)]
        pub const fn set_rfcem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Mask the good unicast counter interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rgufm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Mask the good unicast counter interrupt."]
        #[inline(always)]
        pub const fn set_rgufm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Mmcrimr {
        #[inline(always)]
        fn default() -> Mmcrimr {
            Mmcrimr(0)
        }
    }
    impl core::fmt::Debug for Mmcrimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrimr")
                .field("rfcem", &self.rfcem())
                .field("rgufm", &self.rgufm())
                .finish()
        }
    }
    #[doc = "MMC receive interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrir(pub u32);
    impl Mmcrir {
        #[doc = "CRC-error frames counter reached half its range."]
        #[must_use]
        #[inline(always)]
        pub const fn rfces(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CRC-error frames counter reached half its range."]
        #[inline(always)]
        pub const fn set_rfces(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Good unicast frames counter reached half its range."]
        #[must_use]
        #[inline(always)]
        pub const fn rgufs(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Good unicast frames counter reached half its range."]
        #[inline(always)]
        pub const fn set_rgufs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Mmcrir {
        #[inline(always)]
        fn default() -> Mmcrir {
            Mmcrir(0)
        }
    }
    impl core::fmt::Debug for Mmcrir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmcrir")
                .field("rfces", &self.rfces())
                .field("rgufs", &self.rgufs())
                .finish()
        }
    }
    #[doc = "Good frames transmitted."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctgfcr(pub u32);
    impl Mmctgfcr {
        #[doc = "Frame count."]
        #[must_use]
        #[inline(always)]
        pub const fn tgfc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame count."]
        #[inline(always)]
        pub const fn set_tgfc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmctgfcr {
        #[inline(always)]
        fn default() -> Mmctgfcr {
            Mmctgfcr(0)
        }
    }
    impl core::fmt::Debug for Mmctgfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctgfcr")
                .field("tgfc", &self.tgfc())
                .finish()
        }
    }
    #[doc = "Frames transmitted after more than one collision, half duplex."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctgfmsccr(pub u32);
    impl Mmctgfmsccr {
        #[doc = "Frame count."]
        #[must_use]
        #[inline(always)]
        pub const fn tgfmsccr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame count."]
        #[inline(always)]
        pub const fn set_tgfmsccr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmctgfmsccr {
        #[inline(always)]
        fn default() -> Mmctgfmsccr {
            Mmctgfmsccr(0)
        }
    }
    impl core::fmt::Debug for Mmctgfmsccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctgfmsccr")
                .field("tgfmsccr", &self.tgfmsccr())
                .finish()
        }
    }
    #[doc = "Frames transmitted after a single collision, half duplex."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctgfsccr(pub u32);
    impl Mmctgfsccr {
        #[doc = "Frame count."]
        #[must_use]
        #[inline(always)]
        pub const fn tgfsccr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame count."]
        #[inline(always)]
        pub const fn set_tgfsccr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmctgfsccr {
        #[inline(always)]
        fn default() -> Mmctgfsccr {
            Mmctgfsccr(0)
        }
    }
    impl core::fmt::Debug for Mmctgfsccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctgfsccr")
                .field("tgfsccr", &self.tgfsccr())
                .finish()
        }
    }
    #[doc = "MMC transmit interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctimr(pub u32);
    impl Mmctimr {
        #[doc = "Mask the good transmit frame counter interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn tgfm(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Mask the good transmit frame counter interrupt."]
        #[inline(always)]
        pub const fn set_tgfm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Mmctimr {
        #[inline(always)]
        fn default() -> Mmctimr {
            Mmctimr(0)
        }
    }
    impl core::fmt::Debug for Mmctimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctimr")
                .field("tgfm", &self.tgfm())
                .finish()
        }
    }
    #[doc = "MMC transmit interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctir(pub u32);
    impl Mmctir {
        #[doc = "Good frames transmitted counter reached half its range."]
        #[must_use]
        #[inline(always)]
        pub const fn tgfs(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Good frames transmitted counter reached half its range."]
        #[inline(always)]
        pub const fn set_tgfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Mmctir {
        #[inline(always)]
        fn default() -> Mmctir {
            Mmctir(0)
        }
    }
    impl core::fmt::Debug for Mmctir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmctir")
                .field("tgfs", &self.tgfs())
                .finish()
        }
    }
    #[doc = "Integrated PHY control and status. Link status and speed here describe the internal 10M/100M PHY; under RGMII the external PHY is read over SMI instead."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCfgr(pub u32);
    impl PhyCfgr {
        #[doc = "PHY address used when `PHYADDR_EN` is set."]
        #[must_use]
        #[inline(always)]
        pub const fn rephyaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "PHY address used when `PHYADDR_EN` is set."]
        #[inline(always)]
        pub const fn set_rephyaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Use `REPHYADDR` as the PHY address; otherwise the address is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn phyaddr_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Use `REPHYADDR` as the PHY address; otherwise the address is 1."]
        #[inline(always)]
        pub const fn set_phyaddr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "The integrated PHY has a link."]
        #[must_use]
        #[inline(always)]
        pub const fn link_status(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "The integrated PHY has a link."]
        #[inline(always)]
        pub const fn set_link_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Link speed, 0 = 10M and 1 = 100M."]
        #[must_use]
        #[inline(always)]
        pub const fn speed(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Link speed, 0 = 10M and 1 = 100M."]
        #[inline(always)]
        pub const fn set_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1 = full duplex, 0 = half duplex (the SVD and RM spell the field `DUPlEX`)."]
        #[must_use]
        #[inline(always)]
        pub const fn duplex(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1 = full duplex, 0 = half duplex (the SVD and RM spell the field `DUPlEX`)."]
        #[inline(always)]
        pub const fn set_duplex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Power down the integrated PHY."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_pd(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Power down the integrated PHY."]
        #[inline(always)]
        pub const fn set_phy_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Integrated PHY reset, active low."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_rstn(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Integrated PHY reset, active low."]
        #[inline(always)]
        pub const fn set_phy_rstn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PhyCfgr {
        #[inline(always)]
        fn default() -> PhyCfgr {
            PhyCfgr(0)
        }
    }
    impl core::fmt::Debug for PhyCfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyCfgr")
                .field("rephyaddr", &self.rephyaddr())
                .field("phyaddr_en", &self.phyaddr_en())
                .field("link_status", &self.link_status())
                .field("speed", &self.speed())
                .field("duplex", &self.duplex())
                .field("phy_pd", &self.phy_pd())
                .field("phy_rstn", &self.phy_rstn())
                .finish()
        }
    }
    #[doc = "PTP sub-second increment register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptpssir(pub u32);
    impl Ptpssir {
        #[doc = "Sub-second increment per clock."]
        #[must_use]
        #[inline(always)]
        pub const fn stssi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Sub-second increment per clock."]
        #[inline(always)]
        pub const fn set_stssi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ptpssir {
        #[inline(always)]
        fn default() -> Ptpssir {
            Ptpssir(0)
        }
    }
    impl core::fmt::Debug for Ptpssir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptpssir")
                .field("stssi", &self.stssi())
                .finish()
        }
    }
    #[doc = "PTP timestamp addend."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptsar(pub u32);
    impl Ptptsar {
        #[doc = "Value added to the accumulator every clock in fine update mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tsa(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Value added to the accumulator every clock in fine update mode."]
        #[inline(always)]
        pub const fn set_tsa(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptsar {
        #[inline(always)]
        fn default() -> Ptptsar {
            Ptptsar(0)
        }
    }
    impl core::fmt::Debug for Ptptsar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptsar").field("tsa", &self.tsa()).finish()
        }
    }
    #[doc = "PTP timestamp control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptscr(pub u32);
    impl Ptptscr {
        #[doc = "Enable timestamping into the transmit and receive descriptors."]
        #[must_use]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable timestamping into the transmit and receive descriptors."]
        #[inline(always)]
        pub const fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1 = fine update mode, 0 = coarse update mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tsfcu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1 = fine update mode, 0 = coarse update mode."]
        #[inline(always)]
        pub const fn set_tsfcu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Replace the system time with the update registers; self-clears."]
        #[must_use]
        #[inline(always)]
        pub const fn tssti(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Replace the system time with the update registers; self-clears."]
        #[inline(always)]
        pub const fn set_tssti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Add the update registers to the system time; self-clears."]
        #[must_use]
        #[inline(always)]
        pub const fn tsstu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Add the update registers to the system time; self-clears."]
        #[inline(always)]
        pub const fn set_tsstu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Raise an interrupt when the target time is reached."]
        #[must_use]
        #[inline(always)]
        pub const fn tsite(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Raise an interrupt when the target time is reached."]
        #[inline(always)]
        pub const fn set_tsite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Add the addend register to the accumulator; self-clears."]
        #[must_use]
        #[inline(always)]
        pub const fn tsaru(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Add the addend register to the accumulator; self-clears."]
        #[inline(always)]
        pub const fn set_tsaru(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ptptscr {
        #[inline(always)]
        fn default() -> Ptptscr {
            Ptptscr(0)
        }
    }
    impl core::fmt::Debug for Ptptscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptscr")
                .field("tse", &self.tse())
                .field("tsfcu", &self.tsfcu())
                .field("tssti", &self.tssti())
                .field("tsstu", &self.tsstu())
                .field("tsite", &self.tsite())
                .field("tsaru", &self.tsaru())
                .finish()
        }
    }
    #[doc = "PTP system time, seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptshr(pub u32);
    impl Ptptshr {
        #[doc = "Seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn sts(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Seconds."]
        #[inline(always)]
        pub const fn set_sts(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptshr {
        #[inline(always)]
        fn default() -> Ptptshr {
            Ptptshr(0)
        }
    }
    impl core::fmt::Debug for Ptptshr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptshr").field("sts", &self.sts()).finish()
        }
    }
    #[doc = "PTP timestamp update, seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptshur(pub u32);
    impl Ptptshur {
        #[doc = "Seconds to load or add."]
        #[must_use]
        #[inline(always)]
        pub const fn tsus(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Seconds to load or add."]
        #[inline(always)]
        pub const fn set_tsus(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptshur {
        #[inline(always)]
        fn default() -> Ptptshur {
            Ptptshur(0)
        }
    }
    impl core::fmt::Debug for Ptptshur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptshur")
                .field("tsus", &self.tsus())
                .finish()
        }
    }
    #[doc = "PTP system time, sub-seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptslr(pub u32);
    impl Ptptslr {
        #[doc = "Sub-seconds; a carry increments `STS`."]
        #[must_use]
        #[inline(always)]
        pub const fn stss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Sub-seconds; a carry increments `STS`."]
        #[inline(always)]
        pub const fn set_stss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "1 = the system time is negative."]
        #[must_use]
        #[inline(always)]
        pub const fn stpns(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1 = the system time is negative."]
        #[inline(always)]
        pub const fn set_stpns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ptptslr {
        #[inline(always)]
        fn default() -> Ptptslr {
            Ptptslr(0)
        }
    }
    impl core::fmt::Debug for Ptptslr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptslr")
                .field("stss", &self.stss())
                .field("stpns", &self.stpns())
                .finish()
        }
    }
    #[doc = "PTP timestamp update, sub-seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptslur(pub u32);
    impl Ptptslur {
        #[doc = "Sub-seconds to load or add."]
        #[must_use]
        #[inline(always)]
        pub const fn tsuss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Sub-seconds to load or add."]
        #[inline(always)]
        pub const fn set_tsuss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "1 subtracts the update from the system time, 0 adds it."]
        #[must_use]
        #[inline(always)]
        pub const fn tsupns(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1 subtracts the update from the system time, 0 adds it."]
        #[inline(always)]
        pub const fn set_tsupns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ptptslur {
        #[inline(always)]
        fn default() -> Ptptslur {
            Ptptslur(0)
        }
    }
    impl core::fmt::Debug for Ptptslur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptslur")
                .field("tsuss", &self.tsuss())
                .field("tsupns", &self.tsupns())
                .finish()
        }
    }
    #[doc = "PTP target time, seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptthr(pub u32);
    impl Ptptthr {
        #[doc = "Target seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn ttsh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Target seconds."]
        #[inline(always)]
        pub const fn set_ttsh(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptthr {
        #[inline(always)]
        fn default() -> Ptptthr {
            Ptptthr(0)
        }
    }
    impl core::fmt::Debug for Ptptthr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptptthr")
                .field("ttsh", &self.ttsh())
                .finish()
        }
    }
    #[doc = "PTP target time, sub-seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptpttlr(pub u32);
    impl Ptpttlr {
        #[doc = "Target sub-seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn ttsl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Target sub-seconds."]
        #[inline(always)]
        pub const fn set_ttsl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptpttlr {
        #[inline(always)]
        fn default() -> Ptpttlr {
            Ptpttlr(0)
        }
    }
    impl core::fmt::Debug for Ptpttlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptpttlr")
                .field("ttsl", &self.ttsl())
                .finish()
        }
    }
}
