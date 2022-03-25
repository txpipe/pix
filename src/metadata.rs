use serde::Serialize;
use serde_json::{json, Map, Value};

use crate::config::AppConfig;

#[derive(Serialize)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

pub fn build_template(config: &AppConfig) -> String {
    let mut attributes = Map::new();

    for (index, attr) in config.layers.iter().enumerate() {
        let template = Value::String(format!("<attribute{}>", index));

        attributes.insert(
            attr.display_name.as_ref().unwrap_or(&attr.name).to_owned(),
            template,
        );
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

pub fn build_with_attributes(
    config: &AppConfig,
    attributes: Map<String, Value>,
    count: usize,
) -> String {
    let mut asset_name = Map::new();

    asset_name.insert(
        String::from("name"),
        Value::String(config.display_name.as_ref().map_or_else(
            || format!("{} #{}", config.name, count),
            |display_name| format!("{} #{}", display_name, count),
        )),
    );

    asset_name.insert(
        String::from("image"),
        Value::String(String::from("<ipfs_link>")),
    );

    asset_name.insert(
        String::from("mediaType"),
        Value::String(String::from("image/png")),
    );

    asset_name.insert(
        String::from("files"),
        json!([
          {
            "name": config.display_name.as_ref().map_or_else(
                || format!("{} #{}", config.name, count),
                |display_name| format!("{} #{}", display_name, count),
            ),
            "mediaType": "image/png",
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

    let policy_id = config
        .policy_id
        .clone()
        .unwrap_or_else(|| String::from("<policy_id>"));

    let json = json!({
      "721": {
        policy_id: {
          format!("{}{}", config.name, count): asset_name
        },
        "version": "1.0"
      }
    });

    serde_json::to_string_pretty(&json).expect("this should not fail")
}
