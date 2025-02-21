/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2023, 2024, 2025  Dirk Stolle

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
 -------------------------------------------------------------------------------
*/

use super::configuration::DbConfiguration;
mod ecdc;
mod save;
mod owid;
mod owid_etl_compact;
mod who;
use crate::db::ecdc::DbEcdc;
use crate::db::owid::DbOwid;
use crate::db::owid_etl_compact::DbOwidEtlCompact;
use crate::db::who::DbWho;

pub struct Db
{
  config: DbConfiguration
}

/// enumeration for supported CSV formats
#[derive(Debug, Eq, PartialEq)]
pub enum CsvType
{
  /// ECDC's CSV format as used at <https://data.europa.eu/euodp/data/dataset/covid-19-coronavirus-data>
  /// before 14th December 2020
  Ecdc,

  /// Our World In Data's CSV format as used at <https://covid.ourworldindata.org/data/owid-covid-data.csv>
  Owid,

  /// Our World In Data's compact CSV form ETL as used at <https://catalog.ourworldindata.org/garden/covid/latest/compact/compact.csv>
  OwidEtlCompact,

  /// WHO's CSV format as used at <https://covid19.who.int/data>
  Who
}

impl Db
{
  /**
   * Creates a new instance.
   *
   * @config   application configuration
   * @return   Returns a Result containing the Db object, if successful.
   *           Returns a string with an error message, if the configuration
   *           seems to be invalid.
   */
  pub fn new(config: &DbConfiguration) -> Result<Db, String>
  {
    if config.db_path.is_empty()
    {
      return Err("Path for SQLite database must not be an empty string!".to_string());
    }
    if config.csv_input_file.is_empty()
    {
      return Err("Path of CSV file must be set to a non-empty string!".to_string());
    }

    Ok(Db
    {
      config: DbConfiguration
      {
        csv_input_file: config.csv_input_file.clone(),
        db_path: config.db_path.clone()
      }
    })
  }

  /**
   * Creates the SQLite database from a CSV file.
   *
   * @return Returns whether the operation was successful.
   */
  pub fn create_db(&self) -> bool
  {
    if !std::path::Path::new(&self.config.csv_input_file).is_file()
    {
      eprintln!("Error: {} does not exist or is not a file.",
                self.config.csv_input_file);
      return false;
    }
    match Db::get_csv_type(&self.config.csv_input_file)
    {
      Some(CsvType::Ecdc) => {
        let db = DbEcdc::new(&self.config).unwrap();
        db.create_db()
      },
      Some(CsvType::Owid) => {
        let db = DbOwid::new(&self.config).unwrap();
        db.create_db()
      },
      Some(CsvType::OwidEtlCompact) => {
        let db = DbOwidEtlCompact::new(&self.config).unwrap();
        db.create_db()
      },
      Some(CsvType::Who) => {
        let db = DbWho::new(&self.config).unwrap();
        db.create_db()
      },
      None => {
        eprintln!("File {} does not seem to contain a known CSV format!",
                  self.config.csv_input_file);
        eprintln!("Only CSV format as used by the ECDC, Our World In Data or the WHO can be detected.");
        false
      }
    }
  }

