use super::Element;
use super::Object;
use crate::console_log;
use gloo_timers::future::TimeoutFuture;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::Animation;
use web_sys::AnimationPlayState;

pub fn animate(elem: impl AsRef<Element>, animation: AnimationType) -> Animation {
    let keyframe = animation.keyframes();
    let duration = animation.duration();
    let elem: &Element = elem.as_ref();
    elem.animate_with_f64(Some(&keyframe), duration)
}

pub fn animate_in_background(elem: impl AsRef<Element>, animation: AnimationType) {
    let future = animate_later(elem, animation);
    spawn_local(async move {
        let _ = future.await;
    });
}

pub fn animate_later(elem: impl AsRef<Element>, animation_type: AnimationType) -> JsFuture {
    let animation = animate(elem, animation_type);
    let promise = animation.finished().unwrap();
    JsFuture::from(promise)
}

pub async fn animate_with_timeout(
    elem: impl AsRef<Element>,
    animation_type: AnimationType,
    timeout: Duration,
) {
    let timeout = timeout.as_millis() as u32;
    let timer = TimeoutFuture::new(timeout);
    animate(elem, animation_type);
    timer.await;
    console_log!("timeout");
}

pub async fn animate_then(elem: impl AsRef<Element>, animation_type: AnimationType) {
    let animation = animate(elem, animation_type);
    if animation.play_state() == AnimationPlayState::Running {
        let _ = JsFuture::from(animation.finished().unwrap()).await;
    }
}

use js_sys::Array;
use js_sys::Reflect;
use wasm_bindgen::JsValue;

pub enum AnimationType {
    Jump,
    Shake,
    PopUp,
    SlideIn,
}

impl AnimationType {
    fn duration(&self) -> f64 {
        match self {
            Self::Jump => 400.0,
            Self::Shake => 300.0,
            Self::PopUp => 2000.0,
            Self::SlideIn => 1000.0,
        }
    }

    fn keyframes(&self) -> Object {
        match self {
            AnimationType::Jump => jump_keyframes(),
            AnimationType::Shake => shake_keyframes(),
            AnimationType::PopUp => popup_keyframes(),
            AnimationType::SlideIn => slide_in_keyframes(),
        }
    }
}

fn jump_keyframes() -> Object {
    let normal = KeyframeAttribute::Translate {
        x: NumberAttribute {
            num: 0.0,
            unit: AttributeUnit::None,
        },
        y: NumberAttribute {
            num: 0.0,
            unit: AttributeUnit::None,
        },
    };

    let up = KeyframeAttribute::Translate {
        x: NumberAttribute {
            num: 0.0,
            unit: AttributeUnit::None,
        },
        y: NumberAttribute {
            num: -10.0,
            unit: AttributeUnit::Pixel,
        },
    };

    let begin = normal.as_object();
    let middle = up.as_object();
    let end = normal.as_object();

    let object = get_animation(&[begin, middle, end]);
    console_log!("{:?}", object);
    object
}

fn shake_keyframes() -> Object {
    let left = KeyframeAttribute::translate_x(-2.0, AttributeUnit::Pixel);
    let normal = KeyframeAttribute::base_translate();
    let right = KeyframeAttribute::translate_x(2.0, AttributeUnit::Pixel);

    get_animation(&[
        normal.as_object(),
        left.as_object(),
        right.as_object(),
        left.as_object(),
        right.as_object(),
        left.as_object(),
        normal.as_object(),
    ])
}

fn popup_keyframes() -> Object {
    let hidden = KeyframeAttribute::Display(Display::None);
    let showing = KeyframeAttribute::Display(Display::Block);

    let invisible = KeyframeAttribute::Opacity(NumberAttribute {
        num: 0.0,
        unit: AttributeUnit::Percent,
    });

    let visible = KeyframeAttribute::Opacity(NumberAttribute {
        num: 100.0,
        unit: AttributeUnit::Percent,
    });

    let ease_in = KeyframeAttribute::Easing(Easing::EaseIn);
    let ease_out = KeyframeAttribute::Easing(Easing::EaseOut);

    let quarter_in = KeyframeAttribute::offset(0.25);
    let almost_done = KeyframeAttribute::offset(0.75);

    let start = from_attributes(&[hidden, invisible, ease_out]);
    let visible_start = from_attributes(&[showing, visible, ease_in, quarter_in]);
    let visible_end = from_attributes(&[showing, visible, ease_in, almost_done]);
    let end = from_attributes(&[hidden, invisible, ease_in]);

    get_animation(&[start, visible_start, visible_end, end])
}

