#pragma once
#include "Vec3.h"
class Circle 
{
public:
	Vec3 center;
	double radius;
	Vec3 diff;
	Vec3 spec;
	double shin;
	
	Circle();
	Circle(const Circle& cir);
	Circle(const Vec3& cent, const Vec3& col, double rad);
	Circle(const Vec3& cent, const Vec3& diff, const Vec3& spec, double shin, double rad);
	Circle(double x, double y, double z, double r, double g, double b, double rad);
	
	bool operator==(const Circle& cir) const;
	bool operator!=(const Circle& cir) const;
	Vec3 get_intersect(const Vec3& ray, const Vec3& origin) const;
	Vec3 get_normal(const Vec3& point) const;
};

