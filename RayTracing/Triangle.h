#pragma once
#include "Vec3.h"

class Triangle
{
public:
	Vec3 a; // coordinates for first vertex of triangle
	Vec3 b; // second
	Vec3 c; // third
	Vec3 color;

	Triangle();
	Triangle(const Vec3& a, const Vec3& b, const Vec3& c, const Vec3& col);

	Vec3 get_normal() const;
	Vec3 get_intersect(const Vec3& ray, const Vec3& origin) const;
};

