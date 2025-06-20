pub fn is_dynamic_path(path: &str) -> bool {
    path.split('/')
        .any(|segment| segment.starts_with('{') && segment.ends_with('}'))
}

pub fn extract_wildcard_prefix(path: &str) -> String {
    let mut segments = vec![];
    for segment in path.split('/') {
        if segment.starts_with('{') && segment.ends_with('}') {
            break;
        }
        if !segment.is_empty() {
            segments.push(segment);
        }
    }

    if segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", segments.join("/"))
    }
}

pub fn get_parent_path(path: &str) -> &str {
    match path.rfind('/') {
        Some(pos) if pos > 0 => &path[..pos],
        _ => "/",
    }
}