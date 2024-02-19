#include "Light.h"

Light::Light()
{
	amb = Vec3(0, 0, 0);
	pos = Vec3(0, 0, 0);
	diff = Vec3(0, 0, 0);
	spec = Vec3(0, 0, 0);
}

Light::Light(const Vec3& position, const Vec3& ambient, const Vec3& diffuse, const Vec3& specular)
{
	amb = ambient;
	pos = position;
	diff = diffuse;
	spec = specular;
}
