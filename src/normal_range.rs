use range::Range;
use wobj;

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
