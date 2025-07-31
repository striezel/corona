/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2025  Dirk Stolle

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

pub struct DbOwidEtlCompact
{
  config: DbConfiguration
}

impl DbOwidEtlCompact
{
  /**
   * Creates a new instance.
   *
   * @config   application configuration
   * @return   Returns a Result containing the Db object, if successful.
   *           Returns a string with an error message, if the configuration
   *           seems to be invalid.
   */
  pub fn new(config: &DbConfiguration) -> Result<DbOwidEtlCompact, String>
  {
    if config.db_path.is_empty()
    {
      return Err("Path for SQLite database must not be an empty string!".to_string());
    }
    if config.csv_input_file.is_empty()
    {
      return Err("Path of CSV file must be set to a non-empty string!".to_string());
    }

    Ok(DbOwidEtlCompact
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
        eprintln!("{msg}");
        return false;
      },
      crate::checks::Status::Warn(msg) => println!("Warning: {msg}"),
      _ => ()
    }
    let db = match Database::create(&self.config.db_path)
    {
      Err(e) =>
      {
        eprintln!("Error while creating database: {e}");
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
    DbOwidEtlCompact::parse_csv_into_db(db, &mut reader)
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
      "country",
      "date",
      "total_cases",
      "new_cases",
      "new_cases_smoothed",
      "total_cases_per_million",
      "new_cases_per_million",
      "new_cases_smoothed_per_million",
      "total_deaths",
      "new_deaths",
    ];
    let header_strings = headers.iter().collect::<Vec<_>>();
    if !header_strings.starts_with(&expected_headers)
    {
      eprintln!("Error: CSV headers do not match the expected headers. \
                 Found the following headers: {headers:?}");
      return false;
    }
    if !header_strings.contains(&"code") || !header_strings.contains(&"continent")
    {
      eprintln!("Error: CSV headers for country code and continent are missing. \
                 Found the following headers: {headers:?}");
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
    /// zero-based index of the column that contains the country's name
    const IDX_NAME: usize = 0;

    /// zero-based index of the column that contains the date
    const IDX_DATE: usize = 1;

    /// zero-based index of the column containing the number of new cases
    const IDX_CASES: usize = 3;

    /// zero-based index of the column containing the number of new deaths
    const IDX_DEATHS: usize = 9;

    /// zero-based index of the column containing the ISO-3166 ALPHA-3 country code
    const IDX_ISO3: usize = 49;

    /// zero-based index of the column that contains the continent name
    const IDX_CONTINENT: usize = 50;

    let now: String = chrono::Utc::now().format("%Y-%m-%d").to_string();

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
          eprintln!("Failed to read CSV record! {e}");
          return false;
        }
      }
      let r_len = record.len();
      if r_len < 51
      {
        eprintln!("Error: A line of CSV data does not have enough data elements, \
                   but only {} elements!\nThe line position is '{}'. \
                   It will be skipped.",
                   r_len, record.position().unwrap().line());
        return false;
      }
      let current_iso3_id = record.get(IDX_ISO3).unwrap();
      let current_iso3_id = if current_iso3_id == "OWID_KOS"
      {
        // "XKX" is the temporary code for Kosovo as long as it has no official
        // ISO-3166 code.
        "XKX"
      }
      else
      {
        current_iso3_id
      };
      // Skip "OWID_..." and empty rows.
      if current_iso3_id.starts_with("OWID_") || current_iso3_id.is_empty()
      {
        continue;
      }
      let date = record.get(IDX_DATE).unwrap();
      if date > &now
      {
        continue;
      }
      if current_iso3_id != last_iso3_id
      {
        // Insert data of previous country.
        crate::data::cutoff_non_contiguous_dates(&mut parsed_data);
        if country_id != -1
          && !save::numbers_into_db(db, &country_id, &population, &mut parsed_data)
        {
          eprintln!("Error: Could not insert country data into database!");
          return false;
        }
        parsed_data.truncate(0);
        // new country
        let name = record.get(IDX_NAME).unwrap();
        let new_country = Country {
          country_id: -1,
          name: name.to_string(),
          population: -1,
          geo_id: String::new(),
          country_code: record.get(IDX_ISO3).unwrap().to_string(),
          continent: record.get(IDX_CONTINENT).unwrap().to_string()
        };
        let world_data = world.find_by_country_code(current_iso3_id).unwrap_or(&new_country);
        population = world_data.population;
        // Get country id or insert country.
        country_id = db.get_country_id_or_insert(
          &world_data.geo_id,
          name,
          &i64::from(world_data.population),
          current_iso3_id,
          &new_country.continent
        );
        if country_id == -1
        {
          eprintln!("Error: Could not insert country data into database!");
          return false;
        }
        last_iso3_id = current_iso3_id.to_string();
      }
      // Add current record.
      let cases: f64 = match record.get(IDX_CASES).unwrap().is_empty()
      {
        false => record.get(IDX_CASES).unwrap().parse().unwrap_or(f64::MIN),
        true => 0.0
      };
      let deaths: f64 = match record.get(IDX_DEATHS).unwrap().is_empty()
      {
        false => record.get(IDX_DEATHS).unwrap().parse().unwrap_or(f64::MIN),
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
    crate::data::cutoff_non_contiguous_dates(&mut parsed_data);
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
    let path = std::env::temp_dir().join("test_db_corona_owid_etl_compact.csv");

    let simple_csv =
      "country,date,total_cases,new_cases,new_cases_smoothed,total_cases_per_million,new_cases_per_million,new_cases_smoothed_per_million,total_deaths,new_deaths,new_deaths_smoothed,total_deaths_per_million,new_deaths_per_million,new_deaths_smoothed_per_million,excess_mortality,excess_mortality_cumulative,excess_mortality_cumulative_absolute,excess_mortality_cumulative_per_million,hosp_patients,hosp_patients_per_million,weekly_hosp_admissions,weekly_hosp_admissions_per_million,icu_patients,icu_patients_per_million,weekly_icu_admissions,weekly_icu_admissions_per_million,stringency_index,reproduction_rate,total_tests,new_tests,total_tests_per_thousand,new_tests_per_thousand,new_tests_smoothed,new_tests_smoothed_per_thousand,positive_rate,tests_per_case,total_vaccinations,people_vaccinated,people_fully_vaccinated,total_boosters,new_vaccinations,new_vaccinations_smoothed,total_vaccinations_per_hundred,people_vaccinated_per_hundred,people_fully_vaccinated_per_hundred,total_boosters_per_hundred,new_vaccinations_smoothed_per_million,new_people_vaccinated_smoothed,new_people_vaccinated_smoothed_per_hundred,code,continent,population,population_density,median_age,life_expectancy,gdp_per_capita,extreme_poverty,diabetes_prevalence,handwashing_facilities,hospital_beds_per_thousand,human_development_index
Germany,2020-01-01,,,,,,,,,,,,,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-02,,,,,,,,,,,,,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-03,,,,,,,,,,,,,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-04,1,1,,0.011892554,0.011892554,,0,0,,0.0,0.0,,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-05,1,0,,0.011892554,0.0,,3,3,,0.03567766,0.03567766,,-3.8724844,-3.8724844,-760.6992,-9.128866,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-06,1,0,,0.011892554,0.0,,3,0,,0.03567766,0.0,,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-07,1,0,,0.011892554,0.0,,3,0,,0.03567766,0.0,,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-08,1,0,,0.011892554,0.0,,3,0,,0.03567766,0.0,,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-09,1,0,0.16666667,0.011892554,0.0,0.0019820924,3,0,0.5,0.03567766,0.0,0.005946277,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-10,1,0,0.14285715,0.011892554,0.0,0.0016989362,3,0,0.42857143,0.03567766,0.0,0.005096809,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-11,1,0,0.0,0.011892554,0.0,0.0,3,0,0.42857143,0.03567766,0.0,0.005096809,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-12,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,-2.7893972,-3.3265493,-1317.5996,-15.812019,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-13,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-14,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-15,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-16,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-17,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-18,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-19,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,-5.050321,-3.9041648,-2325.6992,-27.909845,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-20,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-21,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-22,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-23,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-24,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-25,1,0,0.0,0.011892554,0.0,0.0,3,0,0.0,0.03567766,0.0,0.0,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-26,2,1,0.14285715,0.023785109,0.011892554,0.0016989362,6,3,0.42857143,0.07135532,0.03567766,0.005096809,-6.7088203,-4.6138835,-3679.5996,-44.157497,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-27,2,0,0.14285715,0.023785109,0.0,0.0016989362,6,0,0.42857143,0.07135532,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-28,2,0,0.14285715,0.023785109,0.0,0.0016989362,6,0,0.42857143,0.07135532,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-29,2,0,0.14285715,0.023785109,0.0,0.0016989362,6,0,0.42857143,0.07135532,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-30,2,0,0.14285715,0.023785109,0.0,0.0016989362,6,0,0.42857143,0.07135532,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-01-31,2,0,0.14285715,0.023785109,0.0,0.0016989362,6,0,0.42857143,0.07135532,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-01,2,0,0.14285715,0.023785109,0.0,0.0016989362,6,0,0.42857143,0.07135532,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-02,11,9,1.2857143,0.1308181,0.107032984,0.015290426,9,3,0.42857143,0.107032984,0.03567766,0.005096809,-5.2165904,-4.738856,-4767.9004,-57.217785,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-03,11,0,1.2857143,0.1308181,0.0,0.015290426,9,0,0.42857143,0.107032984,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-04,11,0,1.2857143,0.1308181,0.0,0.015290426,9,0,0.42857143,0.107032984,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-05,11,0,1.2857143,0.1308181,0.0,0.015290426,9,0,0.42857143,0.107032984,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-06,11,0,1.2857143,0.1308181,0.0,0.015290426,9,0,0.42857143,0.107032984,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-07,11,0,1.2857143,0.1308181,0.0,0.015290426,9,0,0.42857143,0.107032984,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-08,11,0,1.2857143,0.1308181,0.0,0.015290426,9,0,0.42857143,0.107032984,0.0,0.005096809,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-09,20,9,1.2857143,0.23785108,0.107032984,0.015290426,9,0,0.0,0.107032984,0.0,0.0,-8.777715,-5.432704,-6599.801,-79.20174,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-10,20,0,1.2857143,0.23785108,0.0,0.015290426,9,0,0.0,0.107032984,0.0,0.0,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-11,20,0,1.2857143,0.23785108,0.0,0.015290426,9,0,0.0,0.107032984,0.0,0.0,,,,,,,,,,,,,5.56,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-12,20,0,1.2857143,0.23785108,0.0,0.015290426,9,0,0.0,0.107032984,0.0,0.0,,,,,,,,,,,,,11.11,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-13,20,0,1.2857143,0.23785108,0.0,0.015290426,9,0,0.0,0.107032984,0.0,0.0,,,,,,,,,,,,,11.11,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Germany,2020-02-14,20,0,1.2857143,0.23785108,0.0,0.015290426,9,0,0.0,0.107032984,0.0,0.0,,,,,,,,,,,,,11.11,,,,,,,,,,,,,,,,,,,,,,,DEU,Europe,84086228,240.66582,45.024,,53969.625,0.24379632,6.9,,8.0,0.95
Switzerland,2020-02-23,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,-10.040744,-7.5285325,-902.4,-104.461266,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-02-24,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,0.0,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-02-25,1,1,0.14285715,0.113737434,0.113737434,0.016248206,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-02-26,2,1,0.2857143,0.22747487,0.113737434,0.03249641,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-02-27,12,10,1.7142857,1.3648492,1.1373744,0.19497846,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,13.89,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-02-28,22,10,3.142857,2.5022235,1.1373744,0.3574605,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,19.44,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-02-29,32,10,4.571429,3.639598,1.1373744,0.5199426,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,19.44,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-01,45,13,6.428571,5.1181846,1.4785867,0.7311692,0,0,0.0,0.0,0.0,0.0,-12.117264,-8.049462,-1088.4,-125.99251,,,52,5.949,,,,,19.44,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-02,57,12,8.142858,6.4830337,1.3648492,0.9261477,0,0,0.0,0.0,0.0,0.0,,,,,,,60,6.865,,,,,19.44,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-03,87,30,12.285714,9.895157,3.412123,1.3973457,0,0,0.0,0.0,0.0,0.0,,,,,,,73,8.352,,,,,25.0,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-04,120,33,16.857143,13.648492,3.7533352,1.9172882,0,0,0.0,0.0,0.0,0.0,,,,,,,77,8.81,,,,,25.0,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-05,181,61,24.142857,20.586475,6.9379835,2.7459466,0,0,0.0,0.0,0.0,0.0,,,,,,,90,10.297,,,,,25.0,,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-06,243,62,31.571428,27.638197,7.051721,3.5908532,2,2,0.2857143,0.22747487,0.22747487,0.03249641,,,,,,,109,12.471,,,,,25.0,2.6866,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-07,316,73,40.57143,35.94103,8.302833,4.61449,2,0,0.2857143,0.22747487,0.0,0.03249641,,,,,,,103,11.784,,,,,25.0,2.6852,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-08,365,49,45.714287,41.514164,5.5731344,5.1994257,2,0,0.2857143,0.22747487,0.0,0.03249641,-7.118412,-7.9586716,-1192.4,-138.03148,,,111,12.7,,,,,25.0,2.6837,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-09,434,69,53.857143,49.362045,7.847883,6.125573,3,1,0.42857143,0.3412123,0.113737434,0.048744615,,,,,,,127,14.53,,,,,25.0,2.72,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-10,625,191,76.85714,71.0859,21.72385,8.741534,3,0,0.42857143,0.3412123,0.0,0.048744615,,,,,,,152,17.39,,,,,25.0,2.7602,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-11,835,210,102.14286,94.97076,23.884861,11.617467,5,2,0.71428573,0.5686872,0.22747487,0.08124103,,,,,,,188,21.509,,,,,25.0,2.7938,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-12,1168,333,141.0,132.84532,37.874565,16.036978,8,3,1.1428572,0.9098995,0.3412123,0.12998565,,,,,,,222,25.399,,,,,25.0,2.8106,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-13,1525,357,183.14285,173.44958,40.604263,20.830198,12,4,1.4285715,1.3648492,0.45494974,0.16248205,,,,,,,256,29.289,,,,,33.33,2.8622,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-14,1956,431,234.28572,222.47043,49.020836,26.647057,15,3,1.8571428,1.7060615,0.3412123,0.21122667,,,,,,,313,35.81,,,,,33.33,2.8297,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-15,2373,417,286.85715,269.89893,47.42851,32.626396,20,5,2.5714285,2.2747488,0.5686872,0.29246768,-0.39954165,-7.3120117,-1198.0,-138.67973,,,351,40.158,,,,,33.33,2.7883,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-16,2699,326,323.57144,306.97733,37.078403,36.802185,28,8,3.5714285,3.1846483,0.9098995,0.40620512,,,,,,,442,50.569,,,,,44.44,2.6638,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-17,3759,1060,447.7143,427.53903,120.56168,50.921875,35,7,4.571429,3.9808102,0.79616207,0.5199426,,,,,,,498,56.976,,,,,73.15,2.5693,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-18,4841,1082,572.2857,550.6029,123.0639,65.09031,47,12,6.0,5.3456593,1.3648492,0.6824246,,,,,,,562,64.299,,,,,73.15,2.484,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-19,6047,1206,697.0,687.77026,137.16734,79.274994,55,8,6.714286,6.255559,0.9098995,0.7636656,,,,,,,620,70.934,,,,,73.15,2.4188,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-20,6881,834,765.1429,782.6273,94.85702,87.02538,69,14,8.142858,7.847883,1.5923241,0.9261477,,,,,,,705,80.659,,,,,73.15,2.2923,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-21,8020,1139,866.2857,912.17426,129.54694,98.52911,90,21,10.714286,10.236369,2.3884861,1.2186154,,,,,,,759,86.837,,,,,73.15,2.183,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-22,8710,690,905.2857,990.6531,78.47883,102.964874,109,19,12.714286,12.397381,2.1610112,1.4460902,13.0653305,-5.757391,-1021.19995,-118.21347,,,840,96.105,,,,,73.15,2.0544,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-23,9257,547,936.8571,1052.8674,62.21438,106.555725,127,18,14.142858,14.444654,2.0472739,1.6085722,,,,,,,886,101.368,,,,,73.15,1.9492,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-24,10719,1462,994.2857,1219.1516,166.28413,113.08751,150,23,16.428572,17.060616,2.615961,1.8685436,,,,,,,969,110.864,,,,,73.15,1.8172,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-25,11961,1242,1017.1429,1360.4135,141.26189,115.68722,169,19,17.428572,19.221626,2.1610112,1.982281,,,,,,,1057,120.932,,,,,73.15,1.689,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-26,13029,1068,997.4286,1481.885,121.47158,113.44497,208,39,21.857143,23.657387,4.43576,2.4859753,,,,,,,1115,127.567,,,,,73.15,1.5675,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-27,14143,1114,1037.4286,1608.5885,126.7035,117.99446,246,38,25.285715,27.979408,4.3220224,2.8759322,,,,,,,1182,135.233,,,,,73.15,1.4629,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-28,15448,1305,1061.1428,1757.0159,148.42735,120.691666,278,32,26.857143,31.619007,3.639598,3.0546625,,,,,,,1188,135.919,,,,,73.15,1.3877,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-29,16170,722,1065.7142,1839.1343,82.11843,121.21161,335,57,32.285713,38.10204,6.4830337,3.6720943,25.713408,-3.6135712,-687.7999,-79.61929,,,1174,134.318,,,,,73.15,1.3225,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-30,16602,432,1049.2858,1888.2689,49.13457,119.34306,380,45,36.142857,43.220226,5.1181846,4.110796,,,,,1328,151.937,1175,134.432,399,45.65,,,73.15,1.2702,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967
Switzerland,2020-03-31,17909,1307,1027.1428,2036.9237,148.65483,116.82459,438,58,41.142857,49.816998,6.5967712,4.679483,,,,,1440,164.751,1122,128.368,429,49.082,,,73.15,1.2078,,,,,,,,,,,,,,,,,,,,,,CHE,Europe,8792180,222.5326,41.961,,72278.21,0.037200913,4.6,,4.63,0.967";
    std::fs::write(&path, simple_csv).expect("Unable to write CSV file for test!");

    path.to_str().unwrap().to_string()
  }

