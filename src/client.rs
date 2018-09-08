use std::env;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;
use restson::{RestClient, RestPath, Error};

use {Codec, Country, Language, State, Station, Stats, Tag};
use {CodecResponse, CountryResponse, LanguageResponse, StateResponse, StationResponse, TagResponse};
use {PlayableStationUrl, StationSearch};


pub struct Client {
    rest_client: RestClient,
}

impl Client {
    pub fn new(base_url: &str) -> Client {
        let mut rest_client = match RestClient::new(base_url){
            Ok(rc) => rc,
            Err(err) => panic!("Could not create rest_client!")
        };

        Client { rest_client }
    }

    pub fn get_station_by_id(&mut self, id: u32) -> Result<Station, Error>{
        match self.rest_client.get(id)? {
            StationResponse::Station(station) => Ok(station),
            StationResponse::Stations(mut stations) => Ok(stations.pop().unwrap()),
        }
    }

    pub fn get_all_stations(&mut self) -> Result<Vec<Station>, Error>{
        match self.rest_client.get(())? {
            StationResponse::Stations(stations) => Ok(stations),
            _ => Err(Error::InvalidValue),
        }
    }

    pub fn get_all_codecs(&mut self) -> Result<Vec<Codec>, Error>{
        match self.rest_client.get(())? {
            CodecResponse::Codecs(codecs) => Ok(codecs),
            _ => Err(Error::InvalidValue),
        }
    }

    pub fn get_all_countries(&mut self) -> Result<Vec<Country>, Error>{
        match self.rest_client.get(())? {
            CountryResponse::Countries(countries) => Ok(countries),
            _ => Err(Error::InvalidValue),
        }
    }

    pub fn get_all_languages(&mut self) -> Result<Vec<Language>, Error>{
        match self.rest_client.get(())? {
            LanguageResponse::Languages(languages) => Ok(languages),
            _ => Err(Error::InvalidValue),
        }
    }

    pub fn get_all_states(&mut self) -> Result<Vec<State>, Error>{
        match self.rest_client.get(())? {
            StateResponse::States(states) => Ok(states),
            _ => Err(Error::InvalidValue),
        }
    }

    pub fn get_all_tags(&mut self) -> Result<Vec<Tag>, Error>{
        match self.rest_client.get(())? {
            TagResponse::Tags(tags) => Ok(tags),
            _ => Err(Error::InvalidValue),
        }
    }

    pub fn get_stats(&mut self) -> Result<Stats, Error>{
        self.rest_client.get(())
    }

    pub fn get_playable_station_url(&mut self, station: Station) -> Result<String, Error>{
        let result: PlayableStationUrl = self.rest_client.get(station)?;
        Ok(result.url)
    }

    pub fn search(&mut self, data: StationSearch) -> Result<Vec<Station>, Error>{
        match self.rest_client.post_capture((), &data)? {
            StationResponse::Stations(stations) => Ok(stations),
            _ => Err(Error::InvalidValue),
        }
    }
}
