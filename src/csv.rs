/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2020, 2021, 2022, 2023, 2025  Dirk Stolle
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

use super::configuration::CsvConfiguration;
use crate::data::Country;
use crate::data::NumbersAndIncidence;
use crate::database::Database;
use crate::DateFormat;

use std::path::Path;

pub struct Csv
{
  config: CsvConfiguration
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
  pub fn new(config: &CsvConfiguration) -> Result<Csv, String>
  {
    if config.db_path.is_empty()
    {
      return Err("Path to SQLite database must not be an empty string!".to_string());
    }
    if config.csv_output_file.is_empty()
    {
      return Err("Path of CSV file must be set to a non-empty string!".to_string());
    }

    Ok(Csv
    {
      config: CsvConfiguration
      {
        db_path: config.db_path.clone(),
        csv_output_file: config.csv_output_file.clone(),
        date_format: config.date_format
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

    let db = Database::new(&self.config.db_path);
    let db = match db
    {
      Ok(db) => db,
      Err(_) =>
      {
        eprintln!(
          "Error: Database file {} does not exist or is not readable!",
          self.config.db_path
        );
        return false;
      }
    };
    let countries = db.countries();
    if countries.is_empty()
    {
      // Something is wrong here, there is no data.
      eprintln!(
        "Error: Could not find any countries in the database {}!",
        self.config.db_path
      );
      return false;
    }
    // Do not overwrite existing file.
    let path = Path::new(&self.config.csv_output_file);
    if path.exists()
    {
      eprintln!(
        "Error: A file or directory named {} already exists!",
        self.config.csv_output_file
      );
      return false;
    }
    // Write CSV header.
    let mut writer = match csv::Writer::from_path(&self.config.csv_output_file)
    {
      Ok(w) => w,
      Err(e) =>
      {
        eprintln!("Error: Could not create CSV file! {e}");
        return false;
      }
    };
    const CSV_HEADER: [&str; 13] = [
      "dateRep",
      "day",
      "month",
      "year",
      "cases",
      "deaths",
      "countriesAndTerritories",
      "geoId",
      "countryterritoryCode",
      "popData2019",
      "continentExp",
      "Cumulative_number_for_14_days_of_COVID-19_cases_per_100000",
      "Cumulative_number_for_7_days_of_COVID-19_cases_per_100000"
    ];
    if let Err(e) = writer.write_record(CSV_HEADER)
    {
      eprintln!("Error: Could not write CSV header! {e}");
      return false;
    }
    let date_format = &self.config.date_format;
    // Handle each country.
    for country in countries.iter()
    {
      let numbers = db.numbers_with_incidence(&country.country_id);
      if numbers.is_empty()
      {
        eprintln!(
          "Error while retrieving data for {} ({}) from the database!",
          &country.name, &country.geo_id
        );
        return false;
      }
      for num in numbers.iter()
      {
        let rec = Csv::num_to_vec(num, country, date_format);
        let success = writer.write_record(&rec);
        if success.is_err()
        {
          eprintln!(
            "Error while writing data record for {} to {}! {}",
            &country.name,
            &self.config.csv_output_file,
            success.unwrap_err()
          );
          return false;
        }
      }
    }

    match writer.flush()
    {
      Ok(_) => true,
      Err(e) =>
      {
        eprintln!("Error: Could not flush write buffer! {e}");
        false
      }
    }
  }

  /**
   * Converts data from a NumbersAndIncidence and Country to a vector of strings
   * that can be used to create a CSV record.
   *
   * @param num      Corona case numbers and 14-day incidence value
   * @param country  country data
   * @return Returns a vector of strings that is suitable for a CSV record.
   */
  fn num_to_vec(num: &NumbersAndIncidence, country: &Country, format: &DateFormat) -> Vec<String>
  {
    let day: String = num.date[8..10].trim_start_matches('0').to_string();
    let month = num.date[5..7].trim_start_matches('0').to_string();
    let year = num.date[0..4].to_string();
    /* When .to_string() is removed as suggested by the clippy lint, then the
       build fails with:

       error[E0277]: the size for values of type `str` cannot be known at compilation time
       help: the trait `Sized` is not implemented for `str`

       Therefore, the suggested change cannot be applied here.
     */
    #[allow(clippy::to_string_in_format_args)]
    let date_rep = match format
    {
        DateFormat::Iso8601 =>  format!("{}-{}-{}", year, num.date[5..7].to_string(), num.date[8..10].to_string()),
        DateFormat::LegacyEcdc =>format!("{}/{}/{}", num.date[8..10].to_string(), num.date[5..7].to_string(), year)
    };
    vec![date_rep, day, month, year,
         num.cases.to_string(), num.deaths.to_string(), country.name.clone(),
         country.geo_id.clone(), country.country_code.clone(),
         country.population.to_string(), country.continent.clone(),
         match num.incidence_14d
         {
           Some(i14d) => i14d.to_string(),
           None => String::new()
         },
         match num.incidence_7d
         {
           Some(i7d) => i7d.to_string(),
           None => String::new()
         }
    ]
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  /**
   * Gets path to the corona.db file in data directory.
   *
   * @return Returns path of the SQLite database.
   */
  fn get_sqlite_db_path() -> String
  {
    let db_path = Path::new(file!()) // current file: src/generator.rs
      .parent()                      // parent: src/
      .unwrap()                      // unwrap is save, parent directory exists
      .join("..")                    // up one directory
      .join("data")                  // into directory data/
      .join("corona-ecdc-2020-12-14.db"); // and to the corona.db file;
    db_path.to_str().unwrap().to_string()
  }

  #[test]
  fn successful_execution_ecdc()
  {
    use std::env;
    use std::fs;

    let csv_file_name = env::temp_dir().join("test_csv_corona_ecdc_date.csv");
    let config = CsvConfiguration {
      db_path: get_sqlite_db_path(),
      csv_output_file: csv_file_name.to_str().unwrap().to_string(),
      date_format: DateFormat::LegacyEcdc
    };
    let csv = Csv::new(&config).unwrap();
    assert!(csv.create_csv());
    // Check that CSV file exists.
    assert!(csv_file_name.exists());
    // Check contents.
    let contents = fs::read_to_string(&csv_file_name);
    assert!(contents.is_ok());
    let contents = contents.unwrap();
    // -- Check header line.
    let first_line = contents.lines().next();
    assert!(first_line.is_some());
    assert_eq!("dateRep,day,month,year,cases,deaths,\
                countriesAndTerritories,geoId,countryterritoryCode,popData2019,continentExp,\
                Cumulative_number_for_14_days_of_COVID-19_cases_per_100000,\
                Cumulative_number_for_7_days_of_COVID-19_cases_per_100000",
                first_line.unwrap());
    // -- Check a single line with 14-day incidence value.
    let line = "10/12/2020,10,12,2020,23679,440,Germany,DE,DEU,83019213,Europe,311.5122279,";
    let found = contents.lines().find(|&l| l == line);
    assert!(found.is_some());
    // -- Check a single line without incidence value.
    let line = "12/01/2020,12,1,2020,0,0,Germany,DE,DEU,83019213,Europe,,";
    let found = contents.lines().find(|&l| l == line);
    assert!(found.is_some());
    // clean up
    assert!(fs::remove_file(csv_file_name).is_ok());
  }

  #[test]
  fn successful_execution_iso8601()
  {
    use std::env;
    use std::fs;

    let csv_file_name = env::temp_dir().join("test_csv_corona_iso_date.csv");
    let config = CsvConfiguration {
      db_path: get_sqlite_db_path(),
      csv_output_file: csv_file_name.to_str().unwrap().to_string(),
      date_format: DateFormat::Iso8601
    };
    let csv = Csv::new(&config).unwrap();
    assert!(csv.create_csv());
    // Check that CSV file exists.
    assert!(csv_file_name.exists());
    // Check contents.
    let contents = fs::read_to_string(&csv_file_name);
    assert!(contents.is_ok());
    let contents = contents.unwrap();
    // -- Check header line.
    let first_line = contents.lines().next();
    assert!(first_line.is_some());
    assert_eq!("dateRep,day,month,year,cases,deaths,\
                countriesAndTerritories,geoId,countryterritoryCode,popData2019,continentExp,\
                Cumulative_number_for_14_days_of_COVID-19_cases_per_100000,\
                Cumulative_number_for_7_days_of_COVID-19_cases_per_100000",
                first_line.unwrap());
    // -- Check a single line with 14-day incidence value.
    let line = "2020-12-10,10,12,2020,23679,440,Germany,DE,DEU,83019213,Europe,311.5122279,";
    let found = contents.lines().find(|&l| l == line);
    assert!(found.is_some());
    // -- Check a single line without incidence value.
    let line = "2020-01-12,12,1,2020,0,0,Germany,DE,DEU,83019213,Europe,,";
    let found = contents.lines().find(|&l| l == line);
    assert!(found.is_some());
    // clean up
    assert!(fs::remove_file(csv_file_name).is_ok());
  }
}
