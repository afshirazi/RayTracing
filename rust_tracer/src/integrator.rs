use crate::math::Vec3;
use crate::sampler::Sampler;

pub mod simple_path_integrator;

trait Integrator {
    /// mirrors Li() from PBRT
    fn incident_radiance(ray: Vec3, /* sampled wavelengths, */sampler: impl Sampler) -> Vec3; 
        
}