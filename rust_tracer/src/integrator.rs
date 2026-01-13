use crate::light::PointLight;
use crate::math::Vec3;
use crate::objects::Object;
use crate::sampler::Sampler;

pub mod simple_path_integrator;

pub trait Integrator {
    fn shadow_rays<'a>(
        point: &Vec3,
        main_obj: &Object,
        objects: &[Object],
        lights: &'a [PointLight],
    ) -> Vec<&'a PointLight>;

    /// mirrors Li() from PBRT
    fn incident_radiance(
        ray: &Vec3,
        origin: &Vec3,
        /*sampled wavelengths, */ sampler: &impl Sampler,
        objects: &[Object],
        lights: &[PointLight],
        src_obj: Option<&Object>,
        depth: u8,
    ) -> Vec3;
}
