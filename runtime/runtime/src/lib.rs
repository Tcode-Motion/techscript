//! # TechScript Runtime Crate
//!
//! Provides values, environments, scopes, closures, objects, contexts, configurations,
//! and native registry functions for the TechScript 2.0 language execution backends.

pub mod collections;
pub mod context;
pub mod environment;
pub mod error;
pub mod function;
pub mod native_function;
pub mod object;
pub mod value;

pub use collections::{list_get, list_set, map_get, map_set};
pub use context::{RuntimeConfig, RuntimeContext};
pub use environment::{Binding, Environment};
pub use error::{RuntimeError, RuntimeErrorKind};
pub use function::{Callable, FunctionBody, UserFunction};
pub use native_function::NativeRegistry;
pub use object::{ModelInstance, ObjectId, StructInstance};
pub use value::{DslBlockValue, DslProperty, RuntimeType, RuntimeValue};
