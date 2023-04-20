use super::sweep_event::{EdgeType, SweepEvent};
use super::Operation;
use std::rc::Rc;

pub fn compute_fields(event: &Rc<SweepEvent>, maybe_prev: Option<&Rc<SweepEvent>>, operation: Operation)
{
    if let Some(prev) = maybe_prev {
        if event.is_subject == prev.is_subject {
            event.set_in_out(!prev.is_in_out(), prev.is_other_in_out());
        } else if prev.is_vertical() {
            event.set_in_out(!prev.is_other_in_out(), !prev.is_in_out());
        } else {
            event.set_in_out(!prev.is_other_in_out(), prev.is_in_out());
        }
    } else {
        event.set_in_out(false, true);
    }

    event.set_in_result(in_result(event, operation));
}

fn in_result(event: &SweepEvent, operation: Operation) -> bool
{
    match event.get_edge_type() {
        EdgeType::Normal => match operation {
            Operation::Intersection => !event.is_other_in_out(),
            Operation::Union => event.is_other_in_out(),
            Operation::Difference => {
                (event.is_subject && event.is_other_in_out()) || (!event.is_subject && !event.is_other_in_out())
            }
            Operation::Xor => true,
        },
        EdgeType::SameTransition => operation == Operation::Intersection || operation == Operation::Union,
        EdgeType::DifferentTransition => operation == Operation::Difference,
        EdgeType::NonContributing => false,
    }
}
