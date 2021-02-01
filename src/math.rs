use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub elements: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            elements: [x, y, z],
        }
    }

    pub fn x(&self) -> f32 {
        self.elements[0]
    }

    pub fn y(&self) -> f32 {
        self.elements[1]
    }

    pub fn z(&self) -> f32 {
        self.elements[2]
    }

    pub fn r(&self) -> f32 {
        self.elements[0]
    }

    pub fn g(&self) -> f32 {
        self.elements[1]
    }

    pub fn b(&self) -> f32 {
        self.elements[2]
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.elements[0] + rhs.elements[0],
                self.elements[1] + rhs.elements[1],
                self.elements[2] + rhs.elements[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.elements[0] - rhs.elements[0],
                self.elements[1] - rhs.elements[1],
                self.elements[2] - rhs.elements[2],
            ],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.elements[0] * rhs.elements[0],
                self.elements[1] * rhs.elements[1],
                self.elements[2] * rhs.elements[2],
            ],
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.elements[0] / rhs.elements[0],
                self.elements[1] / rhs.elements[1],
                self.elements[2] / rhs.elements[2],
            ],
        }
    }
}

#[test]
fn test_vec3_init() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(1.0, v.x());
    assert_eq!(2.0, v.y());
    assert_eq!(3.0, v.z());
}

#[test]
fn test_vec3_add() {
    let v = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x(), 2.0);
    assert_eq!(v.y(), 4.0);
    assert_eq!(v.z(), 6.0);
}

#[test]
fn test_vec3_sub() {
    let v = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(0.0, v.x());
    assert_eq!(0.0, v.y());
    assert_eq!(0.0, v.z());
}

#[test]
fn test_vec3_mul() {
    let v = Vec3::new(1.0, 2.0, 3.0) * Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(1.0, v.x());
    assert_eq!(4.0, v.y());
    assert_eq!(9.0, v.z());
}

#[test]
fn test_vec3_div() {
    let v = Vec3::new(1.0, 2.0, 3.0) / Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(1.0, v.x());
    assert_eq!(1.0, v.y());
    assert_eq!(1.0, v.z());
}
