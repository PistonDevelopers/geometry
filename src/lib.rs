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

/// Used to convert indices from a format to indices in an index buffer.
/// The range of these indices becomes a `Geometry` object.
/// Is only used temporarily and not stored as part of scene geometry.
pub struct TextureRange<T>(pub Range);

impl<T> TextureRange<T> {
    /// Adds UVs to array from Wavefront OBJ format.
    pub fn add_wobj_uvs<F>(
        obj: &wobj::obj::Object,
        uvs: &mut Vec<T>,
        mut f: F,
    ) -> TextureRange<T>
        where
            F: FnMut([f32; 2]) -> T,
    {
        let start = uvs.len();
        let n = obj.tex_vertices.len();
        uvs.reserve(n);
        for v in obj.tex_vertices.iter() {
            uvs.push(f([v.x as f32, v.y as f32]));
        }
        TextureRange(Range::new(start, n))
    }
}

/// Used to convert indices from a format to indices in an index buffer.
/// The range of these indices becomes a `Geometry` object.
/// Is only used temporarily and not stored as part of scene geometry.
pub struct NormalRange<T>(pub Range);

impl<T> NormalRange<T> {
    /// Adds normals to array from Wavefront OBJ format.
    pub fn add_wobj_normals<F>(
        obj: &wobj::obj::Object,
        normals: &mut Vec<T>,
        mut f: F,
    ) -> NormalRange<T>
        where
            F: FnMut([f32; 3]) -> T,
    {
        let start = normals.len();
        let n = obj.normals.len();
        normals.reserve(n);
        for v in obj.normals.iter() {
            normals.push(f([v.x as f32, v.y as f32, v.z as f32]));
        }
        NormalRange(Range::new(start, n))
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

impl Geometry {
    /// Adds geometry from Wavefront OBJ format to index buffer.
    pub fn add_wobj_indices<T>(
        geom: &wobj::obj::Geometry,
        vertex_range: &VertexRange<T>,
        indices: &mut Vec<u32>
    ) -> Geometry {
        let offset = vertex_range.0.offset;
        let start = indices.len();
        for shape in geom.shapes.iter() {
            match shape {
                // Extract triangles and offset them relative
                // to the position in the index buffer.
                &wobj::obj::Shape::Triangle((a, _, _), (b, _, _), (c, _, _)) => {
                    indices.push((a + offset) as u32);
                    indices.push((b + offset) as u32);
                    indices.push((c + offset) as u32);
                }
                _ => {}
            }
        }
        Geometry(Range::new(start, indices.len() - start))
    }
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

