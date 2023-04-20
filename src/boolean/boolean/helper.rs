use std::cmp::Ordering;

#[inline]
pub fn less_if(condition: bool) -> Ordering {
    if condition {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[cfg(test)]
pub mod test {
    use parry2d::math::{Point, Real};

    pub fn xy(x: i32, y: i32) -> Point<Real> {
        Point::new(x as Real, y as Real)
    }

	pub fn xyf32(x: f32, y: f32) -> Point<Real> {
        Point::new(x as Real, y as Real)
    }
}
