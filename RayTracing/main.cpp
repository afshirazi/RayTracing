#include <iostream>
#include <fstream>
#include <vector>
#include <cmath>
#include <limits>
#include <algorithm>

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "./stb_image_write.h"

#include "Vec3.h"
#include "Circle.h"
#include "Triangle.h"
#include "Light.h"
#include "Parser.h"


enum ObjType {
    EMPTY, CIRCLE, TRIANGLE
};

std::vector<ObjUni*> objects;
std::vector<Light*> lights;

Vec3 get_ray(const Vec3& eye, int x, int y, int width, int height)
{
    Vec3 lookAt(0, 0, -1);
    Vec3 up(0, 1, 0);
    double a = (double)width / (double)height;
    double fov = 1.2; // change fov here

    Vec3 l = Vec3::normalize(lookAt - eye);
    Vec3 v = Vec3::normalize((l.cross(up)));
    Vec3 u = Vec3::normalize(v.cross(l));

    Vec3 ll = eye + (l * fov) - (v * a) - u;
    Vec3 p = ll + (v * 2 * a * ((double)x / width)) + (u * 2 * ((double)y / height));
    Vec3 dir = Vec3::normalize((p - eye));

    return (dir - eye);
}

std::vector<Light*> shadow_rays(const Vec3& point, int obj_idx)
{
    Vec3 p = Vec3(point.x, point.y, point.z);
    std::vector<Light*> vis_lights;
    for (Light* light : lights) 
    {
        Vec3 light_vec = Vec3::normalize(light->pos - point);
        bool found = false;

        for (int i = 0; i < objects.size(); i++)
        {
            if (i == obj_idx) continue;

            ObjUni* arr_obj = objects[i];
            Vec3 intr;
            if (arr_obj->cir != Circle())
                intr = arr_obj->cir.get_intersect(light_vec, p);
            else if (arr_obj->tri != Triangle())
                intr = arr_obj->tri.get_intersect(light_vec, p);

            if (intr != Vec3(0, 0, 1))
            {
                found = true;
                break;
            }
        }

        if (found) continue;

        vis_lights.push_back(light);
    }

    return vis_lights;
}

