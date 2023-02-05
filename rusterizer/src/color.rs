use std::ops::*;

#[derive(Copy, Clone)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_argb8(&self) -> u32 {
        let mut argb: u32 = self.a as u32;
        argb = (argb << 8) + self.r as u32;
        argb = (argb << 8) + self.g as u32;
        argb = (argb << 8) + self.b as u32;
        argb
    }
    pub fn from_argb8(color: u32) -> Color {
        Color {
            a: (color >> 24) as u8,
            r: (color >> 16) as u8,
            g: (color >> 8) as u8,
            b: color as u8,
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Self {
        Color {
            a: self.a + other.a,
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a + other.a,
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            a: self.a + other.a,
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Color) -> Self {
        Color {
            a: self.a * other.a,
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Color {
            a: self.a * other as u8,
            r: self.r * other as u8,
            g: self.g * other as u8,
            b: self.b * other as u8,
        }
    }
}

impl Div<Color> for Color {
    type Output = Self;
    fn div(self, other: Color) -> Self {
        Color {
            a: self.a / other.a,
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Color {
            a: self.a / other as u8,
            r: self.r / other as u8,
            g: self.g / other as u8,
            b: self.b / other as u8,
        }
    }
}
