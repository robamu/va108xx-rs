#[doc = "Register `PULSEBASEBYTE[%s]` reader"]
pub struct R(crate::R<PULSEBASEBYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULSEBASEBYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULSEBASEBYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULSEBASEBYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULSEBASEBYTE[%s]` writer"]
pub struct W(crate::W<PULSEBASEBYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULSEBASEBYTE_SPEC>;
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
impl From<crate::W<PULSEBASEBYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULSEBASEBYTE_SPEC>) -> Self {
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
#[doc = "Pulse Base Mode Register by Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulsebasebyte](index.html) module"]
pub struct PULSEBASEBYTE_SPEC;
impl crate::RegisterSpec for PULSEBASEBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pulsebasebyte::R](R) reader structure"]
impl crate::Readable for PULSEBASEBYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulsebasebyte::W](W) writer structure"]
impl crate::Writable for PULSEBASEBYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PULSEBASEBYTE[%s]
to value 0"]
impl crate::Resettable for PULSEBASEBYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
