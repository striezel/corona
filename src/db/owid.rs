/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2024  Dirk Stolle

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

use crate::configuration::DbConfiguration;
use crate::data::{Country, Numbers};
use crate::database::Database;
use crate::db::save;
use crate::world::World;
use csv::Reader;

pub struct DbOwid
{
  config: DbConfiguration
}

impl DbOwid
{
  /**
   * Creates a new instance.
   *
   * @config   application configuration
   * @return   Returns a Result containing the Db object, if successful.
   *           Returns a string with an error message, if the configuration
   *           seems to be invalid.
   */
  pub fn new(config: &DbConfiguration) -> Result<DbOwid, String>
  {
    if config.db_path.is_empty()
    {
      return Err("Path for SQLite database must not be an empty string!".to_string());
    }
    if config.csv_input_file.is_empty()
    {
      return Err("Path of CSV file must be set to a non-empty string!".to_string());
    }

    Ok(DbOwid
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
    match crate::checks::sqlite_check()
    {
      crate::checks::Status::Error(msg) =>
      {
        eprintln!("{}", msg);
        return false;
      },
      crate::checks::Status::Warn(msg) => println!("Warning: {}", msg),
      _ => ()
    }
    let db = match Database::create(&self.config.db_path)
    {
      Err(e) =>
      {
        eprintln!("Error while creating database: {}", e);
        return false;
      }
      Ok(base) => base
    };
    if !db.calculate_total_numbers(&false)
    {
      eprintln!("Error: Could not add columns for accumulated numbers to database table!");
      return false;
    }
    self.read_csv(&db)
  }

  /**
   * Fills the SQLite database with values from the CSV file.
   *
   * @param db    an open SQLite database with existing tables
   * @param
   */
  fn read_csv(&self, db: &Database) -> bool
  {
    let mut reader = match Reader::from_path(&self.config.csv_input_file)
    {
      Ok(rdr) => rdr,
      Err(e) => {
        eprintln!("Error: Could not open CSV file {}: {}",
                  &self.config.csv_input_file, e);
        return false;
      }
    };
    // Check headers.
    if !self.check_headers(&mut reader)
    {
      return false;
    }
    // parse CSV values
    DbOwid::parse_csv_into_db(db, &mut reader)
  }

  /**
   * Checks whether the CSV headers match the expected headers.
   *
   * @param reader    an opened CSV reader
   * @return Returns true, if the headers are correct. Returns false otherwise.
   */
  fn check_headers(&self, reader: &mut csv::Reader<std::fs::File>) -> bool
  {
    let headers = match reader.headers()
    {
      Ok(head) => head,
      Err(e) =>
      {
        eprintln!(
          "Error: Could not read header of CSV file {}: {}",
          &self.config.csv_input_file, e
        );
        return false;
      }
    };
    let expected_headers = vec![
      "iso_code",
      "continent",
      "location",
      "date",
      "total_cases",
      "new_cases",
      "new_cases_smoothed",
      "total_deaths",
      "new_deaths",
    ];
    let mut header_part = headers.iter().collect::<Vec<_>>();
    header_part.truncate(expected_headers.len());
    if header_part != expected_headers
    {
      eprintln!("Error: CSV headers do not match the expected headers. \
                 Found the following headers: {:?}", headers);
      return false;
    }
    // Headers match. :)
    true
  }

  /**
   * Parses the CSV data and writes it into the database.
   *
   * @param db        an open SQLite database with existing tables
   * @param reader    an opened CSV reader
   * @return Returns true, if all data was parsed and written to the database
   *         successfully. Returns false otherwise.
   */
  fn parse_csv_into_db(db: &Database, reader: &mut csv::Reader<std::fs::File>) -> bool
  {
    let mut last_iso3_id = String::new();
    let mut country_id: i64 = -1;
    let mut population: i32 = -1;
    let mut record = csv::StringRecord::new();
    let world = World::new();
    let mut parsed_data = Vec::<Numbers>::new();
    loop
    {
      match reader.read_record(&mut record)
      {
        Ok(success) =>
        {
          if !success
          {
            // No more records to read.
            break;
          }
        },
        Err(e) => {
          // Failed to read.
          eprintln!("Failed to read CSV record! {}", e);
          return false;
        }
      }
      let r_len = record.len();
      if r_len < 9
      {
        eprintln!("Error: A line of CSV data does not have enough data elements, \
                   but only {} elements!\nThe line position is '{}'. \
                   It will be skipped.",
                   r_len, record.position().unwrap().line());
        return false;
      }
      let current_iso3_id = record.get(0).unwrap();
      // Skip "OWID_..." rows.
      if current_iso3_id.starts_with("OWID_")
      {
        continue;
      }
      if current_iso3_id != last_iso3_id
      {
        // Insert data of previous country.
        if country_id != -1
          && !save::numbers_into_db(db, &country_id, &population, &mut parsed_data)
        {
          eprintln!("Error: Could not insert country data into database!");
          return false;
        }
        parsed_data.truncate(0);
        // new country
        let name = record.get(2).unwrap();
        let no_country = Country {
          country_id: -1,
          name: name.to_string(),
          population: -1,
          geo_id: String::new(),
          country_code: String::new(),
          continent: record.get(1).unwrap().to_string()
        };
        let world_data = world.find_by_country_code(current_iso3_id).unwrap_or(&no_country);
        population = world_data.population;
        // Get country id or insert country.
        country_id = db.get_country_id_or_insert(
          &world_data.geo_id,
          name,
          &i64::from(world_data.population),
          current_iso3_id,
          &world_data.continent
        );
        if country_id == -1
        {
          eprintln!("Error: Could not insert country data into database!");
          return false;
        }
        last_iso3_id = current_iso3_id.to_string();
      }
      // Add current record.
      let date = record.get(3).unwrap();
      let cases: f64 = match record.get(5).unwrap().is_empty()
      {
        false => record.get(5).unwrap().parse().unwrap_or(f64::MIN),
        true => 0.0
      };
      let deaths: f64 = match record.get(8).unwrap().is_empty()
      {
        false => record.get(8).unwrap().parse().unwrap_or(f64::MIN),
        true => 0.0
      };
      if cases == f64::MIN || deaths == f64::MIN
      {
        eprintln!(
          "Error: Got invalid case numbers on line {}.",
          record.position().unwrap().line()
        );
        return false;
      }
      parsed_data.push(Numbers { date: String::from(date), cases: cases as i32, deaths: deaths as i32});
    }
    // Execute remaining batch inserts, if any are left.
    if !parsed_data.is_empty()
      && !crate::db::save::numbers_into_db(db, &country_id, &population, &mut parsed_data)
    {
      eprintln!("Error: Could not insert country data into database!");
      return false;
    }

    // Done.
    true
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  use crate::data::Country;

  /**
   * Gets path to the temporary corona.csv file suitable for testing.
   *
   * @return Returns path of the CSV file.
   */
  fn get_csv_path() -> String
  {
    let path = std::env::temp_dir().join("test_db_corona_owid.csv");

    let simple_csv =
      "iso_code,continent,location,date,total_cases,new_cases,new_cases_smoothed,total_deaths,new_deaths,new_deaths_smoothed,total_cases_per_million,new_cases_per_million,new_cases_smoothed_per_million,total_deaths_per_million,new_deaths_per_million,new_deaths_smoothed_per_million,reproduction_rate,icu_patients,icu_patients_per_million,hosp_patients,hosp_patients_per_million,weekly_icu_admissions,weekly_icu_admissions_per_million,weekly_hosp_admissions,weekly_hosp_admissions_per_million,total_tests,new_tests,total_tests_per_thousand,new_tests_per_thousand,new_tests_smoothed,new_tests_smoothed_per_thousand,positive_rate,tests_per_case,tests_units,total_vaccinations,people_vaccinated,people_fully_vaccinated,total_boosters,new_vaccinations,new_vaccinations_smoothed,total_vaccinations_per_hundred,people_vaccinated_per_hundred,people_fully_vaccinated_per_hundred,total_boosters_per_hundred,new_vaccinations_smoothed_per_million,new_people_vaccinated_smoothed,new_people_vaccinated_smoothed_per_hundred,stringency_index,population_density,median_age,aged_65_older,aged_70_older,gdp_per_capita,extreme_poverty,cardiovasc_death_rate,diabetes_prevalence,female_smokers,male_smokers,handwashing_facilities,hospital_beds_per_thousand,life_expectancy,human_development_index,population,excess_mortality_cumulative_absolute,excess_mortality_cumulative,excess_mortality,excess_mortality_cumulative_per_million
DEU,Europe,Germany,2020-01-05,1.0,1.0,,3.0,3.0,,0.012,0.012,,0.036,0.036,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,-760.7,-3.87,-3.87,-9.128876
DEU,Europe,Germany,2020-01-06,1.0,0.0,,3.0,0.0,,0.012,0.0,,0.036,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-07,1.0,0.0,,3.0,0.0,,0.012,0.0,,0.036,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-08,1.0,0.0,,3.0,0.0,,0.012,0.0,,0.036,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-09,1.0,0.0,,3.0,0.0,,0.012,0.0,,0.036,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-10,1.0,0.0,0.143,3.0,0.0,0.429,0.012,0.0,0.002,0.036,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-11,1.0,0.0,0.143,3.0,0.0,0.429,0.012,0.0,0.002,0.036,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-12,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,-1317.6001,-3.33,-2.79,-15.812025
DEU,Europe,Germany,2020-01-13,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-14,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-15,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-16,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-17,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-18,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-19,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,-2325.7002,-3.9,-5.05,-27.909857
DEU,Europe,Germany,2020-01-20,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-21,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-22,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-23,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-24,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-25,1.0,0.0,0.0,3.0,0.0,0.0,0.012,0.0,0.0,0.036,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-26,2.0,1.0,0.143,6.0,3.0,0.429,0.024,0.012,0.002,0.072,0.036,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,-3679.6,-4.61,-6.71,-44.1575
DEU,Europe,Germany,2020-01-27,2.0,0.0,0.143,6.0,0.0,0.429,0.024,0.0,0.002,0.072,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-28,2.0,0.0,0.143,6.0,0.0,0.429,0.024,0.0,0.002,0.072,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-29,2.0,0.0,0.143,6.0,0.0,0.429,0.024,0.0,0.002,0.072,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-30,2.0,0.0,0.143,6.0,0.0,0.429,0.024,0.0,0.002,0.072,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-01-31,2.0,0.0,0.143,6.0,0.0,0.429,0.024,0.0,0.002,0.072,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-01,2.0,0.0,0.143,6.0,0.0,0.429,0.024,0.0,0.002,0.072,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-02,11.0,9.0,1.286,9.0,3.0,0.429,0.132,0.108,0.015,0.108,0.036,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,-4767.9004,-4.74,-5.22,-57.217785
DEU,Europe,Germany,2020-02-03,11.0,0.0,1.286,9.0,0.0,0.429,0.132,0.0,0.015,0.108,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-04,11.0,0.0,1.286,9.0,0.0,0.429,0.132,0.0,0.015,0.108,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-05,11.0,0.0,1.286,9.0,0.0,0.429,0.132,0.0,0.015,0.108,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-06,11.0,0.0,1.286,9.0,0.0,0.429,0.132,0.0,0.015,0.108,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-07,11.0,0.0,1.286,9.0,0.0,0.429,0.132,0.0,0.015,0.108,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-08,11.0,0.0,1.286,9.0,0.0,0.429,0.132,0.0,0.015,0.108,0.0,0.005,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-09,20.0,9.0,1.286,9.0,0.0,0.0,0.24,0.108,0.015,0.108,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,-6599.8003,-5.43,-8.78,-79.20173
DEU,Europe,Germany,2020-02-10,20.0,0.0,1.286,9.0,0.0,0.0,0.24,0.0,0.015,0.108,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-11,20.0,0.0,1.286,9.0,0.0,0.0,0.24,0.0,0.015,0.108,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5.56,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-12,20.0,0.0,1.286,9.0,0.0,0.0,0.24,0.0,0.015,0.108,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,11.11,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-13,20.0,0.0,1.286,9.0,0.0,0.0,0.24,0.0,0.015,0.108,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,11.11,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
DEU,Europe,Germany,2020-02-14,20.0,0.0,1.286,9.0,0.0,0.0,0.24,0.0,0.015,0.108,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,11.11,237.016,46.6,21.453,15.957,45229.245,,156.139,8.31,28.2,33.1,,8.0,81.33,0.947,83369840.0,,,,
CHE,Europe,Switzerland,2020-02-23,,0.0,0.0,,0.0,0.0,,0.0,0.0,,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,-890.4,-7.43,-10.02,-103.07215
CHE,Europe,Switzerland,2020-02-24,,0.0,0.0,,0.0,0.0,,0.0,0.0,,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,0.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-02-25,,0.0,0.0,,0.0,0.0,,0.0,0.0,,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,8.33,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-02-26,,0.0,0.0,,0.0,0.0,,0.0,0.0,,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,8.33,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-02-27,,0.0,0.0,,0.0,0.0,,0.0,0.0,,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,13.89,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-02-28,,0.0,0.0,,0.0,0.0,,0.0,0.0,,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,19.44,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-02-29,,0.0,0.0,,0.0,0.0,,0.0,0.0,,0.0,0.0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,19.44,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-01,45.0,45.0,6.429,,0.0,0.0,5.148,5.148,0.735,,0.0,0.0,,,,,,,,52.0,5.949,,,,,,,,,,,,,,,,,,,,,,,19.44,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,-1073.3,-7.94,-11.92,-124.244545
CHE,Europe,Switzerland,2020-03-02,45.0,0.0,6.429,,0.0,0.0,5.148,0.0,0.735,,0.0,0.0,,,,,,,,60.0,6.865,,,,,,,,,,,,,,,,,,,,,,,19.44,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-03,45.0,0.0,6.429,,0.0,0.0,5.148,0.0,0.735,,0.0,0.0,,,,,,,,73.0,8.352,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-04,45.0,0.0,6.429,,0.0,0.0,5.148,0.0,0.735,,0.0,0.0,,,,,,,,77.0,8.81,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-05,45.0,0.0,6.429,,0.0,0.0,5.148,0.0,0.735,,0.0,0.0,,,,,,,,90.0,10.297,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-06,45.0,0.0,6.429,,0.0,0.0,5.148,0.0,0.735,,0.0,0.0,2.69,,,,,,,109.0,12.471,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-07,45.0,0.0,6.429,,0.0,0.0,5.148,0.0,0.735,,0.0,0.0,2.69,,,,,,,103.0,11.784,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-08,365.0,320.0,45.714,2.0,2.0,0.286,41.76,36.611,5.23,0.229,0.229,0.033,2.68,,,,,,,111.0,12.7,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,-1177.0,-7.86,-7.1,-136.2488
CHE,Europe,Switzerland,2020-03-09,365.0,0.0,45.714,2.0,0.0,0.286,41.76,0.0,5.23,0.229,0.0,0.033,2.72,,,,,,,127.0,14.53,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-10,365.0,0.0,45.714,2.0,0.0,0.286,41.76,0.0,5.23,0.229,0.0,0.033,2.76,,,,,,,152.0,17.39,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-11,365.0,0.0,45.714,2.0,0.0,0.286,41.76,0.0,5.23,0.229,0.0,0.033,2.79,,,,,,,188.0,21.509,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-12,365.0,0.0,45.714,2.0,0.0,0.286,41.76,0.0,5.23,0.229,0.0,0.033,2.81,,,,,,,222.0,25.399,,,,,,,,,,,,,,,,,,,,,,,25.0,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-13,365.0,0.0,45.714,2.0,0.0,0.286,41.76,0.0,5.23,0.229,0.0,0.033,2.86,,,,,,,256.0,29.289,,,,,,,,,,,,,,,,,,,,,,,33.33,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-14,365.0,0.0,45.714,2.0,0.0,0.286,41.76,0.0,5.23,0.229,0.0,0.033,2.83,,,,,,,313.0,35.81,,,,,,,,,,,,,,,,,,,,,,,33.33,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-15,2373.0,2008.0,286.857,20.0,18.0,2.571,271.496,229.736,32.819,2.288,2.059,0.294,2.79,,,,,,,351.0,40.158,,,,,,,,,,,,,,,,,,,,,,,33.33,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,-1182.9,-7.22,-0.42,-136.93178
CHE,Europe,Switzerland,2020-03-16,2373.0,0.0,286.857,20.0,0.0,2.571,271.496,0.0,32.819,2.288,0.0,0.294,2.66,,,,,,,442.0,50.569,,,,,,,,,,,,,,,,,,,,,,,44.44,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-17,2373.0,0.0,286.857,20.0,0.0,2.571,271.496,0.0,32.819,2.288,0.0,0.294,2.57,,,,,,,498.0,56.976,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-18,2373.0,0.0,286.857,20.0,0.0,2.571,271.496,0.0,32.819,2.288,0.0,0.294,2.48,,,,,,,562.0,64.299,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-19,2373.0,0.0,286.857,20.0,0.0,2.571,271.496,0.0,32.819,2.288,0.0,0.294,2.42,,,,,,,620.0,70.934,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-20,2373.0,0.0,286.857,20.0,0.0,2.571,271.496,0.0,32.819,2.288,0.0,0.294,2.29,,,,,,,705.0,80.659,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-21,2373.0,0.0,286.857,20.0,0.0,2.571,271.496,0.0,32.819,2.288,0.0,0.294,2.18,,,,,,,759.0,86.837,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-22,8710.0,6337.0,905.286,109.0,89.0,12.714,996.514,725.018,103.574,12.471,10.183,1.455,2.05,,,,,,,840.0,96.105,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,-1004.2,-5.66,13.2,-116.24557
CHE,Europe,Switzerland,2020-03-23,8710.0,0.0,905.286,109.0,0.0,12.714,996.514,0.0,103.574,12.471,0.0,1.455,1.95,,,,,,,886.0,101.368,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-24,8710.0,0.0,905.286,109.0,0.0,12.714,996.514,0.0,103.574,12.471,0.0,1.455,1.82,,,,,,,969.0,110.864,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-25,8710.0,0.0,905.286,109.0,0.0,12.714,996.514,0.0,103.574,12.471,0.0,1.455,1.69,,,,,,,1057.0,120.932,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-26,8710.0,0.0,905.286,109.0,0.0,12.714,996.514,0.0,103.574,12.471,0.0,1.455,1.57,,,,,,,1115.0,127.567,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-27,8710.0,0.0,905.286,109.0,0.0,12.714,996.514,0.0,103.574,12.471,0.0,1.455,1.46,,,,,,,1182.0,135.233,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-28,8710.0,0.0,905.286,109.0,0.0,12.714,996.514,0.0,103.574,12.471,0.0,1.455,1.39,,,,,,,1188.0,135.919,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-29,16170.0,7460.0,1065.714,335.0,226.0,32.286,1850.015,853.501,121.929,38.327,25.857,3.694,1.32,,,,,,,1174.0,134.318,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,-667.7,-3.51,25.95,-77.29254
CHE,Europe,Switzerland,2020-03-30,16170.0,0.0,1065.714,335.0,0.0,32.286,1850.015,0.0,121.929,38.327,0.0,3.694,1.27,399.0,45.65,1328.0,151.937,,,1175.0,134.432,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,
CHE,Europe,Switzerland,2020-03-31,16170.0,0.0,1065.714,335.0,0.0,32.286,1850.015,0.0,121.929,38.327,0.0,3.694,1.21,429.0,49.082,1440.0,164.751,,,1122.0,128.368,,,,,,,,,,,,,,,,,,,,,,,73.15,214.243,43.1,18.436,12.644,57410.166,,99.739,5.59,22.6,28.9,,4.53,83.78,0.955,8740471.0,,,,";
    std::fs::write(&path, simple_csv).expect("Unable to write CSV file for test!");

    path.to_str().unwrap().to_string()
  }

  #[test]
  fn successful_execution()
  {
    let db_file_name = std::env::temp_dir().join("test_db_corona_owid.db");
    let config = DbConfiguration {
      db_path: db_file_name.to_str().unwrap().to_string(),
      csv_input_file: get_csv_path()
    };
    // scope for db
    {
      let db = DbOwid::new(&config).unwrap();
      assert!(db.create_db());
      // Check that DB file exists.
      assert!(db_file_name.exists());
      // Check some content.
      let db = Database::new(&config.db_path).unwrap();
      // Check a country.
      let countries = db.countries();
      {
        let de = Country
        {
          country_id: 1,
          name: String::from("Germany"),
          population: 83019213,
          geo_id: "DE".to_string(),
          country_code: "DEU".to_string(),
          continent: "Europe".to_string()
        };
        let found = countries.iter().find(|&c| c.geo_id == "DE");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(de.country_id, found.country_id);
        assert_eq!(de.name, found.name);
        assert_eq!(de.population, found.population);
        assert_eq!(de.geo_id, found.geo_id);
        assert_eq!(de.country_code, found.country_code);
        assert_eq!(de.continent, found.continent);
        // Check some numbers.
        let numbers = db.numbers_with_incidence(&de.country_id);
        // 1|2020-02-09|9|0|0.0216817280597444|0.0108408640298722|20|9
        let found = numbers.iter().find(|&n| n.date == "2020-02-09");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!("2020-02-09", found.date);
        assert_eq!(9, found.cases);
        assert_eq!(0, found.deaths);
        assert!(found.incidence_14d.is_some());
        assert!(found.incidence_14d.unwrap() > 0.021681);
        assert!(found.incidence_14d.unwrap() < 0.021682);
        assert!(found.incidence_7d.is_some());
        assert!(found.incidence_7d.unwrap() > 0.010840);
        assert!(found.incidence_7d.unwrap() < 0.010841);
      }

      // Check another country.
      let ch = Country
      {
        country_id: 2,
        name: String::from("Switzerland"),
        population: 8544527,
        geo_id: "CH".to_string(),
        country_code: "CHE".to_string(),
        continent: "Europe".to_string()
      };
      let found = countries.iter().find(|&c| c.geo_id == "CH");
      assert!(found.is_some());
      let found = found.unwrap();
      assert_eq!(ch.country_id, found.country_id);
      assert_eq!(ch.name, found.name);
      assert_eq!(ch.population, found.population);
      assert_eq!(ch.geo_id, found.geo_id);
      assert_eq!(ch.country_code, found.country_code);
      assert_eq!(ch.continent, found.continent);
      // Check some numbers.
      let numbers = db.numbers_with_incidence(&ch.country_id);
      // 2|2020-03-29|7460|226|161.471781878622|87.3073489029878|16170|335
      let found = numbers.iter().find(|&n| n.date == "2020-03-29");
      assert!(found.is_some());
      let found = found.unwrap();
      assert_eq!("2020-03-29", found.date);
      assert_eq!(7460, found.cases);
      assert_eq!(226, found.deaths);
      assert!(found.incidence_14d.is_some());
      assert!(found.incidence_14d.unwrap() > 161.471781);
      assert!(found.incidence_14d.unwrap() < 161.471782);
      assert!(found.incidence_7d.is_some());
      assert!(found.incidence_7d.unwrap() > 87.307348);
      assert!(found.incidence_7d.unwrap() < 87.307349);
    }
    // clean up
    assert!(std::fs::remove_file(db_file_name).is_ok());
    assert!(std::fs::remove_file(config.csv_input_file).is_ok());
  }
}
