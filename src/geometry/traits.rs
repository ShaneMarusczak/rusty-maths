/// A trait for shapes that have a calculable area
pub trait Area {
    /// Returns the area of the shape
    fn area(&self) -> f32;
}

/// A trait for shapes that have a calculable perimeter
pub trait Perimeter {
    /// Returns the perimeter of the shape
    fn perimeter(&self) -> f32;
}

/// A trait for circular shapes that have a calculable circumference
pub trait Circumference {
    /// Returns the circumference of the circle
    fn circumference(&self) -> f32;
}

/// A trait for shapes that can calculate arc length given an angle in radians
pub trait ArcLengthRadians {
    /// Returns the length of an arc with the given angle in radians
    fn arc_len_rad(&self, rads: f32) -> f32;
}

/// A trait for shapes that can calculate arc length given an angle in degrees
pub trait ArcLengthDegrees {
    /// Returns the length of an arc with the given angle in degrees
    fn arc_len_deg(&self, degrees: f32) -> f32;
}

/// A trait for shapes that can calculate sector area given an angle in radians
pub trait SectorAreaRadians {
    /// Returns the area of a sector with the given angle in radians
    fn sect_area_rad(&self, rads: f32) -> f32;
}

/// A trait for shapes that can calculate sector area given an angle in degrees
pub trait SectorAreaDegrees {
    /// Returns the area of a sector with the given angle in degrees
    fn sect_area_deg(&self, degrees: f32) -> f32;
}

/// A trait for circular shapes that have a calculable diameter
pub trait Diameter {
    /// Returns the diameter of the circle
    fn diameter(&self) -> f32;
}
