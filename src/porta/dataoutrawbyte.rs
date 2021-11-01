#[doc = "Register `DATAOUTRAWBYTE[%s]` writer"]
pub struct W(crate::W<DATAOUTRAWBYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAOUTRAWBYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DATAOUTRAWBYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAOUTRAWBYTE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Out Register by Byte\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dataoutrawbyte](index.html) module"]
pub struct DATAOUTRAWBYTE_SPEC;
impl crate::RegisterSpec for DATAOUTRAWBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [dataoutrawbyte::W](W) writer structure"]
impl crate::Writable for DATAOUTRAWBYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAOUTRAWBYTE[%s]
to value 0"]
impl crate::Resettable for DATAOUTRAWBYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
