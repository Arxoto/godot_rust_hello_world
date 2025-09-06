use godot::classes::RefCounted;
use godot::prelude::*;

use rust_engine_frame::attrs::dyn_prop_inst_effect::DynPropInstEffect as Inner;
use rust_engine_frame::effects::native_effect::ProxyEffect;

use crate::adapter::fixed_name_wrapper::FixedNameWrapper;

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct DynPropInstEffect {
    pub base: Base<RefCounted>,
    pub inner: Inner<FixedNameWrapper>,
}

#[godot_api]
impl DynPropInstEffect {
    /// 【立即生效】型效果
    #[func]
    fn new_instant(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            inner: Inner::new_instant_cur_val(from_name, effect_name, value),
        })
    }

    #[func]
    fn get_effect_name(&self) -> StringName {
        self.inner.get_effect_name().into()
    }
}
