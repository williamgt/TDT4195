#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec4 color;
in layout(location=2) vec3 normals;

out layout(location=1) vec4 outColor;
out layout(location=2) vec3 outNormals;

uniform layout(location=0) mat4x4 m;
uniform layout(location=1) mat4x4 model;
uniform layout(location=2) mat4x4 viewProjection;

void main()
{
    outColor = color;
    outNormals = normalize(mat3(model) * normals);
    vec4 tempPos = vec4(position, 1.0f);

    gl_Position = viewProjection * model * tempPos;
}