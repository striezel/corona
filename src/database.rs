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

use std::path::Path;

use rusqlite::{Connection, params};

/// struct that contains data of a single country
pub struct Country
{
  pub country_id: i32,
  pub name: String,
  pub population: i32,
  pub geo_id: String,
  pub continent: String
}

/// struct to hold the case numbers for a single day in a single country
pub struct Numbers
{
  pub date: String,
  pub cases: i32,
  pub deaths: i32
}

/// struct to hold 14-day incidence value for a single day in a single country
pub struct Incidence14
{
  pub date: String,
  pub incidence: f64
}

pub struct Database
{
  conn: rusqlite::Connection
}

impl Database
{
  /**
   * Opens an existing SQLite database.
   *
   * @db_path   path of the SQLite database file to open
   * @return    Returns a Result containing the Database object, if successful.
   *            Returns a string with an error message, if the database could
   *            not be opened.
   */
  pub fn new(db_path: &str) -> Result<Database, String>
  {
    let path = Path::new(db_path);
    if !path.is_file() || !path.exists()
    {
      return Err(String::from("Database file does not exist!"));
    }

    let conn = Connection::open(db_path);
    match conn
    {
      Err(_e) => Err(String::from("Failed to open database!")),
      Ok(c) => Ok(Database { conn: c })
    }
  }

  /**
   * Lists all countries in the database.
   *
   * @return Returns a vector of country data.
   */
  pub fn countries(&self) -> Vec<Country>
  {
    let sql = "SELECT countryId, name, population, geoId, continent FROM country".to_owned()
            + " WHERE geoId <> '' AND continent <> 'Other'"
            + " ORDER BY name ASC;";
    let stmt = self.conn.prepare(&sql);
    let mut stmt = match stmt
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let country_iter = stmt.query_map(params![], |row| {
      Ok(Country {
        country_id: row.get(0).unwrap_or(-1),
        name: row.get(1).unwrap_or_else(|_| String::new()),
        population: row.get(2).unwrap_or(-1),
        geo_id: row.get(3).unwrap_or_else(|_| String::new()),
        continent: row.get(4).unwrap_or_else(|_| String::new())
      })
    });
    let country_iter = match country_iter
    {
      Ok(iter) => iter,
      Err(_) => return vec![]
    };
    let mut data: Vec<Country> = Vec::new();
    for country in country_iter
    {
      data.push(country.unwrap());
    }
    data
  }

  /**
   * Lists all continents in the database.
   *
   * @return Returns an array of continent names.
   */
  pub fn continents(&self) -> Vec<String>
  {
    let sql = "SELECT DISTINCT continent FROM country".to_owned()
            + " WHERE continent <> 'Other'"
            + " ORDER BY continent ASC;";
    let stmt = self.conn.prepare(&sql);
    let mut stmt = match stmt
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let continent_iter = stmt.query_map(params![], |row| {
      Ok(row.get(0).unwrap_or_else(|_| String::new()))
    });
    let continent_iter = match continent_iter
    {
      Ok(iter) => iter,
      Err(_) => return vec![]
    };
    let mut data: Vec<String> = Vec::new();
    for continent in continent_iter
    {
      data.push(continent.unwrap());
    }
    data
  }

  /**
   * Lists all countries of a given continent.
   *
   * @param continent   name of the continent
   * @return Returns a vetor of country data.
   */
  pub fn countries_of_continent(&self, continent: &str) -> Vec<Country>
  {
    let sql = "SELECT countryId, name, population, geoId, continent FROM country".to_owned()
           + " WHERE geoId <> '' AND continent = ?"
           + " ORDER BY name ASC;";
    let stmt = self.conn.prepare(&sql);
    let mut stmt = match stmt
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let country_iter = stmt.query_map(params![&continent], |row| {
      Ok(Country {
        country_id: row.get(0).unwrap_or(-1),
        name: row.get(1).unwrap_or_else(|_| String::new()),
        population: row.get(2).unwrap_or(-1),
        geo_id: row.get(3).unwrap_or_else(|_| String::new()),
        continent: row.get(4).unwrap_or_else(|_| String::new())
      })
    });
    let country_iter = match country_iter
    {
      Ok(iter) => iter,
      Err(_) => return vec![]
    };
    let mut data: Vec<Country> = Vec::new();
    for country in country_iter
    {
      data.push(country.unwrap());
    }
    data
  }

