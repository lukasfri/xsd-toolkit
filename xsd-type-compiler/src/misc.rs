pub fn unkeywordify(ident: &str) -> String {
    match ident {
        "type" => "type_".to_string(),
        "ref" => "ref_".to_string(),
        "match" => "match_".to_string(),
        "enum" => "enum_".to_string(),
        "self" => "self_".to_string(),
        "super" => "super_".to_string(),
        "crate" => "crate_".to_string(),
        "extern" => "extern_".to_string(),
        "use" => "use_".to_string(),
        "where" => "where_".to_string(),
        "as" => "as_".to_string(),
        "async" => "async_".to_string(),
        "await" => "await_".to_string(),
        "dyn" => "dyn_".to_string(),
        "union" => "union_".to_string(),
        "static" => "static_".to_string(),
        "const" => "const_".to_string(),
        "fn" => "fn_".to_string(),
        "for" => "for_".to_string(),
        "if" => "if_".to_string(),
        "else" => "else_".to_string(),
        "loop" => "loop_".to_string(),
        "while" => "while_".to_string(),
        "break" => "break_".to_string(),
        "continue" => "continue_".to_string(),
        "return" => "return_".to_string(),
        "in" => "in_".to_string(),
        "let" => "let_".to_string(),
        "impl" => "impl_".to_string(),
        "trait" => "trait_".to_string(),
        "struct" => "struct_".to_string(),
        "override" => "override_".to_string(),
        _ => ident.to_string(),
    }
}

pub fn rustify_name(ident: &str) -> String {
    let ident = unkeywordify(ident).replace("-", "_").replace('#', "");

    //Capitalize_first_letter
    ident
        .chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect::<String>()
}
