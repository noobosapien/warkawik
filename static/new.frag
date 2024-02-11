```glsl
precision mediump float;

uniform float time;
uniform vec2 resolution;

void main() {
    vec2 position = (gl_FragCoord.xy / resolution.xy) * 2.0 - 1.0;
    position.x *= resolution.x / resolution.y;

    float color = 0.0;
    vec2 center = vec2(sin(time * 0.1) * 0.5, cos(time * 0.15) * 0.25);
    color += 0.5 + 0.5 * sin(distance(position, center) * 10.0 - time * 2.0);
    color += 0.5 + 0.5 * cos(length(position - center) * 10.0 - time * 3.0);
    color *= 1.0 - length(position - center) * 0.3;

    gl_FragColor = vec4(color, color * 0.5, 0.0, 1.0);
}
```