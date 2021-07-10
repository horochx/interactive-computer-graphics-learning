use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

pub fn init_gl(canvas: &HtmlCanvasElement) -> WebGlRenderingContext {
    let (width, height) = resize_canvas_to_display_size(canvas, 2);

    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    gl.viewport(0, 0, width, height);

    gl.clear_color(0.0, 0.0, 0.0, 0.0);

    gl
}

pub fn clear_canvas(gl: &WebGlRenderingContext) {
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
}

fn resize_canvas_to_display_size(canvas: &HtmlCanvasElement, multiplier: i32) -> (i32, i32) {
    let width = canvas.client_width() * multiplier;
    let height = canvas.client_height() * multiplier;

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    (width, height)
}

pub fn init_program(
    gl: &WebGlRenderingContext,
    vert_shader_str: &str,
    frag_shader_str: &str,
) -> WebGlProgram {
    let program: WebGlProgram = gl.create_program().unwrap();

    let vert_shader = create_shader(gl, WebGlRenderingContext::VERTEX_SHADER, vert_shader_str);
    let frag_shader = create_shader(gl, WebGlRenderingContext::FRAGMENT_SHADER, frag_shader_str);

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        gl.use_program(Some(&program));

        program
    } else {
        panic!("{}", gl.get_program_info_log(&program).unwrap());
    }
}

fn create_shader(gl: &WebGlRenderingContext, shader_type: u32, source: &str) -> WebGlShader {
    let shader = gl.create_shader(shader_type).unwrap();

    gl.shader_source(&shader, source);

    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        shader
    } else {
        panic!("{}", gl.get_shader_info_log(&shader).unwrap());
    }
}

pub fn auto_create_and_bind_buffer(
    gl: &WebGlRenderingContext,
    buffer_type: u32,
    buffer: Option<WebGlBuffer>,
) -> WebGlBuffer {
    let buffer = if let Some(value) = buffer {
        value
    } else {
        gl.create_buffer().unwrap()
    };

    gl.bind_buffer(buffer_type, Some(&buffer));

    buffer
}

pub fn enable_vertex_attr(
    gl: &WebGlRenderingContext,
    programe: &WebGlProgram,
    attr: &str,
    size: i32,
    type_: u32,
    normalized: bool,
    stride: i32,
    offset: i32,
) {
    let attr_index = gl.get_attrib_location(&programe, attr) as u32;

    gl.vertex_attrib_pointer_with_i32(attr_index, size, type_, normalized, stride, offset);

    gl.enable_vertex_attrib_array(attr_index);
}

pub fn to_f32_array(vertices: &Vec<f32>) -> js_sys::Float32Array {
    js_sys::Float32Array::from(vertices.as_slice())
}
