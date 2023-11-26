uniform shader image;
uniform float radius;
uniform float angle;
uniform float x;
uniform float y;

half4 main(float2 coord) {
    coord.x -= x;
    coord.y -= y;
    float distance = length(coord);
    if (distance < radius) {
        float percent = (radius - distance) / radius;
        float theta = percent * percent * angle;
        float s = sin(theta);
        float c = cos(theta);
        coord = float2(coord.x * c - coord.y * s, coord.x * s + coord.y * c);
    }
    coord.x += x;
    coord.y += y;
    return image.eval(coord);
}
