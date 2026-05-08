use crate::{
    bxdf::{BsdfSample, Bxdf, trowbridge_reitz_distribution::TrowbridgeReitzDistribution},
    math::{Vec3, reflect::fresnel_complex_spec},
    spectrum::sampled_spectrum::SampledSpectrum,
};

use super::BxdfFlags;

#[derive(Clone)]
pub struct ConductorBxdf {
    microfacet_distrib: TrowbridgeReitzDistribution,
    eta: SampledSpectrum,
    k: SampledSpectrum,
}

impl ConductorBxdf {
    pub fn new(
        microfacet_distrib: TrowbridgeReitzDistribution,
        eta: SampledSpectrum,
        k: SampledSpectrum,
    ) -> Self {
        Self {
            microfacet_distrib,
            eta,
            k,
        }
    }

    fn effectively_smooth(&self) -> bool {
        self.microfacet_distrib.effectively_smooth()
    }
}

impl Bxdf for ConductorBxdf {
    fn f(&self, _w_o: &Vec3, _w_i: &Vec3) -> SampledSpectrum {
        if self.effectively_smooth() {
            return SampledSpectrum::filled(0.0);
        }
        unimplemented!("WIP, dependent on microfacet distribution work")
    }

    fn sample_f(&self, w_o: &Vec3, _uc: f32, _u: (f32, f32)) -> Option<BsdfSample> {
        if self.effectively_smooth() {
            let w_i = Vec3::new(-w_o.x, -w_o.y, w_o.z);
            let f = fresnel_complex_spec(w_i.z.abs(), &self.eta, &self.k);

            return Some(BsdfSample::new(f, w_i, 1.0, self.flags()));
        }
        unimplemented!("WIP, dependent on microfacet distribution work")
    }

    fn flags(&self) -> BxdfFlags {
        if self.effectively_smooth() {
            BxdfFlags::SpecularReflection
        } else {
            BxdfFlags::GlossyReflection
        }
    }
}
