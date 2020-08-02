use crate::BrightnessPixel;

const DARK_TO_LIGHTEST_CHARS: &'static str =
    "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

pub fn brightness_to_ascii_pixel(brightness: u8) -> char {
    let target_char_index = (brightness as usize * (DARK_TO_LIGHTEST_CHARS.len() - 1)) / 255;
    let target_char = DARK_TO_LIGHTEST_CHARS
        .chars()
        .nth(target_char_index)
        .unwrap();
    return target_char;
}

pub fn brightness_matrix_to_multiline_ascii_string(img: Vec<Vec<BrightnessPixel>>) -> String {
    let mut out: String = "".to_string();

    for line in img {
        let linestr: String = line.iter().map(|p| {
            let c = brightness_to_ascii_pixel(p.brightness);
            let tripple_c: String = vec!(c, c, c).iter().collect();

            return tripple_c;

        }).collect();
        out = out + "\n" + &linestr;
    }

    return out;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_max_brightness_is_correct() {
        assert_eq!(brightness_to_ascii_pixel(255), '$');
    }

    #[test]
    fn test_low_brightness_is_correct() {
        assert_eq!(brightness_to_ascii_pixel(0), '`');
    }

}
