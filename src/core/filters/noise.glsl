uniform shader image;
uniform float amount;

float rand(half2 co) {
    return fract(sin(dot(co.xy ,half2(12.9898,78.233))) * 43758.5453);
}

half4 main(float2 coord) {
    half4 color = image.eval(coord);
    float diff = (rand(coord) - 0.5) * amount;
    color.r += diff;
    color.g += diff;
    color.b += diff;
    return color;
}