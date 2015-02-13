#![deny(missing_docs)]

//! A structure for storing and look up 3D geometry

extern crate range;

use range::{ Range, ParentRange };

/// A geometry consists of a list of triangles.
/// The triangles are stored separately,
/// in a triangle list `u32` index buffer.
pub struct Geometry(Range);

impl ParentRange for Geometry {
    type Child = u32;

    fn from_range(range: Range) -> Self { Geometry(range) }
    fn range(&self) -> &Range { &self.0 }
    fn range_mut(&mut self) -> &mut Range { &mut self.0 }
}

/// An object consists of a list of geometries.
/// The geometries are stored separately,
/// in a geometry buffer.
pub struct Object(Range);

impl ParentRange for Object {
    type Child = Geometry;

    fn from_range(range: Range) -> Self { Object(range) }
    fn range(&self) -> &Range { &self.0 }
    fn range_mut(&mut self) -> &mut Range { &mut self.0 }
}

/// A model consists of a list of object.
/// The objects are stored separately,
/// in an object buffer.
pub struct Model(Range);

impl ParentRange for Model {
    type Child = Object;

    fn from_range(range: Range) -> Self { Model(range) }
    fn range(&self) -> &Range { &self.0 }
    fn range_mut(&mut self) -> &mut Range { &mut self.0 }
}

