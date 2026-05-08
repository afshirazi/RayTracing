use crate::{
    bxdf::{BsdfSample, Bxdf, trowbridge_reitz_distribution::TrowbridgeReitzDistribution},
    math::{
        Vec3,
        reflect::{fresnel_dielectric, refract},
    },
    spectrum::sampled_spectrum::SampledSpectrum,
};

use super::BxdfFlags;

#[derive(Clone)]
pub struct DielectricBxdf {
    microfacet_distrib: TrowbridgeReitzDistribution,
    eta: f32,
}

impl DielectricBxdf {
    pub fn new(microfacet_distrib: TrowbridgeReitzDistribution, eta: f32) -> Self {
        Self {
            microfacet_distrib,
            eta,
        }
    }

    fn effectively_smooth(&self) -> bool {
        self.microfacet_distrib.effectively_smooth()
    }
}

impl Bxdf for DielectricBxdf {
    fn f(&self, _w_o: &Vec3, _w_i: &Vec3) -> SampledSpectrum {
        if self.effectively_smooth() {
            return SampledSpectrum::filled(0.0);
        }
        unimplemented!("WIP, dependent on microfacet distribution work")
    }

    fn sample_f(&self, w_o: &Vec3, uc: f32, _u: (f32, f32)) -> Option<BsdfSample> {
        if self.effectively_smooth() {
            let w_i;
            let p_refl = fresnel_dielectric(w_o.z, self.eta);
            let p_trans = 1.0 - p_refl;

            let (f, pdf) = if uc < p_refl {
                w_i = Vec3::new(-w_o.x, -w_o.y, w_o.z);
                let refl_cos = p_refl / w_i.z.abs();
                (SampledSpectrum::filled(refl_cos), p_refl)
            } else {
                w_i = refract(w_o, &Vec3::new(0.0, 0.0, 1.0), self.eta)?;
                let trans_cos = p_trans / w_i.z.abs();
                (SampledSpectrum::filled(trans_cos), p_trans)
            };

            return Some(BsdfSample::new(f, w_i, pdf, self.flags()));
        }
        unimplemented!("WIP, dependent on microfacet distribution work")
    }

    fn flags(&self) -> BxdfFlags {
        let flags = if self.eta == 1.0 {
            BxdfFlags::Transmission
        } else {
            BxdfFlags::Transmission | BxdfFlags::Reflection
        };

        flags
            | if self.effectively_smooth() {
                BxdfFlags::Specular
            } else {
                BxdfFlags::Glossy
            }
    }
}
