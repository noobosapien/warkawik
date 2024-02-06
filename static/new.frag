
precision mediump float;

uniform float u_time;
uniform vec2 u_resolution;

float tree(vec2 position, float size) {
    float tree = smoothstep(0.0, size * 0.5, abs(position.x) - size * 0.25);
    tree *= smoothstep(size, 0.0, abs(position.y));
    return tree;
}

float leaf(vec2 position, float size, float offset) {
    float leaf = max(0.0, 1.0 - length(position - vec2(offset, size * 0.5)) / size);
    leaf *= smoothstep(size * 0.5, 0.0, abs(position.x - offset));
    return leaf;
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;
    uv.y = 1.0 - uv.y;
    uv = uv * 2.0 - 1.0;
    uv.x *= u_resolution.x / u_resolution.y;

    vec3 color = mix(vec3(0.8, 0.4, 0.1), vec3(0.1, 0.2, 0.5), uv.y * 0.5 + 0.5);

    float t = tree(uv, 0.2);
    color = mix(color, vec3(0.36, 0.2, 0.1), t);

    float l1 = leaf(uv, 0.1, 0.0);
    float l2 = leaf(uv, 0.1, 0.05);
    float l3 = leaf(uv, 0.1, -0.05);
    color = mix(color, vec3(0.0, 0.5, 0.0), max(max(l1, l2), l3));

    gl_FragColor = vec4(color, 1.0);
}
