use crate::utils::{
    vector::{Triangle, Vertice, Vertices},
    webgl_utils::{buffer_data_to_attr, clear_canvas, init_gl, init_program},
};
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext};
use yew::prelude::*;

pub struct Sierpinski {
    canvas_ref: NodeRef,
    gl: Option<WebGlRenderingContext>,
    programe: Option<WebGlProgram>,
    draw_count: i32,
}

impl Sierpinski {
    fn init_canvas(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas_ref.cast().unwrap();

        let gl = init_gl(&canvas);

        self.gl = Some(gl);
    }

    fn init_shader(&mut self) {
        let gl = self.gl.as_ref().unwrap();

        let programe = init_program(
            gl,
            include_str!("./sierpinski.vert"),
            include_str!("./sierpinski.frag"),
        );

        self.programe = Some(programe);
    }

    #[allow(unused_unsafe)]
    fn update_buffer_data(&mut self) {
        let mut vertices: Vertices = Vertices(vec![]);

        let init_verts = [
            Vertice(-0.5, -0.5, -0.5),
            Vertice(0.5, -0.5, -0.5),
            Vertice(0.0, 0.5, 0.0),
            Vertice(0.0, -0.5, 0.5),
        ];

        //  3D points
        // let mut p = Vertice(0.0, 0.0, 0.0);
        // vertices.add_vertice(p);

        // for _ in 1..5000 {
        //     let i = unsafe { (js_sys::Math::random() * 4.0).floor() as usize };

        //     p = Vertice::mix(&p, &init_verts[i], 0.5);

        //     vertices.add_vertice(p);
        // }

        fn divider(
            a: Vertice,
            b: Vertice,
            c: Vertice,
            d: Vertice,
            vertices: &mut Vertices,
            count: u32,
        ) {
            if count == 0 {
                vertices.add_triangle(Triangle(a, b, c));
                vertices.add_triangle(Triangle(a, b, d));
                vertices.add_triangle(Triangle(a, c, d));
                vertices.add_triangle(Triangle(b, c, d));
            } else {
                let count = count - 1;

                let ab = Vertice::mix(&a, &b, 0.5);
                let ac = Vertice::mix(&a, &c, 0.5);
                let ad = Vertice::mix(&a, &d, 0.5);
                let bc = Vertice::mix(&b, &c, 0.5);
                let bd = Vertice::mix(&b, &d, 0.5);
                let cd = Vertice::mix(&c, &d, 0.5);

                divider(a, ab, ac, ad, vertices, count);
                divider(b, ab, bc, bd, vertices, count);
                divider(c, ac, bc, cd, vertices, count);
                divider(d, ad, bd, cd, vertices, count);
            }
        }

        divider(
            init_verts[0],
            init_verts[1],
            init_verts[2],
            init_verts[3],
            &mut vertices,
            5,
        );

        let gl = self.gl.as_ref().unwrap();
        let programe = self.programe.as_ref().unwrap();

        buffer_data_to_attr(
            gl,
            programe,
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices.0,
            WebGlRenderingContext::STATIC_DRAW,
            "v_position",
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.draw_count = vertices.0.len() as i32;
    }

    fn draw(&self) {
        let gl = self.gl.as_ref().unwrap();

        clear_canvas(gl);
        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, self.draw_count);
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
            programe: None,
            draw_count: 0,
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

        self.update_buffer_data();

        self.draw();
    }
}
