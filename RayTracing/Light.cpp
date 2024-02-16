#include "Light.h"

Light::Light()
{
	pos = Vec3(0, 0, 0);
	diff = Vec3(0, 0, 0);
	spec = Vec3(0, 0, 0);
}

Light::Light(const Vec3& position, const Vec3& diffuse, const Vec3& specular)
{
	pos = position;
	diff = diffuse;
	spec = specular;
}
