attribute vec4 v_position;

void main() {
    gl_Position = v_position;
    gl_PointSize = 1.0;
}