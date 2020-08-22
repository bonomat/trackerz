use yew::{html, Callback, Html, MouseEvent, Properties, ComponentLink, Component, ShouldRender};
use yewtil::{Pure, PureComponent};

struct RoutesTable {
    link: ComponentLink<Self>,
}

enum Msg {
    Click,
}

impl Component for RoutesTable {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MyComponent { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                // Handle Click
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        // Create a callback from a component link to handle it in a component
        let click_callback = self.link.callback(|_: ClickEvent| Msg::Click);
        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
