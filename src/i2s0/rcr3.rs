#[doc = "Reader of register RCR3"]
pub type R = crate::R<u32, super::RCR3>;
#[doc = "Writer for register RCR3"]
pub type W = crate::W<u32, super::RCR3>;
#[doc = "Register RCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDFL`"]
pub type WDFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDFL`"]
pub struct WDFL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Receive Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCE_A {
    #[doc = "0: Receive data channel N is disabled."]
    _0 = 0,
    #[doc = "1: Receive data channel N is enabled."]
    _1 = 1,
}
impl From<RCE_A> for u8 {
    #[inline(always)]
    fn from(variant: RCE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RCE`"]
pub type RCE_R = crate::R<u8, RCE_A>;
impl RCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RCE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RCE_A::_0),
            1 => Val(RCE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCE_A::_1
    }
}
#[doc = "Write proxy for field `RCE`"]
pub struct RCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Receive data channel N is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCE_A::_0)
    }
    #[doc = "Receive data channel N is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&self) -> WDFL_R {
        WDFL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce(&self) -> RCE_R {
        RCE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&mut self) -> WDFL_W {
        WDFL_W { w: self }
    }
    #[doc = "Bits 16:17 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce(&mut self) -> RCE_W {
        RCE_W { w: self }
    }
}
