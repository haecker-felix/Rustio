use restson::{RestClient,RestPath,Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Station {
    pub name: String,
    pub language: String,
    pub country: String,
    pub state: String,
    pub tags: String,
    pub codec: String,
    pub votes: String,
    pub homepage: String,
    pub favicon: String,

    pub id: String,
    pub changeuuid: String,
    pub stationuuid: String,

    pub url: String,
    pub ip: String,
    pub bitrate: String,
    pub hls: String,

    pub lastchangetime: String,
    pub lastcheckok: String,
    pub lastchecktime: String,
    pub lastcheckoktime: String,
    pub clicktimestamp: String,
    pub clickcount: String,
    pub clicktrend: String,
}

impl PartialEq for Station {
    fn eq(&self, other: &Station) -> bool {
        self.id == other.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StationResponse {
    Stations(Vec<Station>),
    Station(Station),
}

impl RestPath<()> for StationResponse {
    fn get_path(_: ()) -> Result<String,Error> {
        Ok(format!("webservice/json/stations"))
    }
}

impl RestPath<u32> for StationResponse {
    fn get_path(param: u32) -> Result<String,Error> {
        Ok(format!("webservice/json/stations/byid/{}", param))
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PlayableStationUrl {
    pub name: String,
    pub url: String,
}

impl RestPath<u32> for PlayableStationUrl {
    fn get_path(param: u32) -> Result<String,Error> {
        Ok(format!("webservice/v2/xml/url/{}", param))
    }
}
