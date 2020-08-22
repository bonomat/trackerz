use yew::{html, Callback, Html, MouseEvent, Properties};
use yewtil::{Pure, PureComponent};

pub type BootstrapTable = Pure<PureTable>;

#[derive(Clone, PartialEq, Properties)]
pub struct PureTable {}

impl PureComponent for PureTable {
    fn render(&self) -> Html {
        html! {
            <table class="table">
              <thead>
                <tr>
                  <th scope="col"></th>
                  <th scope="col">{"First"}</th>
                  <th scope="col">{"Last"}</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <th scope="row">{ "1" }</th>
                  <td>{"Mark"}</td>
                  <td>{"Otto"}</td>
                </tr>
                <tr>
                  <th scope="row">{ "2" }</th>
                  <td>{"Jacob"}</td>
                  <td>{"Thornton"}</td>
                </tr>
                <tr>
                  <th scope="row">{ "3" }</th>
                  <td>{"Larry"}</td>
                  <td>{"the Bird"}</td>
                </tr>
              </tbody>
            </table>
        }
    }
}
