pub enum BrightnessConversionType {
    Average,
    Lightness,
    Luminosity,
}

pub fn rgb_pixel_to_brightness(p: (u8, u8, u8), algo: BrightnessConversionType) -> u8 {
    let red = p.0 as usize;
    let green = p.1 as usize;
    let blue = p.2 as usize;

    let bright = match algo {
        BrightnessConversionType::Average => { ((red + green + blue)/3) as u8},
        BrightnessConversionType::Lightness => { 
            let color_array = vec!(red, green, blue);
            return ((color_array.iter().max().unwrap() + color_array.iter().min().unwrap() )/ 2) as u8;
        },
        BrightnessConversionType::Luminosity => { 
            return ((0.21 * red as f32) + (0.72 * green as f32) + (0.07 * blue as f32)) as u8;
        },
    };

    return bright;
}