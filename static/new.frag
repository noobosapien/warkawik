precision highp float;

uniform float u_time;

void main()
{
    float x = gl_FragCoord.x / 800.0;
    float y = gl_FragCoord.y / 600.0;
    float movement = sin(u_time) * 0.5 + 0.5;
    float r = abs(sin(x * movement + u_time));
    float g = abs(sin(y * movement + u_time + 1.0));
    float b = abs(sin(x * y + u_time + 2.0));
    gl_FragColor = vec4(r, g, b, 1.0);
}
