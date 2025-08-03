//! Core Smalltalk object system implementation
//! 
//! This module provides the fundamental building blocks for the Smalltalk object system,
//! including the base Object trait and object identity management.

pub mod object;
pub mod small_integer;

pub use object::*;
pub use small_integer::*;