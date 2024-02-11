precision highp float;

varying vec2 v_fragTexCoord;

uniform float u_time;
uniform vec2 u_resolution;

void main()
{
    float rocketPosition = mod(u_time, 2.0) - 1.0; // Rocket moves up every 2 seconds
    vec2 rocketShape = vec2(0.1, 0.3); // Width and height of the rocket
    vec2 position = (gl_FragCoord.xy / u_resolution.xy) * 2.0 - 1.0; // Convert pixel position to -1 to 1 range
    position.y -= rocketPosition; // Move the rocket up

    // Check if we're inside the rocket shape
    if (abs(position.x) < rocketShape.x && abs(position.y) < rocketShape.y) {
        gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0); // Red color
    } else {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0); // Black background
    }
}
