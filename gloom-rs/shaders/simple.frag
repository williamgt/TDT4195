#version 430 core

in layout(location=1)  vec4 vertexColour;
in layout(location=2)  vec3 normals;
 
out vec4 color;

void main()
{
    color = vertexColour;
    color = vec4(normals, 1.0f);
}