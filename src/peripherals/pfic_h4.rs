#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Programmable Fast Interrupt Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfic {
    ptr: *mut u8,
}
unsafe impl Send for Pfic {}
unsafe impl Sync for Pfic {}
impl Pfic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr1(self) -> crate::common::Reg<regs::Isr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr2(self) -> crate::common::Reg<regs::Isr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr3(self) -> crate::common::Reg<regs::Isr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr4(self) -> crate::common::Reg<regs::Isr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr5(self) -> crate::common::Reg<regs::Isr5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipr1(self) -> crate::common::Reg<regs::Ipr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipr2(self) -> crate::common::Reg<regs::Ipr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipr3(self) -> crate::common::Reg<regs::Ipr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipr4(self) -> crate::common::Reg<regs::Ipr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipr5(self) -> crate::common::Reg<regs::Ipr5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Interrupt Priority Register."]
    #[inline(always)]
    pub const fn ithresdr(self) -> crate::common::Reg<regs::Ithresdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Interrupt configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Interrupt Global Register."]
    #[inline(always)]
    pub const fn gisr(self) -> crate::common::Reg<regs::Gisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "ID Config Register."]
    #[inline(always)]
    pub const fn vtfidr(self) -> crate::common::Reg<regs::Vtfidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Interrupt 0 address Register."]
    #[inline(always)]
    pub const fn vtfaddrr0(self) -> crate::common::Reg<regs::Vtfaddrr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Interrupt 1 address Register."]
    #[inline(always)]
    pub const fn vtfaddrr1(self) -> crate::common::Reg<regs::Vtfaddrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Interrupt 2 address Register."]
    #[inline(always)]
    pub const fn vtfaddrr2(self) -> crate::common::Reg<regs::Vtfaddrr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Interrupt 3 address Register."]
    #[inline(always)]
    pub const fn vtfaddrr3(self) -> crate::common::Reg<regs::Vtfaddrr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr1(self) -> crate::common::Reg<regs::Ienr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr2(self) -> crate::common::Reg<regs::Ienr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr3(self) -> crate::common::Reg<regs::Ienr3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr4(self) -> crate::common::Reg<regs::Ienr4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr5(self) -> crate::common::Reg<regs::Ienr5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer1(self) -> crate::common::Reg<regs::Irer1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer2(self) -> crate::common::Reg<regs::Irer2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer3(self) -> crate::common::Reg<regs::Irer3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer4(self) -> crate::common::Reg<regs::Irer4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer5(self) -> crate::common::Reg<regs::Irer5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr1(self) -> crate::common::Reg<regs::Ipsr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr2(self) -> crate::common::Reg<regs::Ipsr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr3(self) -> crate::common::Reg<regs::Ipsr3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr4(self) -> crate::common::Reg<regs::Ipsr4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr5(self) -> crate::common::Reg<regs::Ipsr5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr1(self) -> crate::common::Reg<regs::Iprr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr2(self) -> crate::common::Reg<regs::Iprr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr3(self) -> crate::common::Reg<regs::Iprr3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr4(self) -> crate::common::Reg<regs::Iprr4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x028cusize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr5(self) -> crate::common::Reg<regs::Iprr5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr1(self) -> crate::common::Reg<regs::Iactr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr2(self) -> crate::common::Reg<regs::Iactr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr3(self) -> crate::common::Reg<regs::Iactr3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr4(self) -> crate::common::Reg<regs::Iactr4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr5(self) -> crate::common::Reg<regs::Iactr5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "Interrupt Priority Register."]
    #[inline(always)]
    pub const fn iprior(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr5(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0614usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr6(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0618usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr7(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x061cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr8(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr9(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0624usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr10(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0628usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr11(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x062cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr12(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0630usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr13(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0634usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr14(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0638usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr15(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x063cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr16(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr17(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0644usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr18(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0648usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr19(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x064cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr20(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0650usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr21(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0654usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr22(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0658usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr23(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x065cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr24(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0660usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr25(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0664usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr26(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0668usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr27(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x066cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr28(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0670usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr29(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0674usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr30(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0678usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr31(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x067cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr32(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr33(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0684usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr34(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0688usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr35(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x068cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr36(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0690usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr37(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0694usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr38(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0698usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr39(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x069cusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr40(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a0usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr41(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a4usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr42(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a8usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr43(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06acusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr44(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b0usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr45(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b4usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr46(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b8usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr47(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06bcusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr48(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c0usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr49(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c4usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr50(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c8usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr51(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06ccusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr52(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d0usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr53(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d4usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr54(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d8usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr55(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06dcusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr56(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e0usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr57(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e4usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr58(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e8usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr59(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06ecusize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr60(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f0usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr61(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f4usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr62(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f8usize) as _) }
    }
    #[doc = "Interrupt Allocation Register."]
    #[inline(always)]
    pub const fn iallocr63(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06fcusize) as _) }
    }
    #[doc = "interrupts authority register 1."]
    #[inline(always)]
    pub const fn iautr1(self) -> crate::common::Reg<regs::Iautr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "interrupts authority register 2."]
    #[inline(always)]
    pub const fn iautr2(self) -> crate::common::Reg<regs::Iautr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0704usize) as _) }
    }
    #[doc = "interrupts authority register 3."]
    #[inline(always)]
    pub const fn iautr3(self) -> crate::common::Reg<regs::Iautr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0708usize) as _) }
    }
    #[doc = "interrupts authority register 4."]
    #[inline(always)]
    pub const fn iautr4(self) -> crate::common::Reg<regs::Iautr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x070cusize) as _) }
    }
    #[doc = "interrupts authority register 5."]
    #[inline(always)]
    pub const fn iautr5(self) -> crate::common::Reg<regs::Iautr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0710usize) as _) }
    }
    #[doc = "PFIC wake-up instruction pointer register 0."]
    #[inline(always)]
    pub const fn wakeip0(self) -> crate::common::Reg<regs::Wakeip0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0720usize) as _) }
    }
    #[doc = "PFIC wake-up instruction pointer register 1."]
    #[inline(always)]
    pub const fn wakeip1(self) -> crate::common::Reg<regs::Wakeip1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0724usize) as _) }
    }
    #[doc = "PFIC kernel status register 0."]
    #[inline(always)]
    pub const fn cstar0(self) -> crate::common::Reg<regs::Cstar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0780usize) as _) }
    }
    #[doc = "PFIC kernel status register 1."]
    #[inline(always)]
    pub const fn cstar1(self) -> crate::common::Reg<regs::Cstar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0784usize) as _) }
    }
    #[doc = "PFIC Event Enable Register."]
    #[inline(always)]
    pub const fn eenr(self) -> crate::common::Reg<regs::Eenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c80usize) as _) }
    }
    #[doc = "PFIC Event Suspend Register."]
    #[inline(always)]
    pub const fn epr(self) -> crate::common::Reg<regs::Epr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c84usize) as _) }
    }
    #[doc = "PFIC Event Wake Register."]
    #[inline(always)]
    pub const fn ewupr(self) -> crate::common::Reg<regs::Ewupr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c88usize) as _) }
    }
    #[doc = "System Control Register."]
    #[inline(always)]
    pub const fn sctlr(self) -> crate::common::Reg<regs::Sctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Interrupt configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System reset register."]
        #[inline(always)]
        pub const fn sysrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "System reset register."]
        #[inline(always)]
        pub fn set_sysrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "KEYCODE."]
        #[inline(always)]
        pub const fn keycode(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "KEYCODE."]
        #[inline(always)]
        pub fn set_keycode(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "PFIC kernel status register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cstar0(pub u32);
    impl Cstar0 {
        #[doc = "The nested status register of kernel C0 interrupts is used to. query the nested state of kernel C0 interrupts. nest_sta bits from the INEST_CTLR register of the CSR register."]
        #[inline(always)]
        pub const fn cpu_nest_sta_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The nested status register of kernel C0 interrupts is used to. query the nested state of kernel C0 interrupts. nest_sta bits from the INEST_CTLR register of the CSR register."]
        #[inline(always)]
        pub fn set_cpu_nest_sta_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Kernel C0 interrupt active flag register,. which is used to query whether kernel C0 is processing interrupts."]
        #[inline(always)]
        pub const fn cpu_irq_active_0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Kernel C0 interrupt active flag register,. which is used to query whether kernel C0 is processing interrupts."]
        #[inline(always)]
        pub fn set_cpu_irq_active_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Kernel C0 interrupt pending flag register, which is used to query kernel C1 for unhandled interrupts."]
        #[inline(always)]
        pub const fn cpu_irq_pend_0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Kernel C0 interrupt pending flag register, which is used to query kernel C1 for unhandled interrupts."]
        #[inline(always)]
        pub fn set_cpu_irq_pend_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "The kernel C0 global interrupt enable register is used to. query whether the kernel C0 global interrupt is enabled."]
        #[inline(always)]
        pub const fn cpu_globl_ie_0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "The kernel C0 global interrupt enable register is used to. query whether the kernel C0 global interrupt is enabled."]
        #[inline(always)]
        pub fn set_cpu_globl_ie_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Kernel C0 debug mode register, which is used to query whether kernel C0 is in debug mode."]
        #[inline(always)]
        pub const fn cpu_dbg_mode_0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Kernel C0 debug mode register, which is used to query whether kernel C0 is in debug mode."]
        #[inline(always)]
        pub fn set_cpu_dbg_mode_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "The kernel C0 lock status register is used to query whether kernel C0 is in the locked state."]
        #[inline(always)]
        pub const fn cpu_lock_up_0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "The kernel C0 lock status register is used to query whether kernel C0 is in the locked state."]
        #[inline(always)]
        pub fn set_cpu_lock_up_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "The kernel C0 status register is used to obtain the running state of kernel C0."]
        #[inline(always)]
        pub const fn cpu_ex_state_0(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "The kernel C0 status register is used to obtain the running state of kernel C0."]
        #[inline(always)]
        pub fn set_cpu_ex_state_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Cstar0 {
        #[inline(always)]
        fn default() -> Cstar0 {
            Cstar0(0)
        }
    }
    #[doc = "PFIC kernel status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cstar1(pub u32);
    impl Cstar1 {
        #[doc = "The nested status register of kernel C1 interrupts is used to. query the nested state of kernel C1 interrupts. nest_sta bits from the INEST_CTLR register of the CSR register."]
        #[inline(always)]
        pub const fn cpu_nest_sta_1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The nested status register of kernel C1 interrupts is used to. query the nested state of kernel C1 interrupts. nest_sta bits from the INEST_CTLR register of the CSR register."]
        #[inline(always)]
        pub fn set_cpu_nest_sta_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Kernel C1 interrupt active flag register,. which is used to query whether kernel C1 is processing interrupts."]
        #[inline(always)]
        pub const fn cpu_irq_active_1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Kernel C1 interrupt active flag register,. which is used to query whether kernel C1 is processing interrupts."]
        #[inline(always)]
        pub fn set_cpu_irq_active_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Kernel C1 interrupt pending flag register, which is used to query kernel C1 for unhandled interrupts."]
        #[inline(always)]
        pub const fn cpu_irq_pend_1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Kernel C1 interrupt pending flag register, which is used to query kernel C1 for unhandled interrupts."]
        #[inline(always)]
        pub fn set_cpu_irq_pend_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "The kernel C1 global interrupt enable register is used to. query whether the kernel C1 global interrupt is enabled."]
        #[inline(always)]
        pub const fn cpu_globl_ie_1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "The kernel C1 global interrupt enable register is used to. query whether the kernel C1 global interrupt is enabled."]
        #[inline(always)]
        pub fn set_cpu_globl_ie_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Kernel C1 debug mode register, which is used to query whether kernel C1 is in debug mode."]
        #[inline(always)]
        pub const fn cpu_dbg_mode_1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Kernel C1 debug mode register, which is used to query whether kernel C1 is in debug mode."]
        #[inline(always)]
        pub fn set_cpu_dbg_mode_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "The kernel C1 lock status register is used to query whether kernel C1 is in the locked state."]
        #[inline(always)]
        pub const fn cpu_lock_up_1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "The kernel C1 lock status register is used to query whether kernel C1 is in the locked state."]
        #[inline(always)]
        pub fn set_cpu_lock_up_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "The kernel C1 status register is used to obtain the running state of kernel C1."]
        #[inline(always)]
        pub const fn cpu_ex_state_1(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "The kernel C1 status register is used to obtain the running state of kernel C1."]
        #[inline(always)]
        pub fn set_cpu_ex_state_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Cstar1 {
        #[inline(always)]
        fn default() -> Cstar1 {
            Cstar1(0)
        }
    }
    #[doc = "PFIC Event Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eenr(pub u32);
    impl Eenr {
        #[doc = "31-0 Event wake-up enabled."]
        #[inline(always)]
        pub const fn eventen(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "31-0 Event wake-up enabled."]
        #[inline(always)]
        pub fn set_eventen(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for Eenr {
        #[inline(always)]
        fn default() -> Eenr {
            Eenr(0)
        }
    }
    #[doc = "PFIC Event Suspend Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epr(pub u32);
    impl Epr {
        #[doc = "7-0 Event Pending Status, Not Clearable."]
        #[inline(always)]
        pub const fn event_pend7_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "7-0 Event Pending Status, Not Clearable."]
        #[inline(always)]
        pub fn set_event_pend7_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "31-8 event is suspended, write 1 to clear zero."]
        #[inline(always)]
        pub const fn event_pend31_8(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "31-8 event is suspended, write 1 to clear zero."]
        #[inline(always)]
        pub fn set_event_pend31_8(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Epr {
        #[inline(always)]
        fn default() -> Epr {
            Epr(0)
        }
    }
    #[doc = "PFIC Event Wake Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ewupr(pub u32);
    impl Ewupr {
        #[doc = "31-0 Event Wake Register."]
        #[inline(always)]
        pub const fn event_wup(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31-0 Event Wake Register."]
        #[inline(always)]
        pub fn set_event_wup(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ewupr {
        #[inline(always)]
        fn default() -> Ewupr {
            Ewupr(0)
        }
    }
    #[doc = "Interrupt Global Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gisr(pub u32);
    impl Gisr {
        #[doc = "interrupt nesting."]
        #[inline(always)]
        pub const fn neststa(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "interrupt nesting."]
        #[inline(always)]
        pub fn set_neststa(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "interrupt execution."]
        #[inline(always)]
        pub const fn gactsta(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt execution."]
        #[inline(always)]
        pub fn set_gactsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "interrupt pending."]
        #[inline(always)]
        pub const fn gpendsta(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt pending."]
        #[inline(always)]
        pub fn set_gpendsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "global interrupt enable."]
        #[inline(always)]
        pub const fn globl_ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "global interrupt enable."]
        #[inline(always)]
        pub fn set_globl_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "debug mode."]
        #[inline(always)]
        pub const fn dbg_mode(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "debug mode."]
        #[inline(always)]
        pub fn set_dbg_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "lock-out state."]
        #[inline(always)]
        pub const fn lock_up(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "lock-out state."]
        #[inline(always)]
        pub fn set_lock_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "The kernel status register, which is used to obtain the running state of the kernel."]
        #[inline(always)]
        pub const fn ex_state(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "The kernel status register, which is used to obtain the running state of the kernel."]
        #[inline(always)]
        pub fn set_ex_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Gisr {
        #[inline(always)]
        fn default() -> Gisr {
            Gisr(0)
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr1(pub u32);
    impl Iactr1 {
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts2_3(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts2_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts8_9(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts8_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Iactr1 {
        #[inline(always)]
        fn default() -> Iactr1 {
            Iactr1(0)
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr2(pub u32);
    impl Iactr2 {
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iactr2 {
        #[inline(always)]
        fn default() -> Iactr2 {
            Iactr2(0)
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr3(pub u32);
    impl Iactr3 {
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iactr3 {
        #[inline(always)]
        fn default() -> Iactr3 {
            Iactr3(0)
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr4(pub u32);
    impl Iactr4 {
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iactr4 {
        #[inline(always)]
        fn default() -> Iactr4 {
            Iactr4(0)
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr5(pub u32);
    impl Iactr5 {
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Iactr5 {
        #[inline(always)]
        fn default() -> Iactr5 {
            Iactr5(0)
        }
    }
    #[doc = "interrupts authority register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iautr1(pub u32);
    impl Iautr1 {
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub const fn iaut(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub fn set_iaut(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iautr1 {
        #[inline(always)]
        fn default() -> Iautr1 {
            Iautr1(0)
        }
    }
    #[doc = "interrupts authority register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iautr2(pub u32);
    impl Iautr2 {
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub const fn iaut(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub fn set_iaut(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iautr2 {
        #[inline(always)]
        fn default() -> Iautr2 {
            Iautr2(0)
        }
    }
    #[doc = "interrupts authority register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iautr3(pub u32);
    impl Iautr3 {
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub const fn iaut(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub fn set_iaut(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iautr3 {
        #[inline(always)]
        fn default() -> Iautr3 {
            Iautr3(0)
        }
    }
    #[doc = "interrupts authority register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iautr4(pub u32);
    impl Iautr4 {
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub const fn iaut(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub fn set_iaut(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iautr4 {
        #[inline(always)]
        fn default() -> Iautr4 {
            Iautr4(0)
        }
    }
    #[doc = "interrupts authority register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iautr5(pub u32);
    impl Iautr5 {
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub const fn iaut(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "interrupt allocates all registers. and is used to query whether the interrupt responds to each kernel allocation."]
        #[inline(always)]
        pub fn set_iaut(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iautr5 {
        #[inline(always)]
        fn default() -> Iautr5 {
            Iautr5(0)
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr1(pub u32);
    impl Ienr1 {
        #[doc = "INTEN12_31."]
        #[inline(always)]
        pub const fn inten12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "INTEN12_31."]
        #[inline(always)]
        pub fn set_inten12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Ienr1 {
        #[inline(always)]
        fn default() -> Ienr1 {
            Ienr1(0)
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr2(pub u32);
    impl Ienr2 {
        #[doc = "INTEN."]
        #[inline(always)]
        pub const fn inten(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "INTEN."]
        #[inline(always)]
        pub fn set_inten(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ienr2 {
        #[inline(always)]
        fn default() -> Ienr2 {
            Ienr2(0)
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr3(pub u32);
    impl Ienr3 {
        #[doc = "INTEN."]
        #[inline(always)]
        pub const fn inten(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "INTEN."]
        #[inline(always)]
        pub fn set_inten(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ienr3 {
        #[inline(always)]
        fn default() -> Ienr3 {
            Ienr3(0)
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr4(pub u32);
    impl Ienr4 {
        #[doc = "INTEN."]
        #[inline(always)]
        pub const fn inten(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "INTEN."]
        #[inline(always)]
        pub fn set_inten(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ienr4 {
        #[inline(always)]
        fn default() -> Ienr4 {
            Ienr4(0)
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr5(pub u32);
    impl Ienr5 {
        #[doc = "INTEN."]
        #[inline(always)]
        pub const fn inten(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "INTEN."]
        #[inline(always)]
        pub fn set_inten(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Ienr5 {
        #[inline(always)]
        fn default() -> Ienr5 {
            Ienr5(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr1(pub u32);
    impl Ipr1 {
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta2_3(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta2_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn intensta8_9(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_intensta8_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn intensta12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_intensta12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Ipr1 {
        #[inline(always)]
        fn default() -> Ipr1 {
            Ipr1(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr2(pub u32);
    impl Ipr2 {
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipr2 {
        #[inline(always)]
        fn default() -> Ipr2 {
            Ipr2(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr3(pub u32);
    impl Ipr3 {
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipr3 {
        #[inline(always)]
        fn default() -> Ipr3 {
            Ipr3(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr4(pub u32);
    impl Ipr4 {
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipr4 {
        #[inline(always)]
        fn default() -> Ipr4 {
            Ipr4(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr5(pub u32);
    impl Ipr5 {
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Ipr5 {
        #[inline(always)]
        fn default() -> Ipr5 {
            Ipr5(0)
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr1(pub u32);
    impl Iprr1 {
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendrst2_3(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendrst2_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendrst5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendrst5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendrst8(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendrst8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendrst12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendrst12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Iprr1 {
        #[inline(always)]
        fn default() -> Iprr1 {
            Iprr1(0)
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr2(pub u32);
    impl Iprr2 {
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendrst(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendrst(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprr2 {
        #[inline(always)]
        fn default() -> Iprr2 {
            Iprr2(0)
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr3(pub u32);
    impl Iprr3 {
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendrst(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendrst(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprr3 {
        #[inline(always)]
        fn default() -> Iprr3 {
            Iprr3(0)
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr4(pub u32);
    impl Iprr4 {
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendrst(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendrst(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprr4 {
        #[inline(always)]
        fn default() -> Iprr4 {
            Iprr4(0)
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr5(pub u32);
    impl Iprr5 {
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendrst(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendrst(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Iprr5 {
        #[inline(always)]
        fn default() -> Iprr5 {
            Iprr5(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr1(pub u32);
    impl Ipsr1 {
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset2_3(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset2_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset8_9(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset8_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Ipsr1 {
        #[inline(always)]
        fn default() -> Ipsr1 {
            Ipsr1(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr2(pub u32);
    impl Ipsr2 {
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipsr2 {
        #[inline(always)]
        fn default() -> Ipsr2 {
            Ipsr2(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr3(pub u32);
    impl Ipsr3 {
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipsr3 {
        #[inline(always)]
        fn default() -> Ipsr3 {
            Ipsr3(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr4(pub u32);
    impl Ipsr4 {
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipsr4 {
        #[inline(always)]
        fn default() -> Ipsr4 {
            Ipsr4(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr5(pub u32);
    impl Ipsr5 {
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Ipsr5 {
        #[inline(always)]
        fn default() -> Ipsr5 {
            Ipsr5(0)
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer1(pub u32);
    impl Irer1 {
        #[doc = "INTRSET12_31."]
        #[inline(always)]
        pub const fn intrset12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "INTRSET12_31."]
        #[inline(always)]
        pub fn set_intrset12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Irer1 {
        #[inline(always)]
        fn default() -> Irer1 {
            Irer1(0)
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer2(pub u32);
    impl Irer2 {
        #[doc = "INTRSET."]
        #[inline(always)]
        pub const fn intrset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "INTRSET."]
        #[inline(always)]
        pub fn set_intrset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Irer2 {
        #[inline(always)]
        fn default() -> Irer2 {
            Irer2(0)
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer3(pub u32);
    impl Irer3 {
        #[doc = "INTRSET."]
        #[inline(always)]
        pub const fn intrset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "INTRSET."]
        #[inline(always)]
        pub fn set_intrset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Irer3 {
        #[inline(always)]
        fn default() -> Irer3 {
            Irer3(0)
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer4(pub u32);
    impl Irer4 {
        #[doc = "INTRSET."]
        #[inline(always)]
        pub const fn intrset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "INTRSET."]
        #[inline(always)]
        pub fn set_intrset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Irer4 {
        #[inline(always)]
        fn default() -> Irer4 {
            Irer4(0)
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer5(pub u32);
    impl Irer5 {
        #[doc = "INTRSET."]
        #[inline(always)]
        pub const fn intrset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "INTRSET."]
        #[inline(always)]
        pub fn set_intrset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Irer5 {
        #[inline(always)]
        fn default() -> Irer5 {
            Irer5(0)
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr1(pub u32);
    impl Isr1 {
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta2_3(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta2_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta8_9(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta8_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Isr1 {
        #[inline(always)]
        fn default() -> Isr1 {
            Isr1(0)
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr2(pub u32);
    impl Isr2 {
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Isr2 {
        #[inline(always)]
        fn default() -> Isr2 {
            Isr2(0)
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr3(pub u32);
    impl Isr3 {
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Isr3 {
        #[inline(always)]
        fn default() -> Isr3 {
            Isr3(0)
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr4(pub u32);
    impl Isr4 {
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Isr4 {
        #[inline(always)]
        fn default() -> Isr4 {
            Isr4(0)
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr5(pub u32);
    impl Isr5 {
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Isr5 {
        #[inline(always)]
        fn default() -> Isr5 {
            Isr5(0)
        }
    }
    #[doc = "Interrupt Priority Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ithresdr(pub u32);
    impl Ithresdr {
        #[doc = "THRESHOLD."]
        #[inline(always)]
        pub const fn threshold(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "THRESHOLD."]
        #[inline(always)]
        pub fn set_threshold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ithresdr {
        #[inline(always)]
        fn default() -> Ithresdr {
            Ithresdr(0)
        }
    }
    #[doc = "System Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sctlr(pub u32);
    impl Sctlr {
        #[doc = "system leaves the state after an interruption."]
        #[inline(always)]
        pub const fn sleeponexit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "system leaves the state after an interruption."]
        #[inline(always)]
        pub fn set_sleeponexit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "low power mode selection."]
        #[inline(always)]
        pub const fn sleepdeep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "low power mode selection."]
        #[inline(always)]
        pub fn set_sleepdeep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Treat WFI as WFE."]
        #[inline(always)]
        pub const fn wfitowfe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Treat WFI as WFE."]
        #[inline(always)]
        pub fn set_wfitowfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Send-event-on-pend. When set, any pended interrupt (even masked) wakes the core from WFE."]
        #[inline(always)]
        pub const fn sevonpend(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Send-event-on-pend. When set, any pended interrupt (even masked) wakes the core from WFE."]
        #[inline(always)]
        pub fn set_sevonpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Send a one-shot event. Used to wake the other hart from WFE (paired with PFIC_WAKEIPx for dual-core boot)."]
        #[inline(always)]
        pub const fn sendevent(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Send a one-shot event. Used to wake the other hart from WFE (paired with PFIC_WAKEIPx for dual-core boot)."]
        #[inline(always)]
        pub fn set_sendevent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Hart ID of the core reading this register. 0 = hart 0 (V3F on CH32H417), 1 = hart 1 (V5F). Reserved upper bits read as 0."]
        #[inline(always)]
        pub const fn hart_id(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Hart ID of the core reading this register. 0 = hart 0 (V3F on CH32H417), 1 = hart 1 (V5F). Reserved upper bits read as 0."]
        #[inline(always)]
        pub fn set_hart_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "System reset."]
        #[inline(always)]
        pub const fn sysrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "System reset."]
        #[inline(always)]
        pub fn set_sysrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sctlr {
        #[inline(always)]
        fn default() -> Sctlr {
            Sctlr(0)
        }
    }
    #[doc = "Interrupt 0 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vtfaddrr0(pub u32);
    impl Vtfaddrr0 {
        #[doc = "VTF0EN."]
        #[inline(always)]
        pub const fn vtf0en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTF0EN."]
        #[inline(always)]
        pub fn set_vtf0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADDR0."]
        #[inline(always)]
        pub const fn addr0(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "ADDR0."]
        #[inline(always)]
        pub fn set_addr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Vtfaddrr0 {
        #[inline(always)]
        fn default() -> Vtfaddrr0 {
            Vtfaddrr0(0)
        }
    }
    #[doc = "Interrupt 1 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vtfaddrr1(pub u32);
    impl Vtfaddrr1 {
        #[doc = "VTF1EN."]
        #[inline(always)]
        pub const fn vtf1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTF1EN."]
        #[inline(always)]
        pub fn set_vtf1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADDR1."]
        #[inline(always)]
        pub const fn addr1(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "ADDR1."]
        #[inline(always)]
        pub fn set_addr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Vtfaddrr1 {
        #[inline(always)]
        fn default() -> Vtfaddrr1 {
            Vtfaddrr1(0)
        }
    }
    #[doc = "Interrupt 2 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vtfaddrr2(pub u32);
    impl Vtfaddrr2 {
        #[doc = "VTF2EN."]
        #[inline(always)]
        pub const fn vtf2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTF2EN."]
        #[inline(always)]
        pub fn set_vtf2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADDR2."]
        #[inline(always)]
        pub const fn addr2(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "ADDR2."]
        #[inline(always)]
        pub fn set_addr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Vtfaddrr2 {
        #[inline(always)]
        fn default() -> Vtfaddrr2 {
            Vtfaddrr2(0)
        }
    }
    #[doc = "Interrupt 3 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vtfaddrr3(pub u32);
    impl Vtfaddrr3 {
        #[doc = "VTF3EN."]
        #[inline(always)]
        pub const fn vtf3en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTF3EN."]
        #[inline(always)]
        pub fn set_vtf3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADDR3."]
        #[inline(always)]
        pub const fn addr3(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "ADDR3."]
        #[inline(always)]
        pub fn set_addr3(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Vtfaddrr3 {
        #[inline(always)]
        fn default() -> Vtfaddrr3 {
            Vtfaddrr3(0)
        }
    }
    #[doc = "ID Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vtfidr(pub u32);
    impl Vtfidr {
        #[doc = "VTFID0."]
        #[inline(always)]
        pub const fn vtfid0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "VTFID0."]
        #[inline(always)]
        pub fn set_vtfid0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "VTFID1."]
        #[inline(always)]
        pub const fn vtfid1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "VTFID1."]
        #[inline(always)]
        pub fn set_vtfid1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "VTFID2."]
        #[inline(always)]
        pub const fn vtfid2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "VTFID2."]
        #[inline(always)]
        pub fn set_vtfid2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "VTFID3."]
        #[inline(always)]
        pub const fn vtfid3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "VTFID3."]
        #[inline(always)]
        pub fn set_vtfid3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Vtfidr {
        #[inline(always)]
        fn default() -> Vtfidr {
            Vtfidr(0)
        }
    }
    #[doc = "PFIC wake-up instruction pointer register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wakeip0(pub u32);
    impl Wakeip0 {
        #[doc = "Kernel C0 Deep Sleep (Locked) Cancellation Register."]
        #[inline(always)]
        pub const fn shutdown0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Kernel C0 Deep Sleep (Locked) Cancellation Register."]
        #[inline(always)]
        pub fn set_shutdown0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "The PC address register on kernel C0 wake-up,. which is used to reload the PC value after power-on wake-up and reset wake-up."]
        #[inline(always)]
        pub const fn ip_reload0(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "The PC address register on kernel C0 wake-up,. which is used to reload the PC value after power-on wake-up and reset wake-up."]
        #[inline(always)]
        pub fn set_ip_reload0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Wakeip0 {
        #[inline(always)]
        fn default() -> Wakeip0 {
            Wakeip0(0)
        }
    }
    #[doc = "PFIC wake-up instruction pointer register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wakeip1(pub u32);
    impl Wakeip1 {
        #[doc = "Kernel C1 Deep Sleep (Locked) Cancellation Register."]
        #[inline(always)]
        pub const fn shutdown1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Kernel C1 Deep Sleep (Locked) Cancellation Register."]
        #[inline(always)]
        pub fn set_shutdown1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "The PC address register on kernel C1 wake-up,. which is used to reload the PC value after power-on wake-up and reset wake-up."]
        #[inline(always)]
        pub const fn ip_reload1(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "The PC address register on kernel C1 wake-up,. which is used to reload the PC value after power-on wake-up and reset wake-up."]
        #[inline(always)]
        pub fn set_ip_reload1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Wakeip1 {
        #[inline(always)]
        fn default() -> Wakeip1 {
            Wakeip1(0)
        }
    }
}
