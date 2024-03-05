use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SaveFile {
    // general
    pub(crate) day_num: u32,
    // business
    pub(crate) name: String,
    pub(crate) cash: f32,
    pub(crate) scooters_working: u32,
    pub(crate) scooters_broken: u32,
    pub(crate) scooter_parts: u32,
    pub(crate) num_advertisements: u32,
    // weather
    pub(crate) current: String,
    pub(crate) forecast: String,
    pub(crate) temperature: String,
    pub(crate) season: String,
    pub(crate) days_of_season: u8,
}

impl SaveFile {

    pub(crate) fn to_ron(&self) -> String {
        // serialize self elements into string for saving
        // TODO: Handle errors
        ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).unwrap()
    }

    pub(crate) fn from_ron(ron_text: &str) -> Self {
        // deserialize loaded string into self elements
        ron::from_str(ron_text).unwrap()
    }

    pub(crate) fn load_save_file(file_path: &str) -> Result<SaveFile, std::io::Error> {
        let mut content = String::new();
        let mut file = File::open(file_path)?;
        file.read_to_string(&mut content)?;
        Ok(SaveFile::from_ron(content.as_str()))
    }

    pub(crate) fn write_save_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        file.write_all(self.to_ron().as_bytes())?;
        Ok(())
    }

    pub(crate) fn new(
        day_num: u32,
        name: String,
        cash: f32,
        scooters_working: u32,
        scooters_broken: u32,
        scooter_parts: u32,
        num_advertisements: u32,
        current: String,
        forecast: String,
        temperature: String,
        season: String,
        days_of_season: u8
    ) -> Self {
        SaveFile {
            day_num,
            name,
            cash,
            scooters_working,
            scooters_broken,
            scooter_parts,
            num_advertisements,
            current,
            forecast,
            temperature,
            season,
            days_of_season
        }
    }
}