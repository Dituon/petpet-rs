uniform shader image;
half4 main(float2 coord) {
    vec4 color = image.eval(coord).bgra;
    float gray = (color.r + color.g + color.b) / 3.0;
    return half4(gray, gray, gray, color.a);
}