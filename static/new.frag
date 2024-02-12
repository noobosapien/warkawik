precision highp float;

varying vec2 v_fragTexCoord;

uniform float u_time;
uniform vec2 u_resolution;

void main()
{
    float ballRadius = 0.1;
    float ballPositionY = abs(sin(u_time)) * 0.5 + 0.25;
    vec2 ballPosition = vec2(0.5, ballPositionY); // Center X and animated Y
    vec3 ballColor = vec3(243.0/255.0, 211.0/255.0, 43.0/255.0);

    vec2 fragCoordNorm = gl_FragCoord.xy / u_resolution.xy;
    float distanceFromBall = distance(fragCoordNorm, ballPosition);

    if(distanceFromBall < ballRadius)
    {
        gl_FragColor = vec4(ballColor, 1.0);
    }
    else
    {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0); // Background color
    }
}
