use range::{ Range, ParentRange };
use wobj;
use quack::{ Pair, SetAt };
use std::default::Default;

use Position;
use TextureCoords;
use Normal;
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

impl Object {
    /// Adds a new object.
    pub fn add_object<T>(
        obj: &wobj::obj::Object,
        vertices: &mut Vec<T>,
        indices: &mut Vec<u32>,
        geometries: &mut Vec<Geometry>
    ) -> Object
        where
            T: Default,
            (Position, T): Pair<Data = Position, Object = T> + SetAt,
            (TextureCoords, T): Pair<Data = TextureCoords, Object = T> + SetAt,
            (Normal, T): Pair<Data = Normal, Object = T> + SetAt
    {
        let start = geometries.len();
        for geom in obj.geometry.iter() {
            // TODO: How to deal with vertex formats?
            let (geometry, _) = Geometry::add_geometry(
                    geom, obj, vertices, indices
                );
            geometries.push(geometry);
        }
        let n = geometries.len() - start;
        Object(Range::new(start, n))
    }
}
