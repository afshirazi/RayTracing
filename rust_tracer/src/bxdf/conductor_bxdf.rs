use crate::{bxdf::{BsdfSample, Bxdf, trowbridge_reitz_distribution::TrowbridgeReitzDistribution}, math::{Vec3, reflect::fresnel_complex_spec}};

struct ConductorBxdf {
    microfacet_distrib: TrowbridgeReitzDistribution,
    eta: Vec3,
    k: Vec3,
}

impl ConductorBxdf {
    fn new(microfacet_distrib: TrowbridgeReitzDistribution, eta: Vec3, k: Vec3) -> Self {
        Self { microfacet_distrib, eta, k }
    }

    fn effectively_smooth(&self) -> bool {
        self.microfacet_distrib.effectively_smooth()
    }
}

impl Bxdf for ConductorBxdf {
    fn f(&self, _w_o: &Vec3, _w_i: &Vec3) -> Vec3 {
        if self.effectively_smooth() {
            return Vec3::empty_vec()
        }
        unimplemented!("WIP, dependent on microfacet distribution work")
    }

    fn sample_f(&self, w_o: &Vec3, u: f32, uc: (f32, f32)) -> Option<BsdfSample> {
        if self.effectively_smooth() {
            let w_i = Vec3::new(-w_o.x, -w_o.y, w_o.z);
            let f = fresnel_complex_spec(w_i.z as f32, &self.eta, &self.k);

            return Some(BsdfSample::new(f, w_i, 1.0))
        }
        unimplemented!("WIP, dependent on microfacet distribution work")
    }
}