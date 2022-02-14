use crate::{
    config::AppConfig,
    nft_maker::{CreateProjectClass, PolicyClass},
};

pub fn new_project_body(
    config: &AppConfig,
    metadata_info: String,
    expiration_time: String,
) -> CreateProjectClass {
    CreateProjectClass {
        projectname: Some(config.name.to_string()),
        description: None,
        projecturl: config.website.clone(),
        tokenname_prefix: None,
        policy_expires: true,
        policy_locks_date_time: Some(expiration_time),
        payout_walletaddress: None,
        max_nft_supply: 1,
        policy: PolicyClass {
            policy_id: None,
            private_verifykey: None,
            private_signingkey: None,
            policy_script: None,
        },
        metadata: Some(metadata_info),
        address_expiretime: 20,
    }
}
