
precision highp float;

varying vec2 v_fragTexCoord;

uniform float u_time;
uniform vec2 u_resolution;

void main()
{
    float x = gl_FragCoord.x + u_time * 0.05; // Move to the left over time
    float y = gl_FragCoord.y;
    // Simple wave pattern for water effect
    float wave = sin(x * 10.0 + u_time) * 0.5 + 0.5;
    // Mix gray color with wave pattern
    vec3 color = mix(vec3(0.5, 0.5, 0.5), vec3(0.75, 0.75, 0.75), wave);
    gl_FragColor = vec4(color, 1.0);
}
