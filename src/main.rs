use image;
use image::imageops::FilterType;

mod asciiconverter;
use asciiconverter::AsciiConverter;

mod brightnessconverter;
use brightnessconverter::rgb_pixel_to_brightness;

mod options;
mod scaling;

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
    let opts = options::get_default_options();
    let img = image::open("./img/octocat.png").unwrap().into_rgb();

    let (scaled_width, scaled_height) =
        scaling::get_new_size_according_to_config((img.width(), img.height()), opts.scale_config);
    let img = image::imageops::resize(&img, scaled_width, scaled_height, FilterType::CatmullRom);
    let (width, height) = img.dimensions();

    let brightness_pixel_map: Vec<BrightnessPixel> = img
        .enumerate_pixels()
        .map(|(x, y, rgb)| {
            let rgb_tuple = (rgb[0], rgb[1], rgb[2]);
            BrightnessPixel {
                x,
                y,
                brightness: rgb_pixel_to_brightness(
                    rgb_tuple,
                    &opts.brightness_convertion_algorithm,
                ),
            }
        })
        .collect();

    let bright_matrix = pixel_vector_to_matrix(brightness_pixel_map, width, height);

    println!(
        "{}",
        AsciiConverter {
            should_invert_colors: opts.should_invert_colors
        }
        .brightness_matrix_to_multiline_ascii_string(bright_matrix)
    );
}
