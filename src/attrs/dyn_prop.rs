use godot::classes::RefCounted;
use godot::prelude::*;

use rust_engine_frame::attrs::dyn_prop::DynProp as Inner;

use crate::adapter::fixed_name_wrapper::FixedNameWrapper;
use crate::attrs::dyn_prop_dur_effect::DynPropDurEffect;
use crate::attrs::dyn_prop_inst_effect::DynPropInstEffect;
use crate::attrs::dyn_prop_period_effect::DynPropPeriodEffect;

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct DynProp {
    pub base: Base<RefCounted>,
    pub inner: Inner<FixedNameWrapper>,
}

#[godot_api]
impl DynProp {
    #[func]
    fn new(v: f64, the_max: f64, the_min: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new(v, the_max, the_min),
        })
    }

    #[func]
    fn new_with_max(v: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_with_max(v),
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
    pub fn use_inst_effect(&mut self, e: Gd<DynPropInstEffect>) -> f64 {
        self.inner.use_inst_effect(e.bind().inner.clone())
    }

    #[func]
    pub fn refresh_value(&mut self) {
        self.inner.refresh_value();
    }

    #[func]
    pub fn put_dur_effect(&mut self, e: Gd<DynPropDurEffect>) {
        self.inner.put_dur_effect(e.bind().inner.clone());
    }

    #[func]
    pub fn del_dur_effect(&mut self, s: StringName) {
        self.inner.del_dur_effect(&FixedNameWrapper(s));
    }

    #[func]
    pub fn do_put_dur_effect(&mut self, e: Gd<DynPropDurEffect>) {
        self.inner.do_put_dur_effect(e.bind().inner.clone());
    }

    #[func]
    pub fn refresh_period_effect(&mut self) {
        self.inner.refresh_period_effect();
    }

    #[func]
    pub fn put_period_effect(&mut self, e: Gd<DynPropPeriodEffect>) {
        self.inner.put_period_effect(e.bind().inner.clone());
    }

    #[func]
    pub fn del_period_effect(&mut self, s: StringName) {
        self.inner.del_period_effect(&FixedNameWrapper(s));
    }

    #[func]
    pub fn restart_period_effect(&mut self, e: Gd<DynPropPeriodEffect>) {
        self.inner.restart_period_effect(&e.bind().inner.clone());
    }

    #[func]
    pub fn process_time(&mut self, delta: f64) {
        self.inner.process_time(delta);
    }
}
