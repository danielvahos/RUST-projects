

pub mod image {
use crate::gamma::gamma;
use micromath::F32Ext;
use core::ops::Mul;
use core::ops::Div;

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
            g: gamma:: gamma_correct(self.g),
            b: gamma:: gamma_correct(self.b),
        }
    }
    }
    impl Mul<f32> for Color{
    //pub trait Mul<Rhs = Self> {
        type Output= Color;
        fn mul(self, rhs: f32) -> Self::Output{
            let r = ((self.r as f32)* rhs) as u8;
            let g = ((self.g as f32)*rhs) as u8;
            let b = ((self.b as f32) *rhs) as u8;
            Color{r,g,b}
        }
    }

    impl Div<f32> for Color{
        type Output= Color;
        fn div(self, rhs: f32) -> Self::Output{
            let r = ((self.r as f32)/rhs) as u8;
            let g = ((self.g as f32)/rhs) as u8;
            let b = ((self.b as f32)/rhs) as u8;
            Color{r,g,b}
        }
    }

    pub struct Image([Color; 64]);

    impl Image{
        pub fn new_solid(color: Color) -> Self{

            let mut ima: Image;
            for i in 0..64{
                ima.0[i]= color;
            }
            return ima;
        }
        /*
        pub trait Default {
            fn default(color: RED) -> Self{
                let mut ima: Image;
                for i in 0..64{
                    ima.0[i]= color;
                }
            }
        }

        pub trait Index<Idx: Color> {
            type Output: Color;
            fn index(&self, index: Idx) -> &Self::Output{
            }
        }

        pub trait IndexMut<Idx: ?Sized>: Index<Idx> {
            fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
        }
        */
    }
}
