#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Programmable protocol I/O microcontroller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pioc {
    ptr: *mut u8,
}
unsafe impl Send for Pioc {}
unsafe impl Sync for Pioc {}
impl Pioc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Indirectly addressed data read/write ports."]
    #[inline(always)]
    pub const fn sfr_indir_port(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Indirectly addresses the data read/write port of 2."]
    #[inline(always)]
    pub const fn rsfr_indir_port2(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Program counters for the low bytes of the PC."]
    #[inline(always)]
    pub const fn sfr_prg_count(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Status registers."]
    #[inline(always)]
    pub const fn sfr_status_reg(self) -> crate::common::Reg<regs::SfrStatusReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Indirectly addressed address registers."]
    #[inline(always)]
    pub const fn sfr_indir_addr(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "The counter register of timer 0."]
    #[inline(always)]
    pub const fn sfr_tmr0_count(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "The control register of the timer."]
    #[inline(always)]
    pub const fn sfr_timer_ctrl(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "The initial register of timer 0."]
    #[inline(always)]
    pub const fn sfr_tmr0_init(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "Periodic registers for coded bits."]
    #[inline(always)]
    pub const fn sfr_bit_cycle(self) -> crate::common::Reg<regs::SfrBitCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Address registers for indirect address 2."]
    #[inline(always)]
    pub const fn sfr_indir_addr2(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "The port direction sets the register."]
    #[inline(always)]
    pub const fn sfr_port_dir(self) -> crate::common::Reg<regs::SfrPortDir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Port input and output registers."]
    #[inline(always)]
    pub const fn sfr_port_io(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "Coded bit configuration registers."]
    #[inline(always)]
    pub const fn sfr_bit_config(self) -> crate::common::Reg<regs::SfrBitConfig, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "System configuration registers."]
    #[inline(always)]
    pub const fn sfr_sys_cfg(self) -> crate::common::Reg<regs::SfrSysCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "The eMCU reads and writes and the host reads only the registers."]
    #[inline(always)]
    pub const fn sfr_ctrl_rd(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "The host reads and writes and the eMCU reads only registers."]
    #[inline(always)]
    pub const fn sfr_ctrl_wr(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Data exchange registers."]
    #[inline(always)]
    pub const fn sfr_data_exch(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fusize) as _) }
    }
    #[doc = "PIOC data registers 0-31."]
    #[inline(always)]
    pub const fn sfr_data_reg(self, n: usize) -> crate::common::Reg<u8, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 1usize) as _) }
    }
}
pub mod regs {
    #[doc = "Coded bit configuration registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SfrBitConfig(pub u32);
    impl SfrBitConfig {
        #[doc = "The periodic timing state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_bit_cyc_cnt3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[inline(always)]
        pub const fn set_sb_bit_cyc_cnt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_bit_cyc_cnt4(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[inline(always)]
        pub const fn set_sb_bit_cyc_cnt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_bit_cyc_cnt5(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[inline(always)]
        pub const fn set_sb_bit_cyc_cnt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_bit_cyc_cnt6(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[inline(always)]
        pub const fn set_sb_bit_cyc_cnt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Periodic state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_bit_cyc_tail(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Periodic state of the encoded bits."]
        #[inline(always)]
        pub const fn set_sb_bit_cyc_tail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Pin input level sampling point selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_port_in_edge(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pin input level sampling point selection."]
        #[inline(always)]
        pub const fn set_sb_port_in_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "How the coded bits are modulated."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_bit_code_mod(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "How the coded bits are modulated."]
        #[inline(always)]
        pub const fn set_sb_bit_code_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "The transmission of the coded bits is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_bit_tx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The transmission of the coded bits is enabled."]
        #[inline(always)]
        pub const fn set_sb_bit_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for SfrBitConfig {
        #[inline(always)]
        fn default() -> SfrBitConfig {
            SfrBitConfig(0)
        }
    }
    impl core::fmt::Debug for SfrBitConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SfrBitConfig")
                .field("sb_bit_cyc_cnt3", &self.sb_bit_cyc_cnt3())
                .field("sb_bit_cyc_cnt4", &self.sb_bit_cyc_cnt4())
                .field("sb_bit_cyc_cnt5", &self.sb_bit_cyc_cnt5())
                .field("sb_bit_cyc_cnt6", &self.sb_bit_cyc_cnt6())
                .field("sb_bit_cyc_tail", &self.sb_bit_cyc_tail())
                .field("sb_port_in_edge", &self.sb_port_in_edge())
                .field("sb_bit_code_mod", &self.sb_bit_code_mod())
                .field("sb_bit_tx_en", &self.sb_bit_tx_en())
                .finish()
        }
    }
    #[doc = "Periodic registers for coded bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SfrBitCycle(pub u32);
    impl SfrBitCycle {
        #[doc = "Sets the width of the encoded bits in clock cycles, which is the actual number of bits minus 1."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_bit_cycle(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Sets the width of the encoded bits in clock cycles, which is the actual number of bits minus 1."]
        #[inline(always)]
        pub const fn set_sb_bit_cycle(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "The raw bit data of the encoded bit to be sent, which is a double-buffered structure."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_bit_tx_o0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The raw bit data of the encoded bit to be sent, which is a double-buffered structure."]
        #[inline(always)]
        pub const fn set_sb_bit_tx_o0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for SfrBitCycle {
        #[inline(always)]
        fn default() -> SfrBitCycle {
            SfrBitCycle(0)
        }
    }
    impl core::fmt::Debug for SfrBitCycle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SfrBitCycle")
                .field("sb_bit_cycle", &self.sb_bit_cycle())
                .field("sb_bit_tx_o0", &self.sb_bit_tx_o0())
                .finish()
        }
    }
    #[doc = "The port direction sets the register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SfrPortDir(pub u32);
    impl SfrPortDir {
        #[doc = "IO0 Pin Direction Control."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_port_dir0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO0 Pin Direction Control."]
        #[inline(always)]
        pub const fn set_sb_port_dir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO1 Pin Direction Control."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_port_dir1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO1 Pin Direction Control."]
        #[inline(always)]
        pub const fn set_sb_port_dir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO0 Pin Pull-Up Enables."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_port_pu0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO0 Pin Pull-Up Enables."]
        #[inline(always)]
        pub const fn set_sb_port_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO1 Pin Pull-Up Enables."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_port_pu1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO1 Pin Pull-Up Enables."]
        #[inline(always)]
        pub const fn set_sb_port_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_port_mod0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[inline(always)]
        pub const fn set_sb_port_mod0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_port_mod1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[inline(always)]
        pub const fn set_sb_port_mod1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_port_mod2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[inline(always)]
        pub const fn set_sb_port_mod2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_port_mod3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[inline(always)]
        pub const fn set_sb_port_mod3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for SfrPortDir {
        #[inline(always)]
        fn default() -> SfrPortDir {
            SfrPortDir(0)
        }
    }
    impl core::fmt::Debug for SfrPortDir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SfrPortDir")
                .field("sb_port_dir0", &self.sb_port_dir0())
                .field("sb_port_dir1", &self.sb_port_dir1())
                .field("sb_port_pu0", &self.sb_port_pu0())
                .field("sb_port_pu1", &self.sb_port_pu1())
                .field("sb_port_mod0", &self.sb_port_mod0())
                .field("sb_port_mod1", &self.sb_port_mod1())
                .field("sb_port_mod2", &self.sb_port_mod2())
                .field("sb_port_mod3", &self.sb_port_mod3())
                .finish()
        }
    }
    #[doc = "Status registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SfrStatusReg(pub u32);
    impl SfrStatusReg {
        #[doc = "ALU carry flag, whether the result is carried or produced by displacement."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_flag_c(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ALU carry flag, whether the result is carried or produced by displacement."]
        #[inline(always)]
        pub const fn set_sb_flag_c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "A generic bit variable X, defined and used by the application."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_gp_bit_x(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "A generic bit variable X, defined and used by the application."]
        #[inline(always)]
        pub const fn set_sb_gp_bit_x(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ALU zero flag, whether the result is 00H."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_flag_z(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ALU zero flag, whether the result is 00H."]
        #[inline(always)]
        pub const fn set_sb_flag_z(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "a generic bit variable Y, defined and used by the application."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_gp_bit_y(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "a generic bit variable Y, defined and used by the application."]
        #[inline(always)]
        pub const fn set_sb_gp_bit_y(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer Timeout Reset Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_en_tout_rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Timeout Reset Enabled."]
        #[inline(always)]
        pub const fn set_sb_en_tout_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Current stack usage flags."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_stack_used(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Current stack usage flags."]
        #[inline(always)]
        pub const fn set_sb_stack_used(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for SfrStatusReg {
        #[inline(always)]
        fn default() -> SfrStatusReg {
            SfrStatusReg(0)
        }
    }
    impl core::fmt::Debug for SfrStatusReg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SfrStatusReg")
                .field("sb_flag_c", &self.sb_flag_c())
                .field("sb_gp_bit_x", &self.sb_gp_bit_x())
                .field("sb_flag_z", &self.sb_flag_z())
                .field("sb_gp_bit_y", &self.sb_gp_bit_y())
                .field("sb_en_tout_rst", &self.sb_en_tout_rst())
                .field("sb_stack_used", &self.sb_stack_used())
                .finish()
        }
    }
    #[doc = "System configuration registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SfrSysCfg(pub u32);
    impl SfrSysCfg {
        #[doc = "Global Clock Control for eMCUs."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_mst_clk_gate(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global Clock Control for eMCUs."]
        #[inline(always)]
        pub const fn set_sb_mst_clk_gate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force eMCU Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_mst_reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force eMCU Reset."]
        #[inline(always)]
        pub const fn set_sb_mst_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mode and output control switches for IO0 pins."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_mst_io_en0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mode and output control switches for IO0 pins."]
        #[inline(always)]
        pub const fn set_sb_mst_io_en0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Mode and output control switches for IO1 pins."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_mst_io_en1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Mode and output control switches for IO1 pins."]
        #[inline(always)]
        pub const fn set_sb_mst_io_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Configure information bits, software-defined uses."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_mst_cfg_b4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Configure information bits, software-defined uses."]
        #[inline(always)]
        pub const fn set_sb_mst_cfg_b4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SFR_CTRL_WR waiting to read the status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_data_mw_sr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SFR_CTRL_WR waiting to read the status bit."]
        #[inline(always)]
        pub const fn set_sb_data_mw_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SFR_CTRL_RD wait to read the status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_data_sw_mr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SFR_CTRL_RD wait to read the status bit."]
        #[inline(always)]
        pub const fn set_sb_data_sw_mr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Interrupt Request Activation Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn sb_int_req(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Request Activation Bits."]
        #[inline(always)]
        pub const fn set_sb_int_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for SfrSysCfg {
        #[inline(always)]
        fn default() -> SfrSysCfg {
            SfrSysCfg(0)
        }
    }
    impl core::fmt::Debug for SfrSysCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SfrSysCfg")
                .field("sb_mst_clk_gate", &self.sb_mst_clk_gate())
                .field("sb_mst_reset", &self.sb_mst_reset())
                .field("sb_mst_io_en0", &self.sb_mst_io_en0())
                .field("sb_mst_io_en1", &self.sb_mst_io_en1())
                .field("sb_mst_cfg_b4", &self.sb_mst_cfg_b4())
                .field("sb_data_mw_sr", &self.sb_data_mw_sr())
                .field("sb_data_sw_mr", &self.sb_data_sw_mr())
                .field("sb_int_req", &self.sb_int_req())
                .finish()
        }
    }
}
