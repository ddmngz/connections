use super::element_ops;
use element_ops::CollectionVec;
use web_sys::Document;
use web_sys::HtmlDivElement;

mod dots {
    use super::*;

    pub struct Dots {
        dots: [Dot; 4],
        remaining: Option<NumDots>,
    }

    #[repr(usize)]
    #[derive(Copy, Clone)]
    enum NumDots {
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
    }

    impl Dots {
        fn new(document: &Document) -> Self {
            let dots = document.get_elements_by_class_name("dot");
            let dots = CollectionVec::<HtmlDivElement>::new(&dots);
            assert!(dots.len() == 4);
            let dots: [Dot; 4] = std::array::from_fn(|index| Dot(dots[index].clone()));
            let remaining = Some(NumDots::Four);
            Self { dots, remaining }
        }

        fn hide_one(&mut self) {
            let Some(num_dots) = self.remaining else {
                return;
            };
            let i = (num_dots as usize) - 1;
            self.dots[i].hide();
        }
        fn reset(&mut self) {
            self.remaining = Some(NumDots::Four);
            for dot in &self.dots {
                dot.show();
            }
        }
    }

    struct Dot(HtmlDivElement);
    impl Dot {
        fn show(&self) {
            let _ = self.0.class_list().remove_1("hidden");
        }

        fn hide(&self) {
            let _ = self.0.class_list().add_1("hidden");
        }
    }
}

mod pop_up {
    use web_sys::HtmlDialogElement;
    struct PopUp(HtmlDialogElement);
    struct Modal(HtmlDialogElement);
    impl Modal {
        fn new() -> Self {
            todo!()
        }
        fn show(&self) {
            self.0.show_modal();
        }
        fn hide(&self) {
            self.0.close();
        }
    }

    impl PopUp {
        fn new() -> Self {
            todo!()
        }

        fn pop_up(&self) {}
    }
}
mod overlay {}
