use crate::utils::webgl_utils::{
    auto_create_and_bind_buffer, clear_canvas, enable_vertex_attr, init_gl, init_program,
    to_f32_array,
};
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};
use yew::prelude::*;

pub struct Sierpinski {
    canvas_ref: NodeRef,
    gl: Option<WebGlRenderingContext>,
}

impl Sierpinski {
    fn init_canvas(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas_ref.cast().unwrap();

        let gl = init_gl(&canvas);

        self.gl = Some(gl);
    }

    fn init_shader(&self) {
        let gl = self.gl.as_ref().unwrap();

        let programe = &init_program(
            gl,
            include_str!("./sierpinski.vert"),
            include_str!("./sierpinski.frag"),
        );

        auto_create_and_bind_buffer(gl, WebGlRenderingContext::ARRAY_BUFFER, None);

        enable_vertex_attr(
            gl,
            programe,
            "v_position",
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
    }

    fn draw(&self) {
        let vertices: Vec<f32> = vec![0.0, 0.0, 0.0, 0.5, 0.7, 0.0];

        let gl = self.gl.as_ref().unwrap();
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &to_f32_array(&vertices),
            WebGlRenderingContext::STATIC_DRAW,
        );
        clear_canvas(gl);
        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Component for Sierpinski {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            gl: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas ref=self.canvas_ref.clone() />
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if !_first_render {
            return;
        }

        self.init_canvas();

        self.init_shader();

        self.draw();
    }
}
