use range::Range;
use wobj;
use std::default::Default;

use AddTo;
use Vertex;
use Object;
use Geometry;

/// A model consists of a list of object.
/// The objects are stored separately,
/// in an object buffer.
pub struct Model(pub Vec<Range>);

impl Model {
    /// Creates a new model layer.
    pub fn new() -> Model {
        Model(Vec::new())
    }

    /// Adds a model.
    pub fn new_model<T>(
        obj_set: &wobj::obj::ObjSet,
        vertices: &mut Vec<T>,
        indices: &mut Vec<u32>,
        geometries: &mut Geometry,
        objects: &mut Object,
    ) -> Range<AddTo<Model>>
        where T: Vertex + Default
    {
        let start = objects.0.len();
        for obj in obj_set.objects.iter() {
            objects.add_range(Object::new_object(
                obj,
                vertices,
                indices,
                geometries
            ));
        }
        let n = objects.0.len() - start;
        Range::new(start, n)
    }

    /// Adds new model.
    pub fn add_range(&mut self, range: Range<AddTo<Model>>) {
        self.0.push(range.cast())
    }
}
