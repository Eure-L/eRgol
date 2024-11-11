

#[macro_export] macro_rules! get {
    ($var:expr) => {
        *$var.read().unwrap()
    };
}

#[macro_export] macro_rules! set {
    ($var:expr, $val:expr) => {
        *$var.write().unwrap() = $val;
    };
}
