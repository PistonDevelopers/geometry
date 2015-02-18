use range::{ AddTo, Range };
use wobj;
use quack::{ Action, Pair, SetAt };
use std::default::Default;

use Position;
use TextureCoords;
use Normal;
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
    pub fn add_object<T>(
        obj: &wobj::obj::Object,
        vertices: &mut Vec<T>,
        indices: &mut Vec<u32>,
        geometries: &mut Geometry
    ) -> Range<AddTo<Object>>
        where
            T: Default,
            (Position, T): Pair<Data = Position, Object = T> + SetAt,
            (TextureCoords, T): Pair<Data = TextureCoords, Object = T> + SetAt,
            (Normal, T): Pair<Data = Normal, Object = T> + SetAt
    {
        let start = geometries.0.len();
        for geom in obj.geometry.iter() {
            // TODO: How to deal with vertex formats?
            let (add_range, _) = Geometry::add_geometry(
                    geom, obj, vertices, indices
                );
            geometries.action(add_range);
        }
        let n = geometries.0.len() - start;
        Range::new(start, n)
    }
}

quack! {
    obj: Object[]
    get:
    set:
    action:
        fn (range: Range<AddTo<Object>>) -> () [] {
            obj.0.push(range.cast())
        }
}
