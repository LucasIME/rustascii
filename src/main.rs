use image;
use image::imageops::FilterType;

mod asciiconverter;
use asciiconverter::brightness_matrix_to_multiline_ascii_string;

mod brightnessconverter;
use brightnessconverter::rgb_pixel_to_brightness;

#[derive(Debug, Clone, Copy)]
pub struct BrightnessPixel {
    x: u32,
    y: u32,
    brightness: u8,
}

fn pixel_vector_to_matrix(
    vec: Vec<BrightnessPixel>,
    width: u32,
    height: u32,
) -> Vec<Vec<BrightnessPixel>> {
    let mut out = vec![
        vec![
            BrightnessPixel {
                x: 0,
                y: 0,
                brightness: 0
            };
            width as usize
        ];
        height as usize
    ];
    for pixel in vec.iter() {
        out[pixel.y as usize][pixel.x as usize] = *pixel;
    }

    return out;
}

fn main() {
    // let img = image::open("./img/ascii-pineapple.jpg").unwrap();
    // let img = image::imageops::resize(&img, 72, 47, FilterType::CatmullRom);
    let img = image::open("./img/yin-yang-symbol.jpg").unwrap();
    let img = image::imageops::resize(&img, 80, 80, FilterType::CatmullRom);
    let (width, height) = img.dimensions();

    let brightness_pixel_map: Vec<BrightnessPixel> = img
        .enumerate_pixels()
        // .pixels()
        .map(|p|  {
            // TODO: find a way to fix this and pass things in a first class way
            let rgb = (((p.2).0)[0], ((p.2).0)[1], ((p.2).0)[2]);
            BrightnessPixel {
            x: p.0,
            y: p.1,
            brightness: rgb_pixel_to_brightness(rgb, brightnessconverter::BrightnessConversionType::Luminosity)        } 
        })
        .collect();

    let bright_matrix = pixel_vector_to_matrix(brightness_pixel_map, width, height);

    println!(
        "{}",
        brightness_matrix_to_multiline_ascii_string(bright_matrix)
    );
}
