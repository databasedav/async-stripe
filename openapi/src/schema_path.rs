use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

use heck::SnakeCase;

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ComponentPath(String);

impl ComponentPath {
    pub fn new(path: &str) -> Self {
        let path = path.replace('.', "_");
        debug_assert!(path.to_snake_case() == path, "Path should be snakecase");
        Self(path)
    }

    pub fn from_reference(reference: &str) -> Self {
        let root_obj = reference.trim_start_matches("#/components/schemas/");
        Self::new(root_obj)
    }
}

// This is a bit silly...just done because `petgraph` prints graph labels using `Debug` so this
// is a hack to have the labels print more concisely while still using our `ComponentPath` newtype
impl Debug for ComponentPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ComponentPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ComponentPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
