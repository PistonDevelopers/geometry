use range::{ Range, ParentRange };

use Geometry;

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
