#[doc = "Reader of register DATAW3S%sLM"]
pub type R = crate::R<u32, super::DATAW3SLM>;
#[doc = "Writer for register DATAW3S%sLM"]
pub type W = crate::W<u32, super::DATAW3SLM>;
#[doc = "Register DATAW3S%sLM `reset()`'s with value 0"]
impl crate::ResetValue for super::DATAW3SLM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `data`"]
pub type DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `data`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of data entry"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of data entry"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
