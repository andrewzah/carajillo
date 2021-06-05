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
            num: String::from("10"),
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
    #[serde(rename = "$value")]
    pub channel: OpenDictChannel,
}


#[derive(Debug, Deserialize)]
pub struct OpenDictChannel {
    // #[serde(rename = "$value")]
    // pub item: Vec<OpenDictItem>,

    pub description: String,

    pub link: String,

    // #[serde(rename = "$value")]
    pub num: i32,

    // #[serde(rename = "$value")]
    pub start: i32,

    // #[serde(rename = "$value")]
    pub title: String,

    // #[serde(rename = "$value")]
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct OpenDictItem {
    // #[serde(rename = "sense")]
    //  pub sense: Vec<OpenDictSense>,

    // pub word: String,
}

// TODO: typ->type
#[derive(Debug, Deserialize)]
pub struct OpenDictSense {
    definition: String,
    link: Vec<String>,
    origin: String,
    pos: String,
    sense_no: i32,
    target_code: String,
    typ: String,
}

