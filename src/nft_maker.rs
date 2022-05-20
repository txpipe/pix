use reqwest::{
    blocking::Client,
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};

use crate::config::{AppConfig, NftMakerNetwork, NftProjectId};

static MAINNET_URL: &str = "https://api.nft-maker.io";
static TESTNET_URL: &str = "https://api-testnet.nft-maker.io/v2";

impl NftMakerNetwork {
    pub fn to_url_string(&self) -> String {
        match self {
            Self::Mainnet => String::from(MAINNET_URL),
            Self::Testnet => String::from(TESTNET_URL),
        }
    }
}

pub struct NftMakerClient {
    url: String,
    network: NftMakerNetwork,
    apikey: String,
    client: Client,
}

impl NftMakerClient {
    pub fn new(apikey: String, network: NftMakerNetwork) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        if network.is_testnet() {
            let bearer = format!("Bearer {}", apikey);

            headers.insert(AUTHORIZATION, bearer.parse().unwrap());
        }

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            apikey,
            client,
            network,
            url: network.to_url_string(),
        })
    }

    pub fn upload_nft(
        &self,
        nft_project_id: &NftProjectId,
        asset_name: String,
        mimetype: String,
        displayname: String,
        file_from_base64: String,
        metadata_placeholder: Vec<MetadataPlaceholder>,
    ) -> anyhow::Result<()> {
        match self.network {
            NftMakerNetwork::Mainnet => {
                let url = format!("{}/UploadNft/{}/{}", self.url, self.apikey, nft_project_id);

                let body = UploadNftRequest {
                    asset_name: Some(asset_name),
                    preview_image_nft: NftFile {
                        mimetype: Some(mimetype),
                        description: None,
                        displayname: Some(displayname),
                        file_from_IPFS: None,
                        file_froms_url: None,
                        file_from_base64: Some(file_from_base64),
                        metadata_placeholder,
                    },
                    subfiles: None,
                    metadata: None,
                };

                let _ = self.client.post(url).json(&body).send()?;
            }
            NftMakerNetwork::Testnet => {
                let url = format!("{}/UploadNft/{}", self.url, nft_project_id);

                let body = UploadNftRequestV2 {
                    tokenname: Some(asset_name),
                    displayname: Some(displayname),
                    description: None,
                    preview_image_nft: NftFileV2 {
                        mimetype: Some(mimetype),
                        file_from_base64: Some(file_from_base64),
                        file_from_IPFS: None,
                        file_froms_url: None,
                    },
                    subfiles: None,
                    metadata_placeholder,
                    metadata_override: None,
                    price_in_lovelace: None,
                };

                let _ = self.client.post(url).json(&body).send()?;
            }
        }

        Ok(())
    }

    pub fn create_project(
        &self,
        body: &CreateProjectRequest,
    ) -> anyhow::Result<CreateProjectResponse> {
        let url = format!("{}/CreateProject/{}", self.url, self.apikey);

        let create_project_response: CreateProjectResponse =
            self.client.post(url).json(body).send()?.json()?;

        Ok(create_project_response)
    }
}

#[derive(Serialize, Debug)]
pub struct MetadataPlaceholder {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct NftFile {
    pub mimetype: Option<String>,
    pub file_from_base64: Option<String>,
    pub file_froms_url: Option<String>,
    pub file_from_IPFS: Option<String>,
    pub description: Option<String>,
    pub displayname: Option<String>,
    pub metadata_placeholder: Vec<MetadataPlaceholder>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct NftFileV2 {
    pub mimetype: Option<String>,
    pub file_from_base64: Option<String>,
    pub file_froms_url: Option<String>,
    pub file_from_IPFS: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct NftSubFileV2 {
    pub subfile: NftFileV2,
    pub description: Option<String>,
    pub metadata_placeholder: Vec<MetadataPlaceholder>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadNftRequest {
    pub asset_name: Option<String>,
    pub preview_image_nft: NftFile,
    pub subfiles: Option<Vec<NftFile>>,
    pub metadata: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadNftRequestV2 {
    pub tokenname: Option<String>,
    pub displayname: Option<String>,
    pub description: Option<String>,
    pub preview_image_nft: NftFileV2,
    pub subfiles: Option<Vec<NftSubFileV2>>,
    pub metadata_placeholder: Vec<MetadataPlaceholder>,
    pub metadata_override: Option<String>,
    pub price_in_lovelace: Option<i64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadNftResponse {
    pub nft_id: i32,
    pub nft_uid: Option<String>,
    pub ipfs_hash_mainnft: Option<String>,
    pub ipfs_hash_sub_files: Option<Vec<String>>,
    pub metadata: Option<String>,
    pub asset_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NftDetails {
    pub id: i32,
    pub ipfshash: Option<String>,
    pub state: Option<String>,
    pub name: Option<String>,
    pub displayname: Option<String>,
    pub detaildata: Option<String>,
    pub minted: bool,
    pub receiveraddress: Option<String>,
    pub selldate: Option<String>, // DateTime
    pub soldby: Option<String>,
    pub reserveduntil: Option<String>, //DateTime
    pub policyid: Option<String>,
    pub assetid: Option<String>,
    pub assetname: Option<String>,
    pub fingerprint: Option<String>,
    pub initialminttxhash: Option<String>,
    pub title: Option<String>,
    pub series: Option<String>,
    pub ipfs_gateway_address: Option<String>,
    pub metadata: Option<String>,
    pub single_price: Option<i64>,
    pub uid: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    pub projectname: Option<String>,
    pub description: Option<String>,
    pub projecturl: Option<String>,
    pub tokenname_prefix: Option<String>,
    pub policy_expires: bool,
    pub policy_locks_date_time: Option<String>,
    pub payout_walletaddress: Option<String>,
    pub max_nft_supply: i32,
    pub policy: Policy,
    pub metadata: Option<String>,
    pub address_expiretime: i32,
}

impl CreateProjectRequest {
    pub fn new(config: &AppConfig, metadata_info: String, expiration_time: String) -> Self {
        Self {
            projectname: Some(config.name.to_owned()),
            description: None,
            projecturl: None,
            tokenname_prefix: None,
            policy_expires: true,
            policy_locks_date_time: Some(expiration_time),
            payout_walletaddress: None,
            max_nft_supply: 1,
            policy: Default::default(),
            metadata: Some(metadata_info),
            address_expiretime: 20,
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    pub policy_id: Option<String>,
    pub private_verifykey: Option<String>,
    pub private_signingkey: Option<String>,
    pub policy_script: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectResponse {
    pub project_id: i32,
    pub metadata: Option<String>,
    pub policy_id: Option<String>,
    pub policy_script: Option<String>,
    // Datetime
    pub policy_expiration: Option<String>,
    pub uid: Option<String>,
}
