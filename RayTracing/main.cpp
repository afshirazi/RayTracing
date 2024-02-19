#include <iostream>
#include <fstream>
#include <vector>
#include <cmath>
#include <limits>

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "./stb_image_write.h"

#include "Vec3.h"
#include "Circle.h"
#include "Triangle.h"
#include "Quad.h"
#include "Light.h"

std::vector<Circle *> circles;
std::vector<Triangle*> triangles;
std::vector<Light*> lights;

enum ObjType {
    EMPTY, CIRCLE, TRIANGLE, QUAD
};

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

Vec3 get_color(const Vec3 *ray, const Vec3 *origin)
{
    ObjType intersect = EMPTY;
    Circle cir;
    Triangle tri;
    Quad quad;
    Vec3 color;
    double zBuf = -std::numeric_limits<double>::max(); // almost -inf

    for (Circle * _circle : circles) // def cleaner with encapsulation but this works
    {
        Vec3 intr = _circle->get_intersect(*ray, *origin);
        if (intr != Vec3(-1,-1,-1))
        {
            if (intr.z > zBuf)
            {
                intersect = CIRCLE;
                zBuf = intr.z;
                cir = *_circle;
            }
        }
    }
    for (Triangle * _tri : triangles)
    {
        Vec3 intr = _tri->get_intersect(*ray, *origin);
        if (intr != Vec3(-1, -1, -1))
        {
            if (intr.z > zBuf)
            {
                intersect = TRIANGLE;
                zBuf = intr.z;
                tri = *_tri;
            }
        }
    }

    if (intersect == CIRCLE) // I like circles
    {
        Vec3 intr = cir.get_intersect(*ray, *origin);
        Vec3 norm = cir.get_normal(intr);
        //Vec3 refl = Vec3::normalize(norm * (2 * norm.dot((*ray) - (*origin))) - intr);

        Vec3 view_vec = *origin - *ray;
        for (Light* light : lights)
        {
            Vec3 light_vec = Vec3::normalize(light->pos - intr);
            Vec3 light_refl = Vec3::normalize(norm * (2 * norm.dot(light_vec)) - light_vec);
            color.x += light->diff.x * (light_vec.dot(norm)) * cir.diff.x + light->spec.x * std::pow(light_refl.dot(view_vec), cir.shin);
            color.y += light->diff.y * (light_vec.dot(norm)) * cir.diff.y + light->spec.y * std::pow(light_refl.dot(view_vec), cir.shin);
            color.z += light->diff.z * (light_vec.dot(norm)) * cir.diff.z + light->spec.z * std::pow(light_refl.dot(view_vec), cir.shin);
        }
        color.x = color.x < 1.0 ? color.x : 1.0;
        color.y = color.y < 1.0 ? color.y : 1.0;
        color.z = color.z < 1.0 ? color.z : 1.0;

        color.x = color.x > 0 ? color.x : 0;
        color.y = color.y > 0 ? color.y : 0;
        color.z = color.z > 0 ? color.z : 0;

        return Vec3(color.x, color.y, color.z);
    }
    else if (intersect == TRIANGLE)
    {
        Vec3 intr = tri.get_intersect(*ray, *origin);
        Vec3 norm = tri.get_normal();
        //Vec3 refl = Vec3::normalize(norm * (2 * norm.dot((*ray) - (*origin))) - intr);

        Vec3 view_vec = Vec3::normalize(*origin - *ray);
        for (Light* light : lights)
        {
            Vec3 light_vec = Vec3::normalize(light->pos - intr);
            Vec3 light_refl = Vec3::normalize(norm * (2 * norm.dot(light_vec)) - light_vec);
            color.x += light->diff.x * (light_vec.dot(norm)) * tri.diff.x + light->spec.x * std::pow(light_refl.dot(view_vec), tri.shin);
            color.y += light->diff.y * (light_vec.dot(norm)) * tri.diff.y + light->spec.y * std::pow(light_refl.dot(view_vec), tri.shin);
            color.z += light->diff.z * (light_vec.dot(norm)) * tri.diff.z + light->spec.z * std::pow(light_refl.dot(view_vec), tri.shin);
        }
        color.x = color.x < 1.0 ? color.x : 1.0;
        color.y = color.y < 1.0 ? color.y : 1.0;
        color.z = color.z < 1.0 ? color.z : 1.0;

        color.x = color.x > 0 ? color.x : 0;
        color.y = color.y > 0 ? color.y : 0;
        color.z = color.z > 0 ? color.z : 0;

        return Vec3(color.x, color.y, color.z);
    }
    else if (intersect == QUAD)
    {
        for (Light* light : lights)
        {

        }
    }
    else 
        return Vec3(0.5, 0.5, 0.5);
}

int main()
{
    const int width = 1600;
    const int height = 900;

    const Vec3 *eye = new Vec3(0, 0, 0);

    lights.push_back(new Light(Vec3(40, 30, -3), Vec3(0.1, 0.1, 0.1), Vec3(1, 1, 1), Vec3(1, 1, 1)));

    circles.push_back(new Circle(*(new Vec3(0, 0, -10)), *(new Vec3(0.3, 0.2, 0.8)), 2));
    circles.push_back(new Circle(*(new Vec3(-4, -4, -5.5)), *(new Vec3(0.3, 0.7, 0.9)), 1));
    circles.push_back(new Circle(*(new Vec3(4, 4, -11)), *(new Vec3(0.3, 0.7, 0.9)), 1));

    triangles.push_back(new Triangle(*(new Vec3(4, -4, -9)), *(new Vec3(0, 0, -9)), *(new Vec3(-9, -5, -9)), *(new Vec3(0.7, 0.2, 0.3))));
    triangles.push_back(new Triangle(*(new Vec3(-4, -4, -8.5)), *(new Vec3(0, 0, -9)), *(new Vec3(-9, -5, -9)), *(new Vec3(0.1, 0.1, 0.1))));
    triangles.push_back(new Triangle(*(new Vec3(9, 5, -9)), *(new Vec3(4, 4, -9)), *(new Vec3(0, 0, -9)), *(new Vec3(0.4, 0.9, 0.2))));

    unsigned char *img = new unsigned char[height * width * 3];

    for (int j = 0; j < height; j++) {
        for (int i = 0; i < width; i++) {
            Vec3* ray = new Vec3(get_ray(*eye, i, j, width, height));
            Vec3 color = get_color(ray, eye);
            delete ray;
            
            img[j * (width * 3) + i * 3] = (char)(255 * color.x);
            img[j * (width * 3) + i * 3 + 1] = (char)(255 * color.y);
            img[j * (width * 3) + i * 3 + 2] = (char)(255 * color.z);
        }
    }

    delete eye;

    stbi_write_png("test3.png", width, height, 3, img, 3 * width);
    delete[] img;
	return 0;
}