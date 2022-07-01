pub fn fermi_dirac(a: f32, u: f32, kt: f32, x: f32) -> f32 {
    return a / (f32::exp((x - u) / kt) + 1.0);
}

pub fn gaussian(a: f32, x_0: f32, y_0: f32, s_x: f32, s_y: f32, x: f32, y: f32) -> f32 {
    return a * f32::exp(
        -(((x - x_0).powi(2) / (2.0 * s_x.powi(2))) + ((y - y_0).powi(2) / (2.0 * s_y.powi(2)))),
    );
}