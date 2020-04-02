#[doc = "Reader of register MGPCR0"]
pub type R = crate::R<u32, super::MGPCR0>;
#[doc = "Writer for register MGPCR0"]
pub type W = crate::W<u32, super::MGPCR0>;
#[doc = "Register MGPCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MGPCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Arbitrates On Undefined Length Bursts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AULB_A {
    #[doc = "0: No arbitration is allowed during an undefined length burst"]
    _000 = 0,
    #[doc = "1: Arbitration is allowed at any time during an undefined length burst"]
    _001 = 1,
    #[doc = "2: Arbitration is allowed after four beats of an undefined length burst"]
    _010 = 2,
    #[doc = "3: Arbitration is allowed after eight beats of an undefined length burst"]
    _011 = 3,
    #[doc = "4: Arbitration is allowed after 16 beats of an undefined length burst"]
    _100 = 4,
}
impl From<AULB_A> for u8 {
    #[inline(always)]
    fn from(variant: AULB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AULB`"]
pub type AULB_R = crate::R<u8, AULB_A>;
impl AULB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AULB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AULB_A::_000),
            1 => Val(AULB_A::_001),
            2 => Val(AULB_A::_010),
            3 => Val(AULB_A::_011),
            4 => Val(AULB_A::_100),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == AULB_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == AULB_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == AULB_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == AULB_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == AULB_A::_100
    }
}
#[doc = "Write proxy for field `AULB`"]
pub struct AULB_W<'a> {
    w: &'a mut W,
}
impl<'a> AULB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AULB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No arbitration is allowed during an undefined length burst"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(AULB_A::_000)
    }
    #[doc = "Arbitration is allowed at any time during an undefined length burst"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(AULB_A::_001)
    }
    #[doc = "Arbitration is allowed after four beats of an undefined length burst"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(AULB_A::_010)
    }
    #[doc = "Arbitration is allowed after eight beats of an undefined length burst"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(AULB_A::_011)
    }
    #[doc = "Arbitration is allowed after 16 beats of an undefined length burst"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(AULB_A::_100)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Arbitrates On Undefined Length Bursts"]
    #[inline(always)]
    pub fn aulb(&self) -> AULB_R {
        AULB_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Arbitrates On Undefined Length Bursts"]
    #[inline(always)]
    pub fn aulb(&mut self) -> AULB_W {
        AULB_W { w: self }
    }
}
