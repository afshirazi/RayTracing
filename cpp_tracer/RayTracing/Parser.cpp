#include "Parser.h"
#include <iostream>
#include <fstream>
#include <sstream>

#include <algorithm> 
#include <cctype>
#include <locale>

// See 24
inline void ltrim(std::string& s) {
	s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {
		return !std::isspace(ch);
		}));
}

// See 24
inline void rtrim(std::string& s) {
	s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) {
		return !std::isspace(ch);
		}).base(), s.end());
}

// for convenience, copied from StackOverflow: https://stackoverflow.com/questions/216823/how-to-trim-a-stdstring
inline void trim(std::string& s) {
	rtrim(s);
	ltrim(s);
}

Parser::Parser()
{

}

Vec3 parse_vec(std::string vec_str)
{
	std::vector<double> tokens;
	trim(vec_str);
	std::stringstream stream(vec_str);
	std::string tempstr;

	while (std::getline(stream, tempstr, ' '))
	{
		tokens.push_back(std::stod(tempstr));
	}

	return Vec3(tokens[0], tokens[1], tokens[2]);
}

void Parser::parse_file(std::string filename)
{
	std::ifstream file;
	file.open(filename);

	std::string line;

	while (std::getline(file, line))
	{
		trim(line); // clear whitespace 
		
		// toLowerCase (taken from https://stackoverflow.com/questions/313970/how-to-convert-an-instance-of-stdstring-to-lower-case)
		std::transform(line.begin(), line.end(), line.begin(), [](unsigned char c) { return std::tolower(c); }); 
		
		if (line.find("sphere") != std::string::npos) // I just realized I called the class circle instead of sphere hahahaha
		{
			std::string pos_str;
			std::string rad_str;
			std::string diff_str;
			std::string spec_str;
			std::string shin_str;
			std::getline(file, pos_str);
			std::getline(file, rad_str);
			std::getline(file, diff_str);
			std::getline(file, spec_str);
			std::getline(file, shin_str);
			trim(pos_str);
			trim(rad_str);
			trim(diff_str);
			trim(spec_str);
			trim(shin_str);

			Vec3 pos_vec = parse_vec(pos_str.substr(3));
			double rad_val = std::stod(rad_str.substr(3));
			Vec3 diff_vec = parse_vec(diff_str.substr(4));
			Vec3 spec_vec = parse_vec(spec_str.substr(4));
			double shin_val = std::stod(shin_str.substr(4));

			objects.push_back(new ObjUni(new Circle(pos_vec, diff_vec, spec_vec, shin_val, rad_val)));
		}
		else if (line.find("triangle") != std::string::npos)
		{
			std::string a_str;
			std::string b_str;
			std::string c_str;
			std::string diff_str;
			std::string spec_str;
			std::string shin_str;
			std::getline(file, a_str);
			std::getline(file, b_str);
			std::getline(file, c_str);
			std::getline(file, diff_str);
			std::getline(file, spec_str);
			std::getline(file, shin_str);
			trim(a_str);
			trim(b_str);
			trim(c_str);
			trim(diff_str);
			trim(spec_str);
			trim(shin_str);

			Vec3 a_vec = parse_vec(a_str.substr(1));
			Vec3 b_vec = parse_vec(b_str.substr(1));
			Vec3 c_vec = parse_vec(c_str.substr(1));
			Vec3 diff_vec = parse_vec(diff_str.substr(4));
			Vec3 spec_vec = parse_vec(spec_str.substr(4));
			double shin_val = std::stod(shin_str.substr(4));

			objects.push_back(new ObjUni(new Triangle(a_vec, b_vec, c_vec, diff_vec, spec_vec, shin_val)));
		}
		else if (line.find("light") != std::string::npos)
		{
			std::string pos_str;
			std::string diff_str;
			std::string spec_str;
			std::getline(file, pos_str);
			std::getline(file, diff_str);
			std::getline(file, spec_str);
			trim(pos_str);
			trim(diff_str);
			trim(spec_str);

			Vec3 pos_vec = parse_vec(pos_str.substr(3));
			Vec3 diff_vec = parse_vec(diff_str.substr(4));
			Vec3 spec_vec = parse_vec(spec_str.substr(4));
			
			lights.push_back(new Light(pos_vec, diff_vec, spec_vec));
		}
	}

	file.close();
}
