use crate::{
    bxdf::{BsdfSample, Bxdf, trowbridge_reitz_distribution::TrowbridgeReitzDistribution},
    math::{
        Vec3,
        reflect::{fresnel_dielectric, refract},
    },
};

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
    fn f(&self, _w_o: &Vec3, _w_i: &Vec3) -> Vec3 {
        if self.effectively_smooth() {
            return Vec3::empty_vec();
        }
        unimplemented!("WIP, dependent on microfacet distribution work")
    }

    fn sample_f(&self, w_o: &Vec3, uc: f32, _u: (f32, f32)) -> Option<BsdfSample> {
        if self.effectively_smooth() {
            let w_i;
            let p_refl = fresnel_dielectric(w_o.z as f32, self.eta);
            let p_trans = 1.0 - p_refl;

            let (f, pdf) = if uc < p_refl {
                w_i = Vec3::new(-w_o.x, -w_o.y, w_o.z);
                let refl_cos = p_refl as f64 / w_i.z.abs();
                (Vec3::new(refl_cos, refl_cos, refl_cos), p_refl)
            } else {
                w_i = refract(&w_o, &Vec3::new(0.0, 0.0, 1.0), self.eta)?;
                let trans_cos = p_trans as f64 / w_i.z.abs();
                (Vec3::new(trans_cos, trans_cos, trans_cos), p_trans)
            };

            return Some(BsdfSample::new(f, w_i, pdf));
        }
        unimplemented!("WIP, dependent on microfacet distribution work")
    }
}
