use range::{ Range, ParentRange };
use wobj;
use quack::{ Pair, SetAt };
use std::default::Default;

use Normal;
use TextureCoords;
use Position;
use Object;
use Geometry;

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

impl Model {
    /// Adds a model.
    pub fn add_model<T>(
        obj_set: &wobj::obj::ObjSet,
        vertices: &mut Vec<T>,
        indices: &mut Vec<u32>,
        geometries: &mut Vec<Geometry>,
        objects: &mut Vec<Object>,
    ) -> Model
        where
            T: Default,
            (Position, T): Pair<Data = Position, Object = T> + SetAt,
            (TextureCoords, T): Pair<Data = TextureCoords, Object = T> + SetAt,
            (Normal, T): Pair<Data = Normal, Object = T> + SetAt
    {
        let start = objects.len();
        for obj in obj_set.objects.iter() {
            objects.push(Object::add_object(
                obj,
                vertices,
                indices,
                geometries
            ));
        }
        let n = objects.len() - start;
        Model(Range::new(start, n))
    }
}
