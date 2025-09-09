use godot::prelude::*;
use rust_engine_frame::cores::unify_type::FixedName;

/// same as [`FixedName`]
#[derive(PartialEq, Hash, Eq, Clone, Debug)]
pub struct FixedNameWrapper(pub StringName);

impl From<StringName> for FixedNameWrapper {
    fn from(value: StringName) -> Self {
        FixedNameWrapper(value)
    }
}

impl Into<StringName> for &FixedNameWrapper {
    fn into(self) -> StringName {
        self.0.clone()
    }
}

impl FixedName for FixedNameWrapper {}
