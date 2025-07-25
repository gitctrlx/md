use comrak::{markdown_to_html, Options};

// Learn more about comrak at https://docs.rs/comrak/latest/comrak/struct.ExtensionOptions.html
#[tauri::command]
fn md_to_html(md: &str) -> String {
    let mut options = Options::default();
    options.extension.autolink = true;
    options.extension.description_lists = true;
    options.extension.footnotes = true;
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.extension.math_code = true;
    options.extension.math_dollars = true;
    options.extension.multiline_block_quotes = true;
    options.extension.shortcodes = true;
    options.extension.strikethrough = true;
    options.extension.superscript = true;
    options.extension.table = true;
    options.extension.tagfilter = true;
    options.extension.tasklist = true;
    options.extension.header_ids = Some("nav-item-".to_string());

    markdown_to_html(md, &options)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()   
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![md_to_html])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
