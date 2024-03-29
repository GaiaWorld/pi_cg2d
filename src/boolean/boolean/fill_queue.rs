use crate::geo2d::{Polygon, PolygonIter, Rectangle};
use pi_heap::simple_heap::SimpleHeap;
use num_traits::Float;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};

use super::sweep_event::SweepEvent;
use super::Operation;

pub fn fill_queue(
    subject: &[Polygon],
    clipping: &[Polygon],
    sbbox: &mut Rectangle,
    cbbox: &mut Rectangle,
    operation: Operation,
) -> SimpleHeap<Rc<SweepEvent>> {
    let mut event_queue: SimpleHeap<Rc<SweepEvent>> = SimpleHeap::new(Ordering::Greater);
    let mut contour_id = 0u32;

    for polygon in subject {
        contour_id += 1;
        process_polygon(
            polygon.exterior(),
            true,
            contour_id,
            &mut event_queue,
            sbbox,
            true,
        );
        for i in 0..polygon.hole_num() {
            process_polygon(
                polygon.hole(i),
                true,
                contour_id,
                &mut event_queue,
                sbbox,
                false,
            );
        }
    }

    for polygon in clipping {
        let exterior = operation != Operation::Difference;
        if exterior {
            contour_id += 1;
        }
        process_polygon(
            polygon.exterior(),
            false,
            contour_id,
            &mut event_queue,
            cbbox,
            exterior,
        );
        for i in 0..polygon.hole_num() {
            process_polygon(
                polygon.hole(i),
                false,
                contour_id,
                &mut event_queue,
                cbbox,
                false,
            );
        }
    }

    event_queue
}

fn process_polygon<>(
    contour_or_hole: PolygonIter,
    is_subject: bool,
    contour_id: u32,
    event_queue: &mut SimpleHeap<Rc<SweepEvent>>,
    bbox: &mut Rectangle,
    is_exterior_ring: bool,
) {
    for (start, end) in contour_or_hole.lines() {
        if start == end {
            continue; // skip collapsed edges
        }

        let e1 = SweepEvent::new_rc(
            contour_id,
            start.clone(),
            false,
            Weak::new(),
            is_subject,
            is_exterior_ring,
        );
        let e2 = SweepEvent::new_rc(
            contour_id,
            end.clone(),
            false,
            Rc::downgrade(&e1),
            is_subject,
            is_exterior_ring,
        );
        e1.set_other_event(&e2);

        if e1 < e2 {
            e2.set_left(true)
        } else {
            e1.set_left(true)
        }

        bbox.mins.x = Float::min(bbox.mins.x, start.x);
        bbox.mins.y = Float::min(bbox.mins.y, start.y);
        bbox.maxs.x = Float::min(bbox.maxs.x, start.x);
        bbox.maxs.y = Float::min(bbox.maxs.y, start.y);

        event_queue.push(e1);
        event_queue.push(e2);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pi_heap::simple_heap::SimpleHeap;
    use parry2d::math::{Point, Real};
    use std::cmp::Ordering;
    use std::rc::{Rc, Weak};

    fn make_simple(x: Real, y: Real, is_subject: bool) -> Rc<SweepEvent> {
        SweepEvent::new_rc(0, Point::new(x, y), false, Weak::new(), is_subject, true)
    }

    fn check_order_in_queue(first: Rc<SweepEvent>, second: Rc<SweepEvent>) {
        let mut queue: SimpleHeap<Rc<SweepEvent>> = SimpleHeap::new(Ordering::Greater);

        assert_eq!(first.cmp(&second), Ordering::Greater);
        assert_eq!(second.cmp(&first), Ordering::Less);
        {
            queue.push(first.clone());
            queue.push(second.clone());

            let p1 = queue.pop().unwrap();
            let p2 = queue.pop().unwrap();

            assert!(Rc::ptr_eq(&first, &p1));
            assert!(Rc::ptr_eq(&second, &p2));
        }
        {
            queue.push(second.clone());
            queue.push(first.clone());

            let p1 = queue.pop().unwrap();
            let p2 = queue.pop().unwrap();

            assert!(Rc::ptr_eq(&first, &p1));
            assert!(Rc::ptr_eq(&second, &p2));
        }
    }

    #[test]
    fn test_least_by_x() {
        check_order_in_queue(make_simple(0.0, 0.0, false), make_simple(0.5, 0.5, false))
    }

    #[test]
    fn test_least_by_y() {
        check_order_in_queue(make_simple(0.0, 0.0, false), make_simple(0.0, 0.5, false))
    }

    #[test]
    fn test_least_left() {
        let e1 = make_simple(0.0, 0.0, false);
        e1.set_left(true);
        let e2 = make_simple(0.0, 0.0, false);
        e2.set_left(false);

        check_order_in_queue(e2, e1)
    }

    #[test]
    fn test_shared_edge_not_colinear() {
        let other_e1 = make_simple(1.0, 1.0, false);
        let e1 = make_simple(0.0, 0.0, false);
        e1.set_other_event(&other_e1);
        e1.set_left(true);
        let other_e2 = make_simple(2.0, 3.0, false);
        let e2 = make_simple(0.0, 0.0, false);
        e2.set_other_event(&other_e2);
        e2.set_left(true);

        check_order_in_queue(e1, e2)
    }

    #[test]
    fn test_collinear_edges() {
        let other_e1 = make_simple(1.0, 1.0, true);
        let e1 = make_simple(0.0, 0.0, true);
        e1.set_other_event(&other_e1);
        e1.set_left(true);
        let other_e2 = make_simple(2.0, 2.0, false);
        let e2 = make_simple(0.0, 0.0, false);
        e2.set_other_event(&other_e2);
        e2.set_left(true);

        check_order_in_queue(e1, e2)
    }
}
