pub use gamma::*;
use micromath::F32Ext;
use std::ops::Mul;

pub mod image {
#[derive(Default, Copy, Clone)]
//Create three unsigned bytes for primary colors
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

 //Implement three public constants
impl Color{
    pub const RED: Self = Color{
        r:0xff,
        g:0,
        b:0,
    };

    pub const GREEN: Self = Color{
        r:0,
        g:0xff,
        b:0,
    };

    pub const BLUE: Self = Color{
        r:0,
        g:0,
        b:0xff,
    };

    //Gamma correction
    pub fn gamma_correct(&self) -> Self{
        Color{
            r: gamma:: gamma_correct(self.r),
            g: gamma:: gamma_correct(self,g),
            b: gamma:: gamma_correct(self,b),
        }
    }

    pub trait Mul<Rhs = Self> {
        type Output;
        fn mul(self, rhs: Rhs) -> Self::Output{
            r = self.r * rhs.r;
            g = self.g * rhs.g;
            b = self.b * rhs.b;
            Self::new(r,g,b)
        }
    }

    pub trait Div<Rhs = Self> {
        type Output;
        fn div(self, rhs: Rhs) -> Self::Output{
            r = self.r /rhs.r;
            g = self.g / rhs.g;
            b = self.b / rhs.b;
            Self::new(r,g,b)
        }
    }

    }
}
