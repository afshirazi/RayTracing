#include <iostream>
#include <fstream>
#include <vector>
#include <math.h>
#include "Vec3.h"

int width, height;
std::vector<Vec3> circles;

Vec3 get_ray(int x, int y)
{
    Vec3 eye(0, 0, 0);
    Vec3 lookAt(0, 0, -1);
    Vec3 up(0, 1, 0);

    Vec3 l = Vec3::normalize(lookAt - eye);
    Vec3 v = Vec3::normalize((l.cross(up)));
    Vec3 u = v.cross(l);

    Vec3 ll = eye + (l * 0.4142135624) - v - u;
    Vec3 p = ll + (v * 2 * x) + (u * 2 * y);
    Vec3 dir = Vec3::normalize((p - eye));

    return (dir - eye);
}

bool is_intersect(const Vec3& cent, const Vec3& d)
{
    double b = 2 * (d.x * (0 - cent.x) + d.y * (0 - cent.y) + d.z * (0 - cent.z));
    double c = (0 - cent.x) * (0 - cent.x) + (0 - cent.y) * (0 - cent.y) + (0 - cent.z) * (0 - cent.z) - 9;

    double t1 = -b + sqrt(b * b - 4 * c);
    double t2 = -b - sqrt(b * b - 4 * c);

    if (t1 > 0 || t2 > 0)
        return true;
}

Vec3 get_color(const Vec3& ray)
{
    bool intersect = false;

    for (Vec3 circle : circles)
        if (is_intersect(circle, ray))
        {
            //intersect = true;
            break;
        }

    if (intersect)
        return Vec3(0.3, 0.2, 0.8);
    return Vec3(0.5, 0.5, 0.5);
}

int main()
{
    width = 800;
    height = 800;

    // circles assumed to be contstant rad=3 for now
    // color (0.3, 0.2, 0.8) 
    circles.push_back(Vec3(-1, -2, -8));
    circles.push_back(Vec3(3, -2, -7));

    std::ofstream file("test2.ppm");
    file << "P3\n" << width << ' ' << height << "\n255\n";

    for (int i = 0; i < width; i++) {
        for (int j = 0; j < height; j++) {
            Vec3 ray = get_ray(i, j);
            Vec3 color = get_color(ray);
            std::cout << "Pixel: " << i + j << " Ray (" << ray.x << ", " << ray.y << ", " << ray.z << ")" << std::endl;
            file << (int)(color.x * 255) << ' ' << (int)(color.y * 255) << ' ' << (int)(color.z * 255) << '\n';
        }
    }

    file.close();
	return 0;
}