use godot::classes::RefCounted;
use godot::prelude::*;

use rust_engine_frame::attrs::dyn_attr::DynAttr as Inner;

use crate::adapter::fixed_name_wrapper::FixedNameWrapper;
use crate::attrs::dyn_attr_effect::DynAttrEffect;

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct DynAttr {
    pub base: Base<RefCounted>,
    pub inner: Inner<FixedNameWrapper>,
}

#[godot_api]
impl DynAttr {
    #[func]
    fn create(v: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new(v),
        })
    }

    #[func]
    pub fn get_origin(&self) -> f64 {
        self.inner.get_origin()
    }

    #[func]
    pub fn get_current(&self) -> f64 {
        self.inner.get_current()
    }

    #[func]
    pub fn refresh_value(&mut self) {
        self.inner.refresh_value();
    }

    #[func]
    pub fn put_or_stack_effect(&mut self, eff: Gd<DynAttrEffect>) {
        self.inner.put_or_stack_effect(eff.bind().inner.clone());
    }

    #[func]
    pub fn del_effect(&mut self, s: StringName) {
        self.inner.del_effect(&FixedNameWrapper(s));
    }

    #[func]
    pub fn process_time(&mut self, delta: f64) {
        self.inner.process_time(delta);
    }
}
