// #![feature(exclusive_range_pattern)]
// #![allow(illegal_floating_point_literal_pattern)]

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Config {
    /// Below this threshold, the character that is being processed will follow the case of the
    /// previous character. Above this threshold, it will follow the opposite case.
    pub threshold: f32,
}
impl Default for Config {
    fn default() -> Self {
        Self { threshold: 0.2 }
    }
}

pub fn oscillate_case(input: &str) -> String {
    oscillate_case_config(input, Config::default())
}

pub fn oscillate_case_config(input: &str, config: Config) -> String {
    let mut buffer = String::with_capacity(input.len());

    oscillate_case_config_buffer(input, config, &mut buffer);

    buffer
}

pub fn oscillate_case_buffer(input: &str, buffer: &mut String) {
    oscillate_case_config_buffer(input, Config::default(), buffer);
}

pub fn oscillate_case_config_buffer(input: &str, config: Config, buffer: &mut String) {
    use rand::{Rng, SeedableRng};
    use tinymt::TinyMT32;

    let mut generator = TinyMT32::from_entropy();
    let mut prev_uppercase = generator.gen_bool(0.5);

    for ch in input.chars() {
        // match (prev_uppercase, generator.gen_range(0.0, 1.0)) {
        //     (true, 0.0..config.threshold) => buffer.extend(ch.to_uppercase()),
        //     (false, 0.0..config.threshold) => buffer.extend(ch.to_lowercase()),
        //     (true, config.threshold..1.0) => {
        //         buffer.extend(ch.to_lowercase());
        //         prev_uppercase = false;
        //     }
        //     (false, config.threshold..1.0) => {
        //         buffer.extend(ch.to_uppercase());
        //         prev_uppercase = true;
        //     }
        //     (_, _) => unreachable!(),
        // }
        if generator.gen_range(0.0, 1.0) < config.threshold {
            if prev_uppercase {
                buffer.extend(ch.to_uppercase());
            } else {
                buffer.extend(ch.to_lowercase());
            }
        } else {
            if prev_uppercase {
                buffer.extend(ch.to_lowercase());
            } else {
                buffer.extend(ch.to_uppercase());
            }
            prev_uppercase = !prev_uppercase;
        }
    }
}

// pub fn oscillate_case_config_slice(
//     input: &str,
//     config: Config,
//     slice: &mut [u8],
// ) -> Result<&[u8], &[u8]> {
//     todo!()
// }
