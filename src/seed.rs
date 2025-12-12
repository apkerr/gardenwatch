pub mod seed {
    use core::fmt;
    use std::{any::Any, str::FromStr};

    use inquire::{InquireError, Select, Text};
    use native_db::{native_db, Builder, ToKey, db_type::Error};
    use native_model::{native_model, Model};
    use serde::{Deserialize, Serialize};
    use itertools::Itertools;
    use rand::Rng;
    use strum::{EnumString, VariantNames};

    use crate::{data::data, seed::seed::v1::Seed};

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumString, VariantNames)]
    #[strum(serialize_all = "mixed_case")]
    pub enum PlantType {
        Fruit,
        Vegetable,
        Flower,
        Tree,
    }
    impl fmt::Display for PlantType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }


    pub mod v1 {
        use super::*;
        
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[native_model(id = 1, version = 1)]
        #[native_db]
        pub struct Seed {
            #[primary_key]
            pub id: i32,
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
    }

    pub fn create_new() -> Result<(), Box<dyn std::error::Error>> {
        println!("Creating a new seed");
        // TODO: figure out how to use auto-increment for the id
        let id: i32 = rand::rng().random_range(0..1000);
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
        
        let db = data::open()?;
        let rw = db.rw_transaction()?;
        
        let seed = Seed {
            id,
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

        rw.insert(seed)?;
        rw.commit()?;

        Ok(())
    }

    pub fn get() {

    }

    pub fn get_by_id(id: i32) -> Result<Option<Seed>, Error> {
        println!("Getting seed with id {id}");
        let db = data::open()?;

        let r = db.r_transaction()?;
        Ok(r.get().primary(id)?)
    }

    pub fn get_by_type(s_type: &str) -> Result<Option<Vec<Seed>>, Error> {
        let p_type = PlantType::from_str(s_type).unwrap();
        println!("Getting seeds with type {p_type}");
        let db = data::open()?;
        let r = db.r_transaction()?;
        let seeds: Vec<Seed> = r.scan().primary()?.all()?.try_collect()?;
        let matching_seeds: Vec<Seed> = seeds.iter().filter(|s| s.plant_type.eq(&p_type)).cloned().collect();
        match matching_seeds.len() {
            0 => Ok(None),
            _ => Ok(Some(matching_seeds))
        }   
    }

    pub fn get_all() -> Result<Vec<Seed>, Error> {
        println!("Getting all seeds");
        let db = data::open()?;
        let r = db.r_transaction()?;
        Ok(r.scan().primary()?.all()?.try_collect()?)
    }

    pub fn update() {

    }

    pub fn delete() {

    }
}
