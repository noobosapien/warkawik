precision highp float;

uniform float u_time;

void main()
{
    float x = gl_FragCoord.x / 100.0;
    float y = gl_FragCoord.y / 100.0;
    float movement = sin(x * y * 10.0 + u_time * 2.0) * cos(x * y * 10.0 + u_time * 2.0);
    gl_FragColor = vec4(movement, 0.5 * movement, 0.2, 1.0);
}
