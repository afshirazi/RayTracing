#include "Vec3.h"
#include <cmath>

Vec3::Vec3()
{
	x = 0;
	y = 0;
	z = 0;
}

Vec3::Vec3(double xx, double yy, double zz) 
{
	x = xx;
	y = yy;
	z = zz;
}

Vec3::Vec3(const Vec3& vec)
{
	Vec3(vec.x, vec.y, vec.z);
}

Vec3 Vec3::operator+(const Vec3& vec) const
{
	return Vec3(x + vec.x, y + vec.y, z + vec.z);
}

Vec3 Vec3::operator-(const Vec3& vec) const
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

void Vec3::operator+=(const Vec3& vec)
{
	x += vec.x;
	y += vec.y;
	z += vec.z;
}

bool Vec3::operator==(const Vec3& v2) const
{
	if (x == v2.x && y == v2.y && z == v2.z)
		return true;
	return false;
}

bool Vec3::operator!=(const Vec3& v2) const
{
	if (x == v2.x && y == v2.y && z == v2.z)
		return false;
	return true;
}

double Vec3::dot(const Vec3& vec) const
{
	return x * vec.x + y * vec.y + z * vec.z;
}

Vec3 Vec3::cross(const Vec3& vec) const
{
	float i = y * vec.z - vec.y * z;
	float j = x * vec.z - vec.x * z;
	float k = x * vec.y - vec.x * y;
	return Vec3(i, -j, k);
}

double Vec3::mag() const
{
	return std::sqrt(x * x + y * y + z * z);
}

Vec3 Vec3::normalize(const Vec3& vec)
{
	return Vec3(vec * (1 / vec.mag()));
}

double Vec3::euclidian_sq(const Vec3& v1, const Vec3& v2)
{
	double dx = v2.x - v1.x;
	double dy = v2.y - v1.y;
	double dz = v2.z - v1.z;
	return (dx * dx + dy * dy + dz * dz); 
}


