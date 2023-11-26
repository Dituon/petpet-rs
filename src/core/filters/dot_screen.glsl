const float PI = 3.14159265358979323846;

uniform shader image;
uniform float radius;
uniform float angle;
uniform float x;
uniform float y;

float pattern(float2 coord) {
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
    float average = (color.r + color.g + color.b) / 3.0;
    return half4(half3(average * 10.0 - 5.0 + pattern(coord)), color.a);
}
