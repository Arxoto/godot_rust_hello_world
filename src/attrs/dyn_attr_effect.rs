use godot::classes::RefCounted;
use godot::prelude::*;

use rust_engine_frame::attrs::dyn_attr_effect::DynAttrEffect as Inner;
use rust_engine_frame::effects::duration_effect::EffectBuilder;
use rust_engine_frame::effects::native_duration::ProxyDuration;
use rust_engine_frame::effects::native_effect::ProxyEffect;

use crate::adapter::fixed_name_wrapper::FixedNameWrapper;

#[derive(Debug, GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct DynAttrEffect {
    pub base: Base<RefCounted>,
    pub inner: Inner<FixedNameWrapper>,
}

#[godot_api]
impl DynAttrEffect {
    #[func]
    fn create_inf_basic_add(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_basic_add(EffectBuilder::new_infinite(from_name, effect_name, value)),
        })
    }
    #[func]
    fn create_inf_final_multi(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_final_multi(EffectBuilder::new_infinite(
                from_name,
                effect_name,
                value,
            )),
        })
    }
    #[func]
    fn create_inf_basic_percent(
        from_name: StringName,
        effect_name: StringName,
        value: f64,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_basic_percent(EffectBuilder::new_infinite(
                from_name,
                effect_name,
                value,
            )),
        })
    }
    #[func]
    fn create_inf_final_percent(
        from_name: StringName,
        effect_name: StringName,
        value: f64,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_final_percent(EffectBuilder::new_infinite(
                from_name,
                effect_name,
                value,
            )),
        })
    }

    #[func]
    fn create_dur_basic_add(
        from_name: StringName,
        effect_name: StringName,
        value: f64,
        duration_time: f64,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_basic_add(EffectBuilder::new_duration(
                from_name,
                effect_name,
                value,
                duration_time,
            )),
        })
    }
    #[func]
    fn create_dur_final_multi(
        from_name: StringName,
        effect_name: StringName,
        value: f64,
        duration_time: f64,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_final_multi(EffectBuilder::new_duration(
                from_name,
                effect_name,
                value,
                duration_time,
            )),
        })
    }
    #[func]
    fn create_dur_basic_percent(
        from_name: StringName,
        effect_name: StringName,
        value: f64,
        duration_time: f64,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_basic_percent(EffectBuilder::new_duration(
                from_name,
                effect_name,
                value,
                duration_time,
            )),
        })
    }
    #[func]
    fn create_dur_final_percent(
        from_name: StringName,
        effect_name: StringName,
        value: f64,
        duration_time: f64,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_final_percent(EffectBuilder::new_duration(
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
