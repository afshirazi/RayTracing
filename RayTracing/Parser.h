#pragma once
#include "Vec3.h"
#include "Light.h"
#include "Circle.h"
#include "Triangle.h"

#include <vector>
#include <string>

struct ObjUni {
    Triangle tri;
    Circle cir;

    ObjUni() = default;
    ObjUni(Circle* _cir) { cir = *_cir; }
    ObjUni(Triangle* _tri) { tri = *_tri; }
};

class Parser
{
public:
    std::vector<ObjUni*> objects;
    std::vector<Light*> lights;

    Parser();
    void parse_file(std::string filename);
};

