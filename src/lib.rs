#![deny(missing_docs)]

//! A structure for storing and look up 3D geometry

extern crate range;
extern crate "wavefront_obj" as wobj;

use range::{ Range, ParentRange };

/// Used to convert indices from a format to indices in an index buffer.
/// The range of these indices becomes a `Geometry` object.
/// Is only used temporarily and not stored as part of scene geometry.
pub struct VertexRange<T>(pub Range);

impl<T> VertexRange<T> {
    /// Adds vertices to array from Wavefront OBJ format.
    pub fn add_wobj_vertices<F>(
        obj: &wobj::obj::Object,
        vertices: &mut Vec<T>,
        mut f: F,
    ) -> VertexRange<T>
        where
            F: FnMut([f32; 3]) -> T,
    {
        let start = vertices.len();
        let n = obj.vertices.len();
        vertices.reserve(n);
        for v in obj.vertices.iter() {
            vertices.push(f([v.x as f32, v.y as f32, v.z as f32]));
        }
        VertexRange(Range::new(start, n))
    }
}

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

