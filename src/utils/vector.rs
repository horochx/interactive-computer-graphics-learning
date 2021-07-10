pub fn add<const T: usize>(a: &[f32; T], b: &[f32; T]) -> [f32; T] {
    let mut data: [f32; T] = [0.0; T];

    for i in 0..T {
        data[i] = a[i] + b[i];
    }

    data
}

pub fn scale<const T: usize>(p: &[f32; T], s: f32) -> [f32; T] {
    let mut data: [f32; T] = [0.0; T];

    for i in 0..T {
        data[i] = s * p[i];
    }

    data
}

pub fn mix<const T: usize>(a: &[f32; T], b: &[f32; T], r: f32) -> [f32; T] {
    let mut data: [f32; T] = [0.0; T];

    for i in 0..T {
        data[i] = r * a[i] + (1.0 - r) * b[i];
    }

    data
}
