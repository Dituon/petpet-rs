uniform shader image;
half4 main(float2 coord) {
    half4 color = image.eval(coord);
    if ((color.r + color.g + color.b) >= 1.50196078431373) {
        return half4(1.0, 1.0, 1.0, 1.0);
    } else {
        return half4(0.0, 0.0, 0.0, 1.0);
    }
}