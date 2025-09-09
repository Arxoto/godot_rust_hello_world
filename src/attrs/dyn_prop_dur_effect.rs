use godot::classes::RefCounted;
use godot::prelude::*;

use rust_engine_frame::attrs::dyn_prop_dur_effect::DynPropDurEffect as Inner;
use rust_engine_frame::effects::duration_effect::EffectBuilder;
use rust_engine_frame::effects::native_duration::ProxyDuration;
use rust_engine_frame::effects::native_effect::ProxyEffect;

use crate::adapter::fixed_name_wrapper::FixedNameWrapper;

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct DynPropDurEffect {
    pub base: Base<RefCounted>,
    pub inner: Inner<FixedNameWrapper>,
}

#[godot_api]
impl DynPropDurEffect {
    #[func]
    fn create_inf_max_val(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_max_val(EffectBuilder::new_infinite(from_name, effect_name, value)),
        })
    }

    #[func]
    fn create_inf_max_per(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_max_per(EffectBuilder::new_infinite(from_name, effect_name, value)),
        })
    }

    #[func]
    fn create_inf_min_val(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_min_val(EffectBuilder::new_infinite(from_name, effect_name, value)),
        })
    }

    #[func]
    fn create_dur_max_val(
        from_name: StringName,
        effect_name: StringName,
        value: f64,
        duration_time: f64,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_max_val(EffectBuilder::new_duration(
                from_name,
                effect_name,
                value,
                duration_time,
            )),
        })
    }

    #[func]
    fn create_dur_max_per(
        from_name: StringName,
        effect_name: StringName,
        value: f64,
        duration_time: f64,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_max_per(EffectBuilder::new_duration(
                from_name,
                effect_name,
                value,
                duration_time,
            )),
        })
    }

    #[func]
    fn create_dur_min_val(
        from_name: StringName,
        effect_name: StringName,
        value: f64,
        duration_time: f64,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_min_val(EffectBuilder::new_duration(
                from_name,
                effect_name,
                value,
                duration_time,
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

    // impl for ProxyDuration SetterGetter

    #[func]
    fn get_life_time(&self) -> f64 {
        self.inner.get_life_time()
    }

    #[func]
    fn set_life_time(&mut self, v: f64) {
        self.inner.set_life_time(v)
    }

    #[func]
    fn get_duration_time(&self) -> f64 {
        self.inner.get_duration_time()
    }

    #[func]
    fn set_duration_time(&mut self, v: f64) {
        self.inner.set_duration_time(v)
    }

    #[func]
    fn get_period_time(&self) -> f64 {
        self.inner.get_period_time()
    }

    #[func]
    fn set_period_time(&mut self, v: f64) {
        self.inner.set_period_time(v)
    }

    #[func]
    fn get_wait_time(&self) -> f64 {
        self.inner.get_wait_time()
    }

    #[func]
    fn set_wait_time(&mut self, v: f64) {
        self.inner.set_wait_time(v)
    }

    #[func]
    fn get_stack(&self) -> i64 {
        self.inner.get_stack()
    }

    #[func]
    fn set_stack(&mut self, v: i64) {
        self.inner.set_stack(v)
    }

    #[func]
    fn get_max_stack(&self) -> i64 {
        self.inner.get_max_stack()
    }

    #[func]
    fn set_max_stack(&mut self, v: i64) {
        self.inner.set_max_stack(v);
    }

    // to copy final =================================================================================
}
