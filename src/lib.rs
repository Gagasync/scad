pub struct Line {}
pub struct Arc { }
pub struct Circle {}
pub struct Ellipse {}
pub struct Point {}


mod base {
    pub struct Coordinate {
        x: f64,
        y: f64
    }

    impl Coordinate {
        pub fn new(x: f64, y: f64) -> Coordinate { Coordinate { x, y } }
        pub fn tranlate(x: f64, y: f64) -> Coordinate {
            unimplemented!()
        }
    }

    pub struct Area {
        min_pt: Coordinate,
        max_pt: Coordinate,
    }

    impl Area {

        pub fn from_values(x1: f64, y1: f64, x2: f64, y2: f64) -> Area
        {
            let min_x = x1.min(x2);
            let min_y = y1.min(y2);
            let max_x = x1.max(x2);
            let max_y = y1.max(y2);

            Area {
                min_pt: Coordinate::new(min_x, min_y),
                max_pt: Coordinate::new(max_x, max_y)
            }
        }

        pub fn intersects(&self, other: &Area) -> bool { unimplemented!() }
        pub fn intersection(&self, other: &Area) -> Area { unimplemented!() }
        pub fn union(&self, other: &Area) -> Area { unimplemented!() }

    }
}