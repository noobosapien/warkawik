

precision mediump float;

uniform float u_time;
uniform vec2 u_resolution;


float noise(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

float fbm(vec2 st) {
    float value = 0.0;
    float amplitude = 0.5;
    for (int i = 0; i < 10; i++) {
        value += amplitude * noise(st);
        st *= 2.;
        amplitude *= 0.5;
    }
    return value;
}

void main() {
    vec2 st = gl_FragCoord.xy/u_resolution.xy;
    vec3 color = vec3(0.0);
    vec2 q = vec2(0.);
    q.x = fbm(st + 0.00*u_time);
    q.y = fbm(st + vec2(1.0));

    vec2 r = vec2(0.);
    r.x = fbm(st + 1.0*q + vec2(1.7,9.2)+ 0.15*u_time );
    r.y = fbm(st + 1.0*q + vec2(8.3,2.8)+ 0.126*u_time);

    float f = fbm(st+r);

    color = mix(vec3(1.0, 0.5, 0.0), // Lava color start
                vec3(0.5, 0.0, 0.0), // Lava color end
                clamp((f*f)*4.0,0.0,1.0)); // Use f*f to control the mix

    gl_FragColor = vec4(color,1.);
}