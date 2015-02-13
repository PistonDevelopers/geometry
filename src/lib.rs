#![deny(missing_docs)]

//! A structure for storing and look up 3D geometry

extern crate range;

use range::{ Range, ParentRange };

/// A geometry range, pointing to triangle list `u32` index buffer.
pub struct GeometryRange(Range);

impl ParentRange for GeometryRange {
    type Child = u32;

    fn from_range(range: Range) -> Self { GeometryRange(range) }
    fn range(&self) -> &Range { &self.0 }
    fn range_mut(&mut self) -> &mut Range { &mut self.0 }
}

/// An object range, pointing to a geometry range buffer,
/// which points to a triangle list index `I` buffer.
pub struct ObjectRange(Range);

impl ParentRange for ObjectRange {
    type Child = GeometryRange;

    fn from_range(range: Range) -> Self { ObjectRange(range) }
    fn range(&self) -> &Range { &self.0 }
    fn range_mut(&mut self) -> &mut Range { &mut self.0 }
}

