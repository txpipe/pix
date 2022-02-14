use serde_json::{json, Map, Number, Value};

use crate::config::AppConfig;

pub fn new_project_template(
    config: &AppConfig,
    metadata: String,
    expiration_time: String,
) -> String {
    let mut project_map = Map::new();

    project_map.insert(
        String::from("projectname"),
        Value::String(config.name.to_string()),
    );

    project_map.insert(String::from("description"), Value::Null);

    project_map.insert(
        String::from("projecturl"),
        Value::String(
            config
                .website
                .as_ref()
                .unwrap_or(&String::new())
                .to_string(),
        ),
    );

    project_map.insert(String::from("tokennamePrefix"), Value::Null);

    //In order to make an NFT with native script in Cardano a policy that can make or burn tokens must expire
    //i.e. after a certain time it can no longer make or burn tokens.
    project_map.insert(String::from("policyExpires"), Value::Bool(true));

    project_map.insert(
        String::from("policyLocksDateTime"),
        Value::String(expiration_time),
    );

    //Max supply for each unique token should always be 1 for NFTs
    project_map.insert(
        String::from("maxNftSupply"),
        Value::Number(Number::from(1 as i32)),
    );

    project_map.insert(String::from("policy"), Value::Null);

    project_map.insert(String::from("metadata"), Value::String(metadata));

    //Give 20 minutes reservation time for minting one or more NFTs at the same time
    project_map.insert(
        String::from("addressExpiretime"),
        Value::Number(Number::from(20 as i32)),
    );

    let json = json!(project_map);

    serde_json::to_string_pretty(&json).expect("this should not fail")
}
