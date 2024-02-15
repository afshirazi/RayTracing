#pragma once
class Vec3
{
public:
	double x;
	double y;
	double z;

	Vec3();
	Vec3(double x, double y, double z);
	Vec3(const Vec3& vec);

	Vec3 operator+(const Vec3& vec) const;
	Vec3 operator-(const Vec3& vec) const;
	Vec3 operator*(const double c) const;
	Vec3 operator/(const double c) const;
	bool operator==(const Vec3& v2) const;
	bool operator!=(const Vec3& v2) const;
	double dot(const Vec3& vec) const;
	Vec3 cross(const Vec3& vec) const;
	double mag() const;
	static Vec3 normalize(const Vec3 &vec);
};

