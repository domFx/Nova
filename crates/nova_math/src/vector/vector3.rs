use crate::vector::vector4::Vector4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Vector3<f32> {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn add(&self, v2: &Vector3<f32>) -> Self {
        Self {
            x: self.x + v2.x,
            y: self.y + v2.y,
            z: self.z + v2.z,
        }
    }

    pub fn subtract(&self, v2: &Vector3<f32>) -> Self {
        Self {
            x: self.x - v2.x,
            y: self.y - v2.y,
            z: self.z - v2.z,
        }
    }

    pub fn multiply(&self, v2: &Vector3<f32>) -> Self {
        Self {
            x: self.x * v2.x,
            y: self.y * v2.y,
            z: self.z * v2.z,
        }
    }

    pub fn multiply_by_scalar(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn divide(&self, v2: &Vector3<f32>) -> Self {
        Self {
            x: self.x / v2.x,
            y: self.y / v2.y,
            z: self.z / v2.z,
        }
    }

    pub fn get_length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
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
        }
    }

    pub fn normalize_in_place(&mut self) {
        let normalized = self.normalize();

        self.x = normalized.x;
        self.y = normalized.y;
        self.z = normalized.z;
    }

    pub fn equals(&self, v2: &Vector3<f32>) -> bool {
        (self.x == v2.x) && (self.y == v2.y) && (self.z == v2.z)
    }

    pub fn dot_product(&self, v2: &Vector3<f32>) -> f32 {
        let mut dot_product = 0.0;

        dot_product += self.x * v2.x;
        dot_product += self.y * v2.y;
        dot_product += self.z * v2.z;

        dot_product
    }

    pub fn cross_product(&self, v2: &Vector3<f32>) -> Self {
        Self {
            x: (self.y * v2.z) - (self.z * v2.y),
            y: (self.z * v2.x) - (self.x * v2.z),
            z: (self.x * v2.y) - (self.y * v2.x),
        }
    }

    pub fn get_distance(&self, v2: &Vector3<f32>) -> f32 {
        let difference = self.subtract(v2);

        difference.get_length()
    }

    pub fn to_vector4(&self) -> Vector4<f32> {
        Vector4::new(self.x, self.y, self.z, 0.0)
    }
}

#[cfg(test)]
mod vector3_tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        let v3 = v1.add(&v2);

        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 4.0);
        assert_eq!(v3.z, 6.0);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vector3::new(2.0, 5.0, 1.0);
        let v2 = Vector3::new(1.0, 2.0, 4.0);

        let v3 = v1.subtract(&v2);

        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, 3.0);
        assert_eq!(v3.z, -3.0);
    }

    #[test]
    fn test_multiply() {
        let v1 = Vector3::new(9.0, 5.0, -3.0);
        let v2 = Vector3::new(3.0, 4.0, 5.0);

        let v3 = v1.multiply(&v2);

        assert_eq!(v3.x, 27.0);
        assert_eq!(v3.y, 20.0);
        assert_eq!(v3.z, -15.0);
    }

    #[test]
    fn test_multiply_by_scalar() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let scalar = 5.0;

        let v2 = v1.multiply_by_scalar(scalar);

        assert_eq!(v2.x, v1.x * scalar);
        assert_eq!(v2.y, v1.y * scalar);
        assert_eq!(v2.z, v1.z * scalar);
    }

    #[test]
    fn test_divide() {
        let v1 = Vector3::new(9.0, 16.0, -2.0);
        let v2 = Vector3::new(3.0, 4.0, -1.0);

        let v3 = v1.divide(&v2);

        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 4.0);
        assert_eq!(v3.z, 2.0);
    }

    #[test]
    pub fn test_get_length_squared() {
        let v1 = Vector3::new(3.0, 4.0, 5.0);

        let length_squared = v1.get_length_squared();

        assert_eq!(length_squared, 50.0);
    }

    #[test]
    pub fn test_get_length() {
        let v1 = Vector3::new(3.0, 4.0, 5.0);

        let length = v1.get_length();

        assert_eq!(length, f32::sqrt(50.0));
    }

    #[test]
    fn test_normalize() {
        let v1 = Vector3::new(3.0, 4.0, 5.0);

        let v1_normalized = v1.normalize();
        let v1_length = v1.get_length();

        assert_eq!(v1_normalized.x, v1.x / v1_length);
        assert_eq!(v1_normalized.y, v1.y / v1_length);
        assert_eq!(v1_normalized.z, v1.z / v1_length);
    }

    #[test]
    fn test_normalize_in_place() {
        let mut v1 = Vector3::new(3.0, 4.0, 5.0);

        let v1_length = v1.get_length();
        v1.normalize_in_place();

        assert_eq!(v1.x, 3.0 / v1_length);
        assert_eq!(v1.y, 4.0 / v1_length);
        assert_eq!(v1.z, 5.0 / v1_length);
    }

    #[test]
    fn test_equals_true() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(v1.equals(&v2), true)
    }

    #[test]
    fn test_equals_false() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(5.0, 2.0, 3.0);

        assert_eq!(v1.equals(&v2), false)
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        let dot_product = v1.dot_product(&v2);

        assert_eq!(dot_product, 14.0)
    }

    #[test]
    fn test_cross_product_1() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);

        let cross_product = v1.cross_product(&v2);

        assert_eq!(cross_product.x, 0.0);
        assert_eq!(cross_product.y, 0.0);
        assert_eq!(cross_product.z, 0.0);
    }

    #[test]
    fn test_cross_product_2() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        let cross_product = v1.cross_product(&v2);

        assert_eq!(cross_product.x, -3.0);
        assert_eq!(cross_product.y, 6.0);
        assert_eq!(cross_product.z, -3.0);
    }

    #[test]
    fn test_get_distance() {
        let v1 = Vector3::new(1.0, 0.0, 5.0);
        let v2 = Vector3::new(0.0, 2.0, 4.0);

        let distance = v1.get_distance(&v2);

        assert_eq!(distance, f32::sqrt(6.0));
    }
}
