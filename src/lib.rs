#![deny(missing_docs)]

//! A structure for storing and look up 3D geometry

extern crate range;
extern crate "wavefront_obj" as wobj;
#[macro_use]
extern crate quack;

pub use geometry::Geometry;
pub use object::Object;
pub use model::Model;

mod geometry;
mod object;
mod model;

/// Property type for position.
pub struct Position(pub [f32; 3]);

/// Property type for texture coordinates.
pub struct TextureCoords(pub [f32; 2]);

/// Property type for normal.
pub struct Normal(pub [f32; 3]);

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
