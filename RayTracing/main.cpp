#include <iostream>
#include <fstream>

int main()
{
    std::ofstream file("new.ppm");
    file << "P3\n" << 256 << ' ' << 256 << "\n255\n";

    for (int i = 0; i < 256; i++) {
        for (int j = 0; j < 256; j++) {
            double r = i / 255.0;
            double g = j / 255.0;
            double b = (i + j) / (2 * 255.0);

            file << (int)(r * 255) << ' ' << (int)(g * 255) << ' ' << (int)(b * 255) << '\n';
        }
    }

    file.close();
	return 0;
}