use std::fmt::{Debug, Display, Formatter};
use bevy::prelude::Component;

// Component Definitions
#[derive(Component)]
pub struct DebugName(pub String);


#[derive(Component)]
pub struct FunnyCircle(pub f32, pub f32);


// Component Implementations
impl DebugName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl Debug for DebugName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_str(format!("DebugName(\"{}\")", self.0).as_str());
    }
}

impl Display for DebugName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_str(self.0.as_str());
    }
}