mod byond;

byond_fn! { version() {
    Some(env!("CARGO_PKG_VERSION"))
} }