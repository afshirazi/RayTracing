use crate::{
    bxdf::Bxdf,
    integrator::Integrator,
    light::Light,
    math::Vec3,
    objects::{Object, RayOps},
    sampler::Sampler,
};

pub struct SimplePathIntegrator;

impl Integrator for SimplePathIntegrator {
    fn shadow_rays<'a>(
        point: &Vec3,
        main_obj: &Object,
        objects: &Vec<Object>,
        lights: &'a Vec<Light>,
    ) -> Vec<&'a Light> {
        let mut vis_lights = vec![];

        'light_loop: for light in lights {
            let light_dir = (&light.pos - point).norm();

            for object in objects {
                if object == main_obj {
                    continue;
                }

                if let Some(_) = object.get_intersect(&light_dir, point) {
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
        _s: &impl Sampler, // unused for now
        objects: &Vec<Object>,
        lights: &Vec<Light>,
        src_obj: Option<&Object>,
        depth: u8,
    ) -> Vec3 {
        let mut color_buf = Vec3::new(0.0, 0.0, 0.0);

        let filtered_objects: Vec<&Object> = match src_obj {
            Some(o) => objects.iter().filter(|other_obj| *other_obj != o).collect(),
            None => objects.iter().collect(),
        };

        let intr_obj = filtered_objects
            .iter()
            .map(|obj| (obj, obj.get_intersect(ray, origin)))
            .filter(|(_, intr)| intr.is_some())
            .map(|(obj, intr)| (obj, intr.unwrap()))
            .min_by(|(_, lv), (_, rv)| {
                let ld = Vec3::euclid_dist_sq(lv, origin);
                let rd = Vec3::euclid_dist_sq(rv, origin);
                ld.total_cmp(&rd)
            })
            .map(|(obj, _)| *obj);

        if intr_obj.is_none() {
            return Vec3::new(0.3, 0.3, 0.3);
        }

        let intr_obj = intr_obj.unwrap();
        let intr_point = intr_obj.get_intersect(ray, origin).unwrap();
        let normal = intr_obj.get_normal(&intr_point);
        let tan = intr_obj.get_tangent(&intr_point);
        let bsdf = intr_obj.get_mat(&normal, &tan);

        let w_o = ray * -1.0;

        let vis_lights = Self::shadow_rays(&intr_point, intr_obj, objects, lights);

        for light in vis_lights {
            let light_dir = (&light.pos - &intr_point).norm();

            let sample_spectrum = bsdf.f(&w_o, &light_dir);
            color_buf += light.color.elwise_mul(&sample_spectrum) / 1.0; // I know I divide by 1 here but that's cuz point light and delta distribution and whatever man it's written in the book
        }

        let bs = bsdf.sample_f(&w_o, rand::random(), (rand::random(), rand::random()));
        if let Some(bs) = bs
            && depth > 0
        {
            let beta = bs.color * bs.w_i.dot(&normal).abs() / bs.pdf.into();
            color_buf += Self::incident_radiance(
                &bs.w_i,
                &intr_point,
                _s,
                objects,
                lights,
                Some(intr_obj),
                depth - 1,
            )
            .elwise_mul(&beta);
        }

        color_buf
    }
}
