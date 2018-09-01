use restson::{RestClient,RestPath,Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Codec {
    pub value: String,
    pub stationcount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CodecResponse {
    Codecs(Vec<Codec>),
    Codec(Codec),
}

impl RestPath<()> for CodecResponse {
    fn get_path(_: ()) -> Result<String,Error> {
        Ok(format!("webservice/json/codecs"))
    }
}
