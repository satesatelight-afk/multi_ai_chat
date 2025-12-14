mod modules;
#[allow(unused_imports)]
use crate::modules::open_ai_api::{self, ContentItem, OpenAIResponse};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn chat_open_ai(input: String) -> tauri::Result<String> {
    let input = format!(
        "markdown形式で返答してください。プログラムのコードには言語の名前もお願い。最後は---で終わるようにして{}",
        input
    );
    let model = "gpt-5-nano";
    let mut open_ai_api = open_ai_api::OpenAIAPI::new(model);
    let response = open_ai_api.get_method(&input).await.unwrap();
    let reses: Vec<ContentItem> = response
        .output
        .iter()
        .filter_map(|res| res.content.clone())
        .map(|d| d[0].clone())
        .collect();
    let mut chars = vec![];
    for output_item in reses.iter() {
        println!("output_item:{:?}", output_item.text);
        if let Some(output_text) = &output_item.text {
            let first_data = &output_text.chars();
            let last = first_data.clone().count();
            for (u, char) in first_data.clone().into_iter().enumerate() {
                if u == 0 && char.to_string() != "\"".to_string() {
                    chars.push(char.to_string());
                } else if u == last && char.to_string() != "\"".to_string() {
                    chars.push(char.to_string());
                } else {
                    chars.push(char.to_string());
                }
            }
        }
    }
    Ok(chars.join(""))
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![chat_open_ai])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
