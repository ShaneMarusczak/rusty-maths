use crate::geometry::traits::{
    ArcLengthDegrees, ArcLengthRadians, Area, Circumference, Diameter, SectorAreaDegrees,
    SectorAreaRadians,
};
use std::f32::consts::PI;

/// A struct representing a circle
pub struct Circle {
    /// The center point of the circle as (x, y) coordinates
    pub center: (f32, f32),
    /// The radius of the circle
    pub radius: f32,
}

impl Circle {
    /// Creates a new Circle with the given radius, centered at the origin (0.0, 0.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_maths::geometry::circle::Circle;
    ///
    /// let circle = Circle::new(5.0);
    /// assert_eq!(circle.radius, 5.0);
    /// assert_eq!(circle.center, (0.0, 0.0));
    /// ```
    pub fn new(radius: f32) -> Self {
        Circle {
            center: (0.0, 0.0),
            radius,
        }
    }

    /// Creates a new Circle with the given radius and center point
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_maths::geometry::circle::Circle;
    ///
    /// let circle = Circle::with_center(5.0, (10.0, 20.0));
    /// assert_eq!(circle.radius, 5.0);
    /// assert_eq!(circle.center, (10.0, 20.0));
    /// ```
    pub fn with_center(radius: f32, center: (f32, f32)) -> Self {
        Circle { center, radius }
    }
}

impl Diameter for Circle {
    ///Returns this circles diameter
    fn diameter(&self) -> f32 {
        2.0 * &self.radius
    }
}

impl Area for Circle {
    ///Returns this circles area
    fn area(&self) -> f32 {
        self.radius * self.radius * PI
    }
}

impl Circumference for Circle {
    ///Returns this circles circumference
    fn circumference(&self) -> f32 {
        2.0 * self.radius * PI
    }
}

impl ArcLengthRadians for Circle {
    ///Returns the length of an arc on this circle with the given angle in radians
    fn arc_len_rad(&self, rads: f32) -> f32 {
        self.radius * rads
    }
}

impl ArcLengthDegrees for Circle {
    ///Returns the length of an arc on this circle with the given angle in degrees
    fn arc_len_deg(&self, degrees: f32) -> f32 {
        self.circumference() * (degrees / 360.0)
    }
}

impl SectorAreaDegrees for Circle {
    ///Returns the area of a sector in this circle with the given angle in degrees
    fn sect_area_deg(&self, degrees: f32) -> f32 {
        (self.radius * self.radius * deg_to_rad(degrees)) / 2.0
    }
}

impl SectorAreaRadians for Circle {
    ///Returns the area of a sector in this circle with the given angle in radians
    fn sect_area_rad(&self, rads: f32) -> f32 {
        (self.radius * self.radius * rads) / 2.0
    }
}

///Converts degrees to radians
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

///Converts radians to degrees
pub fn rad_to_deg(rads: f32) -> f32 {
    rads * (180.0 / PI)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_test() {
        let circle = Circle {
            center: (1.0, 1.0),
            radius: 1.0,
        };
        assert_eq!(circle.area(), 1.0 * 1.0 * PI);
    }

    #[test]
    fn circumference_test() {
        let circle = Circle {
            center: (1.0, 1.0),
            radius: 1.0,
        };
        assert_eq!(circle.circumference(), 2.0 * 1.0 * PI);
    }

    #[test]
    fn arc_len_deg_test() {
        let circle = Circle {
            center: (1.0, 1.0),
            radius: 1.0,
        };
        assert_eq!(
            circle.arc_len_deg(75.0),
            circle.circumference() * (75.0 / 360.0)
        )
    }

    #[test]
    fn arc_len_rad_test() {
        let circle = Circle {
            center: (1.0, 1.0),
            radius: 1.0,
        };
        assert_eq!(circle.arc_len_rad(PI / 4.0), 1.0 * PI / 4.0);
    }

    #[test]
    fn arc_len_compare_test() {
        let circle = Circle {
            center: (1.0, 1.0),
            radius: 1.0,
        };
        assert_eq!(circle.arc_len_deg(30.0), circle.arc_len_rad(PI / 6.0));
        assert_eq!(circle.arc_len_deg(180.0), circle.arc_len_rad(PI));
    }

    #[test]
    fn sect_area_deg_test() {
        let circle = Circle {
            center: (1.0, 1.0),
            radius: 1.0,
        };
        assert_eq!(circle.sect_area_deg(30.0), deg_to_rad(30.0) / 2.0);
    }

    #[test]
    fn sect_area_rad_test() {
        let circle = Circle {
            center: (1.0, 1.0),
            radius: 1.0,
        };
        assert_eq!(circle.sect_area_rad(PI / 6.0), (PI / 6.0) / 2.0);
    }

    #[test]
    fn sect_area_compare_test() {
        let circle = Circle {
            center: (1.0, 1.0),
            radius: 1.0,
        };
        assert_eq!(circle.sect_area_rad(PI / 6.0), circle.sect_area_deg(30.0));
    }
}
