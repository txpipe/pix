use reqwest::{
    blocking::Client,
    header::{HeaderMap, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};

static BASE_URL: &str = "https://api.nft-maker.io";

pub struct NftMakerClient {
    apikey: String,
    client: Client,
}

impl NftMakerClient {
    pub fn new(apikey: String) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self { apikey, client })
    }

    pub fn upload_nft(
        &self,
        nft_project_id: i32,
        body: &UploadNftRequest,
    ) -> anyhow::Result<UploadNftResponse> {
        let url = format!("{}/UploadNft/{}/{}", BASE_URL, self.apikey, nft_project_id);

        let upload_nft_response: UploadNftResponse =
            self.client.post(url).json(body).send()?.json()?;

        Ok(upload_nft_response)
    }

    //Weirdly enough create project also returns UploadNftResponse
    pub fn create_project(&self, body: &CreateProjectClass) -> anyhow::Result<UploadNftResponse> {
        let url = format!("{}/CreateProject/{}", BASE_URL, self.apikey);

        let create_project_response: reqwest::blocking::Response =
            self.client.post(url).json(body).send()?;

        println!("{:?}", create_project_response);

        Ok(create_project_response.json()?)
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
pub struct UploadNftRequest {
    pub asset_name: Option<String>,
    pub preview_image_nft: NftFile,
    pub subfiles: Vec<NftFile>,
    pub metadata: Option<String>,
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
pub struct CreateProjectClass {
    pub projectname: Option<String>,
    pub description: Option<String>,
    pub projecturl: Option<String>,
    pub tokenname_prefix: Option<String>,
    pub policy_expires: bool,
    pub policy_locks_date_time: Option<String>,
    pub payout_walletaddress: Option<String>,
    pub max_nft_supply: i32,
    pub policy: PolicyClass,
    pub metadata: Option<String>,
    pub address_expiretime: i32,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyClass {
    pub policy_id: Option<String>,
    pub private_verifykey: Option<String>,
    pub private_signingkey: Option<String>,
    pub policy_script: Option<String>,
}