Vec3 get_color(const Vec3 *ray, const Vec3 *origin, int obj_idx, int depth)
{
    ObjType intersect = EMPTY;
    ObjUni* obj = objects[0]; // get rid of error
    Vec3 color;
    int intr_idx = -1;
    double zBuf = -std::numeric_limits<double>::max(); // almost -inf, for camera
    double eucDist = std::numeric_limits<double>::max(); // for use with recursion

    for (int i = 0; i < objects.size(); i++)
    {
        if (i == obj_idx) continue;

        ObjUni* arr_obj = objects[i];
        Vec3 intr;
        ObjType check_type = EMPTY;
        if (arr_obj->cir != Circle()) 
        {
            intr = arr_obj->cir.get_intersect(*ray, *origin);
            check_type = CIRCLE;
        }
        else if (arr_obj->tri != Triangle())
        {
            intr = arr_obj->tri.get_intersect(*ray, *origin);
            check_type = TRIANGLE;
        }

        if (obj_idx == -1 && intr.z < origin->z && intr.z > zBuf) // in front of camera
        {
            zBuf = intr.z;
            obj = arr_obj;
            intr_idx = i;
            intersect = check_type;
        }
        else if (obj_idx != -1 && intr != Vec3(0, 0, 1) && Vec3::euclidian_sq(*origin, intr) < eucDist) // closest to object
        {
            eucDist = Vec3::euclidian_sq(*origin, intr);
            obj = arr_obj;
            intr_idx = i;
            intersect = check_type;
        }
    }

    if (intersect == CIRCLE) // I like circles
    {
        Circle cir = Circle(obj->cir.center, obj->cir.diff, obj->cir.spec, obj->cir.shin, obj->cir.radius);
        Vec3 intr = cir.get_intersect(*ray, *origin);
        Vec3 norm = cir.get_normal(intr);

        Vec3 view_vec = Vec3::normalize(*origin - *ray);
        std::vector<Light*> vis_lights = shadow_rays(intr, intr_idx);
        for (Light* light : vis_lights)
        {
            Vec3 light_vec = Vec3::normalize(light->pos - intr);
            Vec3 light_refl = Vec3::normalize(norm * (2 * norm.dot(light_vec)) - light_vec);
            if ((light_vec).dot(norm) >= 0) // check if light on correct side of object
            {
                color.x += light->diff.x * (light_vec.dot(norm)) * cir.diff.x + light->spec.x * std::pow(light_refl.dot(view_vec), cir.shin);
                color.y += light->diff.y * (light_vec.dot(norm)) * cir.diff.y + light->spec.y * std::pow(light_refl.dot(view_vec), cir.shin);
                color.z += light->diff.z * (light_vec.dot(norm)) * cir.diff.z + light->spec.z * std::pow(light_refl.dot(view_vec), cir.shin);
            }

        }

        if (depth > 0 && cir.spec.dot(cir.spec) > 0.3) // if reached depth or spec is too low (diffuse surface)
        {

            Vec3 in_ray = Vec3::normalize(*origin - *ray);
            Vec3 refl = Vec3::normalize(norm * (2 * norm.dot(in_ray)) - in_ray);
            color += get_color(&refl, &intr, intr_idx, depth - 1);
        }

        return Vec3(color.x, color.y, color.z);
    }
    else if (intersect == TRIANGLE)
    {
        Triangle tri = Triangle(obj->tri.a, obj->tri.b, obj->tri.c, obj->tri.diff, obj->tri.spec, obj->tri.shin);
        Vec3 intr = tri.get_intersect(*ray, *origin);
        Vec3 norm = tri.get_normal();

        Vec3 view_vec = Vec3::normalize(*origin - *ray);
        std::vector<Light*> vis_lights = shadow_rays(intr, intr_idx);
        for (Light* light : vis_lights)
        {
            Vec3 light_vec = Vec3::normalize(light->pos - intr);
            Vec3 light_refl = Vec3::normalize(norm * (2 * norm.dot(light_vec)) - light_vec);
            if ((light_vec).dot(norm) >= 0)
            {
                color.x += light->diff.x * (light_vec.dot(norm)) * tri.diff.x + light->spec.x * std::pow(light_refl.dot(view_vec), tri.shin);
                color.y += light->diff.y * (light_vec.dot(norm)) * tri.diff.y + light->spec.y * std::pow(light_refl.dot(view_vec), tri.shin);
                color.z += light->diff.z * (light_vec.dot(norm)) * tri.diff.z + light->spec.z * std::pow(light_refl.dot(view_vec), tri.shin);
            }

        }

        if (depth > 0 && tri.spec.dot(tri.spec) > 0.3) // if reached depth or spec is too low (diffuse surface)
        {
            Vec3 in_ray = Vec3::normalize(*origin - *ray);
            Vec3 refl = Vec3::normalize(norm * (2 * norm.dot(in_ray)) - in_ray);
            color += get_color(&refl, &intr, intr_idx, depth - 1);
        }

        return Vec3(color.x, color.y, color.z);
    }
    else 
        return Vec3(0.4, 0.4, 0.4);
}

int main(int argc, char* argv[])
{
    int width = 1600;
    int height = 900;

    const Vec3 *eye = new Vec3(0, 0, 0);

    //std::string in_file(argv[1]);
    //std::string out_file(argv[2]);

    Parser file_parser;
    file_parser.parse_file("objs2.txt");
    lights = file_parser.lights;
    objects = file_parser.objects;

    double* raw_img = new double[height * width * 3];
    unsigned char *img = new unsigned char[height * width * 3];

    for (int j = 0; j < height; j++) {
        for (int i = 0; i < width; i++) {
            Vec3* ray = new Vec3(get_ray(*eye, i, j, width, height));
            Vec3 color = get_color(ray, eye, -1, 7);
            delete ray;
            
            raw_img[j * (width * 3) + i * 3] = color.x;
            raw_img[j * (width * 3) + i * 3 + 1] = color.y;
            raw_img[j * (width * 3) + i * 3 + 2] = color.z;
        }
    }

    double max = *std::max_element(raw_img, raw_img + height * width * 3); 
    double min = *std::min_element(raw_img, raw_img + height * width * 3); 
    max = max - (min / 2);
    for (int i = 0; i < height * width * 3; i++) 
    {
        raw_img[i] = (raw_img[i] - (min / 2)) / max; // change black point, normalize to [0,1]
        img[i] = (unsigned char)(255 * raw_img[i]); // change to rgb
    }

    delete eye;

    stbi_write_png("objs2.png", width, height, 3, img, 3 * width);
    delete[] img;
	return 0;
}