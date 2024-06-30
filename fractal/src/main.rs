extern crate num;
extern crate image;
use image::{ColorType, ImageEncoder};
use image::codecs::png::PngEncoder;
use std::fs::File;
use num::Complex;
use std::str::FromStr;
use std::io::Write;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn analy_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

fn analy_complex(s: &str) -> Option<Complex<f64>> {
    match analy_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

fn pixel_en_point(
    bords: (usize, usize),
    pixel: (usize, usize),
    super_ga: Complex<f64>,
    infer_dr: Complex<f64>,
) -> Complex<f64> {
    let (large, haute) = (infer_dr.re - super_ga.re, super_ga.im - infer_dr.im);
    Complex {
        re: super_ga.re + pixel.0 as f64 * large / bords.0 as f64,
        im: super_ga.im - pixel.1 as f64 * haute / bords.1 as f64,
    }
}

fn render(pixels: &mut [u8], bords: (usize, usize), super_ga: Complex<f64>, infer_dr: Complex<f64>) {
    assert!(pixels.len() == bords.0 * bords.1 * 3);
    for lig in 0..bords.1 {
        for col in 0..bords.0 {
            let point = pixel_en_point(bords, (col, lig), super_ga, infer_dr);
            let pixel_index = 3 * (lig * bords.0 + col);
            let value = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
            pixels[pixel_index] = value;     // Rouge
            pixels[pixel_index + 1] = value; // Vert
            pixels[pixel_index + 2] = value; // Bleu
        }
    }
}

fn ecrire_image(
    nomfic: &str,
    pixels: &[u8],
    bords: (usize, usize),
) -> Result<(), std::io::Error> {
    let sortie = File::create(nomfic)?;
    let encodeur = PngEncoder::new(sortie);
    let _ = encodeur.write_image(&pixels, bords.0 as u32, bords.1 as u32, ColorType::Rgb8.into());
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage fractal Nom fichier pixels supga enfdr"
        )
        .unwrap();
        std::process::exit(1);
    }
    let bords = analy_pair(&args[2], 'x').expect("erreur analyse dim");
    let super_ga = analy_complex(&args[3]).expect("Erreur analyse point sup gauche");
    let infer_dr = analy_complex(&args[4]).expect("Erreur analyse point inf droit");
    let mut pixels = vec![0; bords.0 * bords.1 * 3];
    render(&mut pixels, bords, super_ga, infer_dr);
    ecrire_image(&args[1], &pixels, bords).expect("Erreur écriture fichier");
}
