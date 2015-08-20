use range::Range;
use wobj;
use std::default::Default;

use AddTo;
use Vertex;
use Geometry;

/// An object consists of a list of geometries.
/// The geometries are stored separately,
/// in a geometry buffer.
pub struct Object(pub Vec<Range>);

impl Object {
    /// Creates a new object layer.
    pub fn new() -> Object {
        Object(Vec::new())
    }

    /// Adds a new object.
    pub fn new_object<T>(
        obj: &wobj::obj::Object,
        vertices: &mut Vec<T>,
        indices: &mut Vec<u32>,
        geometries: &mut Geometry
    ) -> Range<AddTo<Object>>
        where T: Vertex + Default
    {
        let start = geometries.0.len();
        for geom in obj.geometry.iter() {
            // TODO: How to deal with vertex formats?
            let (add_range, _) = Geometry::new_geometry(
                    geom, obj, vertices, indices
                );
            geometries.add_range(add_range);
        }
        let n = geometries.0.len() - start;
        Range::new(start, n)
    }

    /// Adds new object.
    pub fn add_range(&mut self, range: Range<AddTo<Object>>) {
        self.0.push(range.cast())
    }
}
