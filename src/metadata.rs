use serde::Serialize;
use serde_json::{json, Map, Value};

use crate::config::AppConfig;

#[derive(Serialize)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

pub type Attributes = Vec<Attribute>;

pub fn build_template(config: &AppConfig) -> String {
    let mut attributes = Map::new();

    for (index, attr) in config.layers.iter().enumerate() {
        let template = Value::String(format!("<attribute{}>", index));

        attributes.insert(attr.to_owned(), template);
    }

    let mut asset_name = Map::new();

    asset_name.insert(
        String::from("name"),
        Value::String(String::from("<display_name>")),
    );

    asset_name.insert(
        String::from("image"),
        Value::String(String::from("<ipfs_link>")),
    );

    asset_name.insert(
        String::from("mediaType"),
        Value::String(String::from("<mime_type>")),
    );

    asset_name.insert(
        String::from("files"),
        json!([
          {
            "name": "<display_name>",
            "mediaType": "<mime_type>",
            "src": "<ipfs_link>"
          }
        ]),
    );

    asset_name.insert(String::from("attributes"), Value::Object(attributes));

    if let Some(twitter) = &config.twitter {
        asset_name.insert(String::from("twitter"), Value::String(twitter.to_owned()));
    }

    if let Some(website) = &config.website {
        asset_name.insert(String::from("website"), Value::String(website.to_owned()));
    }

    if let Some(copyright) = &config.copyright {
        asset_name.insert(
            String::from("copyright"),
            Value::String(copyright.to_owned()),
        );
    }

    let json = json!({
      "721": {
        "<policy_id>": {
          "<asset_name>": asset_name
        },
        "version": "1.0"
      }
    });

    serde_json::to_string_pretty(&json).expect("this should not fail")
}
