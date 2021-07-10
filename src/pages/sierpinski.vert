attribute vec4 v_position;
varying vec4 f_color;

void main() {
    f_color = vec4((1.0 + v_position.xyz) / 2.0, 1.0);

    gl_Position = v_position;
    gl_PointSize = 1.0;
}