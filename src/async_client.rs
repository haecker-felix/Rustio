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
    pub task_queue: Arc<Mutex<Vec<Task>>>,
    task_id: Arc<Mutex<i32>>,
    task_queue_enabled: Arc<Mutex<bool>>,
    pub base_url: Arc<String>,
    pub sender: Sender<Message>,
}

impl AsyncClient{
    pub fn new(base_url: &str, sender: Sender<Message>) -> Self{
        let mut task_queue = Arc::new(Mutex::new(Vec::new()));
        let task_id = Arc::new(Mutex::new(0));
        let task_queue_enabled = Arc::new(Mutex::new(true));
        let base_url = Arc::new(base_url.to_string());

        Self{
            task_queue,
            task_id,
            task_queue_enabled,
            base_url,
            sender,
        }
    }

    /// Adds a task to the queue. If you don't want to use a queue set "set_task_queue(false)"
    pub fn add_task(&mut self, task: Task){
        // Increase the unique task id
        *self.task_id.lock().unwrap() += 1;

        // add new task to the queue
        if *self.task_queue_enabled.lock().unwrap() {
            self.task_queue.lock().unwrap().push(task);
        }else{
            self.clear_queue();
            self.task_queue.lock().unwrap().push(task);
        }

    }

    pub fn clear_queue(&mut self){
        self.task_queue.lock().unwrap().clear();
        self.sender.send(Message::Clear).unwrap();
    }

    /// Enabled: Tasks will get added to a queue, and every task gets processed
    /// Disabled: A new tasks replaces the current one
    pub fn set_task_queue(&mut self, queue: bool){
        *self.task_queue_enabled.lock().unwrap() = queue;
    }

    /// Starts the task processing loop
    pub fn start_loop(&mut self){
        let task_queue = self.task_queue.clone();
        let task_id = self.task_id.clone();
        let task_queue_enabled = self.task_queue_enabled.clone();
        let base_url = self.base_url.clone();
        let sender = self.sender.clone();

        thread::spawn(move || {
            loop{
                // Check if a task is available
                if(!task_queue.lock().unwrap().is_empty()){
                    let mut sync_client = Client::new(&base_url);

                    // Get the next task to process
                    let task: Task = task_queue.lock().unwrap().pop().unwrap();

                    // Copy current task id
                    let working_id = task_id.lock().unwrap().clone();

                    // Actual work is being done here. Result get returned with mpsc::Sender.
                    let mut result_message = None;
                    match task.clone(){
                        Task::GetStationById(id) => {
                            let result = vec![sync_client.get_station_by_id(id).unwrap()];
                            result_message = Some(Message::StationAdd(result));
                        },
                        Task::GetAllStations => {
                            let result = sync_client.get_all_stations().unwrap();
                            result_message = Some(Message::StationAdd(result));
                        },
                        Task::GetAllCodecs => {
                            let result = sync_client.get_all_codecs().unwrap();
                            result_message = Some(Message::CodecAdd(result));
                        },
                        Task::GetAllCountries => {
                            let result = sync_client.get_all_countries().unwrap();
                            result_message = Some(Message::CountryAdd(result));
                        },
                        Task::GetAllLanguages => {
                            let result = sync_client.get_all_languages().unwrap();
                            result_message = Some(Message::LanguageAdd(result));
                        },
                        Task::GetAllStates => {
                            let result = sync_client.get_all_states().unwrap();
                            result_message = Some(Message::StateAdd(result));
                        },
                        Task::GetAllTags => {
                            let result = sync_client.get_all_tags().unwrap();
                            result_message = Some(Message::TagAdd(result));
                        },
                        Task::GetStats => {
                            let result = sync_client.get_stats().unwrap();
                            result_message = Some(Message::Stats(result));
                        },
                        Task::GetPlayableStationUrl(station) => {
                            let result = sync_client.get_playable_station_url(station).unwrap();
                            result_message = Some(Message::PlayableStationUrl(result));
                        },
                        Task::Search(search) => {
                            let result = sync_client.search(search).unwrap();
                            result_message = Some(Message::StationAdd(result));
                        },
                        _ => (),
                    };

                    if ( *task_queue_enabled.lock().unwrap() || working_id == *task_id.lock().unwrap()){
                        sender.send(result_message.unwrap());
                    }else {
                        debug!("Task id changed and queue is disabled, so don't send result message");
                    }
                }
            }
        });
    }
}
