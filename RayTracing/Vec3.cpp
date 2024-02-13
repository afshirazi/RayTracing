#include "Vec3.h"
#include <math.h>

Vec3::Vec3(double xx, double yy, double zz) {
	x = xx;
	y = yy;
	z = zz;
}

Vec3::Vec3(const Vec3& vec)
{
	Vec3(vec.x, vec.y, vec.z);
}

Vec3 Vec3::operator+(Vec3 & vec) const
{
	return Vec3(x + vec.x, y + vec.y, z + vec.z);
}

Vec3 Vec3::operator-(Vec3& vec) const
{
	return Vec3(x - vec.x, y - vec.y, z - vec.z);
}

Vec3 Vec3::operator*(double c) const
{
	return Vec3(x * c, y * c, z * c);
}

Vec3 Vec3::operator/(double c) const
{
	return Vec3(x / c, y / c, z / c);
}

double Vec3::dot(Vec3& vec) const
{
	return x * vec.x + y * vec.y + z * vec.z;
}

Vec3 Vec3::cross(Vec3& vec) const
{
	float i = y * vec.z - vec.y * z;
	float j = x * vec.z - vec.x * z;
	float k = x * vec.y - vec.x * y;
	return Vec3(i, -j, k);
}

double Vec3::mag() const
{
	return sqrt(x * x + y * y + z * z);
}

Vec3 Vec3::normalize(Vec3& vec)
{
	return Vec3(vec * (1 / vec.mag()));
}


