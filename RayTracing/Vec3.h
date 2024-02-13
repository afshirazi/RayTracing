#pragma once
class Vec3
{
public:
	double x;
	double y;
	double z;

	Vec3(double x, double y, double z);
	Vec3(const Vec3& vec);
	Vec3 operator+(Vec3& vec) const;
	Vec3 operator-(Vec3& vec) const;
	Vec3 operator*(double c) const;
	Vec3 operator/(double c) const;
	double dot(Vec3& vec) const;
	Vec3 cross(Vec3& vec) const;
	double mag() const;
	static Vec3 normalize(Vec3 &vec);
};
