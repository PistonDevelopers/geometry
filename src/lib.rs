#![deny(missing_docs)]

//! A structure for storing and look up 3D geometry

extern crate range;
extern crate "wavefront_obj" as wobj;

pub use geometry::Geometry;
pub use object::Object;
pub use model::Model;

mod geometry;
mod object;
mod model;
