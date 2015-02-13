use range::Range;
use wobj;

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
