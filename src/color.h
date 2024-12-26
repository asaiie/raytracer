#ifndef COLOR_H
#define COLOR_H

#include "vec3.h"

#include <iostream>

using color = vec3;

inline void write_color(std::ostream& out, const color& pixel_color) {
    auto r = pixel_color.x();
    auto g = pixel_color.y();
    auto b = pixel_color.z();

    int rb = int(255.999 * r);
    int gb = int(255.999 * g);
    int bb = int(255.999 * b);

    out << rb << ' ' << gb << ' ' << bb << '\n';
}

#endif

