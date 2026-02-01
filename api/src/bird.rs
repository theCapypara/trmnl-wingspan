use crate::config::Config;
use serde::{Deserialize, Serialize, de};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Locale {
    De,
    Dk,
    Es,
    Fr,
    Jp,
    Lt,
    Nl,
    Pl,
    Pt,
    Tr,
    Uk,
}

const ALL_LOCALES: &[(&str, Locale)] = &[
    ("de", Locale::De),
    ("dk", Locale::Dk),
    ("es", Locale::Es),
    ("fr", Locale::Fr),
    ("jp", Locale::Jp),
    ("lt", Locale::Lt),
    ("nl", Locale::Nl),
    ("pl", Locale::Pl),
    ("pt", Locale::Pt),
    ("tr", Locale::Tr),
    ("uk", Locale::Uk),
];

impl TryInto<Locale> for &String {
    type Error = ();

    fn try_into(self) -> Result<Locale, Self::Error> {
        match self.as_str() {
            "de" => Ok(Locale::De),
            "dk" => Ok(Locale::Dk),
            "es" => Ok(Locale::Es),
            "fr" => Ok(Locale::Fr),
            "jp" => Ok(Locale::Jp),
            "lt" => Ok(Locale::Lt),
            "nl" => Ok(Locale::Nl),
            "pl" => Ok(Locale::Pl),
            "pt" => Ok(Locale::Pt),
            "tr" => Ok(Locale::Tr),
            "uk" => Ok(Locale::Uk),
            _ => Err(()),
        }
    }
}

fn x_value<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    if let Ok(str_val) = String::deserialize(deserializer) {
        Ok(&str_val == "X")
    } else {
        Ok(false)
    }
}

pub type BirdMasterFile = Vec<BirdMaster>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BirdMaster {
    pub id: i32,
    #[serde(rename(deserialize = "Scientific name"))]
    pub scientific_name: String,
    #[serde(rename(deserialize = "Common name"))]
    pub common_name: String,
    #[serde(rename(deserialize = "Flavor text"))]
    pub flavor_text: String,
    #[serde(rename(deserialize = "Set"))]
    pub set: String,
    #[serde(rename(deserialize = "Color"))]
    pub color: Option<String>,
    #[serde(rename(deserialize = "Nest type"))]
    pub nest_type: String,
    #[serde(rename(deserialize = "Power text"))]
    pub power_text: Option<String>,
    #[serde(rename(deserialize = "Predator"), deserialize_with = "x_value")]
    pub power_is_predator: bool,
    #[serde(rename(deserialize = "Flocking"), deserialize_with = "x_value")]
    pub power_is_flocking: bool,
    #[serde(rename(deserialize = "Bonus card"), deserialize_with = "x_value")]
    pub power_is_bonus_card_related: bool,
    #[serde(rename(deserialize = "Victory points"))]
    pub victory_points: f32,
    #[serde(rename(deserialize = "Egg limit"))]
    pub egg_limit: f32,
    #[serde(rename(deserialize = "Wingspan"))]
    pub wingspan: Value, // Int, Float, String (*)
    #[serde(rename(deserialize = "Forest"), deserialize_with = "x_value")]
    pub can_live_forest: bool,
    #[serde(rename(deserialize = "Grassland"), deserialize_with = "x_value")]
    pub can_live_grassland: bool,
    #[serde(rename(deserialize = "Wetland"), deserialize_with = "x_value")]
    pub can_live_wetland: bool,
    #[serde(rename(deserialize = "Invertebrate"))]
    pub food_invertebrate: Option<f32>,
    #[serde(rename(deserialize = "Seed"))]
    pub food_seed: Option<f32>,
    #[serde(rename(deserialize = "Fish"))]
    pub food_fish: Option<f32>,
    #[serde(rename(deserialize = "Fruit"))]
    pub food_fruit: Option<f32>,
    #[serde(rename(deserialize = "Rodent"))]
    pub food_rodent: Option<f32>,
    #[serde(rename(deserialize = "Nectar"))]
    pub food_nectar: Option<f32>,
    #[serde(rename(deserialize = "Wild (food)"))]
    pub food_wild: Option<f32>,
    #[serde(rename(deserialize = "/ (food cost)"), deserialize_with = "x_value")]
    pub food_cost_is_either: bool,
    #[serde(rename(deserialize = "* (food cost)"), deserialize_with = "x_value")]
    pub food_cost_has_note: bool,
    #[serde(rename(deserialize = "North America"), deserialize_with = "x_value")]
    pub lives_in_north_america: bool,
    #[serde(rename(deserialize = "Central America"), deserialize_with = "x_value")]
    pub lives_in_central_america: bool,
    #[serde(rename(deserialize = "South America"), deserialize_with = "x_value")]
    pub lives_in_south_america: bool,
    #[serde(rename(deserialize = "Europe"), deserialize_with = "x_value")]
    pub lives_in_europe: bool,
    #[serde(rename(deserialize = "Asia"), deserialize_with = "x_value")]
    pub lives_in_asia: bool,
    #[serde(rename(deserialize = "Africa"), deserialize_with = "x_value")]
    pub lives_in_africa: bool,
    #[serde(rename(deserialize = "Oceania"), deserialize_with = "x_value")]
    pub lives_in_oceania: bool,
}

