use std::collections::HashMap;

use chrono::NaiveDate;
use leptos::{server, ServerFnError};
use reqwest::{
    header::{ACCEPT, HOST, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};

#[server(GetItems, "/api")]
pub async fn get_versions() -> Result<Vec<Version>, ServerFnError> {
    Ok(request_docs_from_aws().await)
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Version {
    pub version: String,
    pub end_of_standard_support: String,
    pub end_of_extended_support: String,
}

pub async fn request_docs_from_aws() -> Vec<Version> {
    let client = Client::new();

    let resp = client
        .get("https://docs.aws.amazon.com/eks/latest/userguide/kubernetes-versions.html")
        .header(USER_AGENT, "curl/7.84.0")
        .header(ACCEPT, "*/*")
        .header(HOST, "docs.aws.amazon.com")
        .fetch_mode_no_cors()
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");
    let versions = get_versions_from_response_text(resp).await;
    return versions;
}

async fn get_versions_from_response_text(resp: String) -> Vec<Version> {
    let mut versions: Vec<Version> = vec![];
    let mut table = crop(
        &resp,
        "The following table shows important release",
        "xreflabel=\"Amazon EKS version FAQs\"",
    )
    .await;
    loop {
        let found = crop(&table, "<td tabindex=\"-1\">", "</td>").await;
        let version = crop_version(&found).await;
        let standard_support = get_standard_support(&table).await;
        let extended_support = get_extended_support(&table).await;
        table = crop(&table, &version, "<div id=\"version-deprecation\"").await;
        table = crop(
            &table,
            "<td tabindex=\"-1\"><code class=\"code\">",
            "<div id=\"version-deprecation\"",
        )
        .await;

        if version == "" {
            break;
        }
        versions.push(Version {
            version,
            end_of_standard_support: standard_support,
            end_of_extended_support: extended_support,
        });
    }
    return versions;
}

async fn get_standard_support(table: &String) -> String {
    get_support(&table, 3).await
}
async fn get_extended_support(table: &String) -> String {
    get_support(&table, 4).await
}
async fn get_support(table: &String, td_number: usize) -> String {
    let indices_td_start = find_all_indices(&table, "<td tabindex=\"-1\">").await;
    let indices_td_end = find_all_indices(&table, "</td>").await;
    if indices_td_start.len() <= td_number {
        return "July 24, 2099".to_string();
    }
    let mut cropped = crop_number(
        &table,
        indices_td_start[td_number] + 18,
        indices_td_end[td_number],
    )
    .await;
    if cropped.contains("<span>") {
        cropped = cropped.replace("<span>", "")
    }
    if cropped.contains("</span>") {
        cropped = cropped.replace("</span>", "")
    }
    return cropped;
}

async fn crop(from: &String, start_pattern: &str, end_pattern: &str) -> String {
    let start_bytes = from.find(start_pattern).unwrap_or(0);
    let end_bytes = from.find(end_pattern).unwrap_or(from.len());
    if start_bytes > end_bytes || end_bytes == from.len() {
        return "".to_string();
    }
    from[start_bytes..end_bytes + end_pattern.len()].to_string()
}

async fn crop_number(from: &String, start_number: usize, end_number: usize) -> String {
    if start_number > end_number || end_number == from.len() {
        return "".to_string();
    }
    from[start_number..end_number].to_string()
}

async fn crop_version(from: &String) -> String {
    let s_pattern = "<code class=\"code\">";
    let e_pattern = "</code>";
    let start_bytes = from.rfind(s_pattern).unwrap_or(0);
    let end_bytes = from.rfind(e_pattern).unwrap_or(from.len());
    if start_bytes > end_bytes || end_bytes == from.len() {
        return "".to_string();
    }

    return from[start_bytes + s_pattern.len()..end_bytes].to_string();
}

async fn find_all_indices(from: &String, pattern: &str) -> Vec<usize> {
    from.match_indices(&pattern).map(|(i, _)| i).collect()
}
