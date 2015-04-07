#![deny(missing_docs)]

//! A structure for storing and look up 3D geometry

extern crate range;
extern crate wavefront_obj as wobj;

pub use geometry::Geometry;
pub use object::Object;
pub use model::Model;

mod geometry;
mod object;
mod model;

/// Implemented by vertex types.
pub trait Vertex {
    /// Sets position.
    fn set_position(&mut self, [f32; 3]);
    /// Sets texture coords.
    fn set_texture_coords(&mut self, [f32; 2]);
    /// Sets normal.
    fn set_normal(&mut self, [f32; 3]);
}

/// Description of vertex format.
pub enum VertexFormat {
    /// Vertex contains position coordinates.
    Position,
    /// Vertex contains position and texture coordinates.
    PositionTexture,
    /// Vertex contains position and normal coordinates.
    PositionNormal,
    /// Vertex contains both position, texture and normal coordinates.
    PositionTextureNormal,
}

/// An error with vertex format when adding to index and vertex buffer.
#[derive(Debug)]
pub enum VertexFormatError {
    /// The vertex format must be the same for an entire geometry object.
    ExpectedSameVertexFormatPerGeometry,
}
