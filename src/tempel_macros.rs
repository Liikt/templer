#[macro_export]
macro_rules! template_vars {
    ($($key:expr => $val:expr),*) => {{
        let mut map = std::collections::HashMap::new();

        $(
            let val = template_vars!(@value $val);
            map.insert($key.to_string(), val);
        )*;

        map
    }};

    (@value [$($item:expr),*]) => {
        TempelVar::List(vec![$($item.to_string()),*]);
    };

    (@value $item:expr) => {
        TempelVar::String($item.to_string());
    };
}
