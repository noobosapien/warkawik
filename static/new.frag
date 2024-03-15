
precision mediump float;


void main() {
    // Simple water effect fragment shader
    vec2 uv = gl_FragCoord.xy / vec2(800.0, 600.0); // Assuming a 800x600 screen size
    float time = mod(gl_FragCoord.y + 0.1 * sin(gl_FragCoord.x * 0.02 + 1.0), 20.0);
    vec3 waterColor = vec3(0.0, 0.4 + 0.1 * sin(uv.x * 10.0 + time), 0.7 + 0.1 * cos(uv.y * 10.0 + time));
    gl_FragColor = vec4(waterColor, 1.0);
}