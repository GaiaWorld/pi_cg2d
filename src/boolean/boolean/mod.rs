use crate::geo2d::{Polygon, Rectangle};
use num_traits::Float;
use parry2d::math::{Real, Point};
pub mod compare_segments;
pub mod compute_fields;
mod connect_edges;
mod divide_segment;
pub mod fill_queue;
mod helper;
pub mod possible_intersection;
mod segment_intersection;
mod signed_area;
pub mod subdivide_segments;
pub mod sweep_event;

use self::connect_edges::connect_edges;
use self::fill_queue::fill_queue;
use self::subdivide_segments::subdivide;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operation {
    Intersection,
    Difference,
    Union,
    Xor,
}

pub trait BooleanOp<Rhs = Self>{
    fn boolean(&self, rhs: &Rhs, operation: Operation) -> Vec<Polygon>;

    fn intersection(&self, rhs: &Rhs) -> Vec<Polygon> {
        self.boolean(rhs, Operation::Intersection)
    }

    fn difference(&self, rhs: &Rhs) -> Vec<Polygon> {
        self.boolean(rhs, Operation::Difference)
    }

    fn union(&self, rhs: &Rhs) -> Vec<Polygon> {
        self.boolean(rhs, Operation::Union)
    }

    fn xor(&self, rhs: &Rhs) -> Vec<Polygon> {
        self.boolean(rhs, Operation::Xor)
    }
}

impl BooleanOp for Polygon{
    fn boolean(&self, rhs: &Polygon, operation: Operation) -> Vec<Polygon> {
        boolean_operation(&[self.clone()], &[rhs.clone()], operation)
    }
}

impl BooleanOp<Vec<Polygon>> for Polygon {
    fn boolean(&self, rhs: &Vec<Polygon>, operation: Operation) -> Vec<Polygon> {
        boolean_operation(&[self.clone()], rhs.as_slice(), operation)
    }
}

impl BooleanOp for Vec<Polygon>
{
    fn boolean(&self, rhs: &Vec<Polygon>, operation: Operation) -> Vec<Polygon> {
        boolean_operation(self.as_slice(), rhs.as_slice(), operation)
    }
}

impl BooleanOp<Polygon> for Vec<Polygon>
{
    fn boolean(&self, rhs: &Polygon, operation: Operation) -> Vec<Polygon> {
        boolean_operation(self.as_slice(), &[rhs.clone()], operation)
    }
}

fn boolean_operation(
    subject: &[Polygon],
    clipping: &[Polygon],
    operation: Operation,
) -> Vec<Polygon>
{
    let mut sbbox = Rectangle {
        mins: Point::new(Real::infinity(), Real::infinity()),
        maxs: Point::new(Real::neg_infinity(), Real::neg_infinity()),
    };
    let mut cbbox = sbbox;

    let mut event_queue = fill_queue(subject, clipping, &mut sbbox, &mut cbbox, operation);

    if sbbox.mins.x > cbbox.maxs.x
        || cbbox.mins.x > sbbox.maxs.x
        || sbbox.mins.y > cbbox.maxs.y
        || cbbox.mins.y > sbbox.maxs.y
    {
        return trivial_result(subject, clipping, operation);
    }

    let sorted_events = subdivide(&mut event_queue, &sbbox, &cbbox, operation);

    connect_edges(&sorted_events, operation)
}

fn trivial_result(
    subject: &[Polygon],
    clipping: &[Polygon],
    operation: Operation,
) -> Vec<Polygon>
{
    match operation {
        Operation::Intersection => vec![],
        Operation::Difference => Vec::from(subject),
        Operation::Union | Operation::Xor => subject.iter().chain(clipping).cloned().collect(),
    }
}
