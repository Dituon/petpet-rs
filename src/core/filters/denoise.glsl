uniform shader image;

uniform float exponent;

float rand(half2 co) {
    return fract(sin(dot(co.xy, half2(12.9898, 78.233))) * 43758.5453);
}

half4 main(float2 coord) {
    half4 center = image.eval(coord);
    half4 color = half4(0.0);
    float total = 0.0;
    for (float x = -4.0; x <= 4.0; x += 1.0) {
        for (float y = -4.0; y <= 4.0; y += 1.0) {
            half4 tex = image.eval(coord  + half2(x, y));
            float weight = 1.0 - abs(dot(tex.rgb - center.rgb, half3(0.25)));
            weight = pow(weight, exponent);
            color += tex * weight;
            total += weight;
        }
    }
    return color / total;
}