use crate::brightnessconverter::BrightnessConversionType;

pub struct Options {
    pub scale_config: ScaleConfig,
    pub brightness_convertion_algorithm: BrightnessConversionType,
    pub should_invert_colors: bool,
}

#[derive(Copy, Clone)]
pub enum ScaleConfig {
    OriginalSize,
    FitToTerminal,
    FixedSize(u32, u32),
}

pub fn get_default_options() -> Options {
    return Options {
        scale_config: ScaleConfig::FitToTerminal,
        brightness_convertion_algorithm: BrightnessConversionType::Luminosity,
        should_invert_colors: false,
    };
}
