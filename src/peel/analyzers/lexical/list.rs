use once_cell::sync::Lazy;

pub static INSTRUCTIONS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "+",
        "-",
        "*",
        "/",
        "if",
        "say",
    ]
});
