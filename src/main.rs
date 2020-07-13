#[macro_use]
extern crate clap;
extern crate image;
extern crate num;

use clap::{App , Arg, ArgMatches, SubCommand};
use num::complex::Complex;
use std::str::FromStr;

use self::fractals::mandelbrot::mandelbrot;
use self::interp::cubic::interpolate_rgb;
use self::utils::swatch::write_swatch;

fn main() {
    _main().unwrap();
}

fn _main() -> Result<(), String>{
    let flags = App::new("Fractalinator")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("swatch")
                .about("create gradient swatch")
                .arg(
                    Arg::with_name("colors")
                        .short("c")
                        .long("colors")
                        .value_name("R,G,B")
                        .min_values(2)
                        .help("colon separated rgb values")
                        .required(true)
                )
                .arg(
                    Arg::with_name("positions")
                        .short("p")
                        .long("positions")
                        .min_values(2)
                        .help("list of positions")
                        .required(true)
                )
                .arg(
                    Arg::with_name("outfile")
                        .short("o")
                        .long("outfile")
                        .value_name("FILE")
                        .help("sets the output file")
                )
        )
        .subcommand(
            SubCommand::with_name("generate")
            .about("generate fractal")
            .arg(
                Arg::with_name("outfile")
                    .short("o")
                    .long("outfile")
                    .value_name("FILE")
                    .help("sets the output file"),
            )
            .arg(
                Arg::with_name("dimensions")
                    .short("d")
                    .long("dimensions")
                    .required(true)
                    .number_of_values(2)
                    .value_name("X Y")
                    .validator(is_list_of_f64)
            )
            .arg(
                Arg::with_name("center")
                    .short("C")
                    .long("center")
                    .required(true)
                    .number_of_values(2)
                    .value_name("X Y")
                    .allow_hyphen_values(true)
                    .help("fractal center point")
                    .validator(is_list_of_f64)
                )
            .arg(
                Arg::with_name("zoom")
                    .short("z")
                    .long("zoom")
                    .value_name("float")
                    .validator(is_f64)
                )
            .arg(
                Arg::with_name("colors")
                    .short("c")
                    .long("colors")
                    .value_name("R,G,B")
                    .min_values(2)
                    .help("colon separated rgb values")
                    .requires("positions")
            )
            .arg(
                Arg::with_name("positions")
                    .short("p")
                    .long("positions")
                    .min_values(2)
                    .help("list of positions")
                    .requires("colors")
                )
            .arg(
                Arg::with_name("limit")
                    .short("l")
                    .long("limit")
                    .help("limit for escape time algorithm")
                    .validator(is_u32)
            )
        )
        .get_matches();

    match flags.subcommand() {
        ("swatch", Some(swatch)) => {
            return handle_swatch(swatch);
        }
        ("generate", Some(gen)) => {
            return handle_fractal(gen);
        }
        _ => {
            return Ok(())
        }
    };
}

fn parse_tripple<T:FromStr>(s: &str) -> Option<(T, T, T)>{
    let lst: Vec<&str> = s.split(',').collect();
    if lst.len() < 3 {
        return None;
    }

    match (T::from_str(lst[0]), T::from_str(lst[1]), T::from_str(lst[2])) {
        (Ok(r), Ok(g), Ok(b)) => Some((r,g,b)),
        _ => None
    }
}

fn pixel_to_point(
    dimensions: (u32, u32),
    pixel: (u32, u32),
    _zoom: u32,
    center: Complex<f64>,
) -> Complex<f64> {
    let zoom = if _zoom == 0 { 1 } else { _zoom };
    let half_width = (dimensions.0 as f64 / zoom as f64) / 2.0;
    let half_hieght = (dimensions.1 as f64 / zoom as f64) / 2.0;

    Complex {
        re: center.re - half_width + (pixel.0 as f64 / zoom as f64),
        im: center.im - half_hieght + (pixel.1 as f64 / zoom as f64),
    }
}

