use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use {Station, Codec, Country, Language, State, Tag, Stats};
use Client;
use StationSearch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message{
    StationAdd(Vec<Station>),
    StationRemove(Vec<Station>),

    CodecAdd(Vec<Codec>),
    CodecRemove(Vec<Codec>),

    CountryAdd(Vec<Country>),
    CountryRemove(Vec<Country>),

    LanguageAdd(Vec<Language>),
    LanguageRemove(Vec<Language>),

    StateAdd(Vec<State>),
    StateRemove(Vec<State>),

    TagAdd(Vec<Tag>),
    TagRemove(Vec<Tag>),

    Stats(Stats),
    PlayableStationUrl(String),

    Clear,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Task{
    GetStationById(u32),
    GetAllStations,
    GetAllCodecs,
    GetAllCountries,
    GetAllLanguages,
    GetAllStates,
    GetAllTags,
    GetStats,
    GetPlayableStationUrl(Station),
    Search(StationSearch),
}

pub struct AsyncClient{
    pub next_task: Arc<Mutex<Option<Task>>>,
    pub base_url: Arc<String>,
    pub sender: Sender<Message>,
}

impl AsyncClient{
    pub fn new(base_url: String, sender: Sender<Message>) -> Self{
        let mut next_task = Arc::new(Mutex::new(None));
        let base_url = Arc::new(base_url);

        Self{
            next_task,
            base_url,
            sender,
        }
    }

    pub fn set_task(&mut self, task: Task){
        *self.next_task.lock().unwrap() = Some(task);
    }

    pub fn clear_task(&mut self){
        *self.next_task.lock().unwrap() = None;
        self.sender.send(Message::Clear);
    }

    pub fn start_loop(&mut self){
        let next_task = self.next_task.clone();
        let base_url = self.base_url.clone();
        let sender = self.sender.clone();

        thread::spawn(move || {
            loop{
                // Check if a task is available
                if(next_task.lock().unwrap().clone().is_some()){
                    let mut sync_client = Client::new(&base_url);

                    // Get the task, and unset "next_task"
                    let task: Task = next_task.lock().unwrap().clone().unwrap();
                    *next_task.lock().unwrap() = None;

                    // Actual work is being done here. Result get returned with mpsc::Sender.
                    match task.clone(){
                        Task::GetStationById(id) => {
                            let result = vec![sync_client.get_station_by_id(id).unwrap()];
                            sender.send(Message::StationAdd(result));
                        },
                        Task::GetAllStations => {
                            let result = sync_client.get_all_stations().unwrap();
                            sender.send(Message::StationAdd(result));
                        },
                        Task::GetAllCodecs => {
                            let result = sync_client.get_all_codecs().unwrap();
                            sender.send(Message::CodecAdd(result));
                        },
                        Task::GetAllCountries => {
                            let result = sync_client.get_all_countries().unwrap();
                            sender.send(Message::CountryAdd(result));
                        },
                        Task::GetAllLanguages => {
                            let result = sync_client.get_all_languages().unwrap();
                            sender.send(Message::LanguageAdd(result));
                        },
                        Task::GetAllStates => {
                            let result = sync_client.get_all_states().unwrap();
                            sender.send(Message::StateAdd(result));
                        },
                        Task::GetAllTags => {
                            let result = sync_client.get_all_tags().unwrap();
                            sender.send(Message::TagAdd(result));
                        },
                        Task::GetStats => {
                            let result = sync_client.get_stats().unwrap();
                            sender.send(Message::Stats(result));
                        },
                        Task::GetPlayableStationUrl(station) => {
                            let result = sync_client.get_playable_station_url(station).unwrap();
                            sender.send(Message::PlayableStationUrl(result));
                        },
                        Task::Search(search) => {
                            let result = sync_client.search(search).unwrap();
                            sender.send(Message::StationAdd(result));
                        },
                        _ => (),
                    };
                }
            }
        });
    }
}