  /**
   * Get Covid-19 numbers for a specific country.
   *
   * @param country_id   id of the country
   * @return Returns an array of arrays containing the date, infections and deaths on that day.
   */
  pub fn numbers(&self, country_id: &i32) -> Vec<Numbers>
  {
    let sql = "SELECT date, cases, deaths FROM covid19".to_owned()
            + " WHERE countryId = ?"
            + " ORDER BY date ASC;";
    let stmt = self.conn.prepare(&sql);
    let mut stmt = match stmt
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let rows = stmt.query(params![&country_id]);
    Database::extract_numbers(rows)
  }

  /**
   * Extracts a vector of Numbers from a query result.
   * The query has to contain three columns, where the first column is a date
   * string, the second column is the number of cases and the third column is
   * the number of deaths. Second and third columns must be able to convert to
   * an integer (i32).
   *
   * @param rows  the return value of rusqlite::Statement::query
   * @return  Returns a vector of Numbers.
   */
  fn extract_numbers(rows: Result<rusqlite::Rows, rusqlite::Error>) -> Vec<Numbers>
  {
    let mut rows: rusqlite::Rows = match rows
    {
      Ok(r) => r,
      Err(_) => return vec![]
    };
    let mut data: Vec<Numbers> = Vec::new();
    loop // potential infinite loop
    {
      let row = rows.next();
      match row
      {
        Ok(Some(row)) => data.push(Numbers {
          date: row.get(0).unwrap_or_else(|_e| { String::from("") }),
          cases: row.get(1).unwrap_or(0),
          deaths: row.get(2).unwrap_or(0),
        }),
        Ok(None) => break,
        _ => return vec![]
      }
    }

    data
  }

  /**
   * Get total Covid-19 numbers worldwide.
   *
   * @return Returns an array of arrays containing the date, infections and deaths on that day.
   */
  pub fn numbers_world(&self) -> Vec<Numbers>
  {
    let sql = "SELECT date, SUM(cases), SUM(deaths) FROM covid19".to_owned()
        + " GROUP BY date"
        + " ORDER BY date ASC;";
    let stmt = self.conn.prepare(&sql);
    let mut stmt = match stmt
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let rows = stmt.query(params![]);
    Database::extract_numbers(rows)
  }

  /**
   * Get accumulated Covid-19 numbers for a specific country.
   *
   * @param countryId   id of the country
   * @return Returns an array of arrays containing the date, total infections and total deaths until this date.
   */
  pub fn accumulated_numbers(&self, country_id: &i32) -> Vec<Numbers>
  {
    let sql = "SELECT date, totalCases, totalDeaths FROM covid19".to_owned()
        + " WHERE countryId = ?"
        + " ORDER BY date ASC;";
    let stmt = self.conn.prepare(&sql);
    let mut stmt = match stmt
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let rows = stmt.query(params![&country_id]);
    Database::extract_numbers(rows)
  }

  /**
   * Get accumulated total Covid-19 numbers worldwide.
   *
   * @return Returns a vector of Numbers containing the date, infections and deaths up to that date.
   */
  pub fn accumulated_numbers_world(&self) -> Vec<Numbers>
  {
    let sql = "SELECT date, SUM(totalCases), SUM(totalDeaths) FROM covid19".to_owned()
        + " GROUP BY date"
        + " ORDER BY date ASC;";
    let stmt = self.conn.prepare(&sql);
    let mut stmt = match stmt
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let rows = stmt.query(params![]);
    Database::extract_numbers(rows)
  }

  /**
   * Get the 14-day incidence values of Covid-19 for a specific country.
   *
   * @param countryId   id of the country
   * @return Returns a vector of Incidences.
   *         This may be an empty vector, if no values are known.
   */
  pub fn incidence(&self, country_id: &i32) -> Vec<Incidence14>
  {
    let sql = "SELECT date, round(incidence14, 2) FROM covid19".to_owned()
            + " WHERE countryId = ? AND IFNULL(incidence14, -1.0) >= 0.0"
            + " ORDER BY date ASC;";
    let stmt = self.conn.prepare(&sql);
    let mut stmt = match stmt
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let rows = stmt.query(params![&country_id]);
    let mut rows = match rows
    {
      Ok(r) => r,
      Err(_) => return vec![]
    };
    let mut data: Vec<Incidence14> = Vec::new();
    loop // potential infinite loop
    {
      let row = rows.next();
      match row
      {
        Ok(Some(row)) => data.push(Incidence14 {
          date: row.get(0).unwrap_or_else(|_e| { String::from("") }),
          incidence: row.get(1).unwrap_or(0.0)
        }),
        Ok(None) => break,
        _ => return vec![]
      }
    }

    data
  }

