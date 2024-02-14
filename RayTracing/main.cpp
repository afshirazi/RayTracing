#include <iostream>
#include <fstream>
#include <vector>
#include <math.h>
#include "Vec3.h"
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "./stb_image_write.h"

std::vector<Vec3 *> circles;

Vec3 get_ray(int x, int y, int width, int height)
{
    Vec3 eye(0, 0, 0);
    Vec3 lookAt(0, 0, -1);
    Vec3 up(0, 1, 0);

    Vec3 l = Vec3::normalize(lookAt - eye);
    Vec3 v = Vec3::normalize((l.cross(up)));
    Vec3 u = Vec3::normalize(v.cross(l));

    Vec3 ll = eye + (l /*multiply to change fov*/)-v - u;
    Vec3 p = ll + (v * 2 * ((double)x / width)) + (u * 2 * ((double)y / height));
    Vec3 dir = Vec3::normalize((p - eye));

    return (dir - eye);
}

bool is_intersect(const Vec3& cent, const Vec3& d)
{
    double b = 2 * d.dot(cent * -1);
    double c = (cent * -1).dot(cent * -1) - 4;

    double t1 = -b + sqrt(b * b - 4 * c);
    double t2 = -b - sqrt(b * b - 4 * c);

    t1 = isnan(t1) ? -1 : t1;
    t2 = isnan(t2) ? -1 : t2;

    //std::cout << t1 << " " << t2 << std::endl;
    if (t1 > 0 || t2 > 0)
        return true;
    
    return false;
}

Vec3 get_color(const Vec3& ray)
{
    bool intersect = false;

    for (Vec3 * circle : circles)
    {
        if (is_intersect(*circle, ray))
        {
            
            intersect = true;
            break;
        }
    }

    if (intersect)
        return Vec3(0.3, 0.2, 0.8);
    return Vec3(0.5, 0.5, 0.5);
}

int main()
{
    const int width = 800;
    const int height = 1000;

    // circles contstant radius=2 for now
    // color (0.3, 0.2, 0.8) 
    circles.push_back(new Vec3(-10, -10, -10));
    circles.push_back(new Vec3(-20, -20, -10));

    unsigned char *img = new unsigned char[height * width * 3];

    for (int j = 0; j < height; j++) {
        for (int i = 0; i < width; i++) {
            Vec3 ray = get_ray(i, j, width, height);
            Vec3 color = get_color(ray);
            
            img[j * (width * 3) + i * 3] = (char)(255 * color.x);
            img[j * (width * 3) + i * 3 + 1] = (char)(255 * color.y);
            img[j * (width * 3) + i * 3 + 2] = (char)(255 * color.z);
        }
    }
    
    stbi_write_png("test4.png", width, height, 3, img, 3 * width);
    delete[] img;
	return 0;
}