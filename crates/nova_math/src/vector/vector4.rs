use crate::vector::vector3::Vector3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl Vector4<f32> {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn add(&self, v2: &Vector4<f32>) -> Self {
        Self {
            x: self.x + v2.x,
            y: self.y + v2.y,
            z: self.z + v2.z,
            w: self.w + v2.w,
        }
    }

    pub fn subtract(&self, v2: &Vector4<f32>) -> Self {
        Self {
            x: self.x - v2.x,
            y: self.y - v2.y,
            z: self.z - v2.z,
            w: self.w - v2.w,
        }
    }

    pub fn multiply(&self, v2: &Vector4<f32>) -> Self {
        Self {
            x: self.x * v2.x,
            y: self.y * v2.y,
            z: self.z * v2.z,
            w: self.w * v2.w,
        }
    }

    pub fn multiply_by_scalar(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    pub fn divide(&self, v2: &Vector4<f32>) -> Self {
        Self {
            x: self.x / v2.x,
            y: self.y / v2.y,
            z: self.z / v2.z,
            w: self.w / v2.w,
        }
    }

    pub fn get_length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)
    }

    pub fn get_length(&self) -> f32 {
        f32::sqrt(self.get_length_squared())
    }

    pub fn normalize(&self) -> Self {
        let length = self.get_length();

        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: self.w / length,
        }
    }

    pub fn normalize_in_place(&mut self) {
        let normalized = self.normalize();

        self.x = normalized.x;
        self.y = normalized.y;
        self.z = normalized.z;
        self.w = normalized.w;
    }

    pub fn equals(&self, v2: &Vector4<f32>) -> bool {
        (self.x == v2.x) && (self.y == v2.y) && (self.z == v2.z) && (self.w == v2.w)
    }

    pub fn dot_product(&self, v2: &Vector4<f32>) -> f32 {
        let mut dot_product = 0.0;

        dot_product += self.x * v2.x;
        dot_product += self.y * v2.y;
        dot_product += self.z * v2.z;
        dot_product += self.w * v2.w;

        dot_product
    }

    pub fn to_vector3(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

pub fn vector_4_dot_product(
    ax: f32,
    ay: f32,
    az: f32,
    aw: f32,
    bx: f32,
    by: f32,
    bz: f32,
    bw: f32,
) -> f32 {
    let mut dot_product = 0.0;

    dot_product += ax * bx;
    dot_product += ay * by;
    dot_product += az * bz;
    dot_product += aw * bw;

    dot_product
}

#[cfg(test)]
mod vector4_tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4::new(1.0, 2.0, 3.0, 4.0);

        let v3 = v1.add(&v2);

        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 4.0);
        assert_eq!(v3.z, 6.0);
        assert_eq!(v3.w, 8.0);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vector4::new(2.0, 5.0, 1.0, 2.0);
        let v2 = Vector4::new(1.0, 2.0, 4.0, 3.0);

        let v3 = v1.subtract(&v2);

        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, 3.0);
        assert_eq!(v3.z, -3.0);
        assert_eq!(v3.w, -1.0);
    }

    #[test]
    fn test_multiply() {
        let v1 = Vector4::new(9.0, 5.0, -3.0, 2.0);
        let v2 = Vector4::new(3.0, 4.0, 5.0, 3.0);

        let v3 = v1.multiply(&v2);

        assert_eq!(v3.x, 27.0);
        assert_eq!(v3.y, 20.0);
        assert_eq!(v3.z, -15.0);
        assert_eq!(v3.w, 6.0);
    }

    #[test]
    fn test_multiply_by_scalar() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let scalar = 5.0;

        let v2 = v1.multiply_by_scalar(scalar);

        assert_eq!(v2.x, 5.0);
        assert_eq!(v2.y, 10.0);
        assert_eq!(v2.z, 15.0);
        assert_eq!(v2.w, 20.0);
    }

    #[test]
    fn test_divide() {
        let v1 = Vector4::new(9.0, 16.0, -2.0, 5.0);
        let v2 = Vector4::new(3.0, 4.0, -1.0, 1.0);

        let v3 = v1.divide(&v2);

        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 4.0);
        assert_eq!(v3.z, 2.0);
        assert_eq!(v3.w, 5.0);
    }

    #[test]
    pub fn test_get_length_squared() {
        let v1 = Vector4::new(3.0, 4.0, 5.0, 6.0);

        let length_squared = v1.get_length_squared();

        assert_eq!(length_squared, 86.0);
    }

    #[test]
    pub fn test_get_length() {
        let v1 = Vector4::new(3.0, 4.0, 5.0, 6.0);

        let length = v1.get_length();

        assert_eq!(length, f32::sqrt(86.0));
    }

    #[test]
    fn test_normalize() {
        let v1 = Vector4::new(3.0, 4.0, 5.0, 6.0);

        let v1_normalized = v1.normalize();
        let v1_length = v1.get_length();

        assert_eq!(v1_normalized.x, v1.x / v1_length);
        assert_eq!(v1_normalized.y, v1.y / v1_length);
        assert_eq!(v1_normalized.z, v1.z / v1_length);
        assert_eq!(v1_normalized.w, v1.w / v1_length);
    }

    #[test]
    fn test_normalize_in_place() {
        let mut v1 = Vector4::new(3.0, 4.0, 5.0, 6.0);

        let v1_length = v1.get_length();
        v1.normalize_in_place();

        assert_eq!(v1.x, 3.0 / v1_length);
        assert_eq!(v1.y, 4.0 / v1_length);
        assert_eq!(v1.z, 5.0 / v1_length);
        assert_eq!(v1.w, 6.0 / v1_length);
    }

    #[test]
    fn test_equals_true() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(v1.equals(&v2), true)
    }

    #[test]
    fn test_equals_false() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 254.0);
        let v2 = Vector4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(v1.equals(&v2), false)
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 1.0);
        let v2 = Vector4::new(1.0, 2.0, 3.0, 2.0);

        let dot_product = v1.dot_product(&v2);

        assert_eq!(dot_product, 16.0)
    }
}

impl Default for Vector4<f32> {
    fn default() -> Self {
        Self::zero()
    }
}