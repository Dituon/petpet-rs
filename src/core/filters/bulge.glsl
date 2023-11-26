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
        coord *= mix(1.0, smoothstep(0.0, radius / distance, distance / radius), strength * 0.75);
    }
    coord.x += x;
    coord.y += y;
    return image.eval(coord);
}
