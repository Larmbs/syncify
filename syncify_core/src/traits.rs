//! Module which defines core trait that allows objects to be in sync
use serde::{Deserialize, Serialize};

/// A trait that certifies whether an object meets sync requirements
pub trait Syncable: Serialize + for<'a> Deserialize<'a> {}
impl<T> Syncable for T where T: Serialize + for<'a> Deserialize<'a> {}
