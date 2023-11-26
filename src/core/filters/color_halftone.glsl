const float PI = 3.14159265358979323846;

uniform shader image;
uniform float radius;
uniform float angle;
uniform float x;
uniform float y;

float pattern(float2 coord, float angle) {
    float s = sin(angle), c = cos(angle);
    half2 tex = coord - half2(x, y);
    half2 point = half2(
    c * tex.x - s * tex.y,
    s * tex.x + c * tex.y
    ) * (PI / (radius * 2.828));
    return (sin(point.x) * sin(point.y)) * 4.0;
}

half4 main(float2 coord) {
    half4 color = image.eval(coord);
    half3 cmy = 1.0 - color.rgb;
    float k = min(cmy.x, min(cmy.y, cmy.z));
    cmy = (cmy - k) / (1.0 - k);
    cmy = clamp(cmy * 10.0 - 3.0 + half3(pattern(coord, angle + 0.26179), pattern(coord, angle + 1.30899), pattern(coord, angle)), 0.0, 1.0);
    k = clamp(k * 10.0 - 5.0 + pattern(coord, angle + 0.78539), 0.0, 1.0);
    return half4(1.0 - cmy - k, color.a);
}
