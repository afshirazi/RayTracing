#pragma once
#include "Vec3.h"

class Quad 
{
private:
	Vec3 norm;
	bool is_planar;

public:
	Vec3 a; // coordinates for first vertex of quad
	Vec3 b; // second
	Vec3 c; // third
	Vec3 d; // fourth
	Vec3 color;
	Vec3 amb;
	Vec3 diff;
	Vec3 spec;
	double shin;

	Quad();
	Quad(const Vec3& a, const Vec3& b, const Vec3& c, const Vec3& d, const Vec3& col);

	Vec3 get_normal() const;
	Vec3 get_intersect(const Vec3& ray, const Vec3& origin) const;
};

