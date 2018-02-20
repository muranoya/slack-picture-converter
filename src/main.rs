extern crate image;

use std::process::exit;
use std::env;
use std::cmp::min;

const TONE:u32 = 4;
const TONE_STEP:u32 = 256 / TONE;

fn usage(args: &Vec<String>) {
    println!("{} {}\n", args[0], "[image-file] [tile-size]");
    exit(1);
}

fn mosaic(img: &image::RgbImage, x: u32, tile_x:u32, y: u32, tile_y: u32) -> (u32, u32, u32) {
    let mut sum_r:u32 = 0;
    let mut sum_g:u32 = 0;
    let mut sum_b:u32 = 0;

    let mut yy = 0;
    while yy < tile_y {
        let mut xx = 0;
        while xx < tile_x {
            let rgb = img.get_pixel(x+xx, y+yy);
            sum_r += rgb.data[0] as u32;
            sum_g += rgb.data[1] as u32;
            sum_b += rgb.data[2] as u32;
            xx += 1;
        }
        yy += 1;
    }

    let t2 = tile_x * tile_y;
    let r = sum_r / t2;
    let g = sum_g / t2;
    let b = sum_b / t2;

    return (r-r%TONE_STEP, g-g%TONE_STEP, b-b%TONE_STEP);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        usage(&args);
    }

    let img = image::open(&args[1]).unwrap();
    let rgb = img.to_rgb();
    let tile_size = args[2].parse::<u32>().unwrap();

    let mut y = 0;
    while y < rgb.height() {
        let mut x = 0;
        while x < rgb.width() {
            let (r, g, b) = mosaic(&rgb, x, min(tile_size, rgb.width()-x), y, min(tile_size, rgb.height()-y));
            print!(":p{:<02x}{:<02x}{:<02x}:", r, g, b);
            x += tile_size;
        }
        println!("");
        y += tile_size;
    }
}
