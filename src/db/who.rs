/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2023, 2025  Dirk Stolle
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

pub struct DbWho
{
  config: DbConfiguration
}

impl DbWho
{
  /**
   * Creates a new instance.
   *
   * @config   application configuration
   * @return   Returns a Result containing the Db object, if successful.
   *           Returns a string with an error message, if the configuration
   *           seems to be invalid.
   */
  pub fn new(config: &DbConfiguration) -> Result<DbWho, String>
  {
    if config.db_path.is_empty()
    {
      return Err("Path for SQLite database must not be an empty string!".to_string());
    }
    if config.csv_input_file.is_empty()
    {
      return Err("Path of CSV file must be set to a non-empty string!".to_string());
    }

    Ok(DbWho
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
    DbWho::parse_csv_into_db(db, &mut reader)
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
      "Date_reported",
      "Country_code",
      "Country",
      "WHO_region",
      "New_cases",
      "Cumulative_cases",
      "New_deaths",
      "Cumulative_deaths",
    ];
    if headers != expected_headers
    {
      eprintln!("Error: CSV headers do not match the expected headers. \
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
    let mut last_geo_id = String::new();
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
      if r_len != 8
      {
        eprintln!("Error: A line of CSV data does not have eight data elements, \
                   but {} elements instead!\nThe line position is '{}'. \
                   It will be skipped.",
                   r_len, record.position().unwrap().line());
        return false;
      }
      let current_geo_id = record.get(1).unwrap();
      if current_geo_id != last_geo_id
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
        let name = name.trim_end_matches("[1]");
        let no_country = Country {
          country_id: -1,
          name: name.to_string(),
          population: -1,
          geo_id: current_geo_id.to_string(),
          country_code: String::new(),
          continent: record.get(3).unwrap().to_string()
        };
        let world_data = world.find_by_geo_id(current_geo_id).unwrap_or(&no_country);
        population = world_data.population;
        // Get country id or insert country.
        country_id = db.get_country_id_or_insert(
          current_geo_id,
          name,
          &i64::from(world_data.population),
          &world_data.country_code,
          &world_data.continent
        );
        if country_id == -1
        {
          eprintln!("Error: Could not insert country data into database!");
          return false;
        }
        last_geo_id = current_geo_id.to_string();
      }
      // Add current record.
      let date = record.get(0).unwrap();
      let cases: i32 = match record.get(4).unwrap().is_empty()
      {
        false => record.get(4).unwrap().parse().unwrap_or(i32::MIN),
        true => 0
      };
      let deaths: i32 = match record.get(6).unwrap().is_empty()
      {
        false => record.get(6).unwrap().parse().unwrap_or(i32::MIN),
        true => 0
      };
      if cases == i32::MIN || deaths == i32::MIN
      {
        eprintln!(
          "Error: Got invalid case numbers on line {}.",
          record.position().unwrap().line()
        );
        return false;
      }
      parsed_data.push(Numbers { date: String::from(date), cases, deaths});
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
   * Gets path to the corona_daily.csv file in data directory.
   *
   * @return Returns path of the CSV file.
   */
  fn get_csv_path() -> String
  {
    let path = std::env::temp_dir().join("test_db_corona_who.csv");

    let simple_csv =
      "Date_reported,Country_code,Country,WHO_region,New_cases,Cumulative_cases,New_deaths,Cumulative_deaths
2020-01-03,DE,Germany,EURO,0,0,0,0
2020-01-04,DE,Germany,EURO,1,1,0,0
2020-01-05,DE,Germany,EURO,0,1,0,0
2020-01-06,DE,Germany,EURO,0,1,0,0
2020-01-07,DE,Germany,EURO,0,1,0,0
2020-01-08,DE,Germany,EURO,0,1,0,0
2020-01-09,DE,Germany,EURO,0,1,0,0
2020-01-10,DE,Germany,EURO,0,1,0,0
2020-01-11,DE,Germany,EURO,0,1,0,0
2020-01-12,DE,Germany,EURO,0,1,0,0
2020-01-13,DE,Germany,EURO,0,1,0,0
2020-01-14,DE,Germany,EURO,0,1,0,0
2020-01-15,DE,Germany,EURO,0,1,0,0
2020-01-16,DE,Germany,EURO,0,1,0,0
2020-01-17,DE,Germany,EURO,0,1,0,0
2020-01-18,DE,Germany,EURO,0,1,0,0
2020-01-19,DE,Germany,EURO,0,1,0,0
2020-01-20,DE,Germany,EURO,0,1,0,0
2020-01-21,DE,Germany,EURO,0,1,0,0
2020-01-22,DE,Germany,EURO,0,1,0,0
2020-01-23,DE,Germany,EURO,0,1,0,0
2020-01-24,DE,Germany,EURO,1,2,0,0
2020-01-25,DE,Germany,EURO,0,2,0,0
2020-01-26,DE,Germany,EURO,0,2,0,0
2020-01-27,DE,Germany,EURO,0,2,0,0
2020-01-28,DE,Germany,EURO,0,2,0,0
2020-01-29,DE,Germany,EURO,2,4,0,0
2020-01-30,DE,Germany,EURO,2,6,0,0
2020-01-31,DE,Germany,EURO,0,6,0,0
2020-02-01,DE,Germany,EURO,4,10,0,0
2020-02-02,DE,Germany,EURO,1,11,0,0
2020-02-03,DE,Germany,EURO,0,11,0,0
2020-02-04,DE,Germany,EURO,1,12,0,0
2020-02-05,DE,Germany,EURO,4,16,0,0
2020-02-06,DE,Germany,EURO,2,18,0,0
2020-02-07,DE,Germany,EURO,1,19,0,0
2020-02-08,DE,Germany,EURO,1,20,0,0
2020-02-09,DE,Germany,EURO,0,20,0,0
2020-02-10,DE,Germany,EURO,0,20,0,0
2020-02-11,DE,Germany,EURO,1,21,0,0
2020-02-12,DE,Germany,EURO,2,23,0,0
2020-02-13,DE,Germany,EURO,1,24,0,0
2020-02-14,DE,Germany,EURO,0,24,0,0
2020-02-23,CH,Switzerland,EURO,0,0,0,0
2020-02-24,CH,Switzerland,EURO,1,1,0,0
2020-02-25,CH,Switzerland,EURO,0,1,0,0
2020-02-26,CH,Switzerland,EURO,1,2,0,0
2020-02-27,CH,Switzerland,EURO,10,12,0,0
2020-02-28,CH,Switzerland,EURO,10,22,0,0
2020-02-29,CH,Switzerland,EURO,10,32,0,0
2020-03-01,CH,Switzerland,EURO,13,45,0,0
2020-03-02,CH,Switzerland,EURO,12,57,0,0
2020-03-03,CH,Switzerland,EURO,30,87,0,0
2020-03-04,CH,Switzerland,EURO,33,120,0,0
2020-03-05,CH,Switzerland,EURO,61,181,0,0
2020-03-06,CH,Switzerland,EURO,62,243,2,2
2020-03-07,CH,Switzerland,EURO,73,316,0,2
2020-03-08,CH,Switzerland,EURO,49,365,0,2
2020-03-09,CH,Switzerland,EURO,69,434,1,3
2020-03-10,CH,Switzerland,EURO,191,625,0,3
2020-03-11,CH,Switzerland,EURO,210,835,2,5
2020-03-12,CH,Switzerland,EURO,333,1168,3,8
2020-03-13,CH,Switzerland,EURO,357,1525,4,12
2020-03-14,CH,Switzerland,EURO,431,1956,3,15
2020-03-15,CH,Switzerland,EURO,417,2373,5,20
2020-03-16,CH,Switzerland,EURO,326,2699,8,28
2020-03-17,CH,Switzerland,EURO,1060,3759,7,35
2020-03-18,CH,Switzerland,EURO,1082,4841,12,47
2020-03-19,CH,Switzerland,EURO,1206,6047,8,55
2020-03-20,CH,Switzerland,EURO,834,6881,14,69
2020-03-21,CH,Switzerland,EURO,1139,8020,21,90
2020-03-22,CH,Switzerland,EURO,690,8710,19,109
2020-03-23,CH,Switzerland,EURO,547,9257,18,127
2020-03-24,CH,Switzerland,EURO,1462,10719,23,150
2020-03-25,CH,Switzerland,EURO,1242,11961,19,169
2020-03-26,CH,Switzerland,EURO,1068,13029,39,208
2020-03-27,CH,Switzerland,EURO,1114,14143,38,246
2020-03-28,CH,Switzerland,EURO,1305,15448,32,278
2020-03-29,CH,Switzerland,EURO,722,16170,57,335
2020-03-30,CH,Switzerland,EURO,432,16602,45,380
2020-03-31,CH,Switzerland,EURO,1307,17909,58,438";
    std::fs::write(&path, simple_csv).expect("Unable to write CSV file for test!");

    path.to_str().unwrap().to_string()
  }

  #[test]
  fn successful_execution()
  {
    let db_file_name = std::env::temp_dir().join("test_db_corona_who.db");
    let config = DbConfiguration {
      db_path: db_file_name.to_str().unwrap().to_string(),
      csv_input_file: get_csv_path()
    };
    // scope for db
    {
      let db = DbWho::new(&config).unwrap();
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
        // 1|2020-02-12|2|0|0.022886268507508|0.00843178313434506|23|0
        let found = numbers.iter().find(|&n| n.date == "2020-02-12");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!("2020-02-12", found.date);
        assert_eq!(2, found.cases);
        assert_eq!(0, found.deaths);
        assert!(found.incidence_14d.is_some());
        assert!(found.incidence_14d.unwrap() > 0.022886);
        assert!(found.incidence_14d.unwrap() < 0.022887);
        assert!(found.incidence_7d.is_some());
        assert!(found.incidence_7d.unwrap() > 0.008431);
        assert!(found.incidence_7d.unwrap() < 0.008432);
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
      // 2|2020-03-28|1305|32|157.90224549586|86.9328401677471|15448|278
      let found = numbers.iter().find(|&n| n.date == "2020-03-28");
      assert!(found.is_some());
      let found = found.unwrap();
      assert_eq!("2020-03-28", found.date);
      assert_eq!(1305, found.cases);
      assert_eq!(32, found.deaths);
      assert!(found.incidence_14d.is_some());
      assert!(found.incidence_14d.unwrap() > 157.902245);
      assert!(found.incidence_14d.unwrap() < 157.902246);
      assert!(found.incidence_7d.is_some());
      assert!(found.incidence_7d.unwrap() > 86.932840);
      assert!(found.incidence_7d.unwrap() < 86.932841);
    }
    // clean up
    assert!(std::fs::remove_file(db_file_name).is_ok());
    assert!(std::fs::remove_file(config.csv_input_file).is_ok());
  }
}
