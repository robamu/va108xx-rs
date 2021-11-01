#[doc = "Register `INT_ROM_MBE` reader"]
pub struct R(crate::R<INT_ROM_MBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ROM_MBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ROM_MBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ROM_MBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ROM_MBE` writer"]
pub struct W(crate::W<INT_ROM_MBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ROM_MBE_SPEC>;
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
impl From<crate::W<INT_ROM_MBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ROM_MBE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Memory ROM MBE Interrupt Redirect Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_rom_mbe](index.html) module"]
pub struct INT_ROM_MBE_SPEC;
impl crate::RegisterSpec for INT_ROM_MBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_rom_mbe::R](R) reader structure"]
impl crate::Readable for INT_ROM_MBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_rom_mbe::W](W) writer structure"]
impl crate::Writable for INT_ROM_MBE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ROM_MBE to value 0xffff_ffff"]
impl crate::Resettable for INT_ROM_MBE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
