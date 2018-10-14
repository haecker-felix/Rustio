use restson::{RestClient,RestPath,Error};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StationSearch {
    pub name: String,
    pub name_exact: bool,
    pub country: String,
    pub country_excat: bool,
    pub state: String,
    pub state_exact: bool,
    pub language: String,
    pub language_exact: bool,
    pub tag: String,
    pub tag_exact: bool,
    pub bitrate_min: u32,
    pub bitrate_max: u32,
    pub order: String,
    pub reverse: bool,
    pub offset: u32,
    pub limit: u32,
}

impl StationSearch{
    pub fn new() -> Self{
        StationSearch {
            name: "".to_string(),
            name_exact: false,
            country: "".to_string(),
            country_excat: false,
            state: "".to_string(),
            state_exact: false,
            language: "".to_string(),
            language_exact: false,
            tag: "".to_string(),
            tag_exact: false,
            bitrate_min: 0,
            bitrate_max: 99000,
            order: "".to_string(),
            reverse: false,
            offset: 0,
            limit: 100,
        }
    }

    pub fn search_for_name(name: String, exact: bool, limit: u32) -> Self{
        let mut search = Self::new();
        search.name = name;
        search.name_exact = exact;
        search.limit = limit;
        search
    }
}

impl RestPath<()> for StationSearch {
    fn get_path(_: ()) -> Result<String,Error> {
        Ok(String::from("webservice/json/stations/search"))
    }
}
