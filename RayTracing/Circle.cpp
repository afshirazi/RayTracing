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

Circle::Circle(const Circle& cir)
{
    center = Vec3(cir.center);
    color = Vec3(cir.color);
    radius = cir.radius;

    diff = Vec3(cir.diff);
    spec = Vec3(cir.spec);
    shin = cir.shin;
}

Circle::Circle(const Vec3& cent, const Vec3& col, double rad)
{
	center = cent;
	color = col;
	radius = rad;

    diff = col * 0.4;
    spec = col * 0.7;
    shin = 20;
}

Circle::Circle(const Vec3& cent, const Vec3& col, const Vec3& diffuse, const Vec3& specular, double shininess, double rad)
{
    center = cent;
    color = col;
    radius = rad;

    diff = diffuse;
    spec = specular;
    shin = shininess;
}

Circle::Circle(double x, double y, double z, double r, double g, double b, double rad)
{
	Circle(Vec3(x, y, z), Vec3(r, g, b), rad);
}

bool Circle::operator==(const Circle& cir) const
{
    if (center == cir.center && color == cir.color && radius == cir.radius)
        return true;
    return false;
}

bool Circle::operator!=(const Circle& cir) const
{
    if (center == cir.center && color == cir.color && radius == cir.radius)
        return false;
    return true;
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
    
    return Vec3(0, 0, 1); // return dummy vector if fail // just realized dummy vector needs to be changed, good enough for now
}

Vec3 Circle::get_normal(const Vec3& point) const
{
	return Vec3::normalize(point - center);
}

