#[doc = "Reader of register C6"]
pub type R = crate::R<u8, super::C6>;
#[doc = "Writer for register C6"]
pub type W = crate::W<u8, super::C6>;
#[doc = "Register C6 `reset()`'s with value 0"]
impl crate::ResetValue for super::C6 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "VCO 0 Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VDIV0_A {
    #[doc = "0: Multiply Factor is 24"]
    _0 = 0,
    #[doc = "1: Multiply Factor is 25"]
    _1 = 1,
    #[doc = "2: Multiply Factor is 26"]
    _2 = 2,
    #[doc = "3: Multiply Factor is 27"]
    _3 = 3,
    #[doc = "4: Multiply Factor is 28"]
    _4 = 4,
    #[doc = "5: Multiply Factor is 29"]
    _5 = 5,
    #[doc = "6: Multiply Factor is 30"]
    _6 = 6,
    #[doc = "7: Multiply Factor is 31"]
    _7 = 7,
    #[doc = "8: Multiply Factor is 32"]
    _8 = 8,
    #[doc = "9: Multiply Factor is 33"]
    _9 = 9,
    #[doc = "10: Multiply Factor is 34"]
    _10 = 10,
    #[doc = "11: Multiply Factor is 35"]
    _11 = 11,
    #[doc = "12: Multiply Factor is 36"]
    _12 = 12,
    #[doc = "13: Multiply Factor is 37"]
    _13 = 13,
    #[doc = "14: Multiply Factor is 38"]
    _14 = 14,
    #[doc = "15: Multiply Factor is 39"]
    _15 = 15,
    #[doc = "16: Multiply Factor is 40"]
    _16 = 16,
    #[doc = "17: Multiply Factor is 41"]
    _17 = 17,
    #[doc = "18: Multiply Factor is 42"]
    _18 = 18,
    #[doc = "19: Multiply Factor is 43"]
    _19 = 19,
    #[doc = "20: Multiply Factor is 44"]
    _20 = 20,
    #[doc = "21: Multiply Factor is 45"]
    _21 = 21,
    #[doc = "22: Multiply Factor is 46"]
    _22 = 22,
    #[doc = "23: Multiply Factor is 47"]
    _23 = 23,
    #[doc = "24: Multiply Factor is 48"]
    _24 = 24,
    #[doc = "25: Multiply Factor is 49"]
    _25 = 25,
    #[doc = "26: Multiply Factor is 50"]
    _26 = 26,
    #[doc = "27: Multiply Factor is 51"]
    _27 = 27,
    #[doc = "28: Multiply Factor is 52"]
    _28 = 28,
    #[doc = "29: Multiply Factor is 53"]
    _29 = 29,
    #[doc = "30: Multiply Factor is 54"]
    _30 = 30,
    #[doc = "31: Multiply Factor is 55"]
    _31 = 31,
}
impl From<VDIV0_A> for u8 {
    #[inline(always)]
    fn from(variant: VDIV0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VDIV0`"]
pub type VDIV0_R = crate::R<u8, VDIV0_A>;
impl VDIV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDIV0_A {
        match self.bits {
            0 => VDIV0_A::_0,
            1 => VDIV0_A::_1,
            2 => VDIV0_A::_2,
            3 => VDIV0_A::_3,
            4 => VDIV0_A::_4,
            5 => VDIV0_A::_5,
            6 => VDIV0_A::_6,
            7 => VDIV0_A::_7,
            8 => VDIV0_A::_8,
            9 => VDIV0_A::_9,
            10 => VDIV0_A::_10,
            11 => VDIV0_A::_11,
            12 => VDIV0_A::_12,
            13 => VDIV0_A::_13,
            14 => VDIV0_A::_14,
            15 => VDIV0_A::_15,
            16 => VDIV0_A::_16,
            17 => VDIV0_A::_17,
            18 => VDIV0_A::_18,
            19 => VDIV0_A::_19,
            20 => VDIV0_A::_20,
            21 => VDIV0_A::_21,
            22 => VDIV0_A::_22,
            23 => VDIV0_A::_23,
            24 => VDIV0_A::_24,
            25 => VDIV0_A::_25,
            26 => VDIV0_A::_26,
            27 => VDIV0_A::_27,
            28 => VDIV0_A::_28,
            29 => VDIV0_A::_29,
            30 => VDIV0_A::_30,
            31 => VDIV0_A::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDIV0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDIV0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == VDIV0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == VDIV0_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == VDIV0_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == VDIV0_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == VDIV0_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == VDIV0_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == VDIV0_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == VDIV0_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == VDIV0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == VDIV0_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == VDIV0_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == VDIV0_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == VDIV0_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == VDIV0_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == VDIV0_A::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        *self == VDIV0_A::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == VDIV0_A::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline(always)]
    pub fn is_19(&self) -> bool {
        *self == VDIV0_A::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == VDIV0_A::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline(always)]
    pub fn is_21(&self) -> bool {
        *self == VDIV0_A::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline(always)]
    pub fn is_22(&self) -> bool {
        *self == VDIV0_A::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline(always)]
    pub fn is_23(&self) -> bool {
        *self == VDIV0_A::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == VDIV0_A::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == VDIV0_A::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        *self == VDIV0_A::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        *self == VDIV0_A::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        *self == VDIV0_A::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        *self == VDIV0_A::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == VDIV0_A::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == VDIV0_A::_31
    }
}
#[doc = "Write proxy for field `VDIV0`"]
pub struct VDIV0_W<'a> {
    w: &'a mut W,
}
impl<'a> VDIV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDIV0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Multiply Factor is 24"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDIV0_A::_0)
    }
    #[doc = "Multiply Factor is 25"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDIV0_A::_1)
    }
    #[doc = "Multiply Factor is 26"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(VDIV0_A::_2)
    }
    #[doc = "Multiply Factor is 27"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(VDIV0_A::_3)
    }
    #[doc = "Multiply Factor is 28"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(VDIV0_A::_4)
    }
    #[doc = "Multiply Factor is 29"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(VDIV0_A::_5)
    }
    #[doc = "Multiply Factor is 30"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(VDIV0_A::_6)
    }
    #[doc = "Multiply Factor is 31"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(VDIV0_A::_7)
    }
    #[doc = "Multiply Factor is 32"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(VDIV0_A::_8)
    }
    #[doc = "Multiply Factor is 33"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(VDIV0_A::_9)
    }
    #[doc = "Multiply Factor is 34"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(VDIV0_A::_10)
    }
    #[doc = "Multiply Factor is 35"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(VDIV0_A::_11)
    }
    #[doc = "Multiply Factor is 36"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(VDIV0_A::_12)
    }
    #[doc = "Multiply Factor is 37"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(VDIV0_A::_13)
    }
    #[doc = "Multiply Factor is 38"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(VDIV0_A::_14)
    }
    #[doc = "Multiply Factor is 39"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(VDIV0_A::_15)
    }
    #[doc = "Multiply Factor is 40"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(VDIV0_A::_16)
    }
    #[doc = "Multiply Factor is 41"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut W {
        self.variant(VDIV0_A::_17)
    }
    #[doc = "Multiply Factor is 42"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(VDIV0_A::_18)
    }
    #[doc = "Multiply Factor is 43"]
    #[inline(always)]
    pub fn _19(self) -> &'a mut W {
        self.variant(VDIV0_A::_19)
    }
    #[doc = "Multiply Factor is 44"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(VDIV0_A::_20)
    }
    #[doc = "Multiply Factor is 45"]
    #[inline(always)]
    pub fn _21(self) -> &'a mut W {
        self.variant(VDIV0_A::_21)
    }
    #[doc = "Multiply Factor is 46"]
    #[inline(always)]
    pub fn _22(self) -> &'a mut W {
        self.variant(VDIV0_A::_22)
    }
    #[doc = "Multiply Factor is 47"]
    #[inline(always)]
    pub fn _23(self) -> &'a mut W {
        self.variant(VDIV0_A::_23)
    }
    #[doc = "Multiply Factor is 48"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(VDIV0_A::_24)
    }
    #[doc = "Multiply Factor is 49"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut W {
        self.variant(VDIV0_A::_25)
    }
    #[doc = "Multiply Factor is 50"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut W {
        self.variant(VDIV0_A::_26)
    }
    #[doc = "Multiply Factor is 51"]
    #[inline(always)]
    pub fn _27(self) -> &'a mut W {
        self.variant(VDIV0_A::_27)
    }
    #[doc = "Multiply Factor is 52"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut W {
        self.variant(VDIV0_A::_28)
    }
    #[doc = "Multiply Factor is 53"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut W {
        self.variant(VDIV0_A::_29)
    }
    #[doc = "Multiply Factor is 54"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(VDIV0_A::_30)
    }
    #[doc = "Multiply Factor is 55"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(VDIV0_A::_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME0_A {
    #[doc = "0: External clock monitor is disabled for OSC0."]
    _0 = 0,
    #[doc = "1: External clock monitor is enabled for OSC0."]
    _1 = 1,
}
impl From<CME0_A> for bool {
    #[inline(always)]
    fn from(variant: CME0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CME0`"]
pub type CME0_R = crate::R<bool, CME0_A>;
impl CME0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CME0_A {
        match self.bits {
            false => CME0_A::_0,
            true => CME0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CME0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CME0_A::_1
    }
}
#[doc = "Write proxy for field `CME0`"]
pub struct CME0_W<'a> {
    w: &'a mut W,
}
impl<'a> CME0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CME0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External clock monitor is disabled for OSC0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME0_A::_0)
    }
    #[doc = "External clock monitor is enabled for OSC0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "PLL Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLS_A {
    #[doc = "0: FLL is selected."]
    _0 = 0,
    #[doc = "1: PLL is selected (PRDIV 0 need to be programmed to the correct divider to generate a PLL reference clock in the range of 2-4 MHz prior to setting the PLLS bit)."]
    _1 = 1,
}
impl From<PLLS_A> for bool {
    #[inline(always)]
    fn from(variant: PLLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLS`"]
pub type PLLS_R = crate::R<bool, PLLS_A>;
impl PLLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLS_A {
        match self.bits {
            false => PLLS_A::_0,
            true => PLLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLS_A::_1
    }
}
#[doc = "Write proxy for field `PLLS`"]
pub struct PLLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLL is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLS_A::_0)
    }
    #[doc = "PLL is selected (PRDIV 0 need to be programmed to the correct divider to generate a PLL reference clock in the range of 2-4 MHz prior to setting the PLLS bit)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Loss of Lock Interrrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLIE0_A {
    #[doc = "0: No interrupt request is generated on loss of lock."]
    _0 = 0,
    #[doc = "1: Generate an interrupt request on loss of lock."]
    _1 = 1,
}
impl From<LOLIE0_A> for bool {
    #[inline(always)]
    fn from(variant: LOLIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOLIE0`"]
pub type LOLIE0_R = crate::R<bool, LOLIE0_A>;
impl LOLIE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLIE0_A {
        match self.bits {
            false => LOLIE0_A::_0,
            true => LOLIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOLIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOLIE0_A::_1
    }
}
#[doc = "Write proxy for field `LOLIE0`"]
pub struct LOLIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOLIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOLIE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt request is generated on loss of lock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLIE0_A::_0)
    }
    #[doc = "Generate an interrupt request on loss of lock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLIE0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - VCO 0 Divider"]
    #[inline(always)]
    pub fn vdiv0(&self) -> VDIV0_R {
        VDIV0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme0(&self) -> CME0_R {
        CME0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline(always)]
    pub fn plls(&self) -> PLLS_R {
        PLLS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline(always)]
    pub fn lolie0(&self) -> LOLIE0_R {
        LOLIE0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - VCO 0 Divider"]
    #[inline(always)]
    pub fn vdiv0(&mut self) -> VDIV0_W {
        VDIV0_W { w: self }
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme0(&mut self) -> CME0_W {
        CME0_W { w: self }
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline(always)]
    pub fn plls(&mut self) -> PLLS_W {
        PLLS_W { w: self }
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline(always)]
    pub fn lolie0(&mut self) -> LOLIE0_W {
        LOLIE0_W { w: self }
    }
}
