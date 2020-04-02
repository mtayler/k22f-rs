#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Priority Registers Slave"]
    pub prs0: PRS,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Control Register"]
    pub crs0: CRS,
    _reserved2: [u8; 236usize],
    #[doc = "0x100 - Priority Registers Slave"]
    pub prs1: PRS,
    _reserved3: [u8; 12usize],
    #[doc = "0x110 - Control Register"]
    pub crs1: CRS,
    _reserved4: [u8; 236usize],
    #[doc = "0x200 - Priority Registers Slave"]
    pub prs2: PRS,
    _reserved5: [u8; 12usize],
    #[doc = "0x210 - Control Register"]
    pub crs2: CRS,
    _reserved6: [u8; 236usize],
    #[doc = "0x300 - Priority Registers Slave"]
    pub prs3: PRS,
    _reserved7: [u8; 12usize],
    #[doc = "0x310 - Control Register"]
    pub crs3: CRS,
    _reserved8: [u8; 236usize],
    #[doc = "0x400 - Priority Registers Slave"]
    pub prs4: PRS,
    _reserved9: [u8; 12usize],
    #[doc = "0x410 - Control Register"]
    pub crs4: CRS,
    _reserved10: [u8; 1004usize],
    #[doc = "0x800 - Master General Purpose Control Register"]
    pub mgpcr0: MGPCR0,
    _reserved11: [u8; 252usize],
    #[doc = "0x900 - Master General Purpose Control Register"]
    pub mgpcr1: MGPCR1,
    _reserved12: [u8; 252usize],
    #[doc = "0xa00 - Master General Purpose Control Register"]
    pub mgpcr2: MGPCR2,
    _reserved13: [u8; 508usize],
    #[doc = "0xc00 - Master General Purpose Control Register"]
    pub mgpcr4: MGPCR4,
}
#[doc = "Priority Registers Slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs](prs) module"]
pub type PRS = crate::Reg<u32, _PRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRS;
#[doc = "`read()` method returns [prs::R](prs::R) reader structure"]
impl crate::Readable for PRS {}
#[doc = "`write(|w| ..)` method takes [prs::W](prs::W) writer structure"]
impl crate::Writable for PRS {}
#[doc = "Priority Registers Slave"]
pub mod prs;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crs](crs) module"]
pub type CRS = crate::Reg<u32, _CRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRS;
#[doc = "`read()` method returns [crs::R](crs::R) reader structure"]
impl crate::Readable for CRS {}
#[doc = "`write(|w| ..)` method takes [crs::W](crs::W) writer structure"]
impl crate::Writable for CRS {}
#[doc = "Control Register"]
pub mod crs;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mgpcr0](mgpcr0) module"]
pub type MGPCR0 = crate::Reg<u32, _MGPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR0;
#[doc = "`read()` method returns [mgpcr0::R](mgpcr0::R) reader structure"]
impl crate::Readable for MGPCR0 {}
#[doc = "`write(|w| ..)` method takes [mgpcr0::W](mgpcr0::W) writer structure"]
impl crate::Writable for MGPCR0 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr0;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mgpcr1](mgpcr1) module"]
pub type MGPCR1 = crate::Reg<u32, _MGPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR1;
#[doc = "`read()` method returns [mgpcr1::R](mgpcr1::R) reader structure"]
impl crate::Readable for MGPCR1 {}
#[doc = "`write(|w| ..)` method takes [mgpcr1::W](mgpcr1::W) writer structure"]
impl crate::Writable for MGPCR1 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr1;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mgpcr2](mgpcr2) module"]
pub type MGPCR2 = crate::Reg<u32, _MGPCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR2;
#[doc = "`read()` method returns [mgpcr2::R](mgpcr2::R) reader structure"]
impl crate::Readable for MGPCR2 {}
#[doc = "`write(|w| ..)` method takes [mgpcr2::W](mgpcr2::W) writer structure"]
impl crate::Writable for MGPCR2 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr2;
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mgpcr4](mgpcr4) module"]
pub type MGPCR4 = crate::Reg<u32, _MGPCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MGPCR4;
#[doc = "`read()` method returns [mgpcr4::R](mgpcr4::R) reader structure"]
impl crate::Readable for MGPCR4 {}
#[doc = "`write(|w| ..)` method takes [mgpcr4::W](mgpcr4::W) writer structure"]
impl crate::Writable for MGPCR4 {}
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr4;
