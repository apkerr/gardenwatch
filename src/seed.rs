use core::fmt;
use std::{fs::File, io::{BufReader, BufWriter}, path::PathBuf, str::FromStr};

use actix_web::{HttpResponse, dev::Response, mime::APPLICATION_JSON, web, error, post, Error};
use futures::StreamExt;
use inquire::{InquireError, Select, Text};
use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantNames};

use crate::server::AppState;

const MAX_SIZE: usize = 262_144;

pub struct SeedBank {
    seeds: Seeds,
}
impl SeedBank {
    pub fn init() -> Self {
        SeedBank {
            seeds: Seeds::load("test.seeds".into()).unwrap()
        }  
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumString, VariantNames)]
#[strum(serialize_all = "mixed_case")]
pub enum PlantType {
    Fruit,
    #[strum(serialize="veg")]
    #[strum(serialize="vegetable")]
    Vegetable,
    Flower,
    Tree,
}
impl fmt::Display for PlantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Seeds {
    pub seeds: Vec<Seed>,
}
impl Seeds {
    pub fn load(f_name: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let s = Self::import(f_name)?;
        Ok(Self {seeds: s})
    }

    pub fn get_by_id(&self, id: i32) -> Result<Option<Seed>, Box<dyn std::error::Error>> {
        println!("Getting seed with id {id}");
        todo!()
    }

    pub fn get_by_type(&self, s_type: &str) -> Result<Option<Vec<Seed>>, Box<dyn std::error::Error>> {
        let p_type = PlantType::from_str(s_type).unwrap();
        println!("Getting seeds with type {p_type}");

        let matching_seeds: Vec<Seed> = self.seeds.iter().filter(|s| s.plant_type.eq(&p_type)).cloned().collect();
        match matching_seeds.len() {
            0 => Ok(None),
            _ => Ok(Some(matching_seeds))
        }   
    }

    pub fn get_all(&self) -> Result<Vec<Seed>, Box<dyn std::error::Error>> {
        println!("Getting all seeds");
        Ok(self.seeds.clone())
    }

    pub fn update() {

    }

    pub fn delete() {

    }

    pub fn export(&self, f_name: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("Exporting seeds to {}", &f_name.to_str().unwrap());
        let seeds = self.get_all()?;

        let file = File::create(f_name)?;
        let writer = BufWriter::new(&file);
        serde_json::to_writer_pretty(writer, &seeds)?;
        Ok(())
    }

    pub fn import(f_name: PathBuf) -> Result<Vec<Seed>, Box<dyn std::error::Error>> {
        let file = File::open(f_name)?;
        let reader = BufReader::new(file);
    
        let seeds = serde_json::from_reader(reader)?;
        Ok(seeds)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Seed {
    pub plant_type: PlantType,
    pub name: String,
    pub variety: String,
    pub description: String,
    pub company: String,
    pub company_id: String,
    pub start_instructions: String,
    pub germinate_time: String,
    pub germinate_temp: String,
    pub transplant_time: String,
    pub final_spacing: String,
    pub harvest_time: String,
}

impl Seed {
    pub fn create_new() -> Result<Seed, Box<dyn std::error::Error>> {
        println!("Creating a new seed");
        let types: Vec<&str> = vec!["Fruit", "Vegetable", "Flower", "Tree"];
        let plant_type_input: Result<&str, InquireError> = Select::new("What is the plant type?", types).prompt();

        let plant_type = match plant_type_input {
            Ok(choice) => {
                match choice {
                    "Fruit" => PlantType::Fruit,
                    "Vegetable" => PlantType::Vegetable,
                    "Flower" => PlantType::Flower,
                    "Tree" => PlantType::Tree,
                    _ => panic!("Bad!")
                }
            },
            Err(_) => panic!("Error choosing plant type!"),
        };

        let name = Text::new("What is the seed name?").prompt()?;
        let variety = Text::new("What is the seed variety?").prompt()?;
        let description = Text::new("What is the description?").prompt()?;
        let company = Text::new("What is the seed company?").prompt()?;
        let company_id = Text::new("What is the company seed id?").prompt()?;
        let start_instructions = Text::new("What are the starting instructions?").prompt()?;
        let germinate_time = Text::new("How long before germination?").prompt()?;
        let germinate_temp = Text::new("What is the best germination temperature?").prompt()?;
        let transplant_time = Text::new("How long before transplanting?").prompt()?;
        let final_spacing = Text::new("What is the final plant spacing?").prompt()?;
        let harvest_time = Text::new("How long before harvest?").prompt()?;
            
        let seed = Seed {
            plant_type,
            name,
            variety,
            description,
            company,
            company_id,
            start_instructions,
            germinate_time,
            germinate_temp,
            transplant_time,
            final_spacing,
            harvest_time,
        };

        Ok(seed)
    }
}

pub type SeedResponse = Response<Seed>;

#[derive(Deserialize)]
pub struct SeedRequest {
    pub plant_type: String,
}

// list all seeds
#[get("/seeds")]
pub async fn list(data: actix_web::web::Data<AppState>) -> HttpResponse {
    let seeds = &data.seed_bank.seeds;
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(seeds)
}

// get seed by field
// #[post("/seed")]
pub async fn get_seed(data: actix_web::web::Data<AppState>, payload: web::Json<SeedRequest>) -> Result<HttpResponse, Error> {

    let s = &data.seed_bank.seeds;
    let s_r = s.get_by_type(&payload.plant_type);

    let seed_ret = s_r.unwrap().unwrap();
    
    Ok(HttpResponse::Ok().json(seed_ret))
}

