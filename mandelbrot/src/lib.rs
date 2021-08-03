//! Mandelbrot set example from Programming Rust 2nd Edition (2021)
use log::debug;
use num::Complex;
use std::str::FromStr;

pub mod parser {
    use super::*;
    use std::io::{Error, ErrorKind};

    /// Parse complex number from string or return error if format is wrong.
    pub fn complex_from_str(s: &str) -> Result<Complex<f64>, Error> {
        parse_complex(s.trim_matches(|c| c == '"')).ok_or_else(
            || Error::new(
                ErrorKind::InvalidData,
                format!("wrong input format: {}", s)
            )
        )
    }

    /// Parse output image bounds from string or return error if format is wrong.
    pub fn bounds_from_str(s: &str) -> Result<(usize, usize), Error> {
        parse_pair(s, 'x').ok_or_else(
            || Error::new(
                ErrorKind::InvalidData,
                format!("wrong image size format: {}", s)
            )
        )
    }

}

pub mod renderer {
    use super::*;
    use crossbeam;
    use image::ColorType;
    use image::codecs::png::PngEncoder;
    use std::fs::File;

    /// Render a rectangle of the Mandelbrot set into a buffer of pixels.
    ///
    /// The `bounds` argument gives the which holds one grayscale `pixels`,
    /// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
    /// arguments specify points on the complex plane corresponding to the upper-
    /// left and lower-right corners of the pixel buffer.
    pub fn render(
        pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>
    ) {
        assert_eq!(pixels.len(), bounds.0 * bounds.1);

        debug!("rendering pixels in bounds: {:?}", bounds);

        for row in 0..bounds.1 {
            for column in 0..bounds.0 {
                let point = pixel_to_point(
                    bounds, (column, row),
                    upper_left, lower_right
                );
                pixels[row * bounds.0 + column] =
                    match escape_time(point, 255) {
                        None => 0,
                        Some(count) => 255 - count as u8
                    };
            }
        }
    }

    /// The same as `render` but multithreaded.
    pub fn parallel_render(
        pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
        n_threads: usize,
    ) {
        let rows_per_band = bounds.1 / n_threads + 1;

        debug!("parallel rendering: n_threads={}, rows_per_band={}", n_threads, rows_per_band);

        {
            let bands: Vec<&mut [u8]> =
                pixels.chunks_mut(rows_per_band * bounds.0).collect();
            crossbeam::scope(|spawner| {
                for (i, band) in bands.into_iter().enumerate() {
                    debug!("processing band: {}", i);
                    let top = rows_per_band * i;
                    let height = band.len() / bounds.0;
                    let band_bounds = (bounds.0, height);
                    let band_upper_left =
                        pixel_to_point(bounds, (0, top), upper_left, lower_right);
                    let band_lower_right =
                        pixel_to_point(bounds, (bounds.0, top + height),
                                       upper_left, lower_right);
                    spawner.spawn(move |_| {
                        render(band, band_bounds, band_upper_left, band_lower_right);
                    });
                }
            }).unwrap();
        }
    }

    /// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
    /// file named `filename`.
    pub fn write_image(
        filename: &str,
        pixels: &[u8],
        bounds: (usize, usize)
    ) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;

        let encoder = PngEncoder::new(output);

        match encoder.encode(
            &pixels,
            bounds.0 as u32, bounds.1 as u32,
            ColorType::L8
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData, err.to_string()
                )
            )
        }
    }

}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius 2 centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`. `separator` must be an
/// ASCII character.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

/// Parse a pair of floating-point numbers separated by a comma as a complex
/// number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3.0, 4.0, 1, None)]
    #[case(- 1.0, 2.5, 5, Some(1))]
    #[case(0.9, 0.8, 6, Some(2))]
    fn test_escape_time(
        #[case] re: f64,
        #[case] im: f64,
        #[case] limit: usize,
        #[case] result: Option<usize>
    ) {
        assert_eq!(escape_time(Complex { re, im }, limit), result);
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("10,", ','), None);
        assert_eq!(parse_pair::<i32>(",10", ','), None);
        assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
        assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
        assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
    }

    #[rstest]
    #[case("1.25,-0.0625", Some(Complex { re: 1.25, im: - 0.0625 }))]
    #[case("1,2", Some(Complex { re: 1.0, im: 2.0 }))]
    #[case(",-0.0625", None)]
    fn test_parse_complex(#[case] s: &str, #[case] num: Option<Complex<f64>>) {
        assert_eq!(parse_complex(s), num);
    }

    #[test]
    fn test_pixel_to_point() {
        assert_eq!(
            pixel_to_point((100, 200), (25, 175),
                           Complex { re: -1.0, im: 1.0 },
                           Complex { re: 1.0, im: -1.0 }),
            Complex { re: -0.5, im: -0.75 }
        )
    }

    #[test]
    fn test_render() {
        let mut actual = vec![0; 9];
        let expected = vec![252, 250, 252, 244, 0, 0, 244, 0, 0];

        renderer::render(&mut actual, (3, 3),
                         Complex { re: -1.0, im: 1.0 },
                         Complex { re: 1.0, im: -1.0 });

        assert_eq!(actual, expected);
    }

}