use crate::components::error::Result;
use crate::components::TableError;
use crate::data::track_details::TrackDetail;
use serde::Serialize;
use serde_value::Value;
use std::fmt;
use yew::{html, ComponentLink, Html};
use yewtil::Pure;

pub trait TableData: 'static + Default + Clone + Ord + Serialize {
    /// Returns the Html representation of a field. When None, the field is not rendered.
    fn get_field_as_html(&self, field_name: &str) -> Result<Html>;

    /// Returns a table value given its field name. This value is used as a sorting key for the corresponding column.
    fn get_field_as_value(&self, field_name: &str) -> Result<Value>;
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Column {
    pub name: String,
    pub short_name: Option<String>,
    pub data_property: Option<String>,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.short_name.as_ref().unwrap_or(&self.name))
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
    /// The order of the columns determines the order in which they are displayed.
    pub columns: Vec<Column>,
    pub data: Vec<TrackDetail>,
    pub options: Option<TableOptions>,
    pub state: TableState,
    pub link: ComponentLink<Self>,
}

impl TableData for TrackDetail {
    fn get_field_as_html(&self, field_name: &str) -> Result<Html> {
        let html_repr = match field_name {
            "file" => html! {
                { &self.file }
            },
            "title" => html! {
                { &self.title }
            },
            "description" => html! {
                { &self.description }
            },
            "county" => html! {
                { &self.county }
            },
            n => return Err(TableError::NonRenderableField(n.to_owned())),
        };
        Ok(html_repr)
    }

    fn get_field_as_value(&self, field_name: &str) -> Result<Value> {
        let value = match field_name {
            "file" => serde_value::to_value(&self.file),
            "title" => serde_value::to_value(&self.title),
            "description" => serde_value::to_value(&self.description),
            "county" => serde_value::to_value(&self.county),
            // "start" => serde_value::to_value(&self.start),
            n => return Err(TableError::InvalidFieldName(n.to_owned())),
        };
        Ok(value.unwrap())
    }
}
