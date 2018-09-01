use restson::{RestClient,RestPath,Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub value: String,
    pub country: String,
    pub stationcount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StateResponse {
    States(Vec<State>),
    State(State),
}

impl RestPath<()> for StateResponse {
    fn get_path(_: ()) -> Result<String,Error> {
        Ok(format!("webservice/json/states"))
    }
}
