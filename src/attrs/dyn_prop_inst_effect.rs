use godot::classes::RefCounted;
use godot::prelude::*;

use rust_engine_frame::attrs::dyn_prop_inst_effect::DynPropInstEffect as Inner;
use rust_engine_frame::effects::duration_effect::EffectBuilder;
use rust_engine_frame::effects::native_effect::ProxyEffect;

use crate::adapter::fixed_name_wrapper::FixedNameWrapper;

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct ExPropInstEffect {
    pub base: Base<RefCounted>,
    pub inner: Inner<FixedNameWrapper>,
}

#[godot_api]
impl ExPropInstEffect {
    #[func]
    fn create_val(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_val(EffectBuilder::new_instant(from_name, effect_name, value)),
        })
    }

    #[func]
    fn create_cur_per(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_cur_per(EffectBuilder::new_instant(from_name, effect_name, value)),
        })
    }

    #[func]
    fn create_max_per(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_max_per(EffectBuilder::new_instant(
                from_name,
                effect_name,
                value,
            )),
        })
    }

    // to copy start =================================================================================

    // impl for ProxyEffect SetterGetter

    #[func]
    fn get_effect_name(&self) -> StringName {
        self.inner.get_effect_name().into()
    }

    #[func]
    fn set_effect_name(&mut self, v: StringName) {
        self.inner.set_effect_name(FixedNameWrapper(v));
    }

    #[func]
    fn get_from_name(&self) -> StringName {
        self.inner.get_from_name().into()
    }

    #[func]
    fn set_from_name(&mut self, v: StringName) {
        self.inner.set_from_name(FixedNameWrapper(v));
    }

    #[func]
    fn get_value(&self) -> f64 {
        self.inner.get_value().into()
    }

    #[func]
    fn set_value(&mut self, v: f64) {
        self.inner.set_value(v);
    }

    // to copy final =================================================================================
}
