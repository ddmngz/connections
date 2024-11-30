pub mod collection_vec;
pub use collection_vec::CollectionVec;
use thiserror::Error;
use wasm_bindgen::JsCast;

use js_sys::Object;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlCollection;
use web_sys::HtmlTemplateElement;

use std::time::Duration;

fn get_elem(document: &Document, id: impl AsRef<str>) -> Result<Element, DomError> {
    document
        .get_element_by_id(id.as_ref())
        .ok_or(DomError::NoElem)
}

#[allow(dead_code)]
fn get_collection(document: &Document, id: impl AsRef<str>) -> HtmlCollection {
    let collection = document.get_elements_by_class_name(id.as_ref());
    assert!(
        collection.length() > 0,
        "collection with id {} is empty",
        id.as_ref()
    );
    collection
}

pub fn new<T: JsCast>(document: &Document, id: impl AsRef<str>) -> Result<T, DomError> {
    let element = get_elem(document, &id)?;
    element.dyn_into().map_err(|_| {
        let id = id.as_ref().into();
        DomError::Conversion(id)
    })
}

pub enum CustomElem {
    Board,
    Game,
}

impl AsRef<str> for CustomElem {
    fn as_ref(&self) -> &'static str {
        match self {
            Self::Board => "board-template",
            Self::Game => "connections-game",
        }
    }
}
use web_sys::Node;
pub fn create(elem: CustomElem, document: &Document) -> Node {
    let template: HtmlTemplateElement = new(document, &elem).unwrap();
    template.content().into()
}

pub enum AnimationType {
    Jump,
    Shake,
    PopUp,
    ShowModal,
    SlideIn,
}

impl AnimationType {
    fn duration(&self) -> f64 {
        match self {
            Self::Jump => 400.0,
            Self::Shake => 500.0,
            Self::ShowModal => 500.0,
            Self::PopUp => 2000.0,
            Self::SlideIn => 1000.0,
        }
    }

    fn keyframes(&self) -> Object {
        match self {
            AnimationType::Jump => jump_keyframes(),
            AnimationType::Shake => shake_keyframes(),
            AnimationType::ShowModal => todo!(),
            AnimationType::PopUp => popup_keyframes(),
            AnimationType::SlideIn => slide_in_keyframes(),
        }
    }
}

use crate::dom::console_log;
use gloo_timers::future::TimeoutFuture;
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

fn jump_keyframes() -> Object {
    let array = Array::new();
    let keyframe_0 = Object::new();
    let translate_middle = JsValue::from_str("translate(0,0)");
    let translate_up = JsValue::from_str("translate(0,-10px)");
    let transform_string = JsValue::from_str("transform");

    let _ = Reflect::set(&keyframe_0, &transform_string, &translate_middle);

    let keyframe_1 = Object::new();

    let _ = Reflect::set(&keyframe_1, &transform_string, &translate_up);
    let keyframe_2 = keyframe_0.clone();

    array.push(&keyframe_0.into());
    array.push(&keyframe_1.into());
    array.push(&keyframe_2.into());

    array.into()
}

fn popup_keyframes() -> Object {
    let array = Array::new();
    let beginning = Object::new();
    let middle = Object::new();
    let middle_2 = Object::new();
    let end = Object::new();

    let display = JsValue::from_str("display");
    let block: Object = JsValue::from_str("block").into();
    let none: Object = JsValue::from_str("none").into();

    let easing = JsValue::from_str("easing");
    let ease_out: Object = JsValue::from_str("ease-out").into();
    let ease_in: Object = JsValue::from_str("ease-in").into();

    let opacity = JsValue::from_str("opacity");
    let zero: Object = JsValue::from_str("0").into();
    let one: Object = JsValue::from_f64(1.0).into();

    let offset: Object = JsValue::from_str("offset").into();
    let twenty_five: Object = JsValue::from_f64(0.25).into();
    let seventy_five: Object = JsValue::from_f64(0.75).into();

    let _ = Reflect::set(&beginning, &display, &none);
    let _ = Reflect::set(&beginning, &opacity, &zero);
    let _ = Reflect::set(&beginning, &easing, &ease_out);

    let _ = Reflect::set(&middle, &offset, &twenty_five);
    let _ = Reflect::set(&middle, &display, &block);
    let _ = Reflect::set(&middle, &opacity, &one);
    let _ = Reflect::set(&middle, &easing, &ease_in);
    let _ = Object::assign(&middle_2, &middle);
    let _ = Reflect::set(&middle_2, &offset, &seventy_five);
    let _ = Reflect::set(&end, &easing, &ease_in);

    let _ = Reflect::set(&end, &display, &none);
    let _ = Reflect::set(&end, &opacity, &zero);

    array.push(&beginning.into());
    array.push(&middle.into());
    array.push(&middle_2.into());
    array.push(&end.into());

    array.into()
}

fn shake_keyframes() -> Object {
    let array = Array::new();
    let transform_string = JsValue::from_str("transform");

    let translate_middle: Object = JsValue::from_str("translate(0,0)").into();
    let translate_left: Object = JsValue::from_str("translate(-2px,0)").into();
    let translate_right: Object = JsValue::from_str("translate(2px,0)").into();

    let start = Object::new();
    let left_1 = Object::new();
    let right_1 = Object::new();
    let left_2 = Object::new();
    let right_2 = Object::new();
    let left_3 = Object::new();
    let end = Object::new();

    let _ = Reflect::set(&start, &transform_string, &translate_middle);
    let _ = Reflect::set(&left_1, &transform_string, &translate_left);
    let _ = Reflect::set(&right_1, &transform_string, &translate_right);
    Object::assign(&left_2, &left_1);
    Object::assign(&left_3, &left_1);

    Object::assign(&right_2, &right_1);
    Object::assign(&end, &start);

    array.push(&start);
    array.push(&left_1);
    array.push(&right_1);
    array.push(&left_2);
    array.push(&right_2);
    array.push(&left_3);
    array.push(&end);

    let object = array.into();

    console_log!("{:?}", object);
    object
}

fn slide_in_keyframes() -> Object {
    let array = Array::new();
    let transform_string = JsValue::from_str("transform");

    let under_button: Object = JsValue::from_str("translate(0,-165%)").into();
    let shown: Object = JsValue::from_str("translate(0,-90%)").into();

    let easing = JsValue::from_str("easing");
    let ease_out: Object = JsValue::from_str("ease-out").into();
    let ease_in: Object = JsValue::from_str("ease-in").into();

    let offset: Object = JsValue::from_str("offset").into();
    let twenty_five: Object = JsValue::from_f64(0.25).into();
    let seventy_five: Object = JsValue::from_f64(0.75).into();

    let start = Object::new();
    let display_start = Object::new();
    let display_end = Object::new();
    let end = Object::new();

    let _ = Reflect::set(&start, &transform_string, &under_button);

    let _ = Reflect::set(&display_start, &transform_string, &shown);
    let _ = Reflect::set(&display_start, &offset, &twenty_five);
    Object::assign(&display_end, &start);
    let _ = Reflect::set(&display_end, &easing, &ease_out);
    let _ = Reflect::set(&offset, &offset, &seventy_five);
    let _ = Reflect::set(&display_end, &transform_string, &shown);
    Object::assign(&end, &start);

    array.push(&start);
    array.push(&display_start);
    array.push(&display_end);
    array.push(&end);
    let object = array.into();
    console_log!("{:?}", object);
    object
}

#[derive(Error, Debug)]
pub enum DomError {
    #[error("Element Not Found")]
    NoElem,
    #[error("Error in conversion of element {0}")]
    Conversion(String),
}
