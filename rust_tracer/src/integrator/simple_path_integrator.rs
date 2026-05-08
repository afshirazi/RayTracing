use crate::{
    bxdf::Bxdf,
    integrator::Integrator,
    light::{Light, LightSampleContext, PointLight},
    math::Vec3,
    objects::{Object, RayOps},
    sampler::Sampler,
    spectrum::{
        N_SPECTRUM_SAMPLES,
        sampled_spectrum::{SampledSpectrum, SampledWavelengths},
    },
};

pub struct SimplePathIntegrator;

impl Integrator for SimplePathIntegrator {
    fn shadow_rays<'a>(
        point: &Vec3,
        main_obj: &Object,
        objects: &[Object],
        lights: &'a [PointLight],
    ) -> Vec<&'a PointLight> {
        let mut vis_lights = vec![];

        'light_loop: for light in lights {
            let light_dir = (&light.pos - point).norm();

            for object in objects {
                if object == main_obj {
                    continue;
                }

                if object.get_intersect(&light_dir, point).is_some() {
                    continue 'light_loop;
                }
            }

            vis_lights.push(light);
        }

        vis_lights
    }

    fn incident_radiance(
        ray: &Vec3,
        origin: &Vec3,
        lambdas: &SampledWavelengths,
        sampler: &impl Sampler,
        objects: &[Object],
        lights: &[PointLight],
        src_obj: Option<&Object>,
        depth: u8,
    ) -> SampledSpectrum {
        let mut color_buf = SampledSpectrum::new([0.0; N_SPECTRUM_SAMPLES]);

        let intr_obj = objects
            .iter()
            .map(|obj| (obj, obj.get_intersect(ray, origin)))
            .filter(|(obj, intr)| (src_obj != Some(*obj)) && intr.is_some())
            .map(|(obj, intr)| (obj, intr.unwrap()))
            .min_by(|(_, lv), (_, rv)| {
                let ld = Vec3::euclid_dist_sq(lv, origin);
                let rd = Vec3::euclid_dist_sq(rv, origin);
                ld.total_cmp(&rd)
            })
            .map(|(obj, _)| obj);

        if intr_obj.is_none() {
            return SampledSpectrum::filled(0.3);
        }

        let intr_obj = intr_obj.unwrap();
        let intr_point = intr_obj.get_intersect(ray, origin).unwrap();
        let normal = intr_obj.get_normal(&intr_point);
        let tan = intr_obj.get_tangent(&intr_point);
        let bsdf = intr_obj.get_mat(&normal, &tan);

        let w_o = ray * -1.0;

        let vis_lights = Self::shadow_rays(&intr_point, intr_obj, objects, lights);
        let ctx = LightSampleContext::new(intr_point.clone(), normal.clone());

        for light in vis_lights {
            let light_dir = (&light.pos - &intr_point).norm();

            let bsdf_lookup = bsdf.f(&w_o, &light_dir);
            let li_sample = light.sample_li(&ctx, sampler.get_2d(), lambdas);
            if let Some(ls) = li_sample
                && ls.pdf > 0.0
            {
                color_buf += ls.radiance * bsdf_lookup / ls.pdf;
            }
        }

        let bs = bsdf.sample_f(&w_o, sampler.get_1d(), sampler.get_2d());
        if let Some(bs) = bs
            && depth > 0
        {
            let beta = &bs.color * bs.w_i.dot(&normal).abs() / bs.pdf;
            color_buf += Self::incident_radiance(
                &bs.w_i,
                &intr_point,
                lambdas,
                sampler,
                objects,
                lights,
                Some(intr_obj),
                depth - 1,
            ) * beta;
        }

        color_buf
    }
}
