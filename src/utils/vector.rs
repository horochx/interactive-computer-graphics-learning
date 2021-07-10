#[derive(Clone, Copy)]
pub struct Vertice(pub f32, pub f32, pub f32);

impl Vertice {
    pub fn add(a: &Vertice, b: Vertice) -> Self {
        let _add = |a: f32, b: f32| a + b;

        Self(_add(a.0, b.0), _add(a.1, b.1), _add(a.2, b.2))
    }

    pub fn scale(a: &Vertice, s: f32) -> Self {
        let _scale = |num: f32| num * s;

        Self(_scale(a.0), _scale(a.1), _scale(a.2))
    }

    pub fn mix(a: &Vertice, b: &Vertice, r: f32) -> Self {
        let _mix = |a: f32, b: f32| a * r + (1.0 - r) * b;

        Self(_mix(a.0, b.0), _mix(a.1, b.1), _mix(a.2, b.2))
    }
}

pub struct Triangle(pub Vertice, pub Vertice, pub Vertice);

pub struct Vertices(pub Vec<f32>);

impl Vertices {
    pub fn add_vertice(self: &mut Self, vertice: Vertice) {
        self.0.push(vertice.0);
        self.0.push(vertice.1);
        self.0.push(vertice.2);
    }

    pub fn add_triangle(self: &mut Self, triangle: Triangle) {
        self.add_vertice(triangle.0);
        self.add_vertice(triangle.1);
        self.add_vertice(triangle.2);
    }
}
