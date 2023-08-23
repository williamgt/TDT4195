#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec3 color;

out layout(location=1) vec3 outColor;
out vec4 gl_Position;

void main()
{
    outColor = color;
    outColor = vec3(1.0f, 1.0f, 1.0f);
    gl_Position = vec4(position, 1.0f);
}