use serde::{Serialize};
use std::fs;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

use crate::config;

#[derive(Serialize)]
struct PageData<T> {
    props: T,
    component: String,
}


pub fn render_with_props<T>(component: String, props: T) -> String where T: Serialize {
    let app_regex = Regex::new(r"<!-- *?@app\(\) *?-->").unwrap();
    let app_html = config::get_app_html();
    let page_data = PageData {
        component,
        props
    };
    let serialized_page_data: String = serde_json::to_string(&page_data).unwrap();
    let app_html_with_page_data = app_regex.replace(&app_html, format!(r#"<div id="root" data-page='{}'></div>"#, serialized_page_data));

    return app_html_with_page_data.to_string();
}
