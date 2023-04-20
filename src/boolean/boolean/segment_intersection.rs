use parry2d::math::{Point, Real};

#[derive(Debug, Clone, PartialEq)]
pub enum LineIntersection

{
    None,
    Point(Point<Real>),
    Overlap(Point<Real>, Point<Real>),
}

pub fn intersection(
    a1: Point<Real>,
    a2: Point<Real>,
    b1: Point<Real>,
    b2: Point<Real>,
) -> LineIntersection

{
    let va = Point::new(a2.x - a1.x, a2.y - a1.y);
    let vb = Point::new(b2.x - b1.x, b2.y - b1.y);
    let e = Point::new(b1.x - a1.x, b1.y - a1.y);
    let mut kross = cross_product(va, vb);
    let mut sqr_kross = kross * kross;
    let sqr_len_a = dot_product(va, va);

    if sqr_kross > 0.0 {
        let s = cross_product(e, vb) / kross;
        if s < 0.0 || s > 1.0 {
            return LineIntersection::None;
        }
        let t = cross_product(e, va) / kross;
        if t < 0.0 || t > 1.0 {
            return LineIntersection::None;
        }

        if s == 0.0 || s == 1.0 {
            return LineIntersection::Point(mid_point(a1, s, va));
        }
        if t == 0.0 || t == 1.0 {
            return LineIntersection::Point(mid_point(b1, t, vb));
        }

        return LineIntersection::Point(mid_point(a1, s, va));
    }

    kross = cross_product(e, va);
    sqr_kross = kross * kross;

    if sqr_kross > 0.0 {
        return LineIntersection::None;
    }

    let sa = dot_product(va, e) / sqr_len_a;
    let sb = sa + dot_product(va, vb) / sqr_len_a;
    let smin = sa.min(sb);
    let smax = sa.max(sb);

    if smin <= 1.0 && smax >= 0.0 {
        if smin == 1.0 {
            return LineIntersection::Point(mid_point(a1, smin, va));
        }
        if smax == 0.0 {
            return LineIntersection::Point(mid_point(a1, smax, va));
        }

        return LineIntersection::Overlap(
            mid_point(a1, smin.max(0.0), va),
            mid_point(a1, smax.min(1.0), va),
        );
    }

    LineIntersection::None
}

fn mid_point(p: Point<Real>, s: Real, d: Point<Real>) -> Point<Real>

{
    Point::new(p.x + s * d.x, p.y + s * d.y)
}

#[inline]
fn cross_product(a: Point<Real>, b: Point<Real>) -> Real

{
    a.x * b.y - a.y * b.x
}

#[inline]
fn dot_product(a: Point<Real>, b: Point<Real>) -> Real

{
    a.x * b.x + a.y * b.y
}

#[cfg(test)]
mod test {
    use crate::boolean::boolean::helper::test::xyf32;

    use super::super::helper::test::xy;
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 0), xy(2, 2)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 0), xy(10, 2)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(2, 2), xy(3, 3), xy(0, 6), xy(2, 4)),
            LineIntersection::None
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 0), xy(0, 1)),
            LineIntersection::Point(xyf32(0.5, 0.5))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, 1), xy(0, 0)),
            LineIntersection::Point(xy(0, 0))
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, 1), xy(1, 1)),
            LineIntersection::Point(xy(1, 1))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xyf32(0.5, 0.5), xy(1, 0)),
            LineIntersection::Point(xyf32(0.5, 0.5))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(10, 10), xy(1, 1), xy(5, 5)),
            LineIntersection::Overlap(xy(1, 1), xy(5, 5))
        );
        assert_eq!(
            intersection(xy(1, 1), xy(10, 10), xy(1, 1), xy(5, 5)),
            LineIntersection::Overlap(xy(1, 1), xy(5, 5))
        );
        assert_eq!(
            intersection(xy(3, 3), xy(10, 10), xy(0, 0), xy(5, 5)),
            LineIntersection::Overlap(xy(3, 3), xy(5, 5))
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, 0), xy(1, 1)),
            LineIntersection::Overlap(xy(0, 0), xy(1, 1))
        );
        assert_eq!(
            intersection(xy(1, 1), xy(0, 0), xy(0, 0), xy(1, 1)),
            LineIntersection::Overlap(xy(1, 1), xy(0, 0))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 1), xy(2, 2)),
            LineIntersection::Point(xy(1, 1))
        );
        assert_eq!(
            intersection(xy(1, 1), xy(0, 0), xy(1, 1), xy(2, 2)),
            LineIntersection::Point(xy(1, 1))
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(2, 2), xy(4, 4)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, -1), xy(1, 0)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(1, 1), xy(0, 0), xy(0, -1), xy(1, 0)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(0, -1), xy(1, 0), xy(0, 0), xy(1, 1)),
            LineIntersection::None
        );

        assert_eq!(
            intersection(xyf32(0.0, 0.5), xyf32(1.0, 1.5), xy(0, 1), xy(1, 0)),
            LineIntersection::Point(xyf32(0.25, 0.75))
        );
    }
}
