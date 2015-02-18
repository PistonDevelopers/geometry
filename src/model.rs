use range::{ AddTo, Range };
use wobj;
use quack::{ Action, Pair, SetAt };
use std::default::Default;

use Normal;
use TextureCoords;
use Position;
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
    pub fn add_model<T>(
        obj_set: &wobj::obj::ObjSet,
        vertices: &mut Vec<T>,
        indices: &mut Vec<u32>,
        geometries: &mut Geometry,
        objects: &mut Object,
    ) -> Range<AddTo<Model>>
        where
            T: Default,
            (Position, T): Pair<Data = Position, Object = T> + SetAt,
            (TextureCoords, T): Pair<Data = TextureCoords, Object = T> + SetAt,
            (Normal, T): Pair<Data = Normal, Object = T> + SetAt
    {
        let start = objects.0.len();
        for obj in obj_set.objects.iter() {
            objects.action(Object::add_object(
                obj,
                vertices,
                indices,
                geometries
            ));
        }
        let n = objects.0.len() - start;
        Range::new(start, n)
    }
}

quack! {
    obj: Model[]
    get:
    set:
    action:
        fn (range: Range<AddTo<Model>>) -> () [] {
            obj.0.push(range.cast())
        }
}
