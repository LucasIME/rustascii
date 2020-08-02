use image;
use image::imageops::FilterType;
use image::GenericImage;
use image::GenericImageView;

mod asciiconverter;
use asciiconverter::brightness_matrix_to_multiline_ascii_string;

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
        .map(|p| BrightnessPixel {
            x: p.0,
            y: p.1,
            brightness: ((p.2)[0] as u16 + (p.2)[1] as u16 + (p.2)[2] as u16) as u8 / 3,
        })
        .collect();

    let bright_matrix = pixel_vector_to_matrix(brightness_pixel_map, width, height);

    println!(
        "{}",
        brightness_matrix_to_multiline_ascii_string(bright_matrix)
    );
}
