pub mod bitmap;
pub mod complex;

use std::env;

use crate::bitmap::bitmap::{BitMap, Pixel};
use crate::complex::complex::*;

/**
 * Checks if number c is inside Mandelbrot set, by checking if in max_iter iteratins
 * it will stay under bound 
 */
fn mandelbrot(c: Complex<f64>, bound: f64, max_iter: usize) -> bool {
    let mut z = Complex::new(0., 0.);
    for _ in 0 .. max_iter {
        z = z.clone() * z + c.clone(); 
        if z.clone().module_sq() > bound {
            return false;
        }
    }
    true
}

/** 
 * Preapares geneartor for bitmap with Mandelbrot set with point (0, 0) in (cent_x, cent_y) pixel, and length of
 * pixels set to pixel_size
 */
fn create_generator((cent_x, cent_y) : (i64, i64), pixel_size : f64, bound: f64, max_iter: usize) -> impl Fn(u32, u32) -> Pixel {
    move |x, y| {
        let c = Complex::new(x as f64 - cent_x as f64, y as f64 - cent_y as f64) * pixel_size;
        if mandelbrot(c, bound, max_iter) {
            Pixel::BLACK
        } else {
            Pixel::WHITE
        }
    }
}

/**
 * Generates bitmap with fragment of Mandelbrot set plot
 */
fn get_bitmap_mandelbrot((x1, y1): (f64, f64), (x2, y2): (f64, f64), pixel_size : f64, bound: f64, max_iter: usize) -> BitMap {
    let width = ((x2 - x1) / pixel_size).ceil() as u32;
    let height = ((y2 - y1) / pixel_size).ceil() as u32;
    let center = ((-x1 / pixel_size).round() as i64, (y2 / pixel_size).round() as i64); 

    BitMap::new_from_generator(&create_generator(center, pixel_size, bound, max_iter), width, height)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.contains(&String::from("--help")) || args.contains(&String::from("/?")) {
        println!("Mandelbrot set drawer");
        println!("");
        println!("Wrong arguments please try {} <file> <x1> <y1> <x2> <y2> <pixel_size> <bound> <max_iter>", args[0]);
        println!("where:");
        println!("- <file>              - defines name of file to which plot will be saved");
        println!("- <x1> <y1> <x2> <y2> - defines part of plot to be rendered");
        println!("- <pixel_size>        - defines length of single pixel on plot (default=0.001)");
        println!("- <bound>             - defines what is maximal module of muber before being regected (default=2.0)");
        println!("- <max_iter>          - defines number of iteration in checking of pixel is in set (default=80)");
    } else {
        let file = &args[1];

        let x1 = if args.len() <= 2 {
            -1.0
        } else {
            args[2].parse().expect("{x1 is not a number}")
        };

        let y1 = if args.len() <= 3 {
            -1.0
        } else {
            args[3].parse().expect("{y1 is not a number}")
        };

        let x2 = if args.len() <= 4 {
            x1 + 2.0
        } else {
            args[4].parse().expect("{x2 is not a number}")
        };

        let y2 = if args.len() <= 5 {
            y1 + 2.0
        } else {
            args[5].parse().expect("{y2 is not a number}")
        };

        let pixel_size = if args.len() <= 6 {
            0.001
        } else {
            args[6].parse().expect("{pixel_size is not a number}")
        };

        let bound = if args.len() <= 7 {
            2.0
        } else {
            args[7].parse().expect("{bound is not a number}")
        }; 

        let max_iter = if args.len() <= 8 {
            80
        } else {
            args[8].parse().expect("{max_iter is not a positive integer}")
        };

        if x1 >= x2 {
            println!("Wrong arguments; x1 should be less than x2, but {} >= {}", x1, x2)
        } else if y1 >= y2 {
            println!("Wrong arguments: y1 should be less than y2, but {} >= {}", y1, y2)
        } else if bound <= 0.0 {
            println!("Wrong arguments: needs to be greater than 0, but {} <0 0", bound)
        } else {
            let bm = get_bitmap_mandelbrot((x1, y1), (x2, y2), pixel_size, bound * bound, max_iter);
            match bm.save_as_bmp(file) {
                Ok(()) => print!("Plot generated"),
                Err(e) => print!("Error occured during saving to file: {}", e)
            }
        }
    }
}
