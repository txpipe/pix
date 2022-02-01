use reqwest::blocking::Client;
use serde::Deserialize;

const BASE_URL: &str = "https://api.nft-maker.io";

pub struct NftMakerClient {
    apikey: String,
    client: Client,
}

impl NftMakerClient {
    pub fn new(apikey: String) -> Self {
        let client = Client::new();

        Self { apikey, client }
    }

    pub fn upload_nft(&self, nft_project_id: i32) -> anyhow::Result<UploadNftResponse> {
        let url = format!("{}/UploadNft/{}/{}", BASE_URL, self.apikey, nft_project_id);

        let upload_nft_response: UploadNftResponse = self.client.post(url).send()?.json()?;

        Ok(upload_nft_response)
    }
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
