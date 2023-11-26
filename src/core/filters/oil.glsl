const int MAX_LEVELS = 8;
const float MAX_RANGE = 16.0;
const float MAX_SKIP = 4.0;

uniform float skip;
uniform float range;
uniform float levels;

half4 color0 = half4(0.0);
half4 color1 = half4(0.0);
half4 color2 = half4(0.0);
half4 color3 = half4(0.0);
half4 color4 = half4(0.0);
half4 color5 = half4(0.0);
half4 color6 = half4(0.0);
half4 color7 = half4(0.0);

float count0 = 0.0;
float count1 = 0.0;
float count2 = 0.0;
float count3 = 0.0;
float count4 = 0.0;
float count5 = 0.0;
float count6 = 0.0;
float count7 = 0.0;

uniform shader image;

half4 tx(float2 coord, float2 l){
    return image.eval(coord + l);
}

half4 main(float2 coord) {
    for(float i = -MAX_RANGE; i < MAX_RANGE; i += MAX_SKIP){
        // (i * skip < -range || i * skip > range) Err: program too large
        if (i < -range || i > range) { continue; }
        for(float j = -MAX_RANGE; j < MAX_RANGE; j += MAX_SKIP){
            if (j < -range || j > range) { continue; }
            half4 t = image.eval(coord + float2(i, j));
            int lv = int(dot(half4(1.0), t) / 4.0 * levels);

            switch(lv) {
                case 0:
                count0 += 1.0;
                color0 += t;
                break;
                case 1:
                count1 += 1.0;
                color1 += t;
                break;
                case 2:
                count2 += 1.0;
                color2 += t;
                break;
                case 3:
                count3 += 1.0;
                color3 += t;
                break;
                case 4:
                count4 += 1.0;
                color4 += t;
                break;
                case 5:
                count5 += 1.0;
                color5 += t;
                break;
                case 6:
                count6 += 1.0;
                color6 += t;
                break;
                case 7:
                count7 += 1.0;
                color7 += t;
                break;
            }
        }
    }

    int mx_index = 0;
    float mx_val = count0;

    if (count1 > mx_val) { mx_index = 1; mx_val = count1; }
    if (count2 > mx_val) { mx_index = 2; mx_val = count2; }
    if (count3 > mx_val) { mx_index = 3; mx_val = count3; }
    if (count4 > mx_val) { mx_index = 4; mx_val = count4; }
    if (count5 > mx_val) { mx_index = 5; mx_val = count5; }
    if (count6 > mx_val) { mx_index = 6; mx_val = count6; }
    if (count7 > mx_val) { mx_index = 7; mx_val = count7; }

    switch (mx_index) {
        case 0: return color0 / mx_val;
        case 1: return color1 / mx_val;
        case 2: return color2 / mx_val;
        case 3: return color3 / mx_val;
        case 4: return color4 / mx_val;
        case 5: return color5 / mx_val;
        case 6: return color6 / mx_val;
        case 7: return color7 / mx_val;
        default: return half4(0.0);
    }
}