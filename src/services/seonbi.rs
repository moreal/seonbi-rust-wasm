use reqwest::Error;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SeonbiResponse {
    pub result_html: String,
}

pub async fn translate(source: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://seonbi.moreal.dev/")
        .header("Content-Type", "application/json")
        .body(
            json!({
                "sourceHtml": source,
                "preset": "ko-kr",
            })
            .to_string(),
        )
        .send()
        .await?
        .json::<SeonbiResponse>()
        .await?;

    Ok(response.result_html)
}