fn is_f64(s: String) -> Result<(), String> {
    if let Err(..) = s.parse::<f64>() {
        return Err(String::from("Not a valid f64!"))
    }
    Ok(())
}

fn is_u32(s: String) -> Result<(), String> {
    if let Err(..) = s.parse::<u32>() {
        return Err(String::from("Not a valid u32!"))
    }
    Ok(())
}

fn is_list_of_f64(s: String) -> Result<(), String> {
    let lst: Vec<&str> = s.split(' ').collect();
    for f in lst {
        if let Err(e) = is_f64(f.to_string()){
            return Err(e)
        }
    }
    Ok(())
}

fn handle_swatch(cmd: &ArgMatches) -> Result<(), String> {
    let out_file = cmd.value_of("outfile").unwrap_or("unnamed.png").to_string();
    let color_points: Option<Vec<(u8, u8, u8)>> = match cmd.values_of("colors") {
        None => None,
        Some(v) => Some(v.map(|rgb| parse_tripple(rgb).unwrap()).collect()),
    };
    let positions: Option<Vec<f64>> = match cmd.values_of("positions") {
        None => None,
        Some(v) => Some(v.map(|f| f.parse::<f64>().unwrap()).collect()),
    };
    let rgb_vec = match color_points {
        None => vec![(255,255,255)],
        Some(c) => match positions {
            None => vec![(255,255,255)],
            Some(p) => interpolate_rgb(&c, p)
        }
    };
    write_swatch(&rgb_vec, &out_file).unwrap();
    Ok(())
}

fn handle_fractal(cmd: &ArgMatches) -> Result<(), String> {
    let out_file = cmd.value_of("outfile").unwrap_or("unnamed.png");
    let zoom = cmd.value_of("zoom").unwrap_or("1").parse::<u32>().unwrap();
    let dim_str = cmd
        .values_of("dimensions")
        .unwrap()
        .collect::<Vec<&str>>();
    let center = cmd
        .values_of("center")
        .unwrap()
        .collect::<Vec<&str>>();
    let (imgx, imgy) = (
        dim_str[0].parse::<u32>().unwrap(),
        dim_str[1].parse::<u32>().unwrap(),
    );
    let color_points: Option<Vec<(u8, u8, u8)>> = match cmd.values_of("colors") {
        None => None,
        Some(v) => Some(v.map(|rgb| parse_tripple(rgb).unwrap()).collect()),
    };
    let positions: Option<Vec<f64>> = match cmd.values_of("positions") {
        None => None,
        Some(v) => Some(v.map(|f| f.parse::<f64>().unwrap()).collect()),
    };

    let rgb_vec = match color_points {
        None => vec![(255,255,255)],
        Some(c) => match positions {
            None => vec![(255,255,255)],
            Some(p) => interpolate_rgb(&c, p)
        }
    };

    let limit = match cmd.value_of("limit"){
        None => 255,
        Some(s) => s.parse::<u32>().unwrap(),
    };


    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let center = Complex {
        re: center[0].parse::<f64>().unwrap(),
        im: center[1].parse::<f64>().unwrap(),
    };
    for x in 0..imgx {
        for y in 0..imgy {
            let z = pixel_to_point((imgx, imgy), (x, y), zoom, center);
            Complex::new(imgx as f64, imgy as f64);

            let lim = match mandelbrot(z.clone(), limit) {
                None => 0,
                Some(count) => count,
            };
            let pixel = imgbuf.get_pixel_mut(x, y);

            if lim != 0 {
                let one_over_log_of_2: f64 = 1.0 / (2.0 as f64).ln();
                let smooth = (one_over_log_of_2).ln() * one_over_log_of_2;
                let color_i = ((lim as f64 + 1.0 - smooth).sqrt() * 256.0) as usize % rgb_vec.len() - 1;
                let cp = rgb_vec[color_i];
                *pixel = image::Rgb([cp.0, cp.1, cp.2]);
            }
        }
    }

    imgbuf.save(out_file).unwrap();
    Ok(())
}

mod fractals;
mod interp;
mod utils;
