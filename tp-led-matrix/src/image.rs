

pub mod image {
use crate::gamma::gamma;
use micromath::F32Ext;
use core::ops::Mul;
use core::ops::Div;
use core::ops::Index;
use core::ops::IndexMut;
use core::convert::AsRef;


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
        type Output= Color;
        fn mul(self, rhs: f32) -> Self::Output{
            let r = ((self.r as f32)* rhs).round() as u8;
            let g = ((self.g as f32)*rhs).round() as u8;
            let b = ((self.b as f32) *rhs).round() as u8;
            Color{r,g,b}
        }
    }

    impl Div<f32> for Color{
        type Output= Color;
        fn div(self, rhs: f32) -> Self::Output{
            let r = ((self.r as f32)/rhs).round() as u8;
            let g = ((self.g as f32)/rhs).round() as u8;
            let b = ((self.b as f32)/rhs).round() as u8;
            Color{r,g,b}
        }
    }

    #[repr(transparent)]
    pub struct Image([Color; 64]);

    impl Image{
        pub fn new_solid(color: Color) -> Self{
            //Define the type and mutable of ima
            let mut ima: Image= Image([Color::BLUE;64]);//Initialize it with a value, for example BLUE
            for i in 0..64{
                ima.0[i]= color;
            }
            return ima;
        }

        //Function for giving access to the content of one particular row
        pub fn row(&self, row: usize) -> &[Color]{
            match row{
                0 => &self.0[0..8],
                1 => &self.0[8..16],
                2 => &self.0[16..24],
                3 => &self.0[24..32],
                4 => &self.0[32..40],
                5 => &self.0[40..48],
                6 => &self.0[48..56],
                7 => &self.0[56..64],
                _ => &self.0[56..64],

            }

        }


        pub fn gradient(color: Color) -> Self{
            //Define a (mutable) image for using it
            let mut ima: Image= Image([Color{r:0, g:0, b:0};64]); //Initializa with random values
            //Do a boucle for the rows and columns
            for i in 0..8{
                for j in 0..8{
                    //Define the RHS with the formula for the gradient
                    let rhs= (1+i*i + j) as f32;//(1+row*row + col)
                    //According to the line, and column define a color for each pizel
                    match i{
                        0 => ima.0[j+8*0]= color.div(rhs),
                        1 => ima.0[j+8*1]= color.div(rhs),
                        2 => ima.0[j+8*2]= color.div(rhs),
                        3 => ima.0[j+8*3]= color.div(rhs),
                        4 => ima.0[j+8*4]= color.div(rhs),
                        5 => ima.0[j+8*5]= color.div(rhs),
                        6 => ima.0[j+8*6]= color.div(rhs),
                        7 => ima.0[j+8*7]= color.div(rhs),
                        _ => ima.0[j+8*7]= color.div(rhs),
                    }
                }
            }
            return ima;
        }

    }

    pub trait Default{
        fn default(&self) -> Self;
    }
    impl Default for Image {
        fn default(&self) -> Self{
            let ima: Image= Image([Color{r:0, g:0, b:0};64]);//define the values as 0, having BLACK as default
            return ima;
        }
    }

    impl Index<(usize, usize)> for Image{
        type Output =Color;
        fn index(&self, ind:(usize, usize)) -> &Self::Output{
            match ind.0{
                0 => &self.0[ind.1 + (8)*0],
                1 => &self.0[ind.1 + (8)*1],
                2 => &self.0[ind.1 + (8)*2],
                3 => &self.0[ind.1 + (8)*3],
                4 => &self.0[ind.1 + (8)*4],
                5 => &self.0[ind.1 + (8)*5],
                6 => &self.0[ind.1 + (8)*6],
                7 => &self.0[ind.1 + (8)*7],
                _ => &self.0[ind.1 + (8)*7],
            }
        }
    }
 
    impl IndexMut<(usize, usize)> for Image {
        fn index_mut(&mut self, indm:(usize, usize)) -> &mut Self::Output{
            match indm.0{
                0 => &mut self.0[indm.1 + (8)*0],
                1 => &mut self.0[indm.1 + (8)*1],
                2 => &mut self.0[indm.1 + (8)*2],
                3 => &mut self.0[indm.1 + (8)*3],
                4 => &mut self.0[indm.1 + (8)*4],
                5 => &mut self.0[indm.1 + (8)*5],
                6 => &mut self.0[indm.1 + (8)*6],
                7 => &mut self.0[indm.1 + (8)*7],
                _ => &mut self.0[indm.1 + (8)*7],
        }
    }
    }

}//close pub mod image
