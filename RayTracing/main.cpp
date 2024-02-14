#include <iostream>
#include <fstream>
#include <vector>
#include <math.h>
#include "Vec3.h"

int width, height;
std::vector<Vec3 *> circles;

Vec3 get_ray(int x, int y, int width, int height)
{
    Vec3 eye(0, 0, 0);
    Vec3 lookAt(0, 0, -1);
    Vec3 up(0, 1, 0);

    Vec3 l = Vec3::normalize(lookAt - eye);
    Vec3 v = Vec3::normalize((l.cross(up)));
    Vec3 u = Vec3::normalize(v.cross(l));

    Vec3 ll = eye + (l ) - v - u;
    Vec3 p = ll + (v * 2 * ((double)x / width)) + (u * 2 * ((double)y / height));
    Vec3 dir = Vec3::normalize((p - eye));

    return (dir - eye);
}

bool is_intersect(const Vec3& cent, const Vec3& d)
{
    double b = 2 * d.dot(cent * -1);
    double c = (cent * -1).dot(cent * -1) - 4;

    //double t1 = -b + sqrt(b * b - 4 * c);
    //double t2 = -b - sqrt(b * b - 4 * c);

    //if (t1 > 0 || t2 > 0)
    //    return true;
    
    return ((b * b - 4 * c) >= 0);
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
    width = 800;
    height = 800;

    // circles contstant radius=2 for now
    // color (0.3, 0.2, 0.8) 
    circles.push_back(new Vec3(-10, -10, -10));
    circles.push_back(new Vec3(-20, -20, -10));

    std::ofstream file("test20.ppm");
    file << "P3\n" << width << ' ' << height << "\n255\n";

    for (int i = 0; i < width; i++) {
        for (int j = 0; j < height; j++) {
            Vec3 ray = get_ray(i, j, width, height);
            Vec3 color = get_color(ray);
            //std::cout << "Pixel: " << i + j << " Ray (" << ray.x << ", " << ray.y << ", " << ray.z << ")" << std::endl;
            file << (int)(color.x * 255) << ' ' << (int)(color.y * 255) << ' ' << (int)(color.z * 255) << '\n';
        }
    }

    file.close();
	return 0;
}