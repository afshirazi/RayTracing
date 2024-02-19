#include "Circle.h"
#include <cmath>
#include <iostream>

Circle::Circle()
{
	center = Vec3(0, 0, 0);
	color = Vec3(0, 0, 0);
	radius = 0;

    diff = Vec3(0, 0, 0);
    spec = Vec3(0, 0, 0);
    shin = 0;
}

Circle::Circle(const Vec3& cent, const Vec3& col, double rad)
{
	center = cent;
	color = col;
	radius = rad;

    amb = Vec3(0.2, 0.2, 0.2);
    diff = col * 0.4;
    spec = col * 0.7;
    shin = 2;
}

Circle::Circle(const Vec3& cent, const Vec3& col, const Vec3& ambient, const Vec3& diffuse, const Vec3& specular, double shininess, double rad)
{
    center = cent;
    color = col;
    radius = rad;

    amb = ambient;
    diff = diffuse;
    spec = specular;
    shin = shininess;
}

Circle::Circle(double x, double y, double z, double r, double g, double b, double rad)
{
	Circle(Vec3(x, y, z), Vec3(r, g, b), rad);
}

Vec3 Circle::get_intersect(const Vec3& ray, const Vec3& origin) const
{
    double b = 2 * ray.dot(origin - center);
    double c = (origin - center).dot(origin - center) - radius * radius;

    double t1 = (- b + std::sqrt(b * b - 4 * c)) / 2;
    double t2 = (- b - std::sqrt(b * b - 4 * c)) / 2;

    t1 = std::isnan(t1) ? -1 : t1;
    t2 = std::isnan(t2) ? -1 : t2;

    
    if (t1 > 0 && t2 > 0)
    {
        if (t1 <= t2)
            return (origin + ray * t1);
        else
            return (origin + ray * t2);
    }
    else if (t1 > 0)
    {
        return (origin + ray * t1);
    }
    else if (t2 > 0)
    {
        return (origin + ray * t2);
    }
    
    return Vec3(-1, -1, -1); // return dummy vector if fail
}

Vec3 Circle::get_normal(const Vec3& point) const
{
	return Vec3::normalize(point - center);
}

