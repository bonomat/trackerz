use crate::components::table::{Column, Table, TableData, TableOptions, TableOrder, TableState};
use crate::data::track_details::TrackDetail;
use std::cmp::Reverse;
use yew::html;
use yew::prelude::*;

/// Properties of the Table component.
#[derive(Clone, PartialEq, Default, yew::Properties)]
pub struct TableProps {
    pub columns: Vec<Column>,
    pub data: Vec<TrackDetail>,
    pub options: Option<TableOptions>,
}

#[derive(Debug)]
pub enum Msg {
    SortColumn(usize),
}

impl Component for Table {
    type Message = Msg;
    type Properties = TableProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let column_number = props.columns.len();
        Table {
            columns: props.columns,
            data: props.data,
            options: props.options,
            state: TableState {
                order: vec![TableOrder::default(); column_number],
            },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SortColumn(i) => {
                use TableOrder::*;

                for (j, x) in self.state.order.iter_mut().enumerate() {
                    if j != i {
                        *x = Unordered
                    } else {
                        *x = x.rotate()
                    }
                }

                match self.columns[i].data_property.as_ref() {
                    Some(f) => {
                        match self.state.order[i] {
                            Unordered => self.data.sort(),
                            Ascending => self
                                .data
                                .sort_by_cached_key(|x| x.get_field_as_value(&f).unwrap()),
                            Descending => self
                                .data
                                .sort_by_cached_key(|x| Reverse(x.get_field_as_value(&f).unwrap())),
                        }
                        true
                    }
                    None => false,
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.columns = props.columns;
        self.data = props.data;
        true
    }

    fn view(&self) -> Html {
        let get_orderable_class = || {
            if self.is_orderable() {
                "is-orderable"
            } else {
                ""
            }
        };

        html! {
            <table class=("table table-striped", get_orderable_class()),>
                <thead>
                    { for self.columns.iter().enumerate().map(|(i, col)| self.view_column(i, &col)) }
                </thead>
                <tbody>
                    { for self.data.iter().map(|d| self.view_row(&d)) }
                </tbody>
            </table>
        }
    }
}

impl Table {
    pub fn is_orderable(&self) -> bool {
        if let Some(options) = &self.options {
            options.orderable
        } else {
            false
        }
    }

    fn view_column<'a>(&'a self, index: usize, column: &'a Column) -> Html {
        let get_header_sorting_class = |index: usize| {
            use TableOrder::*;
            match self.state.order[index] {
                Unordered => "",
                Ascending => "is-sorting-ascending",
                Descending => "is-sorting-descending",
            }
        };

        let th_view = |child| {
            if self.is_orderable() {
                let index = index.clone();
                let click_callback = self.link.callback(move |_| Msg::SortColumn(index));
                html! { <th scope="col" onclick=click_callback>{ child }</th> }
            } else {
                html! { <th scope="col">{ child }</th> }
            }
        };

        let html_view = html! {
            <span>
                <abbr title=&column.name,>
                    { column }
                </abbr><i class=("sorting-control", get_header_sorting_class(index)),></i>
            </span>
        };

        th_view(html_view)
    }

    fn view_row(&self, datum: &TrackDetail) -> Html {
        html! {
            <tr>
                {
                    for self.columns.iter()
                        .map(|c| { c.data_property.as_ref().unwrap_or(&c.name) })
                        .map(|name| { datum.get_field_as_html(name) })
                        .filter_map(|h| h.ok())
                        .map(|el| html! { <td>{ el }</td> })
                }
            </tr>
        }
    }
}
