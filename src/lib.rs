#![deny(missing_docs)]

//! A structure for storing and look up 3D geometry

extern crate range;
extern crate "wavefront_obj" as wobj;

pub use vertex_range::VertexRange;
pub use texture_range::TextureRange;
pub use normal_range::NormalRange;
pub use geometry::Geometry;
pub use object::Object;
pub use model::Model;

mod vertex_range;
mod texture_range;
mod normal_range;
mod geometry;
mod object;
mod model;
