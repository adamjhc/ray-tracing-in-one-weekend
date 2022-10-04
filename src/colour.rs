use std::io::Write;

use crate::vec3::Vec3;

pub type Colour = Vec3;

impl Colour {
    pub fn write_to(&self, std: &mut impl Write) {
        std.write_all(
            format!(
                "{} {} {}\n",
                (255.999 * self.x) as i32,
                (255.999 * self.y) as i32,
                (255.999 * self.z) as i32,
            )
            .as_bytes(),
        )
        .unwrap();
    }
}
