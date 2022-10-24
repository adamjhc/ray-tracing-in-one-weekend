use crate::{colour::Colour, hittable_list::HittableList};

pub struct World {
    pub background: Colour,
    pub objects: HittableList,
}

impl World {
    pub fn new(background: Colour, objects: HittableList) -> Self {
        Self {
            background,
            objects,
        }
    }
}
