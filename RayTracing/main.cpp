#include <iostream>
#include <fstream>
#include <vector>
#include "Vec3.h"

int width, height;
Vec3 back_col;
std::vector<Vec3> circles;

Vec3 get_color(int x, int y)
{
    Vec3 eye(0, 0, 0);
    Vec3 lookAt(0, 0, -1);
    Vec3 up(0, 1, 0);

    Vec3 l = (lookAt - eye) / ((lookAt - eye).mag());
    Vec3 v = (l.cross(up)) / ((l.cross(up)).mag());
    Vec3 u = v.cross(l);


}

int main()
{
    width = 800;
    height = 800;

    back_col = Vec3(0.5, 0.5, 0.5); // grey

    // circles assumed to be contstant rad=3 for now
    // color (0.3, 0.2, 0.8) 
    circles.push_back(Vec3(-1, -2, -8));
    circles.push_back(Vec3(3, -2, -7));

    std::ofstream file("xddd.ppm");
    file << "P3\n" << width << ' ' << height << "\n255\n";

    for (int i = 0; i < width; i++) {
        for (int j = 0; j < height; j++) {
            Vec3 color = get_color(i, j);
            file << (int)(color.x * 255) << ' ' << (int)(color.y * 255) << ' ' << (int)(color.z * 255) << '\n';
        }
    }

    file.close();
	return 0;
}