pub fn entropy(weights: &Vec<f32>) -> f32 {
    return weights.iter().fold(0.0, |acc, p| acc + (-p) * p.log2());
}

pub fn normalize(weights: &mut Vec<f32>) {
    let sum: f32 = weights.iter().sum();
    weights.iter_mut().for_each(|w| *w /= sum);
}

pub fn hadamard_product(a: &mut Vec<f32>, b: &Vec<f32>) {
    a.iter_mut().zip(b).for_each(|(a_i, b_i)| *a_i *= *b_i);
}

pub fn fermi_dirac(a: f32, u: f32, kt: f32, x: f32) -> f32 {
    return a / (f32::exp((x - u) / kt) + 1.0);
}

pub fn gaussian(a: f32, x_0: f32, y_0: f32, s_x: f32, s_y: f32, x: f32, y: f32) -> f32 {
    return a * f32::exp(
        -(((x - x_0).powi(2) / (2.0 * s_x.powi(2))) + ((y - y_0).powi(2) / (2.0 * s_y.powi(2)))),
    );
}
