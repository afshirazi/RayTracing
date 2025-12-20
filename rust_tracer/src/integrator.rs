use crate::light::Light;
use crate::math::Vec3;
use crate::objects::Object;
use crate::sampler::Sampler;

pub mod simple_path_integrator;

pub trait Integrator {
    fn shadow_rays<'a>(
        point: &Vec3,
        main_obj: &Object,
        objects: &Vec<Object>,
        lights: &'a Vec<Light>,
    ) -> Vec<&'a Light>;

    /// mirrors Li() from PBRT
    fn incident_radiance(
        ray: &Vec3,
        origin: &Vec3,
        /*sampled wavelengths, */ sampler: &impl Sampler,
        objects: &Vec<Object>,
        lights: &Vec<Light>,
        src_obj: Option<&Object>,
        depth: u8,
    ) -> Vec3;
}
