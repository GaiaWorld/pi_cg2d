use super::compare_segments::compare_segments;
use super::compute_fields::compute_fields;
use super::possible_intersection::possible_intersection;
use super::sweep_event::SweepEvent;
use super::Operation;
use crate::boolean::splay::SplaySet;
use crate::geo2d::Rectangle;
use pi_heap::simple_heap::SimpleHeap;
use std::rc::Rc;

pub fn subdivide(
    event_queue: &mut SimpleHeap<Rc<SweepEvent>>,
    sbbox: &Rectangle,
    cbbox: &Rectangle,
    operation: Operation,
) -> Vec<Rc<SweepEvent>>
{
    let mut sweep_line = SplaySet::<Rc<SweepEvent>, _>::new(compare_segments);
    let mut sorted_events: Vec<Rc<SweepEvent>> = Vec::new();
    let rightbound = sbbox.maxs.x.min(cbbox.maxs.x);

    while let Some(event) = event_queue.pop() {
        sorted_events.push(event.clone());

        if operation == Operation::Intersection && event.point.x > rightbound
            || operation == Operation::Difference && event.point.x > sbbox.maxs.x
        {
            break;
        }

        if event.is_left() {
            sweep_line.insert(event.clone());

            let maybe_prev = sweep_line.prev(&event);
            let maybe_next = sweep_line.next(&event);

            compute_fields(&event, maybe_prev, operation);

            if let Some(next) = maybe_next {
                if possible_intersection(&event, &next, event_queue) == 2 {
                    compute_fields(&event, maybe_prev, operation);
                    compute_fields(&event, Some(next), operation);
                }
            }

            if let Some(prev) = maybe_prev {
                if possible_intersection(&prev, &event, event_queue) == 2 {
                    let maybe_prev_prev = sweep_line.prev(&prev);

                    compute_fields(&prev, maybe_prev_prev, operation);
                    compute_fields(&event, Some(prev), operation);
                }
            }
        } else if let Some(other_event) = event.get_other_event() {
            if sweep_line.contains(&other_event) {
                let maybe_prev = sweep_line.prev(&other_event).cloned();
                let maybe_next = sweep_line.next(&other_event).cloned();

                if let (Some(prev), Some(next)) = (maybe_prev, maybe_next) {
                    possible_intersection(&prev, &next, event_queue);
                }

                sweep_line.remove(&other_event);
            }
        }
    }

    sorted_events
}
