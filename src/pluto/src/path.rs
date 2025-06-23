use matchit::Match;

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

pub fn wildcard_base_path<T>(path: &str, lookup: &Result<Match<T>, String>) -> String {
    match lookup {
        Ok(lookup) if !lookup.params.is_empty() => {
            let total_segments = path.trim_matches('/').split('/').collect::<Vec<_>>();
            let param_count = lookup.params.len();
            let static_segments = &total_segments[..total_segments.len().saturating_sub(param_count)];
            format!("/{}", static_segments.join("/"))
        },
        _ => "".to_string(),
    }
}