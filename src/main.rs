// #![allow(dead_code)]
// #![allow(unused_variables)]


/*
Author : Mark T
  Date : 6/21/2023

  Main file for running processes
*/

mod math;

extern crate image;

use hsv;

use std::env::args;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

use crate::math::structs;

use clap::Parser;

// type GenDataType = f64;
type GenDataType = structs::Complex;

#[derive(Debug, Default)]
struct Config {
    count:                       u64, // Index of the generated image
    c_init: Option<structs::Complex>, // Initial C value for when swap_zc is used
    size_x:                      u32, // Sets Image Width
    size_y:                      u32, // Sets Image Height
    max_i:                       u64, // Sets Maximum Iterations for Generator
    gen_formula:              String, // Specifies Formula for Generator
}

/// The kyros fractal imgae generator rewritten in rust. 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The amount of pixels to generate
    #[arg(short, long, default_value_t = 256)]
    pixels: u32,

    /// The amount of iterations to run per pixel
    #[arg(long, default_value_t = 1024)]
    iterations: u64,

    /// The generation function to use
    #[arg(short, long, default_value_t = ("SD".to_string()))] // The LSP lies, parentheses are needed
    formula: String,
}

/// Function for exiting the program early with an error message. 
fn error_exit(error_msg: String) {
    print!("[Exit code : 1 | {:?}]", error_msg);
}

/// Function for easily getting input values from stdin. 
fn input(out_msg: String) -> String {
    print!("{}", out_msg);
    let _ = std::io::stdout().flush();
    let mut v: String = String::default();
    let _ = std::io::stdin().read_line(&mut v).unwrap();
    return v.trim().to_string();
}

/// Function for initializing interactive configuration. 
fn interactive_config() -> Config {

    // Initializes Configuration Values
    let mut configuration = Config::default();

    return configuration;
}

/// Function for getting the mathematical space of a point.  
fn get_math_value(value: u32, max_ref: u32) -> f64 {
    4f64 * (value as f64) / (max_ref as f64 - 1f64) - 2f64
}

/// Function for getting image from configuration and generator function. 
fn eval_function(config: &Config, generator_function: &dyn Fn(structs::Complex, structs::Complex) -> GenDataType) -> image::RgbImage {
    // Unpacks Image Configuration
    let size_x: u32 = config.size_x;
    let size_y: u32 = config.size_y;
    let max_i: u64 = config.max_i;
    let c_init: Option<structs::Complex> = config.c_init;
    
    let mut c = math::structs::Complex { real: 0f64, imaginary: 0f64, };
    let mut z: math::structs::Complex;

    // Sets Initial 'c' Value (If set)
    let is_julia: bool = match c_init {
        Some(value) => {
            c = value;
            true
        },
        None => false,
    };

    // Initializes Image Buffer
    let mut img = image::ImageBuffer::new(size_x, size_y);
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    // Goes through each pixel
    for i in 0..size_y {
        for j in 0..size_x {

             // Sets Initial Z Value
            z = math::structs::Complex {
                real : get_math_value(j, size_x),
                imaginary : get_math_value(i, size_y),
            };

            if is_julia == false {
                c = z;
            }

            // Runs Math
            let mut iteration: u64 = 0;
            loop {
                if iteration == max_i { break; }
                if z.is_greater(2.0) { break; }
                z = generator_function(c, z);
                iteration += 1;
            };

            let z_output = iteration as f64;

            let pixel = img.get_pixel_mut(j, i);
            // Gets color value
            let out_rgb: (u8, u8, u8);

            if z_output == 0. {out_rgb = (255, 255, 255)}
            else if z_output == max_i as f64 {out_rgb = (0, 0, 0)}
            else {
                out_rgb = hsv::hsv_to_rgb(
                    ( 9f64 * z_output as f64 ) % 360f64,
                    1f64,
                    1f64,
                );
            };

            *pixel = image::Rgb([out_rgb.0, out_rgb.1, out_rgb.2]);
        }
        print!("\t {:.2}% | {} / {}\r", 100f64*(i as f64 + 1f64) / size_y as f64, i+1, size_y);
    }
    println!();
    return img;
}


/// Main function of the program
fn main() {
    // Defines Initial Values
    let cli_args = Args::parse();
    println!("{:?}", cli_args);

    let config: Config;

    if false {
        config = interactive_config();
    }

    else {
        config = Config {
            count: 0,
            c_init: None,
            size_x: 256,
            size_y: 256,
            max_i: 1024,
            gen_formula: "SD".to_string(),
        };
    }

    println!("{:?}", config);

    // Initializes generators into a hashmap
    let mut generators: HashMap<String, &dyn Fn(structs::Complex, structs::Complex) -> GenDataType> = HashMap::new();
    generators.insert("SD".to_string(),  &math::formula::SD);
    generators.insert("R".to_string(),   &math::formula::R);
    generators.insert("BS".to_string(),  &math::formula::BS);
    generators.insert("SYM".to_string(), &math::formula::SYM);

    let generator_function: &dyn Fn(structs::Complex, structs::Complex) -> GenDataType;

    generator_function = match generators.get(&config.gen_formula) {
        Some(function_found) => function_found,
        None => {
            error_exit("Function generation method not found!".to_string());
            std::process::exit(1);
        }
    };

    // Sets the starting time
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Runs Config, gets 32 byte img object
    let img = eval_function(&config, generator_function);
    println!("Saving File!");
    img.save(format!("out#{:}.png", config.count)).unwrap();

    // Finished Timings
    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Show Completion Message
    println!("[Finished in {:.2}s]", end_time - start_time);
    let _ = std::io::stdin().read_line(&mut String::new());
}
