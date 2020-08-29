use crate::scaling::CHAR_SCALING_FACTOR;
use crate::BrightnessPixel;

const DARK_TO_LIGHTEST_CHARS: &'static str =
    " .\'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

pub struct AsciiConverter {
    pub should_invert_colors: bool,
}

impl AsciiConverter {
    pub fn brightness_to_ascii_pixel(&self, brightness: u8) -> char {
        let mut target_char_index =
            (brightness as usize * (DARK_TO_LIGHTEST_CHARS.len() - 1)) / 255;

        if self.should_invert_colors {
            target_char_index = DARK_TO_LIGHTEST_CHARS.len() - target_char_index - 1;
        }

        let target_char = DARK_TO_LIGHTEST_CHARS
            .chars()
            .nth(target_char_index)
            .unwrap();
        return target_char;
    }

    pub fn brightness_matrix_to_multiline_ascii_string(
        &self,
        img: Vec<Vec<BrightnessPixel>>,
    ) -> String {
        let mut out: String = "".to_string();

        for line in img {
            let linestr: String = line
                .iter()
                .map(|p| {
                    let c = self.brightness_to_ascii_pixel(p.brightness);
                    let tripple_c: String = [c; CHAR_SCALING_FACTOR].iter().collect();

                    return tripple_c;
                })
                .collect();
            out = out + "\n" + &linestr;
        }

        return out;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_max_brightness_is_correct() {
        let converter = AsciiConverter {
            should_invert_colors: false,
        };
        assert_eq!(converter.brightness_to_ascii_pixel(255), '$');
    }

    #[test]
    fn test_low_brightness_is_correct() {
        let converter = AsciiConverter {
            should_invert_colors: false,
        };
        assert_eq!(converter.brightness_to_ascii_pixel(0), '`');
    }

    #[test]
    fn test_max_brightness_is_correct_when_switched() {
        let converter = AsciiConverter {
            should_invert_colors: true,
        };
        assert_eq!(converter.brightness_to_ascii_pixel(255), '`');
    }

    #[test]
    fn test_low_brightness_is_correct_when_switched() {
        let converter = AsciiConverter {
            should_invert_colors: true,
        };
        assert_eq!(converter.brightness_to_ascii_pixel(0), '$');
    }
}
