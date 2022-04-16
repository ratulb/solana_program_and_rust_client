//Our error handling is not robust - that is intentional
//We care more about descriptive error messages that we 
//may encounter rather than handling them in this learning exercise
pub type Result<T> = std::result::Result<T, String>;
