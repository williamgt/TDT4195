#version 430 core

in layout(location=1)  vec3 vertexColour;
 
out vec4 color;

void main()
{
    if(vertexColour.r == 1.0f) {
      color = vec4(1.0f, 0.0f, 0.0f, 1.0f);  
    } 
    else if (vertexColour.g == 1.0f) {
        color = vec4(0.0f, 1.0f, 0.0f, 1.0f);  
    }
    else if (vertexColour.b == 1.0f) {
        color = vec4(0.0f, 0.0f, 1.0f, 1.0f);  
    }
    else {
        color = vec4(vertexColour, 1.0f);
    }
    //color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}