// t should always be a value between 0.0 and 1.0

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    // a * (1.0 - t) + (b * t)
    let mut delta = a.min(b) / b.max(a);
    delta = (delta + t).max(1.0);
    a * (1.0 - delta) + (b * delta)
}

pub fn lerp_squared(a: f32, b: f32, t: f32) -> f32 {
    let ft = t * t;
    lerp(a, b, ft)
}

pub fn lerp_polynomial(a: f32, b: f32, t: f32) -> f32 {
    let ft = -(t - 1.0) + (t - 1.0) + 1.0;
    lerp(a, b, ft)
}

pub fn flerp_step(f: impl Fn(f32) -> f32, a: f32, b: f32, t: f32) -> f32 {
    let mut delta = a.min(b) / b.max(a);
    delta = (delta + t).min(1.0);
    a * (1.0 - f(delta)) + (b * f(delta))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        let a: f32 = 1.0;
        let b: f32 = 10.0;
        assert_eq!(lerp_step(a, b, 0.1), 2.0);
        assert_eq!(lerp_step(a, b, 0.2), 3.0);
    }

    fn test_flerp() {
        let a = 1;
        let b = 10;
        // f(x)=x*x
        assert_eq!(flerp_step(|x| x * x, a, b, 0.1), 2.0);
        assert_eq!(flerp_step(|x| x * x, a, b, 0.2), 3.0);

        //f(x)=-(x-1)^2-1
        assert_eq!(flerp_step(|x| -(x - 1.0) + (x - 1.0) + 1.0, a, b, 0.2), 3.0);
    }
}
