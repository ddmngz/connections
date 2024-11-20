use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::Range;
use wasm_bindgen::JsCast;
use web_sys::HtmlCollection;

pub struct CollectionVec<T: JsCast> {
    array: Vec<T>,
}

impl<T: JsCast> DerefMut for CollectionVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}

//impl<T: JsCast, const N: ?Sized> TryInto<[T; N]> for CollectionVec<T> {}

impl<T: JsCast> Deref for CollectionVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<T: JsCast> CollectionVec<T> {
    pub fn new(collection: &HtmlCollection) -> Self {
        let mut array = Vec::new();
        let mut i = 0;
        while let Some(elem) = collection.get_with_index(i) {
            array.push(elem.dyn_into().unwrap());
            i += 1;
        }
        Self { array }
    }
}

impl<T: JsCast> Index<usize> for CollectionVec<T> {
    type Output = T;
    fn index(&self, at: usize) -> &T {
        &self.array[at]
    }
}

impl<T: JsCast> Index<Range<usize>> for CollectionVec<T> {
    type Output = [T];
    fn index(&self, at: Range<usize>) -> &[T] {
        &self.array[at]
    }
}

impl<T: JsCast> IntoIterator for CollectionVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}
