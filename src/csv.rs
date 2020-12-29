/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2020  Dirk Stolle
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

use super::Configuration;
use crate::database::Database;

use std::path::Path;

pub struct Csv
{
  config: Configuration
}

impl Csv
{
  /**
   * Creates a new Csv instance.
   *
   * @config   application configuration
   * @return   Returns a Result containing the Csv object, if successful.
   *           Returns a string with an error message, if the configuration
   *           seems to be invalid.
   */
  pub fn new(config: &Configuration) -> Result<Csv, String>
  {
    if config.db_path.is_empty()
    {
      return Err("Path to SQLite database must not be an empty string!".to_string());
    }
    if config.output_directory.is_empty()
    {
      return Err("Path of CSV file must be set to a non-empty string!".to_string());
    }

    Ok(Csv
    {
      config: Configuration
      {
        db_path: config.db_path.clone(),
        output_directory: config.output_directory.clone(),
        op: config.op.clone()
      }
    })
  }

  /**
   * Creates the CSV file.
   *
   * @return Returns whether the operation was successful.
   */
  pub fn create_csv(&self) -> bool
  {
    let db = Database::new(&self.config.db_path);
    let db = match db
    {
      Ok(db) => db,
      Err(_) => {
        eprintln!("Error: Database file {} does not exist or is not readable!", self.config.db_path);
        return false;
      }
    };
    let countries = db.countries();
    if countries.is_empty()
    {
      // Something is wrong here, there is no data.
      eprintln!("Error: Could not find any countries in the database {}!",
                self.config.db_path);
      return false;
    }
    // Do not overwrite existing file.
    let path = Path::new(&self.config.output_directory);
    if path.exists()
    {
      eprintln!("Error: A file or directory named {} already exists!",
                self.config.output_directory);
      return false;
    }
    // Write CSV header.
    // TODO!
    let mut writer = match csv::Writer::from_path(&self.config.output_directory)
    {
      Ok(w) => w,
      Err(e) => {
        eprintln!("Error: Could not create CSV file! {}", e);
        return false;
      }
    };
    const CSV_HEADER: [&str; 11] = ["dateRep", "day", "month", "year", "cases", "deaths",
      "countriesAndTerritories", "geoId","countryterritoryCode", "popData2019","continentExp"];
    match writer.write_record(&CSV_HEADER)
    {
      Err(e) => {
        eprintln!("Error: Could not write CSV header! {}", e);
        return false;
      },
      _ => ()
    }
    // Handle each country.
    for country in countries.iter()
    {
      let numbers = db.numbers(&country.country_id);
      if numbers.is_empty()
      {
        eprintln!("Error while generating file for {} ({})!", &country.name, &country.geo_id);
        return false;
      }
      for num in numbers.iter()
      {
        let rec = vec![num.date.clone(), num.date[8..10].to_string(), num.date[5..7].to_string(), num.date[0..4].to_string(),
                       num.cases.to_string(), num.deaths.to_string(), country.name.clone(),
                       country.geo_id.clone(), String::new(), country.population.to_string(), country.continent.clone()];
        let success = writer.write_record(&rec);
        if success.is_err()
        {
          eprintln!("Error while writing data record for {} to {}! {}",
                    &country.name, &self.config.output_directory,
                    success.unwrap_err());
          return false;
        }
      }
    }

    match writer.flush()
    {
      Ok(_) => true,
      Err(e) => {
        eprintln!("Error: Could not flush write buffer! {}", e);
        false
      }
    }
  }
}
