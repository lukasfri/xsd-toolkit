pub enum Min<T> {
    Inclusive(T),
    Exclusive(T),
}

pub enum Max<T> {
    Inclusive(T),
    Exclusive(T),
}

pub struct NumericTextTemplate<T> {
    pub min: Option<Min<T>>,
    pub max: Option<Max<T>>,
}

pub enum WhiteSpace {
    Preserve,
    Replace,
    Collapse,
}

pub struct TextTemplate {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub length: Option<usize>,
    pub patterns: Vec<String>,
    pub enumerations: Vec<String>,
    pub white_space: Option<WhiteSpace>,
}

pub struct ListTemplate {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub length: Option<usize>,
}

// Need to think about how to handle different types for parsing - numeric, text, etc.