  /**
   * Gets the CSV type of the file by checking the CSV headers.
   *
   * @param reader    an opened CSV reader
   * @return Returns the detected CSV type, if a match was found.
   *         Returns None otherwise.
   */
  pub fn get_csv_type(file_path: &str) -> Option<CsvType>
  {
    use std::io::{BufRead, BufReader};

    let file = match std::fs::File::open(file_path)
    {
      Ok(f) => f,
      Err(e) => {
        eprintln!("Failed to open file {}: {}", file_path, e);
        return None;
      }
    };
    let reader = BufReader::new(file);
    let line_one = match reader.lines().next()
    {
      None => return None,
      Some(Ok(line)) => line,
      Some(Err(_)) => return None
    };

    if line_one.contains("dateRep,day,month,year,cases,deaths")
    {
      return Some(CsvType::Ecdc);
    }
    if line_one.contains("iso_code,continent,location,date,")
      && line_one.contains(",new_cases,") && line_one.contains(",new_deaths,")
    {
      return Some(CsvType::Owid);
    }
    if line_one.contains("country,date,total_cases,new_cases,")
      && line_one.contains(",new_deaths,") && line_one.contains(",code,continent,")
    {
      return Some(CsvType::OwidEtlCompact);
    }
    if line_one.contains("Date_reported,Country_code,Country,WHO_region")
    {
      return Some(CsvType::Who);
    }

    // Unknown CSV type.
    None
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  /**
   * Gets path to the ECDC corona_daily.csv file in data directory.
   *
   * @return Returns path of the CSV file.
   */
  fn get_ecdc_csv_path() -> String
  {
    use std::path::Path;

    let csv_path = Path::new(file!()) // current file: src/db/mod.rs
      .parent()
      .unwrap() // parent: src/db/
      .join("..") // up one directory
      .join("..") // up another directory
      .join("data") // into directory data/
      .join("corona-daily-ecdc-2020-12-14.csv"); // and to the corona-daily.csv file;
    csv_path.to_str().unwrap().to_string()
  }

  #[test]
  fn get_csv_type_ecdc()
  {
    let detected = Db::get_csv_type(&get_ecdc_csv_path());
    assert_eq!(detected, Some(CsvType::Ecdc));
  }

  #[test]
  fn get_csv_type_ecdc_no_bom_14_and_7()
  {
    let mut path = std::env::temp_dir();
    path.push("ecdc_no_bom_14_7.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"dateRep,day,month,year,cases,deaths,countriesAndTerritories,geoId,countryterritoryCode,popData2019,continentExp,Cumulative_number_for_14_days_of_COVID-19_cases_per_100000,Cumulative_number_for_7_days_of_COVID-19_cases_per_100000\n
                                   2021-11-23,23,11,2021,104,-60,Afghanistan,AF,AFG,38041757,Asia,1.6245306440499054,0.9621006726897499").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, Some(CsvType::Ecdc));
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_csv_type_ecdc_no_bom_14_only()
  {
    let mut path = std::env::temp_dir();
    path.push("ecdc_no_bom_14_only.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"dateRep,day,month,year,cases,deaths,countriesAndTerritories,geoId,countryterritoryCode,popData2019,continentExp,Cumulative_number_for_14_days_of_COVID-19_cases_per_100000\n
                                   14/12/2020,14,12,2020,746,6,Afghanistan,AF,AFG,38041757,Asia,9.01377925").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, Some(CsvType::Ecdc));
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_csv_type_ecdc_with_bom_14_and_7()
  {
    let mut path = std::env::temp_dir();
    path.push("ecdc_with_bom_14_7.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"\xEF\xBB\xBFdateRep,day,month,year,cases,deaths,countriesAndTerritories,geoId,countryterritoryCode,popData2019,continentExp,Cumulative_number_for_14_days_of_COVID-19_cases_per_100000,Cumulative_number_for_7_days_of_COVID-19_cases_per_100000\n
                                   2021-11-23,23,11,2021,104,-60,Afghanistan,AF,AFG,38041757,Asia,1.6245306440499054,0.9621006726897499").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, Some(CsvType::Ecdc));
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_csv_type_ecdc_with_bom_14_only()
  {
    let mut path = std::env::temp_dir();
    path.push("ecdc_with_bom_14_only.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"\xEF\xBB\xBFdateRep,day,month,year,cases,deaths,countriesAndTerritories,geoId,countryterritoryCode,popData2019,continentExp,Cumulative_number_for_14_days_of_COVID-19_cases_per_100000\n
                                   14/12/2020,14,12,2020,746,6,Afghanistan,AF,AFG,38041757,Asia,9.01377925").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, Some(CsvType::Ecdc));
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_csv_type_owid_no_bom()
  {
    let mut path = std::env::temp_dir();
    path.push("owid_no_bom.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"iso_code,continent,location,date,total_cases,new_cases,new_cases_smoothed,total_deaths,new_deaths,new_deaths_smoothed,total_cases_per_million,new_cases_per_million,new_cases_smoothed_per_million,total_deaths_per_million,new_deaths_per_million,new_deaths_smoothed_per_million,reproduction_rate,icu_patients,icu_patients_per_million,hosp_patients,hosp_patients_per_million,weekly_icu_admissions,weekly_icu_admissions_per_million,weekly_hosp_admissions,weekly_hosp_admissions_per_million,total_tests,new_tests,total_tests_per_thousand,new_tests_per_thousand,new_tests_smoothed,new_tests_smoothed_per_thousand,positive_rate,tests_per_case,tests_units,total_vaccinations,people_vaccinated,people_fully_vaccinated,total_boosters,new_vaccinations,new_vaccinations_smoothed,total_vaccinations_per_hundred,people_vaccinated_per_hundred,people_fully_vaccinated_per_hundred,total_boosters_per_hundred,new_vaccinations_smoothed_per_million,new_people_vaccinated_smoothed,new_people_vaccinated_smoothed_per_hundred,stringency_index,population_density,median_age,aged_65_older,aged_70_older,gdp_per_capita,extreme_poverty,cardiovasc_death_rate,diabetes_prevalence,female_smokers,male_smokers,handwashing_facilities,hospital_beds_per_thousand,life_expectancy,human_development_index,population,excess_mortality_cumulative_absolute,excess_mortality_cumulative,excess_mortality,excess_mortality_cumulative_per_million
                                   AFG,Asia,Afghanistan,2020-01-05,,0.0,,,0.0,,,0.0,,,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,54.422,18.6,2.581,1.337,1803.987,,597.029,9.59,,,37.746,0.5,64.83,0.511,41128772.0,,,,").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, Some(CsvType::Owid));
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_csv_type_owid_etl_compact_no_bom()
  {
    let mut path = std::env::temp_dir();
    path.push("owid_etl_compact_no_bom.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"country,date,total_cases,new_cases,new_cases_smoothed,total_cases_per_million,new_cases_per_million,new_cases_smoothed_per_million,total_deaths,new_deaths,new_deaths_smoothed,total_deaths_per_million,new_deaths_per_million,new_deaths_smoothed_per_million,excess_mortality,excess_mortality_cumulative,excess_mortality_cumulative_absolute,excess_mortality_cumulative_per_million,hosp_patients,hosp_patients_per_million,weekly_hosp_admissions,weekly_hosp_admissions_per_million,icu_patients,icu_patients_per_million,weekly_icu_admissions,weekly_icu_admissions_per_million,stringency_index,reproduction_rate,total_tests,new_tests,total_tests_per_thousand,new_tests_per_thousand,new_tests_smoothed,new_tests_smoothed_per_thousand,positive_rate,tests_per_case,total_vaccinations,people_vaccinated,people_fully_vaccinated,total_boosters,new_vaccinations,new_vaccinations_smoothed,total_vaccinations_per_hundred,people_vaccinated_per_hundred,people_fully_vaccinated_per_hundred,total_boosters_per_hundred,new_vaccinations_smoothed_per_million,new_people_vaccinated_smoothed,new_people_vaccinated_smoothed_per_hundred,code,continent,population,population_density,median_age,life_expectancy,gdp_per_capita,extreme_poverty,diabetes_prevalence,handwashing_facilities,hospital_beds_per_thousand,human_development_index
                                   Afghanistan,2020-01-01,,,,,,,,,,,,,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,AFG,Asia,40578846,62.215546,16.752,,1516.2733,,10.9,48.214695,0.39,0.462").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, Some(CsvType::OwidEtlCompact));
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_csv_type_who_no_bom()
  {
    let mut path = std::env::temp_dir();
    path.push("who_no_bom.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"Date_reported,Country_code,Country,WHO_region,New_cases,Cumulative_cases,New_deaths,Cumulative_deaths
                                   2020-01-03,AF,Afghanistan,EMRO,0,0,0,0").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, Some(CsvType::Who));
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_csv_type_who_with_bom()
  {
    let mut path = std::env::temp_dir();
    path.push("who_with_bom.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"\xEF\xBB\xBFDate_reported,Country_code,Country,WHO_region,New_cases,Cumulative_cases,New_deaths,Cumulative_deaths
                                   2020-01-03,AF,Afghanistan,EMRO,0,0,0,0").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, Some(CsvType::Who));
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_csv_type_none_no_bom()
  {
    let mut path = std::env::temp_dir();
    path.push("none_no_bom.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"C,S,V
                                   2020-01-03,123,45").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, None);
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_csv_type_none_with_bom()
  {
    let mut path = std::env::temp_dir();
    path.push("none_with_bom.csv");
    let path = path.to_str().unwrap();
    assert!(std::fs::write(path, b"\xEF\xBB\xBFC,S,V
                                   2020-01-03,123,45").is_ok());
    let detected = Db::get_csv_type(&path);
    assert_eq!(detected, None);
    assert!(std::fs::remove_file(path).is_ok());
  }
}
