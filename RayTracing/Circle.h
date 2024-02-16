#pragma once
#include "Vec3.h"

class Circle
{
public:
	Vec3 center;
	Vec3 color;
	double radius;
	Vec3 diff;
	Vec3 spec;
	double shin;
	
	Circle();
	Circle(const Vec3& vec, const Vec3& col, double rad);
	Circle(double x, double y, double z, double r, double g, double b, double rad);
	
	Vec3 get_intersect(const Vec3& ray, const Vec3& origin) const;
	Vec3 get_normal(const Vec3& point) const;
};

