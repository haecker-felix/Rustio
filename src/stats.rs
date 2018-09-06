use restson::{RestClient,RestPath,Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub stations: String,
    pub stations_broken: String,
    pub tags: String,
    pub clicks_last_hour: String,
    pub clicks_last_day: String,
    pub languages: String,
    pub countries: String,
}

impl RestPath<()> for Stats {
    fn get_path(_: ()) -> Result<String,Error> {
        Ok(format!("webservice/json/stats"))
    }
}
