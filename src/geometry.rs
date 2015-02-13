use range::{ Range, ParentRange };
use wobj;

use VertexRange;

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
