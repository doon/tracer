use std::ops;

const EPSILON: f64 = 1.0e-6;

fn equals(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    fn color(red: f64, green: f64, blue: f64) -> Tuple {
        Tuple {
            x: red,
            y: green,
            z: blue,
            w: 0.0,
        }
    }

    fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        Tuple::new(
            self.x / magnitude,
            self.y / magnitude,
            self.z / magnitude,
            self.w / magnitude,
        )
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(&self, other: &Self) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn red(&self) -> f64 {
        self.x
    }

    fn green(&self) -> f64 {
        self.y
    }

    fn blue(&self) -> f64 {
        self.z
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equals(self.x, other.x)
            && equals(self.y, other.y)
            && equals(self.z, other.z)
            && equals(self.w, other.w)
    }
}

impl ops::Add for Tuple {
    type Output = Self;
    fn add(self, other: Tuple) -> Self {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Tuple) -> Self {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl ops::Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        )
    }
}

impl ops::Mul for Tuple {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Tuple::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Tuple::new(
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point() {
        let p = Tuple::point(4.0, -3.0, 4.0);
        assert_eq!(
            p,
            Tuple {
                x: 4.0,
                y: -3.0,
                z: 4.0,
                w: 1.0
            }
        );
    }

    #[test]
    fn create_vector() {
        let v = Tuple::vector(4.0, -3.0, 4.0);
        assert_eq!(
            v,
            Tuple {
                x: 4.0,
                y: -3.0,
                z: 4.0,
                w: 0.0
            }
        );
    }

    #[test]
    fn add_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtract_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtract_vector_from_point() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let v1 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(p1 - v1, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negation_operator() {
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(-v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiple_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn magnitude_of_vector_neg_1_2_3() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalize_vector_4_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalize_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), Tuple::vector(0.267261, 0.534522, 0.801783));
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize().magnitude(), 1 as f64)
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(&v2), 20 as f64);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(v1.cross(&v2), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Tuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Tuple::color(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4 as f64);
        assert_eq!(c.blue(), 1.7 as f64);
    }

    #[test]
    fn adding_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Tuple::color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Tuple::color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiple_color_by_scalar() {
        let c = Tuple::color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Tuple::color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_colors() {
        let c1 = Tuple::color(1.0, 0.2, 0.4);
        let c2 = Tuple::color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Tuple::color(0.9, 0.2, 0.04));
    }
}
