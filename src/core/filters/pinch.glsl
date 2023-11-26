uniform shader image;
uniform float radius;
uniform float strength;
uniform float x;
uniform float y;

half4 main(float2 coord) {
    coord.x -= x;
    coord.y -= y;
    float distance = length(coord);
    if (distance < radius) {
        float percent = distance / radius;
        coord *= mix(1.0, pow(percent, 1.0 + strength * 0.75) * radius / distance, 1.0 - percent);
    }
    coord.x += x;
    coord.y += y;
    return image.eval(coord);
}