  #[test]
  fn successful_execution()
  {
    let db_file_name = std::env::temp_dir().join("test_db_corona_owid_etl_compact.db");
    let config = DbConfiguration {
      db_path: db_file_name.to_str().unwrap().to_string(),
      csv_input_file: get_csv_path()
    };
    // scope for db
    {
      let db = DbOwidEtlCompact::new(&config).unwrap();
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
      // 2|2020-03-29|722|57|161.471781878622|87.3073489029878|16170|335
      let found = numbers.iter().find(|&n| n.date == "2020-03-29");
      assert!(found.is_some());
      let found = found.unwrap();
      assert_eq!("2020-03-29", found.date);
      assert_eq!(722, found.cases);
      assert_eq!(57, found.deaths);
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

  /**
   * Gets path to the temporary corona.csv file suitable for testing.
   *
   * @return Returns path of the CSV file.
   */
  fn get_csv_overhead_lines_path() -> String
  {
    let path = std::env::temp_dir().join("test_db_corona_owid_etl_compact_overhead.csv");

    let overhead_csv = "country,date,total_cases,new_cases,new_cases_smoothed,total_cases_per_million,new_cases_per_million,new_cases_smoothed_per_million,total_deaths,new_deaths,new_deaths_smoothed,total_deaths_per_million,new_deaths_per_million,new_deaths_smoothed_per_million,excess_mortality,excess_mortality_cumulative,excess_mortality_cumulative_absolute,excess_mortality_cumulative_per_million,hosp_patients,hosp_patients_per_million,weekly_hosp_admissions,weekly_hosp_admissions_per_million,icu_patients,icu_patients_per_million,weekly_icu_admissions,weekly_icu_admissions_per_million,stringency_index,reproduction_rate,total_tests,new_tests,total_tests_per_thousand,new_tests_per_thousand,new_tests_smoothed,new_tests_smoothed_per_thousand,positive_rate,tests_per_case,total_vaccinations,people_vaccinated,people_fully_vaccinated,total_boosters,new_vaccinations,new_vaccinations_smoothed,total_vaccinations_per_hundred,people_vaccinated_per_hundred,people_fully_vaccinated_per_hundred,total_boosters_per_hundred,new_vaccinations_smoothed_per_million,new_people_vaccinated_smoothed,new_people_vaccinated_smoothed_per_hundred,code,continent,population,population_density,median_age,life_expectancy,gdp_per_capita,extreme_poverty,diabetes_prevalence,handwashing_facilities,hospital_beds_per_thousand,human_development_index
Albania,2020-03-01,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,29,3,0.01,0.001,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-02,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,31,2,0.011,0.001,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-03,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,36,5,0.013,0.002,4,0.001,0.0,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-04,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,42,6,0.015,0.002,4,0.001,0.0,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-05,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,50,8,0.018,0.003,5,0.002,0.0,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-06,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,53,3,0.019,0.001,5,0.002,0.0,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-07,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,58,5,0.02,0.002,5,0.002,0.0,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-08,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,8.33,,59,1,0.021,0.0,4,0.001,0.0,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-09,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,36.11,,77,18,0.027,0.006,7,0.002,0.0,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-10,0,0,0.0,0.0,0.0,0.0,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,41.67,,114,37,0.04,0.013,11,0.004,0.0,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-11,2,2,0.2857143,0.70730984,0.70730984,0.10104427,0,0,0.0,0.0,0.0,0.0,,,,,,,,,,,,,51.85,,157,43,0.055,0.015,16,0.006,0.25510204,55.999996,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-12,10,8,1.4285715,3.5365493,2.8292394,0.5052213,1,1,0.14285715,0.35365492,0.35365492,0.050522134,,,,,,,,,,,,,51.85,,298,141,0.104,0.049,35,0.012,0.83819246,40.25,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-13,15,5,2.142857,5.304824,1.7682747,0.757832,1,0,0.14285715,0.35365492,0.0,0.050522134,,,,,,,,,,,,,78.7,,457,159,0.16,0.056,58,0.02,1.3659898,35.855556,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-14,23,8,3.2857144,8.134064,2.8292394,1.1620091,1,0,0.14285715,0.35365492,0.0,0.050522134,,,,,,,,,,,,,78.7,,505,48,0.177,0.017,64,0.022,2.0994081,31.76123,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-15,33,10,4.714286,11.670613,3.5365493,1.6672304,1,0,0.14285715,0.35365492,0.0,0.050522134,,,,,,,,,,,,,81.48,,532,27,0.186,0.009,68,0.024,3.0898044,28.293833,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-16,38,5,5.428571,13.438888,1.7682747,1.919841,1,0,0.14285715,0.35365492,0.0,0.050522134,,,,,,,,,,,,,81.48,,563,31,0.197,0.011,69,0.024,4.2137322,25.696615,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-17,42,4,6.0,14.853507,1.4146197,2.1219296,1,0,0.14285715,0.35365492,0.0,0.050522134,,,,,,,,,,,,,81.48,,605,42,0.212,0.015,70,0.025,5.438222,23.692337,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-18,51,9,7.0,18.036402,3.1828945,2.4755845,1,0,0.14285715,0.35365492,0.0,0.050522134,,,,,,,,,,,,,81.48,,665,60,0.233,0.021,73,0.026,6.552983,17.182133,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-19,55,4,6.428571,19.451021,1.4146197,2.273496,1,0,0.0,0.35365492,0.0,0.0,,,,,,,,,,,,,81.48,,697,32,0.244,0.011,57,0.02,7.5810633,14.9488,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-20,59,4,6.285714,20.86564,1.4146197,2.2229738,2,1,0.14285715,0.70730984,0.35365492,0.050522134,,,,,,,,,,,,,81.48,,732,35,0.256,0.012,39,0.014,9.355725,11.968497,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-21,64,5,5.857143,22.633915,1.7682747,2.0714076,2,0,0.14285715,0.70730984,0.0,0.050522134,,,,,,,,,,,,,81.48,,778,46,0.273,0.016,39,0.014,10.76778,10.137108,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-22,70,6,5.285714,24.755846,2.1219296,1.869319,2,0,0.14285715,0.70730984,0.0,0.050522134,,,,,,,,,,,,,81.48,,811,33,0.284,0.012,40,0.014,11.665139,9.157583,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-23,76,6,5.428571,26.877775,2.1219296,1.919841,2,0,0.14285715,0.70730984,0.0,0.050522134,,,,,,,,,,,,,84.26,,,,,,41,0.014,12.4327,8.420741,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-24,98,22,8.0,34.658184,7.7804084,2.8292394,3,1,0.2857143,1.0609648,0.35365492,0.10104427,,,,,,,,,,,,,84.26,1.1474,,,,,40,0.014,14.065352,7.46836,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-25,104,6,7.571429,36.780113,2.1219296,2.677673,4,1,0.42857143,1.4146197,0.35365492,0.1515664,,,,,,,,,,,,,84.26,1.1372,922,111,0.323,0.039,37,0.013,15.618821,6.676677,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-26,123,19,9.714286,43.499557,6.719444,3.4355052,5,1,0.5714286,1.7682747,0.35365492,0.20208853,,,,,,,,,,,,,84.26,1.1245,1025,103,0.359,0.036,47,0.016,16.960321,6.101187,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-27,146,23,12.428572,51.63362,8.134064,4.395426,5,0,0.42857143,1.7682747,0.0,0.1515664,,,,,,,,,,,,,84.26,1.1058,1127,102,0.395,0.036,56,0.02,17.828415,5.8585014,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-28,174,28,15.714286,61.535957,9.902338,5.5574346,6,1,0.5714286,2.1219296,0.35365492,0.20208853,,,,,,,,,,,,,84.26,1.0922,1206,79,0.422,0.028,61,0.021,19.363102,5.4618273,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-29,186,12,16.571428,65.779816,4.2438593,5.8605676,8,2,0.85714287,2.8292394,0.70730984,0.3031328,,,,,,,,,,,,,84.26,1.0862,1317,111,0.461,0.039,72,0.025,20.76333,5.001436,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-30,197,11,17.285715,69.67002,3.8902042,6.1131783,10,2,1.1428572,3.5365493,0.70730984,0.40417707,,,,,,,,,,,,,84.26,1.083,1407,90,0.493,0.032,80,0.028,21.958576,4.583646,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-03-31,212,15,16.285715,74.974846,5.304824,5.7595234,10,0,1.0,3.5365493,0.0,0.35365492,1.3115113,-2.5931146,-164.99988,-57.554417,,,,,,,,,84.26,1.0833,1552,145,0.544,0.051,95,0.033,21.550413,4.702693,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-04-30,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-05-31,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-06-30,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-07-31,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-08-31,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-09-30,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-10-31,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-11-30,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789
Albania,2020-12-31,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,ALB,Europe,2827615,103.197624,35.943,,15492.067,0.021277364,10.2,,2.89,0.789";
    std::fs::write(&path, overhead_csv).expect("Unable to write CSV file for test!");

    path.to_str().unwrap().to_string()
  }

  #[test]
  fn overhead_cutoff()
  {
    let db_file_name = std::env::temp_dir().join("test_db_corona_owid_etl_compact_overhead.db");
    let config = DbConfiguration {
      db_path: db_file_name.to_str().unwrap().to_string(),
      csv_input_file: get_csv_overhead_lines_path()
    };
    // scope for db
    {
      let db = DbOwidEtlCompact::new(&config).unwrap();
      assert!(db.create_db());
      // Check that DB file exists.
      assert!(db_file_name.exists());
      // Check some content.
      let db = Database::new(&config.db_path).unwrap();
      // Check a country.
      let countries = db.countries();
      {
        let albania = Country
        {
          country_id: 1,
          name: String::from("Albania"),
          population: 2862427,
          geo_id: "AL".to_string(),
          country_code: "ALB".to_string(),
          continent: "Europe".to_string()
        };
        let found = countries.iter().find(|&c| c.geo_id == "AL");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(albania.country_id, found.country_id);
        assert_eq!(albania.name, found.name);
        assert_eq!(albania.population, found.population);
        assert_eq!(albania.geo_id, found.geo_id);
        assert_eq!(albania.country_code, found.country_code);
        assert_eq!(albania.continent, found.continent);
        // Check some numbers.
        let numbers = db.numbers_with_incidence(&albania.country_id);
        // 1|2020-03-23|6|0|2.65508954464166|1.32754477232083|76|2
        let found = numbers.iter().find(|&n| n.date == "2020-03-23");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!("2020-03-23", found.date);
        assert_eq!(6, found.cases);
        assert_eq!(0, found.deaths);
        assert!(found.incidence_14d.is_some());
        assert!(found.incidence_14d.unwrap() > 2.655089);
        assert!(found.incidence_14d.unwrap() < 2.655090);
        assert!(found.incidence_7d.is_some());
        assert!(found.incidence_7d.unwrap() > 1.327544);
        assert!(found.incidence_7d.unwrap() < 1.327545);

        // "Overhead" data should be cut off and should not be found.
        let found = numbers.iter().find(|&n| n.date == "2020-04-30");
        assert!(found.is_none(), "Overhead entry for 2020-04-30 is not cut off during db creation.");
        let found = numbers.iter().find(|&n| n.date == "2020-05-31");
        assert!(found.is_none());
        let found = numbers.iter().find(|&n| n.date == "2020-06-30");
        assert!(found.is_none());
        let found = numbers.iter().find(|&n| n.date == "2020-07-31");
        assert!(found.is_none());
        let found = numbers.iter().find(|&n| n.date == "2020-08-31");
        assert!(found.is_none());
        let found = numbers.iter().find(|&n| n.date == "2020-09-30");
        assert!(found.is_none());
        let found = numbers.iter().find(|&n| n.date == "2020-10-31");
        assert!(found.is_none());
        let found = numbers.iter().find(|&n| n.date == "2020-11-30");
        assert!(found.is_none());
        let found = numbers.iter().find(|&n| n.date == "2020-12-31");
        assert!(found.is_none());
      }
    }
    // clean up
    assert!(std::fs::remove_file(db_file_name).is_ok());
    assert!(std::fs::remove_file(config.csv_input_file).is_ok());
  }
}
