use restson::{RestClient,RestPath,Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub value: String,
    pub stationcount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CountryResponse {
    Countries(Vec<Country>),
    Country(Country),
}

impl RestPath<()> for CountryResponse {
    fn get_path(_: ()) -> Result<String,Error> {
        Ok(format!("webservice/json/countries"))
    }
}
