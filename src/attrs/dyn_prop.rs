use godot::classes::RefCounted;
use godot::prelude::*;

use rust_engine_frame::attrs::dyn_prop::DynProp as Inner;

use crate::adapter::fixed_name_wrapper::FixedNameWrapper;
use crate::attrs::dyn_attr_effect::ExAttrEffect;
use crate::attrs::dyn_prop_dur_effect::ExPropDurEffect;
use crate::attrs::dyn_prop_inst_effect::ExPropInstEffect;
use crate::attrs::dyn_prop_period_effect::ExPropPeriodEffect;

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct ExProp {
    pub base: Base<RefCounted>,
    pub inner: Inner<FixedNameWrapper>,
}

#[godot_api]
impl ExProp {
    #[func]
    fn create(v: f64, the_max: f64, the_min: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new(v, the_max, the_min),
        })
    }

    #[func]
    fn create_by_max(v: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_by_max(v),
        })
    }

    #[func]
    pub fn get_current(&self) -> f64 {
        self.inner.get_current()
    }

    #[func]
    pub fn get_max(&self) -> f64 {
        self.inner.get_max()
    }

    #[func]
    pub fn get_min(&self) -> f64 {
        self.inner.get_min()
    }

    #[func]
    pub fn use_inst_effect(&mut self, e: Gd<ExPropInstEffect>) -> f64 {
        let alter_result = self.inner.use_inst_effect(e.bind().inner.clone());
        alter_result.delta
    }

    #[func]
    pub fn refresh_value(&mut self) {
        self.inner.refresh_value();
    }

    #[func]
    pub fn put_dur_effect(&mut self, e: Gd<ExPropDurEffect>) {
        self.inner.put_dur_effect(e.bind().inner.clone());
    }

    #[func]
    pub fn del_dur_effect(&mut self, s: StringName) {
        self.inner.del_dur_effect(&FixedNameWrapper(s));
    }

    #[func]
    pub fn get_max_dur_effect_names(&self) -> Array<StringName> {
        self.inner
            .get_max_dur_effect_names()
            .into_iter()
            .map(|s| s.0)
            .collect()
    }

    #[func]
    pub fn get_max_dur_effect_by_name(&self, s: StringName) -> Option<Gd<ExAttrEffect>> {
        if let Some(eff) = self.inner.get_max_dur_effect_by_name(&FixedNameWrapper(s)) {
            let outer = Gd::from_init_fn(|base| ExAttrEffect { base, inner: eff });
            Some(outer)
        } else {
            None
        }
    }

    #[func]
    pub fn get_min_dur_effect_names(&self) -> Array<StringName> {
        self.inner
            .get_min_dur_effect_names()
            .into_iter()
            .map(|s| s.0)
            .collect()
    }

    #[func]
    pub fn get_min_dur_effect_by_name(&self, s: StringName) -> Option<Gd<ExAttrEffect>> {
        if let Some(eff) = self.inner.get_min_dur_effect_by_name(&FixedNameWrapper(s)) {
            let outer = Gd::from_init_fn(|base| ExAttrEffect { base, inner: eff });
            Some(outer)
        } else {
            None
        }
    }

    #[func]
    pub fn do_put_dur_effect(&mut self, e: Gd<ExPropDurEffect>) {
        self.inner.do_put_dur_effect(e.bind().inner.clone());
    }

    #[func]
    pub fn refresh_period_effect(&mut self) {
        self.inner.refresh_period_effect();
    }

    #[func]
    pub fn put_period_effect(&mut self, e: Gd<ExPropPeriodEffect>) {
        self.inner.put_period_effect(e.bind().inner.clone());
    }

    #[func]
    pub fn del_period_effect(&mut self, s: StringName) {
        self.inner.del_period_effect(&FixedNameWrapper(s));
    }

    #[func]
    pub fn restart_period_effect(&mut self, e: Gd<ExPropPeriodEffect>) {
        self.inner.restart_period_effect(&e.bind().inner);
    }

    #[func]
    pub fn get_period_effect_names(&self) -> Array<StringName> {
        self.inner
            .get_period_effect_names()
            .into_iter()
            .map(|s| s.0)
            .collect()
    }

    #[func]
    pub fn get_period_effect_by_name(&self, s: StringName) -> Option<Gd<ExPropPeriodEffect>> {
        if let Some(eff) = self.inner.get_period_effect_by_name(&FixedNameWrapper(s)) {
            let outer = Gd::from_init_fn(|base| ExPropPeriodEffect { base, inner: eff });
            Some(outer)
        } else {
            None
        }
    }

    #[func]
    pub fn process_time(&mut self, delta: f64) {
        self.inner.process_time(delta);
    }
}
