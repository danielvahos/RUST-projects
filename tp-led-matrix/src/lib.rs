#![no_std]
pub mod image;
pub mod gamma;
//pub mod matrix;

pub use image::{Color, Image}; //Reexporting types Color and Image

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
