use lazy_static::lazy_static;

macro_rules! get_env {
    ($name:expr, $default:expr, $type:ty) => {{
        std::env::var($name)
            .ok()
            .and_then(|val| val.parse::<$type>().ok())
            .unwrap_or($default)
    }};
}

lazy_static! {
    pub static ref API_POST: String = get_env!("API_POST", "3000".to_string(), String);
}
