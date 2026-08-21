export const vertex=`#version 300 es
precision highp float;layout(location=0)in vec3 position;layout(location=1)in vec3 normal;layout(location=2)in vec2 uv;uniform mat4 projection;uniform mat4 view;out vec3 n;out vec2 tex;out float depth;void main(){vec4 p=view*vec4(position,1.);gl_Position=projection*p;n=mat3(view)*normal;tex=uv;depth=-p.z;}`;
export const fragment=`#version 300 es
precision highp float;in vec3 n;in vec2 tex;uniform sampler2D flag;out vec4 color;void main(){vec3 nn=normalize(gl_FrontFacing?n:-n);vec3 light=normalize(vec3(-.3,.65,.8));float diffuse=.32+.68*max(dot(nn,light),0.);vec3 c=texture(flag,tex).rgb;color=vec4(c*diffuse+pow(max(dot(reflect(-light,nn),vec3(0,0,1)),0.),24.)*.08,1.);}`;
