use serde::Deserialize;

use crate::API_BASE_URL;

pub struct OpenDictRequest {
    pub key: String,
    pub target_type: String,
    pub part: String,
    pub query: String,
    pub start: String,
    pub num: String,
}

impl OpenDictRequest {
    pub fn new(key: &str, word: &str) -> Self {
        OpenDictRequest {
            key: String::from(key),
            target_type: String::from("search"),
            part: String::from("word"),
            query: String::from(word),
            start: String::from("1"),
            num: String::from("100"),
        }
    }

    pub fn build_url(&self) -> String {
        let mut output = String::from(API_BASE_URL);

        output.push_str(&self.target_type);
        output.push('?');

        output.push_str("key=");
        output.push_str(&self.key);
        output.push('&');

        output.push_str("target_type=");
        output.push_str(&self.target_type);
        output.push('&');

        output.push_str("part=");
        output.push_str(&self.part);
        output.push('&');

        output.push_str("q=");
        output.push_str(&self.query);
        output.push('&');

        output.push_str("start=");
        output.push_str(&self.start);
        output.push('&');

        output.push_str("num=");
        output.push_str(&self.num);

        output
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenDictResponse {
    pub title: String,
    pub link: String,
    pub total: String,
    pub description: String,
    pub start: String,

    #[serde(rename = "item")]
    pub items: Vec<OpenDictItem>,

    #[serde(rename = "lastBuildDate")]
    pub last_build_date: String,
}

impl OpenDictResponse {
    pub fn senses(&self) -> Vec<OpenDictSense> {
        let mut out = vec![];

        for item in &self.items {
            for sense in &item.senses {
                out.push(sense.clone())
            }
        }

        out
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenDictItem {
    pub word: String,

    #[serde(rename = "sense")]
    pub senses: Vec<OpenDictSense>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenDictSense {
    pub definition: String,
    pub link: Vec<String>,
    pub origin: String,
    pub pos: Option<String>,
    pub sense_no: i32,
    pub target_code: String,

    #[serde(rename = "type")]
    pub typ: String,
}

