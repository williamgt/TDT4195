#version 430 core

in layout(location=1)  vec3 vertexColour;
 
out vec4 color;

void main()
{
    vec3 newColor = vec3(0.0f, vertexColour.y, vertexColour.z);
    if(newColor.y <= 0 && newColor.z <= 0) {
        newColor.x = 1.0f;
    } 
    color = vec4(newColor, 1.0f);
}