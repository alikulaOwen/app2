pub fn process_text(input: &str) -> String {
    let cleaned = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    format!(
        "Lowercase: {}\nUppercase: {}\nCapitalized: {}",
        cleaned.to_lowercase(),
        cleaned.to_uppercase(),
        capitalize(&cleaned)
    )
}

fn capitalize(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
