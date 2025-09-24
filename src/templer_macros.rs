#[macro_export]
macro_rules! template_vars {
    ($($key:expr => $val:tt),*) => {{
        let mut map = std::collections::HashMap::new();

        $(
            let val = template_vars!(@value $val);
            map.insert($key.to_string(), val);
        )*

        map
    }};

    (@value [$($item:tt),*]) => {
        TemplerVar::List(vec![$($item.to_string()),*])
    };

    (@value $item:expr) => {
        TemplerVar::String($item.to_string())
    };
}
