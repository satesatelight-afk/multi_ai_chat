use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct OpenAIResponse {
    pub output: Vec<OutputItem>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct OutputItem {
    pub content: Option<Vec<ContentItem>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ContentItem {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIAPI {
    pub model: String,
    pub input: String,
    pub text: OpenAIText,
    pub reasoning: OpenAIReasoning,
}
/**
モデルに返答の詳細度を制御するパラメータです。low, medium, highがあり、mediumがデフォルトです。
*/
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIText {
    pub verbosity: String,
}

/**
モデルが応答を生成する前に生成する推論トークンの数を制御するパラメータ.
minimal, low, medium, highがあり、mediumがデフォルトでです。
highにするほど深く推論してくれ、レイテンシを抑える場合はminimalを指定してください。
*/
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIReasoning {
    effort: String,
}

#[allow(dead_code)]
impl OpenAIAPI {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
            input: String::new(),
            text: OpenAIText {
                verbosity: "medium".to_string(),
            },
            reasoning: OpenAIReasoning {
                effort: "minimal".to_string(),
            },
        }
    }

    pub async fn get_method(&mut self, input: &str) -> anyhow::Result<OpenAIResponse> {
        let api_key = "******************************";
        self.input = input.to_string();
        let client = Client::new();
        let res = client
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(api_key)
            .json(&self)
            .send()
            .await?;
        let response: OpenAIResponse = res.json().await?;

        anyhow::Ok(response)
    }
}

#[tokio::test]
async fn open_ai_api_test() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let model = "gpt-5-nano";
    let input = "pythonについて教えて。markdown形式で返答でお願い";

    let mut openai_api = OpenAIAPI::new(model);
    let response = openai_api.get_method(&input).await?;
    let reses: Vec<ContentItem> = response
        .output
        .iter()
        .filter_map(|res| res.content.clone())
        .map(|d| d[0].clone())
        .collect();
    for output_item in reses.iter() {
        println!("output_item:{:?}", output_item.text);
        // let f = std::fs::File::create("test.md")?;
        // let buf = std::io::BufWriter::new(f);
        if let Some(output_text) = &output_item.text {
            let first_data = &output_text.chars();
            let last = first_data.clone().count();
            let mut chars = vec![];
            for (u, char) in first_data.clone().into_iter().enumerate() {
                if u == 0 && char.to_string() != "\"".to_string() {
                    chars.push(char.to_string());
                } else if u == last && char.to_string() != "\"".to_string() {
                    chars.push(char.to_string());
                } else {
                    chars.push(char.to_string());
                }
            }
            // let output_text: String = chars.join("");
            // serde_json::to_writer_pretty(buf, &output_text)?;
        }
    }
    anyhow::Ok(())
}
