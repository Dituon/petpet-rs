const float PI = 3.14159265358979323846;

uniform shader image;
uniform float hue;
uniform float saturation;
uniform float brightness;

half4 main(float2 coord) {
    half4 color = image.eval(coord);
    color.rgb += brightness;
    float angle = hue * PI;
    float s = sin(angle), c = cos(angle);
    half3 weights = (half3(2.0 * c, -sqrt(3.0) * s - c, sqrt(3.0) * s - c) + 1.0) / 3.0;
    float len = length(color.rgb);
    color.rgb = half3(
        dot(color.rgb, weights.xyz),
        dot(color.rgb, weights.zxy),
        dot(color.rgb, weights.yzx)
    );

    float average = (color.r + color.g + color.b) / 3.0;
    if (saturation > 0.0) {
        color.rgb += (average - color.rgb) * (1.0 - 1.0 / (1.001 - saturation));
    } else {
        color.rgb += (average - color.rgb) * (-saturation);
    }
    return color;
}