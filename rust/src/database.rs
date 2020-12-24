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
  country_id: i32,
  name: String,
  population: i32,
  geo_id: String,
  continent: String
}

/// struct to hold the case numbers for a single day in a single country
pub struct Numbers
{
  date: String,
  cases: i32,
  deaths: i32
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
      Err(_e) => return Err(String::from("Failed to open database!")),
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
        name: row.get(1).unwrap_or(String::from("")),
        population: row.get(2).unwrap_or(-1),
        geo_id: row.get(3).unwrap_or(String::from("")),
        continent: row.get(4).unwrap_or(String::from(""))
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
    return data;
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
      Ok(String::from(row.get(0).unwrap_or(String::from(""))))
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
        .join("..").join("..") // up two directories
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
}
