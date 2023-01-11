use std::env;
use std::fs;
use serde_json::Value;
use once_cell::sync::OnceCell;
use dotenv::dotenv;
use regex::Regex;

static IS_PROD: OnceCell<bool> = OnceCell::new();
static MANIFEST: OnceCell<Value> = OnceCell::new();
static APP_HTML: OnceCell<String> = OnceCell::new(); 

pub fn init() {
    dotenv().ok();
    IS_PROD.set(env::var("APP_ENV").unwrap_or(String::from("development")).eq("production"));

    let manifest_content = fs::read_to_string(String::from("./public/dist/manifest.json")).expect("Could not find manifest.json");
    let manifest_json = serde_json::from_str::<Value>(&manifest_content).expect("Could not parse manifest.json");
    MANIFEST.set(manifest_json);
    
    let app_html_content = fs::read_to_string(String::from("./src/index.html")).expect("Could not find index.html");
    let app_html = template(app_html_content);
    APP_HTML.set(app_html);
}

pub fn is_prod() -> &'static bool {
    return IS_PROD.get().unwrap();
}

pub fn get_mainfest() -> &'static Value {
    return MANIFEST.get().unwrap();
}

pub fn get_app_html() -> &'static String {
    return APP_HTML.get().unwrap();
}

fn template(html: String) -> String {
    let app_head_regex = Regex::new(r"<!-- *?@app-head\(\) *?-->").unwrap();
    let react_refresh_regex = Regex::new(r"<!-- *?@react-refresh\(\) *?-->").unwrap();
    let script_regex = Regex::new(r#"<!-- *? @script\((.*)(?:src="(.*)")(.*?)\) *?-->"#).unwrap();
    let style_regex = Regex::new(r#"<!-- *? @style\((.*)(?:href="(.*)")(.*?)\) *?-->"#).unwrap();

    let mut app_html: String = html;
    let mut manifest: Value = get_mainfest().clone();
    
    if *is_prod() {
        app_html = app_head_regex.replace(&app_html, "").to_string();
        app_html = react_refresh_regex.replace(&app_html, "").to_string();
    } else {
        app_html = app_head_regex.replace(&app_html, r#"<script type="module" src="http://localhost:5173/@vite/client"></script>"#).to_string();
        app_html = react_refresh_regex.replace(&app_html, "<script type=\"module\">\nimport RefreshRuntime from \"http://localhost:5173/@react-refresh\"\nRefreshRuntime.injectIntoGlobalHook(window)\nwindow.$RefreshReg$ = () => {}\nwindow.$RefreshSig$ = () => (type) => type\nwindow.__vite_plugin_react_preamble_installed__ = true\n</script>").to_string();
    };

    let app_html_clone = app_html.clone();
    for script in script_regex.find_iter(&app_html_clone) {
        let props = script_regex.captures(script.as_str()).unwrap();
        let before = props.get(1).map_or("", |m| m.as_str());
        let src = props.get(2).map_or("", |m| m.as_str());
        let after = props.get(3).map_or("", |m| m.as_str());

        let localhost_src =  format!("http://localhost:5173/{}", src).to_string();
        let prod_src =  format!("/dist/{}", manifest[src]["file"].as_str().unwrap()).to_string();
        
        let file = if *is_prod() { &prod_src } else {  &localhost_src };
        let script_tag = format!("<script {} src=\"{}\" {}></script>", before, file, after);
        app_html.replace_range(script.start()..script.end(), &script_tag);
    };
    
    let app_html_clone = app_html.clone();
    for style in style_regex.find_iter(&app_html_clone) {
        let props = style_regex.captures(style.as_str()).unwrap();
        let before = props.get(1).map_or("", |m| m.as_str());
        let href = props.get(2).map_or("", |m| m.as_str());
        let after = props.get(3).map_or("", |m| m.as_str());

        let localhost_href =  format!("http://localhost:5173/{}", href).to_string();
        let prod_href =  format!("/dist/{}", manifest[href]["file"].as_str().unwrap()).to_string();
        
        let file = if *is_prod() { &prod_href } else {  &localhost_href };
        let href_tag = format!("<link {} href=\"{}\" {} />", before, file, after);
        app_html.replace_range(style.start()..style.end(), &href_tag);
    };

    return app_html;
}