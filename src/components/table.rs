use crate::{components::button::BootstrapButton, data::track_details::TrackDetail};
use serde::Serialize;
use serde_value::Value;
use std::{cmp::Reverse, error::Error, fmt, str::FromStr};
use url::Url;
use wasm_bindgen::{prelude::*, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::{html, prelude::*, ComponentLink, Html};

pub type Result<T> = std::result::Result<T, TableError>;

#[wasm_bindgen(module = "/js/wasm_bridge.js")]
extern "C" {
    // #[wasm_bindgen(catch)]
    async fn read_gpx(url: &str) -> JsValue;
    async fn read_kml(url: &str) -> JsValue;
    fn remove(layer: &str);
}

pub trait TableData: 'static + Default + Clone + Ord + Serialize {
    /// Returns the Html representation of a field. When None, the field is not
    /// rendered.
    fn get_field_as_html(
        &self,
        field_name: &str,
        file_name: &str,
        link: &ComponentLink<Table>,
    ) -> Result<Html>;

    /// Returns a table value given its field name. This value is used as a
    /// sorting key for the corresponding column.
    fn get_field_as_value(&self, field_name: &str) -> Result<Value>;
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Column {
    pub name: String,
    pub data_property: Option<String>,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl Column {
    pub fn new(property: Option<String>, name: String) -> Self {
        Self {
            name,
            data_property: property,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct TableOptions {
    pub orderable: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TableOrder {
    Unordered = 0,
    Ascending,
    Descending,
}

impl Default for TableOrder {
    fn default() -> Self {
        TableOrder::Unordered
    }
}

impl TableOrder {
    pub fn rotate(&self) -> Self {
        use TableOrder::*;
        match *self {
            Unordered => Ascending,
            Ascending => Descending,
            Descending => Unordered,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct TableState {
    pub order: Vec<TableOrder>,
}

/// The a table with columns holding data.
#[derive(Clone)]
pub struct Table {
    /// The order of the columns determines the order in which they are
    /// displayed.
    pub columns: Vec<Column>,
    pub data: Vec<TrackDetail>,
    pub options: Option<TableOptions>,
    pub state: TableState,
    pub link: ComponentLink<Self>,
}

impl TableData for TrackDetail {
    fn get_field_as_html(
        &self,
        field_name: &str,
        file_name: &str,
        on_signal: &ComponentLink<Table>,
    ) -> Result<Html> {
        let file_name = file_name.to_string();
        let html_repr = match field_name {
            "title" => html! {
                { &self.title }
            },
            "description" => html! {
                { &self.description }
            },
            "country" => html! {
                { &self.country }
            },
            "show" => html! {
               <BootstrapButton title="Show" onsignal=on_signal.callback(move |_| TableCallbackMsg::Add(file_name.clone())) />
            },
            "hide" => html! {
               <BootstrapButton title="Hide" onsignal=on_signal.callback(move |_| TableCallbackMsg::Remove(file_name.clone()))  />
            },
            n => return Err(TableError::NonRenderableField(n.to_owned())),
        };
        Ok(html_repr)
    }

    fn get_field_as_value(&self, field_name: &str) -> Result<Value> {
        let value = match field_name {
            "title" => serde_value::to_value(&self.title),
            "description" => serde_value::to_value(&self.description),
            "country" => serde_value::to_value(&self.country),
            "show" => serde_value::to_value(true),
            "hide" => serde_value::to_value(true),
            n => return Err(TableError::InvalidFieldName(n.to_owned())),
        };
        Ok(value.unwrap())
    }
}

/// Properties of the Table component.
#[derive(Clone, PartialEq, Default, yew::Properties)]
pub struct TableProps {
    pub columns: Vec<Column>,
    pub data: Vec<TrackDetail>,
    pub options: Option<TableOptions>,
}

#[derive(Debug)]
pub enum TableCallbackMsg {
    SortColumn(usize),
    Add(String),
    Remove(String),
}

impl Component for Table {
    type Message = TableCallbackMsg;
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
        let window = web_sys::window().expect("no global `window` exists");
        let location = window.location();
        let location: String = location.href().expect("To get URL");
        let mut url = Url::from_str(&location).unwrap();
        match msg {
            TableCallbackMsg::SortColumn(i) => {
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
            TableCallbackMsg::Add(file_name) => {
                spawn_local(async move {
                    debug!("adding: {:?}", &file_name);
                    let js_value = if file_name.contains(".kml") {
                        url.path_segments_mut()
                        .unwrap()
                        .push("trackz")
                        .push("kml")
                        .push(&file_name);
                        let url = url.as_str();
                        read_kml(url).await
                    } else {
                        url.path_segments_mut()
                            .unwrap()
                            .push("trackz")
                            .push("gpx")
                            .push(&file_name);
                        let url = url.as_str();
                        read_gpx(url).await
                    };
                    debug!("received: {:?}", js_value.as_string().unwrap());
                    debug!("added: {:?}", file_name);
                });
                true
            }
            TableCallbackMsg::Remove(message) => {
                spawn_local(async move {
                    debug!("remove: {:?}", message);
                    remove("latest");
                });
                true
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
                let click_callback = self
                    .link
                    .callback(move |_| TableCallbackMsg::SortColumn(index));
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

    fn view_row(&self, track_detail: &TrackDetail) -> Html {
        html! {
            <tr>
                {
                    for self.columns.iter()
                        .map(|c| { c.data_property.as_ref().unwrap_or(&c.name) })
                        .map(|name| {
                            track_detail.get_field_as_html(name, &track_detail.file, &self.link)
                        })
                        .filter_map(|h| h.ok())
                        .map(|el| html! { <td>{ el }</td> })
                }
            </tr>
        }
    }
}

#[derive(Debug)]
pub enum TableError {
    NonRenderableField(String),
    InvalidFieldName(String),
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            TableError::InvalidFieldName(field_name) => {
                format!("Invalid field name given: '{}'.", field_name)
            }
            TableError::NonRenderableField(field_name) => format!(
                "Could not render field '{}' for which no HTML representation is defined.",
                field_name
            ),
        };
        write!(f, "{}", msg)
    }
}

impl Error for TableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match self {
            TableError::InvalidFieldName(_) => "Invalid field name given.",
            TableError::NonRenderableField(_) => "Field has no HTML representation defined.",
        }
    }
}
