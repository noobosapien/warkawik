precision highp float;


uniform float u_time;
uniform vec2 u_resolution;

void main() {
    vec2 position = (gl_FragCoord.xy / u_resolution.xy) * 2.0 - 1.0;
    position.x *= u_resolution.x / u_resolution.y;

    // Water effect modification
    float wave = sin(position.x * 10.0 + u_time) * cos(position.y * 10.0 + u_time);
    float color = 0.5 + 0.5 * wave;
    gl_FragColor = vec4(vec3(color), 1.0);
}
