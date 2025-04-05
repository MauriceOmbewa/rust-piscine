#[derive(Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
        let temp = Light{
            alias: String::from(alias),
            brightness: 0,
        };
        temp
	}
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for l in lights{
        if l.alias == alias{
            l.brightness = value;
        }
    }
}