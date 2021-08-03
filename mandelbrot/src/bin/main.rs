use env_logger;
use structopt::{self, StructOpt};
use mandelbrot::{parser, renderer};
use num::Complex;

#[derive(StructOpt, Debug)]
#[structopt(name = "mandelbrot")]
struct Args {

    #[structopt(short, long)]
    filename: String,

    #[structopt(short, long, parse(try_from_str = parser::bounds_from_str))]
    pixels: (usize, usize),

    #[structopt(short, long, parse(try_from_str = parser::complex_from_str))]
    upper_left: Complex<f64>,

    #[structopt(short, long, parse(try_from_str = parser::complex_from_str))]
    lower_right: Complex<f64>
}

fn main() {
    env_logger::init();

    let args = Args::from_args();

    let mut pixels = vec![0; args.pixels.0 * args.pixels.1];

    renderer::parallel_render(&mut pixels,
                              args.pixels,
                              args.upper_left,
                              args.lower_right,
                              8);

    renderer::write_image(&args.filename, &pixels, args.pixels).expect("error writing PNG file");
}
