pub trait Area {
    fn area(&self) -> f32;
}

pub trait Perimeter {
    fn perimeter(&self) -> f32;
}

pub trait Circumference {
    fn circumference(&self) -> f32;
}

pub trait ArcLengthRadians {
    fn arc_len_rad(&self, rads: f32) -> f32;
}

pub trait ArcLengthDegrees {
    fn arc_len_deg(&self, degrees: f32) -> f32;
}

pub trait SectorAreaRadians {
    fn sect_area_rad(&self, rads: f32) -> f32;
}

pub trait SectorAreaDegrees {
    fn sect_area_deg(&self, degrees: f32) -> f32;
}

pub trait Diameter {
    fn diameter(&self) -> f32;
}