#pragma once
#include "Vec3.h"

class Light
{
public:
	Vec3 pos;
	Vec3 amb;
	Vec3 diff;
	Vec3 spec;

	Light();
	Light(const Vec3& position, const Vec3& ambient, const Vec3& diffuse, const Vec3& specular);
};

