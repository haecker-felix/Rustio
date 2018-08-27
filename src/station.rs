#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash)]
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

impl Station{}

impl PartialEq for Station {
    fn eq(&self, other: &Station) -> bool {
        self.id == other.id
    }
}