  /**
   * Checks whether the table covid19 already has the columns totalCases and
   * totalDeaths, and creates them, if they are missing.
   *
   * @return Returns whether the operation was successful.
   */
  pub fn calculate_total_numbers(&self) -> bool
  {
    let mut has_total_cases = false;
    let mut has_total_deaths = false;
    let mut stmt = match self.conn.prepare("PRAGMA table_info(covid19);")
    {
      Ok(x) => x,
      Err(_) => return false
    };
    let mut rows = match stmt.query(params![])
    {
      Ok(r) => r,
      Err(_) => return false
    };
    loop // potential infinite loop
    {
      match rows.next()
      {
        Ok(Some(row)) => {
          let name = row.get(0).unwrap_or_else(|_e| { String::new() });
          if name == "totalCases"
          {
            has_total_cases = true;
          }
          else if name == "totalDeaths"
          {
            has_total_deaths = true;
          }
        },
        Ok(None) => break,
        _ => return false
      }
    }

    if !has_total_cases
    {
      if !self.calculate_total_cases()
      {
        return false;
      }
    }
    if !has_total_deaths
    {
      if !self.calculate_total_deaths()
      {
        return false;
      }
    }

    true
  }

  /**
   * Creates the column totalCases and calculates all required values for it.
   * This may take quite a while.
   *
   * @return Returns whether the operation was successful.
   */
  fn calculate_total_cases(&self) -> bool
  {
    // add new column
    match self.conn.execute("ALTER TABLE covid19 ADD COLUMN totalCases INTEGER;", params![])
    {
      Ok(_) => println!("Info: Added column totalCases to table."),
      Err(e) => {
        eprintln!("Could not add column totalCases to table covid19! {}", e);
        return false;
      }
    };
    // perform actual calculation
    eprintln!("Calculating accumulated number of cases for each day and \
               country. This may take a while...");
    match self.conn.execute(
      "UPDATE covid19 AS c1 \
       SET totalCases=(SELECT SUM(cases) FROM covid19 AS c2 \
       WHERE c2.countryId = c1.countryId AND c2.date <= c1.date);",
      params![])
    {
      Ok(affected) => println!("{} rows have been updated.", affected),
      Err(e)=> {
        eprintln!("Could not update totalCases in table covid19! {}", e);
        return false;
      }
    };

    true
  }

  /**
   * Creates the column totalCases and calculates all required values for it.
   * This may take quite a while.
   *
   * @return Returns whether the operation was successful.
   */
  fn calculate_total_deaths(&self) -> bool
  {
    // add new column
    match self.conn.execute("ALTER TABLE covid19 ADD COLUMN totalDeaths INTEGER;", params![])
    {
      Ok(_) => println!("Info: Added column totalDeaths to table."),
      Err(e)=> {
        eprintln!("Could not add column totalDeaths to table covid19! {}", e);
        return false;
      }
    };
    // Update may take ca. two minutes.
    println!("Calculating accumulated number of deaths for each day and country. \
              This may take a while...");
    match self.conn.execute(
      "UPDATE covid19 AS c1 \
       SET totalDeaths=(SELECT SUM(deaths) FROM covid19 AS c2 \
       WHERE c2.countryId = c1.countryId AND c2.date <= c1.date);",
      params![])
    {
      Ok(affected) => println!("{} rows have been updated.", affected),
      Err(e)=> {
        eprintln!("Could not update totalDeaths in table covid19! {}", e);
        return false;
      }
    }

    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /**
   * Gets a database instance connected to the corona.db file in data directory.
   *
   * @return Returns an open database.
   */
  fn get_sqlite_db() -> Database
  {
    let db_path = Path::new(file!()) // current file: src/database.rs
        .parent().unwrap() // parent: src/
        .join("..") // up one directory
        .join("data") // into directory data/
        .join("corona.db"); // and to the corona.db file;
    let db = Database::new(db_path.to_str().unwrap());
    assert!(db.is_ok());
    return db.unwrap();
  }

  #[test]
  fn continents()
  {
    let db = get_sqlite_db();

    let continents = db.continents();
    // Vector of continents must not be empty.
    assert!(!continents.is_empty());
    // Some continents shall be contained in the vector.
    assert!(continents.contains(&String::from("Asia")));
    assert!(continents.contains(&String::from("Africa")));
    assert!(continents.contains(&String::from("America")));
    assert!(continents.contains(&String::from("Europe")));
    assert!(continents.contains(&String::from("Oceania")));
    // "Other" should be filtered from list.
    assert!(!continents.contains(&String::from("Other")));
  }

  #[test]
  fn countries()
  {
    let db = get_sqlite_db();

    let countries = db.countries();
    // Vector of countries must not be empty.
    assert!(!countries.is_empty());
    // There should be more than 200 countries.
    assert!(countries.len() > 200);
    // Check whether a specific country is in the vector.
    let germany = Country {
      country_id: 76,
      name: String::from("Germany"),
      population: 83019213,
      geo_id: String::from("DE"),
      continent: String::from("Europe")
    };
    let found = countries.iter().find(|&c| c.name == "Germany");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany.country_id, found.country_id);
    assert_eq!(germany.name, found.name);
    assert_eq!(germany.population, found.population);
    assert_eq!(germany.geo_id, found.geo_id);
    assert_eq!(germany.continent, found.continent);
  }

