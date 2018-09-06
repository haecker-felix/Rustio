use restson::{RestClient,RestPath,Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Language {
    pub value: String,
    pub stationcount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LanguageResponse {
    Languages(Vec<Language>),
    Language(Language),
}

impl RestPath<()> for LanguageResponse {
    fn get_path(_: ()) -> Result<String,Error> {
        Ok(format!("webservice/json/languages"))
    }
}
