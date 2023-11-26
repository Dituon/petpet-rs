uniform shader image;
uniform float brightness;
uniform float contrast;

half4 main(float2 coord) {
    half4 color = image.eval(coord);
    color.rgb += brightness;
    if (contrast > 0.0) {
        color.rgb = (color.rgb - 0.5) / (1.0 - contrast) + 0.5;
    } else {
        color.rgb = (color.rgb - 0.5) * (1.0 + contrast) + 0.5;
    }
    return color;
}