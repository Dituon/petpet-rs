const float PI = 3.14159265358979323846;
uniform shader image;

uniform float scale;
uniform float stretch;
uniform float angle;
uniform float amount;
uniform float turbulence;
uniform float time;

float rand(float2 co){return fract(sin(dot(co.xy ,float2(12.9898,78.233))) * 43758.5453);}
float rand (float2 co, float l) {return rand(float2(rand(co), l));}
float rand (float2 co, float l, float t) {return rand(float2(rand(co, l), t));}

float perlin(float2 p, float dim, float time) {
    float2 pos = floor(p * dim);
    float2 posx = pos + float2(1.0, 0.0);
    float2 posy = pos + float2(0.0, 1.0);
    float2 posxy = pos + float2(1.0);

    float c = rand(pos, dim, time);
    float cx = rand(posx, dim, time);
    float cy = rand(posy, dim, time);
    float cxy = rand(posxy, dim, time);

    float2 d = fract(p * dim);
    d = -0.5 * cos(d * PI) + 0.5;

    float ccx = mix(c, cx, d.x);
    float cycxy = mix(cy, cxy, d.x);
    float center = mix(ccx, cycxy, d.y);

    return center * 2.0 - 1.0;
}

half4 main(float2 coord) {
    float c = cos(angle);
    float s = sin(angle);
    float nx = c * coord.x + s * coord.y;
    float ny = -s * coord.x + c * coord.y;
    nx /= scale;
    ny /= scale * stretch;
    coord.x += amount * perlin(float2(nx + 0.5, ny), turbulence, time);
    coord.y += amount * perlin(float2(nx, ny + 0.5), turbulence, time);
    return image.eval(coord);
}