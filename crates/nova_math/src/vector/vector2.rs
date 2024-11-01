use crate::vector::vector3::Vector3;
use std::ops::{Add, Div, Mul, Sub};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Default
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + PartialEq,
{
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Vector2<T> {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }

    pub fn add(&self, v2: &Vector2<T>) -> Self {
        Self {
            x: self.x + v2.x,
            y: self.y + v2.y,
        }
    }

    pub fn subtract(&self, v2: &Vector2<T>) -> Self {
        Self {
            x: self.x - v2.x,
            y: self.y - v2.y,
        }
    }

    pub fn multiply(&self, v2: &Vector2<T>) -> Self {
        Self {
            x: self.x * v2.x,
            y: self.y * v2.y,
        }
    }

    pub fn multiply_by_scalar(&self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn divide(&self, v2: &Vector2<T>) -> Self {
        Self {
            x: self.x / v2.x,
            y: self.y / v2.y,
        }
    }

    pub fn equals(&self, v2: &Vector2<T>) -> bool {
        self.x == v2.x && self.y == v2.y
    }
}

impl Vector2<f32> {
    pub fn normalize_in_place(&mut self) {
        let normalized = self.normalize();

        self.x = normalized.x;
        self.y = normalized.y;
    }

    pub fn get_length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn get_length(&self) -> f32 {
        f32::sqrt(self.get_length_squared())
    }

    pub fn normalize(&self) -> Vector2<f32> {
        let vector_length: f32 = self.get_length();

        Self {
            x: self.x / vector_length,
            y: self.y / vector_length,
        }
    }

    pub fn get_distance(&self, v2: &Vector2<f32>) -> f32 {
        let difference = self.subtract(v2);

        difference.get_length()
    }

    pub fn to_vector3(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, 0.0)
    }
}

#[cfg(test)]
mod vector2_tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(1.0, 2.0);

        let v3 = v1.add(&v2);

        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 4.0);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vector2::new(2.0, 5.0);
        let v2 = Vector2::new(1.0, 2.0);

        let v3 = v1.subtract(&v2);

        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, 3.0);
    }

    #[test]
    fn test_multiply() {
        let v1 = Vector2::new(9.0, 5.0);
        let v2 = Vector2::new(3.0, 4.0);

        let v3 = v1.multiply(&v2);

        assert_eq!(v3.x, 27.0);
        assert_eq!(v3.y, 20.0);
    }

    #[test]
    fn test_multiply_by_scalar() {
        let v1 = Vector2::new(9.0, 5.0);

        let scalar = 5.0;
        let v2 = v1.multiply_by_scalar(scalar);

        assert_eq!(v2.x, v1.x * 5.0);
        assert_eq!(v2.y, v1.y * 5.0);
    }

    #[test]
    fn test_divide() {
        let v1 = Vector2::new(9.0, 16.0);
        let v2 = Vector2::new(3.0, 4.0);

        let v3 = v1.divide(&v2);

        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 4.0);
    }

    #[test]
    fn test_get_length_squared() {
        let v1 = Vector2::new(2.0, 3.0);

        let len_squared = v1.get_length_squared();

        assert_eq!(len_squared, 13.0);
    }

    #[test]
    fn test_get_length() {
        let v1 = Vector2::new(3.0, 4.0);

        let len_squared = v1.get_length();

        assert_eq!(len_squared, 5.0);
    }

    #[test]
    fn test_normalize() {
        let v1 = Vector2::new(3.0, 4.0);

        let v1_normalized = v1.normalize();

        assert_eq!(v1_normalized.x, 3.0 / 5.0);
        assert_eq!(v1_normalized.y, 4.0 / 5.0);
    }

    #[test]
    fn test_normalize_in_place() {
        let mut v1 = Vector2::new(3.0, 4.0);

        v1.normalize_in_place();

        assert_eq!(v1.x, 3.0 / 5.0);
        assert_eq!(v1.y, 4.0 / 5.0);
    }

    #[test]
    fn test_equals_true() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(1.0, 2.0);

        assert_eq!(v1.equals(&v2), true)
    }

    #[test]
    fn test_equals_false() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(5.0, 2.0);

        assert_eq!(v1.equals(&v2), false)
    }

    #[test]
    fn test_get_distance() {
        let v1 = Vector2::new(6.0, 10.0);
        let v2 = Vector2::new(3.0, 6.0);

        let distance = v1.get_distance(&v2);

        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_to_vector3() {
        let v1 = Vector2::new(1.0, 2.0);

        let v2 = v1.to_vector3();

        assert_eq!(v2.x, v1.x);
        assert_eq!(v2.y, v1.y);
        assert_eq!(v2.z, 0.0);
    }

    #[test]
    fn test_zero() {
        let v1: Vector2<u32> = Vector2::zero();

        assert_eq!(v1.x, 0u32);

        let v2: Vector2<f32> = Vector2::zero();

        assert_eq!(v2.x, 0f32);
    }
}
