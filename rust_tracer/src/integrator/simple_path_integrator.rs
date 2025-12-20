use crate::{
    integrator::Integrator,
    light::Light,
    math::Vec3,
    objects::{Object, RayOps},
    sampler::Sampler,
};

pub struct SimplePathIntegrator;

impl SimplePathIntegrator {
    pub fn new() -> Self {
        SimplePathIntegrator
    }
}

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

                match object.get_intersect(&light_dir, point) {
                    Some(_) => continue 'light_loop,
                    None => (),
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

        let vis_lights = Self::shadow_rays(&intr_point, intr_obj, objects, lights);

        for light in vis_lights {
            let light_dir = (&light.pos - &intr_point).norm();
            let light_refl = (&(2.0 * (light_dir.dot(&normal)) * &normal) - &light_dir).norm();

            let light_intensity = 250.0 * Vec3::euclid_dist_sq(&light.pos, &intr_point).recip(); // TODO remove hardcode

            let diffuse_term = light_dir.dot(&normal); // doubles to check if light is on correct side of object
            let spec_term = (origin - &intr_point).norm().dot(&light_refl);
            if diffuse_term > 0.0 {
                color_buf.x +=
                    intr_obj.get_diff().x * diffuse_term * light.diff.x * light_intensity
                        + intr_obj.get_spec().x
                            * spec_term.powf(intr_obj.get_shin())
                            * light.spec.x
                            * light_intensity;
                color_buf.y +=
                    intr_obj.get_diff().y * diffuse_term * light.diff.y * light_intensity
                        + intr_obj.get_spec().y
                            * spec_term.powf(intr_obj.get_shin())
                            * light.spec.y
                            * light_intensity;
                color_buf.z +=
                    intr_obj.get_diff().z * diffuse_term * light.diff.z * light_intensity
                        + intr_obj.get_spec().z
                            * spec_term.powf(intr_obj.get_shin())
                            * light.spec.z
                            * light_intensity;
            }
        }

        if depth > 0 && intr_obj.get_spec().dot(intr_obj.get_spec()) > 0.3 {
            let in_ray = origin - &intr_point;
            let refl_ray = (&(2.0 * (in_ray.dot(&normal)) * &normal) - &in_ray).norm();
            color_buf = color_buf
                + Self::incident_radiance(
                    &refl_ray,
                    &intr_point,
                    _s,
                    objects,
                    lights,
                    Some(intr_obj),
                    depth - 1,
                ) * 0.5;
        }

        color_buf
    }
}
