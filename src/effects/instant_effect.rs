use godot::classes::RefCounted;
use godot::prelude::*;

use rust_engine_frame::effects::instant_effect::InstantEffect as NativeInstantEffect;
use rust_engine_frame::effects::native_effect::ProxyEffect;

/// 最基础的效果 仅描述如何生效 不实现具体效果
#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct InstantEffect {
    pub base: Base<RefCounted>,
    pub effect: NativeInstantEffect<StringName>,
}

#[godot_api]
impl InstantEffect {
    /// 【立即生效】型效果
    #[func]
    fn new_instant(from_name: StringName, effect_name: StringName, value: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            effect: NativeInstantEffect::new_instant(from_name, effect_name, value),
        })
    }

    #[func]
    fn get_effect_name(&self) -> StringName {
        self.effect.get_effect_name().clone()
    }
}
