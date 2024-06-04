pub(crate) fn split_url_into_parts(url: impl AsRef<str>) -> Vec<String> {
    url.as_ref()
        .split('/')
        .filter_map(|part| {
            if part.is_empty() {
                return None;
            }

            Some(part.to_string())
        })
        .collect::<Vec<String>>()
}
