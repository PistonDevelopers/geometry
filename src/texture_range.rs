use range::Range;
use wobj;

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
