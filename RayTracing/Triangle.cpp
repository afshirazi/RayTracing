#include "Triangle.h"
#include <cmath>
#include <limits>

Triangle::Triangle()
{
	a = Vec3(0, 0.5, 0);
	b = Vec3(-0.5, 0, 0);
	c = Vec3(0.5, 0, 0);
	color = Vec3(0, 0, 0);

	diff = Vec3(0, 0, 0);
	spec = Vec3(0, 0, 0);
	shin = 0;
}

Triangle::Triangle(const Vec3& aa, const Vec3& bb, const Vec3& cc, const Vec3& col)
{
	a = aa;
	b = bb;
	c = cc;
	color = col;

	diff = Vec3(0.2, 0.2, 0.2);
	spec = Vec3(0.7, 0.7, 0.7);
	shin = 10;
}

Vec3 Triangle::get_normal() const
{
	Vec3 ab = b - a;
	Vec3 ac = c - a;
	Vec3 norm = ab.cross(ac);

	return Vec3::normalize(norm);
}

Vec3 Triangle::get_intersect(const Vec3& ray, const Vec3& origin) const
{
	constexpr double epsilon = std::numeric_limits<double>::epsilon();

	Vec3 edge1 = b - a;
	Vec3 edge2 = c - a;
	Vec3 ray_cross_e2 = ray.cross(edge2);
	double det = edge1.dot(ray_cross_e2);

	if (det > -epsilon && det < epsilon)
		return Vec3(-1, -1, -1);    // This ray is parallel to this triangle.

	double inv_det = 1.0 / det;
	Vec3 s = origin - a;
	double u = inv_det * s.dot(ray_cross_e2);

	if (u < 0 || u > 1)
		return Vec3(-1, -1, -1);

	Vec3 s_cross_e1 = s.cross(edge1);
	double v = inv_det * ray.dot(s_cross_e1);

	if (v < 0 || u + v > 1)
		return Vec3(-1, -1, -1);

	// At this stage we can compute t to find out where the intersection point is on the line.
	double t = inv_det * edge2.dot(s_cross_e1);

	if (t > epsilon) // ray intersection
	{
		Vec3 p = origin + ray * t;
		return Vec3(p.x, p.y, p.z);
	}
	else // This means that there is a line intersection but not a ray intersection.
		return Vec3(-1, -1, -1);
}
