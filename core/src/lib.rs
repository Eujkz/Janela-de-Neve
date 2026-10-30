//! The snow itself, free of any platform: the simulation, its drifts and the geometry they use.
//! The Mac app has its own in Swift, and the site its own in JavaScript.

pub mod geometry;
pub mod layer;
pub mod mod_config; // SNOWHUSH_MODKIT
pub mod simulation;
pub mod version;

#[cfg(test)]
mod tests;
