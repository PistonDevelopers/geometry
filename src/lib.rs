#![deny(missing_docs)]

//! A structure for storing and look up 3D geometry

extern crate range;

use range::{ Range, ParentRange };

/// A geometry range, pointing to triangle list index buffer.
pub struct GeometryRange<I>(Range);

impl<I> ParentRange for GeometryRange<I> {
    type Child = I;

    fn from_range(range: Range) -> Self { GeometryRange(range) }
    fn range(&self) -> &Range { &self.0 }
    fn range_mut(&mut self) -> &mut Range { &mut self.0 }
}

