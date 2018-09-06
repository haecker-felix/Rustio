use restson::{RestClient,RestPath,Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub value: String,
    pub stationcount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TagResponse {
    Tags(Vec<Tag>),
    Tag(Tag),
}

impl RestPath<()> for TagResponse {
    fn get_path(_: ()) -> Result<String,Error> {
        Ok(format!("webservice/json/tags"))
    }
}