#[derive(Debug, Deserialize)]
pub struct I18nFile {
    pub birds: HashMap<String, BirdI18n>,
}

#[derive(Debug, Deserialize)]
pub struct BirdI18n {
    #[serde(rename = "Scientific name")]
    pub scientific_name: Option<String>,
    #[serde(rename = "Common name")]
    pub common_name: Option<String>,
    #[serde(rename = "Power text")]
    pub power_text: Option<String>,
    #[serde(rename = "Flavor text")]
    pub flavor_text: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BirdResult {
    pub scientific_name: String,
    pub common_name: String,
    pub power_text: String,
    pub flavor_text: String,
    pub info: BirdMaster,
}

#[derive(Debug)]
pub struct Bird {
    pub master: BirdMaster,
    pub i8n: HashMap<Locale, BirdI18n>,
}

impl Bird {
    pub fn produce(&self, locale: Option<Locale>) -> BirdResult {
        let locale_entry = locale.and_then(|locale| self.i8n.get(&locale));
        if let Some(locale_entry) = locale_entry {
            BirdResult {
                scientific_name: locale_entry
                    .scientific_name
                    .clone()
                    .unwrap_or_else(|| self.master.scientific_name.clone()),
                common_name: locale_entry
                    .common_name
                    .clone()
                    .unwrap_or_else(|| self.master.common_name.clone()),
                power_text: locale_entry
                    .power_text
                    .clone()
                    .or_else(|| self.master.power_text.clone())
                    .unwrap_or_default(),
                flavor_text: locale_entry
                    .flavor_text
                    .clone()
                    .unwrap_or_else(|| self.master.flavor_text.clone()),
                info: self.master.clone(),
            }
        } else {
            BirdResult {
                scientific_name: self.master.scientific_name.clone(),
                common_name: self.master.common_name.clone(),
                power_text: self.master.power_text.clone().unwrap_or_default(),
                flavor_text: self.master.flavor_text.clone(),
                info: self.master.clone(),
            }
        }
    }
}

fn read_i18n_file(
    locales: &mut HashMap<Locale, I18nFile>,
    locale: Locale,
    path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let i18n_fp = File::open(path)?;
    let reader = BufReader::new(i18n_fp);
    let i18n_file: I18nFile = serde_json::from_reader(reader)?;
    locales.insert(locale, i18n_file);
    Ok(())
}

pub fn load_birds(config: &Config) -> HashMap<String, Bird> {
    let mut out = HashMap::new();

    let master_fp = File::open(config.wingsearch.join("src/assets/data/master.json"))
        .expect("Failed to open master bird file.");
    let reader = BufReader::new(master_fp);
    let master_file: BirdMasterFile =
        serde_json::from_reader(reader).expect("Failed to parse master bird file.");

    let mut locale_files = HashMap::new();
    for (code, locale) in ALL_LOCALES {
        let path = config
            .wingsearch
            .join(format!("src/assets/data/i18n/{code}.json"));
        if let Err(err) = read_i18n_file(&mut locale_files, *locale, path) {
            eprintln!("Failed to read localization for {code}: {err}");
        }
    }

    for entry in master_file {
        let id_str = entry.id.to_string();
        let mut i18ns = HashMap::new();
        for (locale, file) in &mut locale_files {
            if let Some(bird_in_locale_file) = file.birds.remove(&id_str) {
                i18ns.insert(*locale, bird_in_locale_file);
            }
        }
        out.insert(
            id_str,
            Bird {
                master: entry,
                i8n: i18ns,
            },
        );
    }

    out
}
