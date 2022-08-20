pub fn entropy(weights: &Vec<f32>) -> f32 {
    let mut entropy = 0.0;
    for p in weights {
        entropy -= (p * p.log2()).min(0.0)
    }
    entropy
}

pub fn normalize(weights: &mut Vec<f32>) {
    let mut sum = 0.0;
    for weight in weights.iter_mut() {
        // Reset weight to 0 if it is not normal or negative
        if !weight.is_normal() || *weight < 0.0 {
            *weight = 0.0;
        } else {
            sum += *weight;
        }
    }
    for weight in weights.iter_mut() {
        *weight /= sum;
    }
}

pub fn add(a: &mut Vec<f32>, b: &Vec<f32>) {
    a.iter_mut().zip(b).for_each(|(a_i, b_i)| *a_i += *b_i);
}

pub fn hadamard_product(a: &mut Vec<f32>, b: &Vec<f32>) {
    a.iter_mut().zip(b).for_each(|(a_i, b_i)| *a_i *= *b_i);
}

pub fn fermi_dirac(a: f32, u: f32, kt: f32, x: f32) -> f32 {
    a / (f32::exp((x - u) / kt) + 1.0)
}

pub fn gaussian(a: f32, x_0: f32, y_0: f32, s_x: f32, s_y: f32, x: f32, y: f32) -> f32 {
    a * f32::exp(
        -(((x - x_0).powi(2) / (2.0 * s_x.powi(2))) + ((y - y_0).powi(2) / (2.0 * s_y.powi(2)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_entropy() {
        let zeroes = vec![0.0; 3];
        assert_eq!(entropy(&zeroes), 0.0);

        let ones = vec![1.0; 3];
        assert_eq!(entropy(&ones), 0.0);

        let p = vec![0.5, 0.25, 0.25];
        assert_eq!(entropy(&p), 1.5);

        let p1 = vec![0.5, 0.25, 0.0];
        assert_eq!(entropy(&p1), 1.0);

        let p1 = vec![0.5, 0.25, 0.0, 0.0];
        assert_eq!(entropy(&p1), 1.0);

        let p2 = vec![1.0, 0.0, 0.0];
        assert_eq!(entropy(&p2), 0.0);
    }

    #[test]
    fn test_math_normalize() {
        let mut zeroes = vec![0.0f32; 3];
        normalize(&mut zeroes);
        assert!(zeroes.iter().all(|x| x.is_nan()));

        let mut negative = vec![1.0, 1.0, -1.0];
        normalize(&mut negative);
        assert_eq!(negative, vec![0.5, 0.5, 0.0]);

        let mut nan = vec![1.0, 1.0, f32::NAN];
        normalize(&mut nan);
        assert_eq!(nan, vec![0.5, 0.5, 0.0]);

        let mut p = vec![2.0, 1.0, 1.0];
        normalize(&mut p);
        assert_eq!(p, vec![0.5, 0.25, 0.25]);
    }

    #[test]
    fn test_math_hadamard_product() {
        let mut ones = vec![1.0; 3];
        let mut zeroes = vec![0.0f32; 3];
        let mut a = vec![2.0, 1.0, 1.0];
        let b = vec![2.0, 1.0, 2.0];

        hadamard_product(&mut zeroes, &a);
        assert_eq!(zeroes, vec![0.0f32; 3]);

        hadamard_product(&mut ones, &a);
        assert_eq!(ones, a);

        hadamard_product(&mut a, &b);
        assert_eq!(a, vec![4.0, 1.0, 2.0]);
    }

    #[test]
    fn test_math_add() {
        let mut ones = vec![1.0; 3];
        let mut zeroes = vec![0.0f32; 3];
        let mut a = vec![2.0, 1.0, 1.0];
        let b = vec![2.0, 1.0, 2.0];

        add(&mut zeroes, &a);
        assert_eq!(zeroes, a);

        add(&mut ones, &a);
        assert_eq!(ones, vec![3.0, 2.0, 2.0]);

        add(&mut a, &b);
        assert_eq!(a, vec![4.0, 2.0, 3.0]);
    }
}
