// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! debug {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