  #[test]
  fn countries_of_continent()
  {
    let db = get_sqlite_db();

    let countries = db.countries_of_continent("Europe");
    // Vector of countries must not be empty.
    assert!(!countries.is_empty());
    // There should be less than 200 countries, because unlike countries() it
    // does not list all countries.
    assert!(countries.len() < 200);
    // Check whether a specific country is in the vector.
    let germany = Country {
      country_id: 76,
      name: String::from("Germany"),
      population: 83019213,
      geo_id: String::from("DE"),
      continent: String::from("Europe")
    };
    let found = countries.iter().find(|&c| c.name == "Germany");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany.country_id, found.country_id);
    assert_eq!(germany.name, found.name);
    assert_eq!(germany.population, found.population);
    assert_eq!(germany.geo_id, found.geo_id);
    assert_eq!(germany.continent, found.continent);
    // Check that some other country is not found.
    let not_found = countries.iter().find(|&c| c.name == "China");
    assert!(not_found.is_none());

    // Check for another continent.
    let countries = db.countries_of_continent("Asia");
    // Vector of countries must not be empty.
    assert!(!countries.is_empty());
    // There should be less than 200 countries, because unlike countries() it
    // does not list all countries.
    assert!(countries.len() < 200);
    // Check whether a specific country is in the vector.
    let china = Country {
      country_id: 43,
      name: String::from("China"),
      population: 1433783692,
      geo_id: String::from("CN"),
      continent: String::from("Asia")
    };
    let found = countries.iter().find(|&c| c.name == "China");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(china.country_id, found.country_id);
    assert_eq!(china.name, found.name);
    assert_eq!(china.population, found.population);
    assert_eq!(china.geo_id, found.geo_id);
    assert_eq!(china.continent, found.continent);
    // Check that some other country is not found.
    let not_found = countries.iter().find(|&c| c.name == "Germany");
    assert!(not_found.is_none());
  }

  #[test]
  fn numbers()
  {
    let db = get_sqlite_db();

    let numbers = db.numbers(&76);
    // Vector of data must not be empty.
    assert!(!numbers.is_empty());
    // There should be more than 300 entries, ...
    assert!(numbers.len() > 300);
    // ... but less than 600, because vector has only data from one country.
    assert!(numbers.len() < 600);
    // Check whether a specific value is in the vector.
    let germany_2020_12_10 = Numbers {
      date: String::from("2020-12-10"),
      cases: 23679,
      deaths: 440
    };
    let found = numbers.iter().find(|&n| n.date == "2020-12-10");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_12_10.date, found.date);
    assert_eq!(germany_2020_12_10.cases, found.cases);
    assert_eq!(germany_2020_12_10.deaths, found.deaths);
    // Check another similar value.
    let germany_2020_03_28 = Numbers {
      date: String::from("2020-03-28"),
      cases: 6294,
      deaths: 72
    };
    let found = numbers.iter().find(|&n| n.date == "2020-03-28");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_03_28.date, found.date);
    assert_eq!(germany_2020_03_28.cases, found.cases);
    assert_eq!(germany_2020_03_28.deaths, found.deaths);
  }

  #[test]
  fn numbers_world()
  {
    let db = get_sqlite_db();

    let numbers = db.numbers_world();
    // Vector of data must not be empty.
    assert!(!numbers.is_empty());
    // There should be more than 300 entries, ...
    assert!(numbers.len() > 300);
    // Check whether a specific value is in the vector.
    // 2020-06-30|161815|3704
    let world_2020_06_30 = Numbers {
      date: String::from("2020-06-30"),
      cases: 161815,
      deaths: 3704
    };
    let found = numbers.iter().find(|&n| n.date == "2020-06-30");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(world_2020_06_30.date, found.date);
    assert_eq!(world_2020_06_30.cases, found.cases);
    assert_eq!(world_2020_06_30.deaths, found.deaths);
    // Check another value (2020-01-30|1757|38).
    let world_2020_01_30 = Numbers {
      date: String::from("2020-01-30"),
      cases: 1757,
      deaths: 38
    };
    let found = numbers.iter().find(|&n| n.date == "2020-01-30");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(world_2020_01_30.date, found.date);
    assert_eq!(world_2020_01_30.cases, found.cases);
    assert_eq!(world_2020_01_30.deaths, found.deaths);
  }

  #[test]
  fn accumulated_numbers()
  {
    let db = get_sqlite_db();

    let numbers = db.accumulated_numbers(&76);
    // Vector of data must not be empty.
    assert!(!numbers.is_empty());
    // There should be more than 300 entries, ...
    assert!(numbers.len() > 300);
    // ... but less than 600, because vector has only data from one country.
    assert!(numbers.len() < 600);
    // Check whether a specific value is in the vector.
    let germany_accumulated_2020_03_30 = Numbers {
      date: String::from("2020-03-30"),
      cases: 57298,
      deaths: 455
    };
    let found = numbers.iter().find(|&n| n.date == "2020-03-30");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_accumulated_2020_03_30.date, found.date);
    assert_eq!(germany_accumulated_2020_03_30.cases, found.cases);
    assert_eq!(germany_accumulated_2020_03_30.deaths, found.deaths);
    // Check another similar value.
    let germany_accumulated_2020_06_30 = Numbers {
      date: String::from("2020-06-30"),
      cases: 194259,
      deaths: 8973
    };
    let found = numbers.iter().find(|&n| n.date == "2020-06-30");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_accumulated_2020_06_30.date, found.date);
    assert_eq!(germany_accumulated_2020_06_30.cases, found.cases);
    assert_eq!(germany_accumulated_2020_06_30.deaths, found.deaths);
  }

  #[test]
  fn accumulated_numbers_world()
  {
    let db = get_sqlite_db();

    let numbers = db.accumulated_numbers_world();
    // Vector of data must not be empty.
    assert!(!numbers.is_empty());
    // There should be more than 300 entries, ...
    assert!(numbers.len() > 300);
    // Check whether a specific value is in the vector.
    // 2020-04-03|1038420|53448
    let world_one_million = Numbers {
      date: String::from("2020-04-03"),
      cases: 1038420,
      deaths: 53448
    };
    let found = numbers.iter().find(|&n| n.date == "2020-04-03");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(world_one_million.date, found.date);
    assert_eq!(world_one_million.cases, found.cases);
    assert_eq!(world_one_million.deaths, found.deaths);
    // Check another value (2020-09-29|33483079|1002884).
    let world_one_million_gone = Numbers {
      date: String::from("2020-09-29"),
      cases: 33483079,
      deaths: 1002884
    };
    let found = numbers.iter().find(|&n| n.date == "2020-09-29");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(world_one_million_gone.date, found.date);
    assert_eq!(world_one_million_gone.cases, found.cases);
    assert_eq!(world_one_million_gone.deaths, found.deaths);
  }

  #[test]
  fn incidence()
  {
    let db = get_sqlite_db();

    let incidences = db.incidence(&76);
    // Vector of data must not be empty.
    assert!(!incidences.is_empty());
    // There should be more than 300 entries, ...
    assert!(incidences.len() > 300);
    // ... but less than 600, because vector has only data from one country.
    assert!(incidences.len() < 600);
    // Check whether a specific value is in the vector.
    let germany_2020_10_23 = Incidence14 {
      date: String::from("2020-10-23"),
      incidence: 106.76 // 106.759624, rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2020-10-23");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_10_23.date, found.date);
    assert_eq!(germany_2020_10_23.incidence, found.incidence);
    // Check another value (2020-02-12|0.01325).
    let germany_2020_02_12 = Incidence14 {
      date: String::from("2020-02-12"),
      incidence: 0.01 // 0.01325, rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2020-02-12");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_02_12.date, found.date);
    assert_eq!(germany_2020_02_12.incidence, found.incidence);
  }
}
