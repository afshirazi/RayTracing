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

std::vector<Circle *> circles;
std::vector<Triangle*> triangles;

Vec3 get_ray(const Vec3& eye, int x, int y, int width, int height)
{
    Vec3 lookAt(0, 0, -1);
    Vec3 up(0, 1, 0);
    double a = (double)width / (double)height;

    Vec3 l = Vec3::normalize(lookAt - eye);
    Vec3 v = Vec3::normalize((l.cross(up)));
    Vec3 u = Vec3::normalize(v.cross(l));

    Vec3 ll = eye + (l * 1.2/*multiply to change fov*/) - (v * a) - u;
    Vec3 p = ll + (v * 2 * a * ((double)x / width)) + (u * 2 * ((double)y / height));
    Vec3 dir = Vec3::normalize((p - eye));

    return (dir - eye);
}

Vec3 get_color(const Vec3 *ray, const Vec3 *origin)
{
    bool intersect = false;
    Vec3 color;
    double zBuf = -std::numeric_limits<double>::max();

    for (Circle * circle : circles)
    {
        Vec3 intr = circle->get_intersect(*ray, *origin);
        if (intr != Vec3(-1,-1,-1))
        {
            intersect = true;
            if (intr.z > zBuf)
            {
                zBuf = intr.z;
                color = circle->color;
            }
        }
    }
    for (Triangle * tri : triangles)
    {
        Vec3 intr = tri->get_intersect(*ray, *origin);
        if (intr != Vec3(-1, -1, -1))
        {
            intersect = true;
            if (intr.z > zBuf)
            {
                zBuf = intr.z;
                color = tri->color;
            }
        }
    }

    if (intersect)
        return Vec3(color.x, color.y, color.z);
    else 
        return Vec3(0.5, 0.5, 0.5);
}

int main()
{
    const int width = 1600;
    const int height = 900;

    const Vec3 *eye = new Vec3(0, 0, 0);

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

    stbi_write_png("test35.png", width, height, 3, img, 3 * width);
    delete[] img;
	return 0;
}