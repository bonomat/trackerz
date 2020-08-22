// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! debug {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
#[macro_export]
macro_rules! column {
    ($a:expr) => {{
        $crate::components::table::Column {
            data_property: Some($a.to_string()),
            name: $a.to_string(),
        }
    }};
    ($a:expr, $b:expr) => {{
        $crate::components::table::Column {
            data_property: Some($a.to_string()),
            name: $b.to_string(),
        }
    }};
}

#[macro_export]
macro_rules! columns {
( $( ( $($args:expr),* ) )+ ) => {
    vec![$(
        $crate::column![$($args),*]
    ),+];
};
}
