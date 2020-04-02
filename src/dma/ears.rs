#[doc = "Reader of register EARS"]
pub type R = crate::R<u32, super::EARS>;
#[doc = "Writer for register EARS"]
pub type W = crate::W<u32, super::EARS>;
#[doc = "Register EARS `reset()`'s with value 0"]
impl crate::ResetValue for super::EARS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable asynchronous DMA request in stop for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_0_A {
    #[doc = "0: Disable asynchronous DMA request for channel 0."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 0."]
    _1 = 1,
}
impl From<EDREQ_0_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_0`"]
pub type EDREQ_0_R = crate::R<bool, EDREQ_0_A>;
impl EDREQ_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_0_A {
        match self.bits {
            false => EDREQ_0_A::_0,
            true => EDREQ_0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_0_A::_1
    }
}
#[doc = "Write proxy for field `EDREQ_0`"]
pub struct EDREQ_0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_0_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Enable asynchronous DMA request in stop for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_1_A {
    #[doc = "0: Disable asynchronous DMA request for channel 1"]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 1."]
    _1 = 1,
}
impl From<EDREQ_1_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_1`"]
pub type EDREQ_1_R = crate::R<bool, EDREQ_1_A>;
impl EDREQ_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_1_A {
        match self.bits {
            false => EDREQ_1_A::_0,
            true => EDREQ_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_1_A::_1
    }
}
#[doc = "Write proxy for field `EDREQ_1`"]
pub struct EDREQ_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_1_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Enable asynchronous DMA request in stop for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_2_A {
    #[doc = "0: Disable asynchronous DMA request for channel 2."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 2."]
    _1 = 1,
}
impl From<EDREQ_2_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_2`"]
pub type EDREQ_2_R = crate::R<bool, EDREQ_2_A>;
impl EDREQ_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_2_A {
        match self.bits {
            false => EDREQ_2_A::_0,
            true => EDREQ_2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_2_A::_1
    }
}
#[doc = "Write proxy for field `EDREQ_2`"]
pub struct EDREQ_2_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_2_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_2_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Enable asynchronous DMA request in stop for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_3_A {
    #[doc = "0: Disable asynchronous DMA request for channel 3."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 3."]
    _1 = 1,
}
impl From<EDREQ_3_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_3`"]
pub type EDREQ_3_R = crate::R<bool, EDREQ_3_A>;
impl EDREQ_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_3_A {
        match self.bits {
            false => EDREQ_3_A::_0,
            true => EDREQ_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_3_A::_1
    }
}
#[doc = "Write proxy for field `EDREQ_3`"]
pub struct EDREQ_3_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_3_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_3_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop for channel 0."]
    #[inline(always)]
    pub fn edreq_0(&self) -> EDREQ_0_R {
        EDREQ_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop for channel 1."]
    #[inline(always)]
    pub fn edreq_1(&self) -> EDREQ_1_R {
        EDREQ_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop for channel 2."]
    #[inline(always)]
    pub fn edreq_2(&self) -> EDREQ_2_R {
        EDREQ_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop for channel 3."]
    #[inline(always)]
    pub fn edreq_3(&self) -> EDREQ_3_R {
        EDREQ_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop for channel 0."]
    #[inline(always)]
    pub fn edreq_0(&mut self) -> EDREQ_0_W {
        EDREQ_0_W { w: self }
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop for channel 1."]
    #[inline(always)]
    pub fn edreq_1(&mut self) -> EDREQ_1_W {
        EDREQ_1_W { w: self }
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop for channel 2."]
    #[inline(always)]
    pub fn edreq_2(&mut self) -> EDREQ_2_W {
        EDREQ_2_W { w: self }
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop for channel 3."]
    #[inline(always)]
    pub fn edreq_3(&mut self) -> EDREQ_3_W {
        EDREQ_3_W { w: self }
    }
}
