use yew::prelude::*;

pub struct App {
    count: isize,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Increment,
    Decrement,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { count: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.count += 1;
                true
            }
            Msg::Decrement => {
                self.count -= 1;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <button onclick=self.link.callback(|_|Msg::Increment)>{"+"}</button>
                <p>{self.count}</p>
                <button onclick=self.link.callback(|_|Msg::Decrement)>{"-"}</button>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn create_works() {
        let app = App::create((), ComponentLink::<App>::new());
        assert_eq!(0, app.count);
    }

    #[test_case(Msg::Increment, -4, -3,true)]
    #[test_case(Msg::Increment, -1, 0,true)]
    #[test_case(Msg::Increment, 0, 1, true)]
    #[test_case(Msg::Increment, 3, 4, true)]
    #[test_case(Msg::Decrement, 1, 0, true)]
    #[test_case(Msg::Decrement, 0, -1,true)]
    #[test_case(Msg::Decrement, -2, -3,true)]
    #[test_case(Msg::Decrement, 3, 2, true)]
    fn update_works(
        msg: Msg,
        before_count: isize,
        expected_count: isize,
        expected_should_render: bool,
    ) {
        let mut app = App::create((), ComponentLink::<App>::new());
        app.count = before_count;
        assert_eq!(expected_should_render, app.update(msg));
        assert_eq!(expected_count, app.count);
    }

    #[test_case(4)]
    #[test_case(3)]
    #[test_case(0)]
    #[test_case(-1)]
    #[test_case(-2)]
    fn view_works(count: isize) {
        let mut app = App::create((), ComponentLink::<App>::new());
        app.count = count;
        let expected = html! {
            <div class="container">
                <button onclick=app.link.callback(|_|Msg::Increment)>{"+"}</button>
                <p>{count}</p>
                <button onclick=app.link.callback(|_|Msg::Decrement)>{"-"}</button>
            </div>
        };

        let actual = app.view();
        assert_eq!(expected, actual);
    }
}
