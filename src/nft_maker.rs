use reqwest::blocking::Client;

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

    pub fn upload_nft(&self, nft_project_id: &str) {
        let url = format!("{}/UploadNft/{}/{}", BASE_URL, self.apikey, nft_project_id);

        self.client.post(url);
    }
}
