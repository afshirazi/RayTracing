#pragma once
#include "Vec3.h"

class Triangle 
{
public:
	Vec3 a; // coordinates for first vertex of triangle
	Vec3 b; // second
	Vec3 c; // third
	Vec3 diff;
	Vec3 spec;
	double shin;

	Triangle();
	Triangle(const Vec3& a, const Vec3& b, const Vec3& c, const Vec3& col);
	Triangle(const Vec3& a, const Vec3& b, const Vec3& c, const Vec3& diff, const Vec3& spec, double shin);

	bool operator==(const Triangle& tri) const;
	bool operator!=(const Triangle& tri) const;
	Vec3 get_normal() const;
	Vec3 get_intersect(const Vec3& ray, const Vec3& origin) const;
};

