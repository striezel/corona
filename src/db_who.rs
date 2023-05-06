/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2023  Dirk Stolle
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
use crate::database::Database;
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
    let mut last_geo_id = String::new();
    let mut country_id: i64 = -1;
    let mut record = csv::StringRecord::new();
    let mut batch = String::new();
    let mut batch_count: u32 = 0;
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
        // new country
        let name = record.get(2).unwrap();
        let country_code = String::new(); // TODO: Get three letter code from World.
        let population: i64 = -1; // TODO: Get population from World.
        let continent = record.get(3).unwrap(); //TODO: Get from World.
        // Get country id or insert country.
        country_id = db.get_country_id_or_insert(
          current_geo_id,
          name,
          &population,
          &country_code,
          continent
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
      let cases: i64 = match record.get(4).unwrap().is_empty()
      {
        false => record.get(4).unwrap().parse().unwrap_or(i64::MIN),
        true => 0
      };
      let deaths: i64 = match record.get(6).unwrap().is_empty()
      {
        false => record.get(5).unwrap().parse().unwrap_or(i64::MIN),
        true => 0
      };
      if cases == i64::MIN || deaths == i64::MIN
      {
        eprintln!(
          "Error: Got invalid case numbers on line {}.",
          record.position().unwrap().line()
        );
        return false;
      }
      // TODO: Calculate incidence values.
      let incidence14 = f64::NAN;
      let incidence7 = f64::NAN;
      if batch.is_empty()
      {
        batch = String::from(
          "INSERT INTO covid19 (countryId, date, cases, deaths, incidence14, incidence7) VALUES "
        );
        batch_count = 0;
      }

      batch.push('(');
      batch.push_str(&country_id.to_string());
      batch.push_str(", ");
      batch.push_str(&Database::quote(date));
      batch.push_str(", ");
      batch.push_str(&cases.to_string());
      batch.push_str(", ");
      batch.push_str(&deaths.to_string());
      batch.push_str(", ");
      if incidence14.is_nan()
      {
        batch.push_str("NULL");
      }
      else
      {
        batch.push_str(&incidence14.to_string());
      }
      batch.push_str(", ");
      if incidence7.is_nan()
      {
        batch.push_str("NULL");
      }
      else
      {
        batch.push_str(&incidence7.to_string());
      }
      batch.push_str("),");
      batch_count += 1;

      // Perform one insert for every 250 data sets.
      if batch_count >= 250 && !batch.is_empty()
      {
        // replace last ',' with ';' to make it valid SQL syntax
        batch = batch[0..batch.len() - 1].to_string();
        batch.push(';');
        if !db.batch(&batch)
        {
          eprintln!("Error: Could not batch-insert case numbers into database!");
          return false;
        }

        batch.truncate(0);
        batch_count = 0;
      } // if batch
    }
    // Execute remaining batch inserts, if any are left.
    if batch_count > 0 && !batch.is_empty()
    {
      // replace last ',' with ';' to make it valid SQL syntax
      batch = batch[0..batch.len() - 1].to_string();
      batch.push(';');
      if !db.batch(&batch)
      {
        eprintln!("Error: Could not batch-insert case numbers into database!");
        return false;
      }
    } // if batch remains

    // Done.
    true
  }
}

/*  TODO: Write proper test for this operation.
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
    use std::path::Path;

    let csv_path = Path::new(file!()) // current file: src/generator.rs
      .parent()
      .unwrap() // parent: src/
      .join("..") // up one directory
      .join("data") // into directory data/
      .join("corona-daily-who.csv"); // and to the corona-daily.csv file;
    csv_path.to_str().unwrap().to_string()
  }

  #[test]
  fn successful_execution()
  {
    use std::env;
    use std::fs;

    let db_file_name = env::temp_dir().join("test_csv_corona_who.db");
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
      let wf = Country
      {
        country_id: 210,
        name: String::from("Wallis and Futuna"),
        population: -1,
        geo_id: String::from("WF"),
        country_code: String::new(),
        continent: String::from("Oceania")
      };
      let found = countries.iter().find(|&c| c.geo_id == "WF");
      assert!(found.is_some());
      let found = found.unwrap();
      assert_eq!(wf.country_id, found.country_id);
      assert_eq!(wf.name, found.name);
      assert_eq!(wf.population, found.population);
      assert_eq!(wf.geo_id, found.geo_id);
      assert_eq!(wf.country_code, found.country_code);
      assert_eq!(wf.continent, found.continent);
      // Check some numbers.
      let numbers = db.numbers_with_incidence(&wf.country_id);
      let found = numbers.iter().find(|&n| n.date == "2020-11-26");
      assert!(found.is_some());
      let found = found.unwrap();
      assert_eq!("2020-11-26", found.date);
      assert_eq!(1, found.cases);
      assert_eq!(0, found.deaths);
      assert!(found.incidence_14d.is_none());
    }
    // clean up
    assert!(fs::remove_file(db_file_name).is_ok());
  }
}
*/
