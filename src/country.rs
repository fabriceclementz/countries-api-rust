use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::error::CountryError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Country {
    pub code: String,
    pub name: String,
    pub native: String,
    pub phone: String,
    pub continent: String,
    pub capital: String,
    pub currency: String,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CountryRepository {
    countries: Vec<Country>,
}

impl CountryRepository {
    pub fn new() -> Result<Self, CountryError> {
        let countries = load_countries_from_csv()?;
        Ok(Self { countries })
    }

    #[tracing::instrument(skip(self))]
    pub fn all(&self) -> &Vec<Country> {
        info!("fetching all countries");
        &self.countries
    }

    #[tracing::instrument(skip(self))]
    pub fn by_code(&self, code: &str) -> Result<&Country, CountryError> {
        info!("searching for country with: {}", code);
        for country in &self.countries {
            if country.code == code {
                return Ok(country);
            }
        }

        debug!("country not found for: {}", code);

        Err(CountryError::CountryNotFound)
    }
}

#[tracing::instrument]
fn load_countries_from_csv() -> Result<Vec<Country>, CountryError> {
    let path = "src/data/countries.csv";
    info!("loading countries at {}", path);

    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;

    let mut countries = vec![];
    for result in reader.deserialize() {
        let country = result?;
        countries.push(country);
    }

    info!("{} countries successfully loaded", countries.len());

    Ok(countries)
}
