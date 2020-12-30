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

use super::configuration::DbConfiguration;

pub struct Db
{
  config: DbConfiguration
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
    // TODO: Implement function!
    eprintln!("Error: Creation of SQLite database is not implemented yet.");
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /**
   * Gets path to the corona_daily.csv file in data directory.
   *
   * @return Returns path of the SQLite database.
   */
  fn get_csv_path() -> String
  {
    use std::path::Path;

    let csv_path = Path::new(file!()) // current file: src/generator.rs
        .parent().unwrap() // parent: src/
        .join("..") // up one directory
        .join("data") // into directory data/
        .join("corona-daily.csv"); // and to the corona-daily.csv file;
    csv_path.to_str().unwrap().to_string()
  }

  #[test]
  fn successful_execution()
  {
    use std::env;
    use std::fs;

    let db_file_name = env::temp_dir().join("test_csv_corona.db");
    let config = DbConfiguration {
      db_path: db_file_name.to_str().unwrap().to_string(),
      csv_input_file: get_csv_path()
    };
    let db = Db::new(&config).unwrap();
    assert!(db.create_db());
    // Check that DB file exists.
    assert!(db_file_name.exists());
    // clean up
    assert!(fs::remove_file(db_file_name).is_ok());
  }
}