fn slide_in_keyframes() -> Object {
    let hidden = KeyframeAttribute::translate_y(-165.0, AttributeUnit::Percent);
    let showing = KeyframeAttribute::translate_y(-90.0, AttributeUnit::Percent);

    let ease_out = KeyframeAttribute::Easing(Easing::EaseOut);

    let almost_done = KeyframeAttribute::offset(0.75);
    let quarter_in = KeyframeAttribute::offset(0.25);

    //let start = from_attributes(&[hidden]);
    //
    let start = hidden.as_object();
    let visible_start = from_attributes(&[showing, quarter_in]);
    let visible_end = from_attributes(&[showing, almost_done, ease_out]);
    let end = hidden.as_object();

    get_animation(&[start, visible_start, visible_end, end])
}

fn get_animation(keyframes: &[Object]) -> Object {
    let array = Array::new();
    for keyframe in keyframes {
        array.push(keyframe);
    }
    array.into()
}

#[derive(Copy, Clone)]
enum KeyframeAttribute {
    Easing(Easing),
    Translate {
        x: NumberAttribute,
        y: NumberAttribute,
    },
    Opacity(NumberAttribute),
    Display(Display),
    Offset(NumberAttribute),
}

impl KeyframeAttribute {
    fn keyvalue_pair(&self) -> (JsValue, Object) {
        let (key, value) = match self {
            Self::Easing(easing) => ("easing", JsValue::from_str(easing.as_ref())),
            Self::Translate { x, y } => (
                "transform",
                JsValue::from_str(&format!("translate({},{})", x, y)),
            ),

            Self::Opacity(opacity) => ("opacity", JsValue::from_str(&opacity.to_string())),
            Self::Display(display) => ("display", JsValue::from_str(display.as_ref())),
            Self::Offset(NumberAttribute { num, .. }) => ("offset", JsValue::from_f64(*num)),
        };
        (JsValue::from_str(key), value.into())
    }

    const fn offset(offset: f64) -> Self {
        Self::Offset(NumberAttribute {
            num: offset,
            unit: AttributeUnit::Percent,
        })
    }

    const fn base_translate() -> Self {
        Self::Translate {
            x: NumberAttribute {
                num: 0.0,
                unit: AttributeUnit::None,
            },
            y: NumberAttribute {
                num: 0.0,
                unit: AttributeUnit::None,
            },
        }
    }

    const fn translate_x(num: f64, unit: AttributeUnit) -> Self {
        Self::Translate {
            x: NumberAttribute { num, unit },
            y: NumberAttribute { num: 0.0, unit },
        }
    }

    const fn translate_y(num: f64, unit: AttributeUnit) -> Self {
        Self::Translate {
            x: NumberAttribute { num: 0.0, unit },
            y: NumberAttribute { num, unit },
        }
    }

    fn as_object(&self) -> Object {
        let object = Object::new();
        let (key, value) = self.keyvalue_pair();
        let _ = Reflect::set(&object, &key, &value);
        object
    }
}

fn from_attributes(attributes: &[KeyframeAttribute]) -> Object {
    let keyframe = Object::new();
    for attribute in attributes {
        let (key, value) = attribute.keyvalue_pair();
        let _ = Reflect::set(&keyframe, &key, &value);
    }
    keyframe
}

#[derive(Copy, Clone)]
struct NumberAttribute {
    num: f64,
    unit: AttributeUnit,
}

use std::fmt;
impl fmt::Display for NumberAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.num, self.unit.as_ref())
    }
}

#[derive(Copy, Clone)]
enum AttributeUnit {
    Pixel,
    Percent,
    None,
}

impl AsRef<str> for AttributeUnit {
    fn as_ref(&self) -> &str {
        match self {
            Self::Pixel => "px",
            Self::Percent => "%",
            Self::None => "",
        }
    }
}

#[derive(Copy, Clone)]
enum Easing {
    EaseIn,
    EaseOut,
}

impl AsRef<str> for Easing {
    fn as_ref(&self) -> &str {
        match self {
            Self::EaseIn => "ease-in",
            Self::EaseOut => "ease-out",
        }
    }
}

#[derive(Copy, Clone)]
enum Display {
    Block,
    None,
}

impl AsRef<str> for Display {
    fn as_ref(&self) -> &str {
        match self {
            Self::Block => "block",
            Self::None => "none",
        }
    }
}
