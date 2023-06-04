/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2020, 2021, 2022  Dirk Stolle
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

use crate::data::{
  Country, Incidence14, Incidence7, IncidenceWithDay,
  Numbers, NumbersAndIncidence, NumbersAndIncidenceAndTotals
};
use std::collections::HashMap;
use std::path::Path;

use rusqlite::{named_params, params, Connection};

pub struct Database
{
  conn: rusqlite::Connection
}

impl Database
{
  /**
   * Opens an existing SQLite database.
   *
   * @param     db_path   path of the SQLite database file to open
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
   * Creates a new SQLite database containing empty tables.
   *
   * @param db_path   path of the SQLite database file to create
   * @return Returns a Result containing the Database object, if successful.
   *         Returns a string with an error message, if the database could not
   *         be created.
   */
  pub fn create(db_path: &str) -> Result<Database, String>
  {
    let path = Path::new(db_path);
    if path.exists()
    {
      return Err(format!("The file or directory {} already exists!", db_path));
    }
    // Create database file.
    let conn = match Connection::open(db_path)
    {
      Err(_e) => return Err(String::from("Failed to create database!")),
      Ok(c) => c
    };
    // Create tables.
    let sql = "CREATE TABLE country (\n  \
               countryId INTEGER PRIMARY KEY NOT NULL,\n  \
               name TEXT NOT NULL,\n  \
               population INTEGER,\n  \
               geoId TEXT NOT NULL,\n  \
               countryCode TEXT,\n  \
               continent TEXT\n\
               );";
    if let Err(e) = conn.execute(sql, params![])
    {
      return Err(format!("Could not create table country in database. {}", e));
    }
    let sql = "CREATE TABLE covid19 (\n  \
               countryId INTEGER NOT NULL,\n  \
               date TEXT,\n  \
               cases INTEGER,\n  \
               deaths INTEGER,\n  \
               incidence14 REAL,\n  \
               incidence7 REAL\n\
               );";
    if let Err(e) = conn.execute(sql, params![])
    {
      return Err(format!("Could not create table covid19 in database. {}", e));
    }
    Ok(Database { conn })
  }

  /**
   * Lists all countries in the database.
   *
   * @return Returns a vector of country data.
   */
  pub fn countries(&self) -> Vec<Country>
  {
    let sql = "SELECT countryId, name, population, geoId, countryCode, continent FROM country \
               WHERE geoId <> '' AND continent <> 'Other' \
               ORDER BY name ASC;";
    let mut stmt = match self.conn.prepare(sql)
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
        country_code: row.get(4).unwrap_or_else(|_| String::new()),
        continent: row.get(5).unwrap_or_else(|_| String::new())
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
   * Gets the country id for given country data based on the geo id. If no match is found, a new
   * record with the given data will be added into the database and its country id will be returned.
   *
   * @param geo_id        geo id of the country (e. g. "DE" for Germany)
   * @param name          name of the country (e. g. "Germany")
   * @param population    number of inhabitants of the country
   * @param country_code  ISO-3166 ALPHA-3 country code (e.g. "DEU" for Germany)
   * @param continent     name of the continent (e. g. "Europe")
   */
  pub fn get_country_id_or_insert(&self, geo_id: &str, name: &str, population: &i64, country_code: &str, continent: &str) -> i64
  {
    let mut stmt = match self
      .conn
      .prepare("SELECT countryId FROM country WHERE geoId= ? LIMIT 1;")
    {
      Ok(statement) => statement,
      Err(_) => return -1
    };
    let rows = stmt.query_map(params![geo_id], |row| Ok(row.get(0).unwrap_or(-1i64)));
    let rows = match rows
    {
      Ok(mapped_rows) => mapped_rows,
      Err(_) => return -1
    };
    // Allow this loop here, because the non-looping alternative would result
    // in more obfuscated code.
    #[allow(clippy::never_loop)]
    for id in rows.flatten()
    {
      return id;
    }
    // The requested geo id was not found - insert new country.
    let mut stmt = match self.conn.prepare(
      "INSERT INTO country (name, population, geoId, countryCode, continent) \
       VALUES (@countryname, @pop, @geo, @code, @continent);"
    )
    {
      Ok(statement) => statement,
      Err(_) => return -1 // failed to prepare statement
    };
    if stmt.execute(named_params!{ "@countryname": &name,
                                   "@pop": population,
                                   "@geo": &geo_id,
                                   "@code": &country_code,
                                   "@continent": &continent}).is_err()
    {
      return -1;
    };

    self.conn.last_insert_rowid()
  }

  /**
   * Lists all continents in the database.
   *
   * @return Returns an array of continent names.
   */
  pub fn continents(&self) -> Vec<String>
  {
    let sql = "SELECT DISTINCT continent FROM country \
               WHERE continent <> 'Other' \
               ORDER BY continent ASC;";
    let mut stmt = match self.conn.prepare(sql)
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
   * @return Returns a vector of country data.
   */
  pub fn countries_of_continent(&self, continent: &str) -> Vec<Country>
  {
    let sql = "SELECT countryId, name, population, geoId, countryCode, continent FROM country \
               WHERE geoId <> '' AND continent = ? \
               ORDER BY name ASC;";
    let mut stmt = match self.conn.prepare(sql)
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
        country_code: row.get(4).unwrap_or_else(|_| String::new()),
        continent: row.get(5).unwrap_or_else(|_| String::new())
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
    let sql = "SELECT date, cases, deaths FROM covid19 \
               WHERE countryId = ? \
               ORDER BY date ASC;";
    let mut stmt = match self.conn.prepare(sql)
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let rows = stmt.query(params![&country_id]);
    Database::extract_numbers(rows)
  }

  /**
   * Get Covid-19 numbers and incidence values for a specific country.
   *
   * @param country_id   id of the country
   * @return Returns an array of objects containing the date, infections and deaths on that day.
   */
  pub fn numbers_with_incidence(&self, country_id: &i32) -> Vec<NumbersAndIncidence>
  {
    let sql = "SELECT date, cases, deaths, IFNULL(incidence14, -1.0), IFNULL(incidence7, -1.0) FROM covid19 \
               WHERE countryId = ? \
               ORDER BY date DESC;";
    let mut stmt = match self.conn.prepare(sql)
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let mut rows = match stmt.query(params![&country_id])
    {
      Ok(r) => r,
      Err(_) => return vec![]
    };

    const DELTA: f64 = 0.000001;
    let mut data: Vec<NumbersAndIncidence> = Vec::new();
    // potential infinite loop
    loop
    {
      let row = rows.next();
      match row
      {
        Ok(Some(row)) =>
        {
          let i14d = row.get(3).unwrap_or(-1.0f64);
          let i7d = row.get(4).unwrap_or(-1.0f64);
          data.push(NumbersAndIncidence {
            date: row.get(0).unwrap_or_else(|_e| String::from("")),
            cases: row.get(1).unwrap_or(0),
            deaths: row.get(2).unwrap_or(0),
            incidence_14d: if (i14d + 1.0).abs() < DELTA { None } else { Some(i14d) },
            incidence_7d: if (i7d + 1.0).abs() < DELTA { None } else { Some(i7d) },
          })
        },
        Ok(None) => break,
        _ => return vec![]
      }
    }

    data
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
    // potential infinite loop
    loop
    {
      let row = rows.next();
      match row
      {
        Ok(Some(row)) => data.push(Numbers {
          date: row.get(0).unwrap_or_else(|_e| String::from("")),
          cases: row.get(1).unwrap_or(0),
          deaths: row.get(2).unwrap_or(0)
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
    let sql = "SELECT date, SUM(cases), SUM(deaths) FROM covid19 \
               GROUP BY date \
               ORDER BY date ASC;";
    let mut stmt = match self.conn.prepare(sql)
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
    let sql = "SELECT date, totalCases, totalDeaths FROM covid19 \
               WHERE countryId = ? \
               ORDER BY date ASC;";
    let mut stmt = match self.conn.prepare(sql)
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
    let sql = "SELECT date, SUM(totalCases), SUM(totalDeaths) FROM covid19 \
               GROUP BY date \
               ORDER BY date ASC;";
    let mut stmt = match self.conn.prepare(sql)
    {
      Ok(x) => x,
      Err(_) => return vec![]
    };
    let rows = stmt.query(params![]);
    Database::extract_numbers(rows)
  }

  /**
   * Get the 7-day incidence values of Covid-19 for a specific country.
   *
   * @param countryId   id of the country
   * @return Returns a vector of Incidences.
   *         This may be an empty vector, if no values are known.
   */
  pub fn incidence7(&self, country_id: &i32) -> Vec<Incidence7>
  {
    let sql = "SELECT date, round(incidence7, 2) FROM covid19 \
               WHERE countryId = ? AND ABS(IFNULL(incidence7, -1.0)+1.0) > 0.000001 \
               ORDER BY date ASC;";
    let mut stmt = match self.conn.prepare(sql)
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
    let mut data: Vec<Incidence7> = Vec::new();
    // potential infinite loop
    loop
    {
      let row = rows.next();
      match row
      {
        Ok(Some(row)) => data.push(Incidence7 {
          date: row.get(0).unwrap_or_else(|_e| String::from("")),
          incidence_7d: row.get(1).unwrap_or(0.0)
        }),
        Ok(None) => break,
        _ => return vec![]
      }
    }

    data
  }

  /**
   * Get the 7-day incidence values of Covid-19 for a specific country, separated by year.
   * E. g., if there is data for three years (2020, 2021, 2022), then the map will have
   * those years as keys, and the associated values are the incidence values for that year,
   * sorted in ascending order by day of year (ranges from 1 to 366).
   *
   * @param countryId   id of the country
   * @return Returns a map of vectors of incidences.
   *         This may be an empty map, if no values are known.
   */
  pub fn incidence7_by_year(&self, country_id: &i32) -> HashMap<u16, Vec<IncidenceWithDay>>
  {
    let sql = "SELECT CAST(strftime('%Y', date) AS INTEGER), \
                            CAST(ltrim(strftime('%j', date), '0') AS INTEGER), \
                            round(incidence7, 2) FROM covid19 \
               WHERE countryId = ? AND ABS(IFNULL(incidence7, -1.0)+1.0) > 0.000001 \
               ORDER BY date ASC;";
    let mut stmt = match self.conn.prepare(sql)
    {
      Ok(x) => x,
      Err(_) => return HashMap::new()
    };
    let rows = stmt.query(params![&country_id]);
    let mut rows = match rows
    {
      Ok(r) => r,
      Err(_) => return HashMap::new()
    };

    let mut data = HashMap::new();
    let mut current_year = 0u16;
    let mut current_data = Vec::new();

    // potential infinite loop
    loop
    {
      let row = rows.next();
      match row
      {
        Ok(Some(row)) =>
        {
          let year = row.get(0).unwrap_or(0u16);
          if current_year != year
          {
            if !current_data.is_empty()
            {
              data.insert(current_year, current_data);
            }
            current_year = year;
            current_data = Vec::new();
          }
          current_data.push(IncidenceWithDay {
            day_of_year: row.get(1).unwrap_or(0u16),
            incidence: row.get(2).unwrap_or(0.0)
          });
        },
        Ok(None) => break,
        _ => return HashMap::new()
      }
    }

    if !current_data.is_empty()
    {
      data.insert(current_year, current_data);
    }

    data
  }

  /**
   * Get the 14-day incidence values of Covid-19 for a specific country.
   *
   * @param countryId   id of the country
   * @return Returns a vector of Incidences.
   *         This may be an empty vector, if no values are known.
   */
  pub fn incidence14(&self, country_id: &i32) -> Vec<Incidence14>
  {
    let sql = "SELECT date, round(incidence14, 2) FROM covid19 \
               WHERE countryId = ? AND ABS(IFNULL(incidence14, -1.0)+1.0) > 0.000001 \
               ORDER BY date ASC;";
    let mut stmt = match self.conn.prepare(sql)
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
    // potential infinite loop
    loop
    {
      let row = rows.next();
      match row
      {
        Ok(Some(row)) => data.push(Incidence14 {
          date: row.get(0).unwrap_or_else(|_e| String::from("")),
          incidence_14d: row.get(1).unwrap_or(0.0)
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
   * @param verbose  whether the progress should be printed to standard output
   * @return Returns whether the operation was successful.
   */
  pub fn calculate_total_numbers(&self, verbose: &bool) -> bool
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
    // potential infinite loop
    loop
    {
      match rows.next()
      {
        Ok(Some(row)) =>
        {
          let name = row.get(1).unwrap_or_else(|_e| String::new());
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

    if !has_total_cases && !self.calculate_total_cases(verbose)
    {
      return false;
    }
    if !has_total_deaths && !self.calculate_total_deaths(verbose)
    {
      return false;
    }

    true
  }

  /**
   * Gets the number of records in the covid19 table.
   *
   * @return Returns the number of columns in case of success.
   *         Returns an error message otherwise.
   */
  fn covid19_records(&self) -> Result<i64, String>
  {
    let mut stmt = match self.conn.prepare("SELECT COUNT(*) FROM covid19;")
    {
      Ok(x) => x,
      Err(_) => return Err(String::from("Failed to prepare database statement!"))
    };
    let iter = stmt.query_map(params![], |row| {
      Ok(row.get(0).unwrap_or(-1))
    });
    let mut iter = match iter
    {
      Ok(iter) => iter,
      Err(_) => return Err(String::from("Failed to retrieve result of database query!"))
    };
    return match iter.next()
    {
      Some(n) => Ok(n.unwrap()),
      None => Err(String::from("Failed to retrieve result of database query!"))
    }
  }

  /**
   * Creates the column totalCases and calculates all required values for it.
   * This may take quite a while.
   *
   * @param verbose  whether the progress should be printed to standard output
   * @return Returns whether the operation was successful.
   */
  fn calculate_total_cases(&self, verbose: &bool) -> bool
  {
    // add new column
    match self.conn.execute(
      "ALTER TABLE covid19 ADD COLUMN totalCases INTEGER;",
      params![]
    )
    {
      Ok(_) => if *verbose
      {
        println!("Info: Added column totalCases to table.");
      },
      Err(e) => {
        eprintln!("Could not add column totalCases to table covid19! {}", e);
        return false;
      }
    };
    // Some older SQLite versions do not work with the UPDATE statement below,
    // if the table is empty, so check for that and leave early, if necessary.
    match self.covid19_records()
    {
      Ok(0) => return true,
      Ok(i64::MIN..=-1_i64) => {
        eprintln!("Failed to get rows of table covid19!");
        return false;
      },
      Ok(1_i64..) => {
        // Do not return here, rows need to be updated.
      },
      Err(e) => {
        eprintln!("Failed to get rows of table covid19! {}", e);
        return false;
      }
    }
    // perform actual calculation
    if *verbose
    {
      println!("Calculating accumulated number of cases for each day and \
                country. This may take a while...");
    }
    match self.conn.execute(
      "UPDATE covid19 AS c1 \
       SET totalCases=(SELECT SUM(cases) FROM covid19 AS c2 \
       WHERE c2.countryId = c1.countryId AND c2.date <= c1.date);",
      params![]
    )
    {
      Ok(affected) => if *verbose
      {
        println!("{} rows have been updated.", affected);
      },
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
   * @param verbose  whether the progress should be printed to standard output
   * @return Returns whether the operation was successful.
   */
  fn calculate_total_deaths(&self, verbose: &bool) -> bool
  {
    // add new column
    match self.conn.execute(
      "ALTER TABLE covid19 ADD COLUMN totalDeaths INTEGER;",
      params![]
    )
    {
      Ok(_) => if *verbose
      {
        println!("Info: Added column totalDeaths to table.");
      },
      Err(e)=> {
        eprintln!("Could not add column totalDeaths to table covid19! {}", e);
        return false;
      }
    };
    // Some older SQLite versions do not work with the UPDATE statement below,
    // if the table is empty, so check for that and leave early, if necessary.
    match self.covid19_records()
    {
      Ok(0) => return true,
      Ok(i64::MIN..=-1_i64) => {
        eprintln!("Failed to get rows of table covid19!");
        return false;
      },
      Ok(1_i64..) => {
        // Do not return here, rows need to be updated.
      },
      Err(e) => {
        eprintln!("Failed to get rows of table covid19! {}", e);
        return false;
      }
    }
    // Update may take ca. two minutes.
    if *verbose
    {
      println!("Calculating accumulated number of deaths for each day and country. \
                This may take a while...");
    }
    match self.conn.execute(
      "UPDATE covid19 AS c1 \
       SET totalDeaths=(SELECT SUM(deaths) FROM covid19 AS c2 \
       WHERE c2.countryId = c1.countryId AND c2.date <= c1.date);",
      params![]
    )
    {
      Ok(affected) => if *verbose
      {
        println!("{} rows have been updated.", affected);
      },
      Err(e)=> {
        eprintln!("Could not update totalDeaths in table covid19! {}", e);
        return false;
      }
    }

    true
  }

  /**
   * Executes a batch SQL statement.
   *
   * @param sql   the SQL statement(s) to execute
   * @return  Returns whether the statements were executed successfully.
   */
  pub fn batch(&self, sql: &str) -> bool
  {
    let res = self.conn.execute_batch(sql);
    if res.is_err()
    {
      eprintln!("Error: Batch statement failed! {}", res.unwrap_err());
      eprintln!("Statement is:\n{}\n", sql);
      return false;
    }
    true
  }

  /**
   * Inserts data for a given country id into the database.
   *
   * @param country_id   id of an existing country in the database
   * @param data         slice of data to insert into the database
   * @return Returns whether the operation was successful.
   */
  pub fn insert_data(&self, country_id: &i64, data: &[NumbersAndIncidenceAndTotals]) -> bool
  {
    if data.is_empty()
    {
      // No data means there is nothing to do here.
      return true;
    }
    if country_id <= &0
    {
      eprintln!("Error: Country id must be a positive number!");
      return false;
    }

    // Build insert statement.
    let mut batch = String::from(
      "INSERT INTO covid19 (countryId, date, cases, deaths, incidence14, \
     incidence7, totalCases, totalDeaths) VALUES "
    );
    // Reserve 60 bytes for every data record to avoid frequent reallocation.
    batch.reserve(60 * data.len());
    let country_id = country_id.to_string();
    for elem in data.iter()
    {
      batch.push('(');
      batch.push_str(&country_id);
      batch.push_str(", ");
      batch.push_str(&Database::quote(&elem.date));
      batch.push_str(", ");
      batch.push_str(&elem.cases.to_string());
      batch.push_str(", ");
      batch.push_str(&elem.deaths.to_string());
      batch.push_str(", ");
      match elem.incidence_14d
      {
        Some(float) => batch.push_str(&float.to_string()),
        None => batch.push_str("NULL")
      }
      batch.push_str(", ");
      match elem.incidence_7d
      {
        Some(float) => batch.push_str(&float.to_string()),
        None => batch.push_str("NULL")
      }
      batch.push_str(", ");
      batch.push_str(&elem.total_cases.to_string());
      batch.push_str(", ");
      batch.push_str(&elem.total_deaths.to_string());
      batch.push_str("),");
    }

    // replace last ',' with ';' to make it valid SQL syntax
    batch.truncate(batch.len() - 1);
    batch.push(';');

    // Insert all data in one go.
    self.batch(&batch)
  }

  /**
   * Quotes an ASCII string for use in an SQLite statement.
   *
   * @param s   the string that shall be quoted.
   * @return  Returns the quoted string.
   */
  pub fn quote(s: &str) -> String
  {
    let mut result = String::from("'");
    for c in s.chars()
    {
      if c != '\''
      {
        result.push(c);
      }
      else
      {
        result.push_str("''");
      }
    }
    result.push('\'');
    result
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  /**
   * Gets a database instance connected to the old ECDC-based corona.db file
   * in the data directory.
   *
   * @remarks The records in that SQLite database are stale, apart from possible
   *          future schema updates, so they can be considered "constant" data.
   * @return Returns an open database.
   */
  fn get_sqlite_db_ecdc() -> Database
  {
    let db_path = Path::new(file!()) // current file: src/database.rs
      .parent()                      // parent: src/
      .unwrap()                      // safe to unwrap, because directory exists
      .join("..")                    // up one directory
      .join("data")                  // into directory data/
      .join("corona-ecdc-2020-12-14.db"); // and to the corona.db file;
    let db = Database::new(db_path.to_str().unwrap());
    assert!(db.is_ok());
    return db.unwrap();
  }

  /**
   * Gets a database instance connected to the old corona.db file which still
   * used the RKI data for Germany in the data directory.
   *
   * @remarks The records in that SQLite database are stale, apart from possible
   *          future schema updates, so they can be considered "constant" data.
   * @return Returns an open database.
   */
  fn get_sqlite_db_rki() -> Database
  {
    let db_path = Path::new(file!()) // current file: src/database.rs
      .parent()                      // parent: src/
      .unwrap()                      // safe to unwrap, because directory exists
      .join("..")                    // up one directory
      .join("data")                  // into directory data/
      .join("corona-2021-11-23.db"); // and to the corona.db file;
    let db = Database::new(db_path.to_str().unwrap());
    assert!(db.is_ok());
    return db.unwrap();
  }

  /**
   * Gets a database instance connected to the old JHU corona.db file in data directory.
   *
   * @remarks The records in that SQLite database are stale, apart from possible
   *          future schema updates, so they can be considered "constant" data.
   * @return Returns an open database.
   */
  fn get_sqlite_db_jhu() -> Database
  {
    let db_path = Path::new(file!()) // current file: src/database.rs
      .parent()                      // parent: src/
      .unwrap()                      // safe to unwrap, because directory exists
      .join("..")                    // up one directory
      .join("data")                  // into directory data/
      .join("corona-jhu-2023-03-09.db"); // and to the *.db file;
    let db = Database::new(db_path.to_str().unwrap());
    assert!(db.is_ok());
    return db.unwrap();
  }

  #[test]
  fn create_db()
  {
    let path = std::env::temp_dir().join("database_creation_test.db");
    // scope for db
    {
      let created = Database::create(path.to_str().unwrap());
      // Creation should be successful and file should exist.
      assert!(created.is_ok());
      assert!(path.exists());
      assert!(path.is_file());
    }
    // clean up
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn create_db_on_existing_path()
  {
    let path = std::env::temp_dir().join("database_creation_test_existing.db");
    // scope for db
    {
      let created = Database::create(path.to_str().unwrap());
      // Creation should be successful and file should exist.
      assert!(created.is_ok());
      assert!(path.exists());
      // Second creation attempt at same path should fail!
      let created_again = Database::create(path.to_str().unwrap());
      assert!(created_again.is_err());
    }
    // clean up
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn continents()
  {
    let db = get_sqlite_db_ecdc();

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
    let db = get_sqlite_db_ecdc();

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
      country_code: String::from("DEU"),
      continent: String::from("Europe")
    };
    let found = countries.iter().find(|&c| c.name == "Germany");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany.country_id, found.country_id);
    assert_eq!(germany.name, found.name);
    assert_eq!(germany.population, found.population);
    assert_eq!(germany.geo_id, found.geo_id);
    assert_eq!(germany.country_code, found.country_code);
    assert_eq!(germany.continent, found.continent);
  }

  #[test]
  fn countries_of_continent()
  {
    let db = get_sqlite_db_ecdc();

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
      country_code: String::from("DEU"),
      continent: String::from("Europe")
    };
    let found = countries.iter().find(|&c| c.name == "Germany");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany.country_id, found.country_id);
    assert_eq!(germany.name, found.name);
    assert_eq!(germany.population, found.population);
    assert_eq!(germany.geo_id, found.geo_id);
    assert_eq!(germany.country_code, found.country_code);
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
      country_code: String::from("CHN"),
      continent: String::from("Asia")
    };
    let found = countries.iter().find(|&c| c.name == "China");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(china.country_id, found.country_id);
    assert_eq!(china.name, found.name);
    assert_eq!(china.population, found.population);
    assert_eq!(china.geo_id, found.geo_id);
    assert_eq!(china.country_code, found.country_code);
    assert_eq!(china.continent, found.continent);
    // Check that some other country is not found.
    let not_found = countries.iter().find(|&c| c.name == "Germany");
    assert!(not_found.is_none());
  }

  #[test]
  fn get_country_id_or_insert()
  {
    let path = std::env::temp_dir().join("get_country_id_test_simple.db");
    // scope for db
    {
      let db = Database::create(&path.to_str().unwrap()).unwrap();

      // geo_id: &str, name: &str, population: &i64, country_code: &str, continent
      let id = db.get_country_id_or_insert("XX", "Wonderland", &421337, "WON", "Utopia");
      // Id -1 means an error occurred.
      assert!(id != -1);
      // First country usually gets id one.
      assert_eq!(1i64, id);
      // Country list should now contain the country.
      let countries = db.countries();
      let wonderland = Country {
        country_id: 1,
        geo_id: String::from("XX"),
        name: String::from("Wonderland"),
        population: 421337,
        country_code: String::from("WON"),
        continent: String::from("Utopia")
      };
      let found = countries.iter().find(|&c| c.name == "Wonderland");
      assert!(found.is_some());
      let found = found.unwrap();
      assert_eq!(wonderland.country_id, found.country_id);
      assert_eq!(wonderland.name, found.name);
      assert_eq!(wonderland.population, found.population);
      assert_eq!(wonderland.geo_id, found.geo_id);
      assert_eq!(wonderland.country_code, found.country_code);
      assert_eq!(wonderland.continent, found.continent);
    }
    // clean up
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn get_country_id_or_insert_twice()
  {
    let path = std::env::temp_dir().join("get_country_id_test_twice_inserted.db");
    // scope for db
    {
      let db = Database::create(&path.to_str().unwrap()).unwrap();

      // geo_id: &str, name: &str, population: &i64, country_code: &str, continent
      let first_id = db.get_country_id_or_insert("XX", "Wonderland", &421337, "WON", "Utopia");
      // Id -1 means an error occurred.
      assert!(first_id != -1);
      // Inserting the same country again should return the same id.
      let second_id = db.get_country_id_or_insert("XX", "Wonderland", &421337, "WON", "Utopia");
      assert!(second_id != -1);
      assert_eq!(first_id, second_id);
      // But inserting another country should not return the same id.
      let third_id = db.get_country_id_or_insert("ZZ", "Neuland", &42, "TBL", "Internet");
      assert!(third_id != -1);
      assert!(first_id != third_id);
    }
    // clean up
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn numbers()
  {
    let db = get_sqlite_db_ecdc();

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
  fn numbers_with_incidence()
  {
    let db = get_sqlite_db_ecdc();

    let numbers = db.numbers_with_incidence(&76);
    // Vector of data must not be empty.
    assert!(!numbers.is_empty());
    // There should be more than 300 entries, ...
    assert!(numbers.len() > 300);
    // ... but less than 600, because vector has only data from one country.
    assert!(numbers.len() < 600);
    // Check whether a specific value is in the vector.
    let germany_2020_12_10 = NumbersAndIncidence {
      date: String::from("2020-12-10"),
      cases: 23679,
      deaths: 440,
      incidence_14d: Some(311.5122279),
      incidence_7d: None
    };
    let found = numbers.iter().find(|&n| n.date == "2020-12-10");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_12_10.date, found.date);
    assert_eq!(germany_2020_12_10.cases, found.cases);
    assert_eq!(germany_2020_12_10.deaths, found.deaths);
    assert!(found.incidence_14d.is_some());
    assert_eq!(germany_2020_12_10.incidence_14d, found.incidence_14d);
    assert_eq!(germany_2020_12_10.incidence_7d, found.incidence_7d);
    // Check another value, but without incidence data.
    let germany_2020_01_01 = NumbersAndIncidence {
      date: String::from("2020-01-01"),
      cases: 0,
      deaths: 0,
      incidence_14d: None,
      incidence_7d: None
    };
    let found = numbers.iter().find(|&n| n.date == "2020-01-01");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_01_01.date, found.date);
    assert_eq!(germany_2020_01_01.cases, found.cases);
    assert_eq!(germany_2020_01_01.deaths, found.deaths);
    assert!(found.incidence_14d.is_none());
    assert_eq!(germany_2020_01_01.incidence_14d, found.incidence_14d);
    assert!(found.incidence_7d.is_none());
    assert_eq!(germany_2020_01_01.incidence_7d, found.incidence_7d);
  }

  #[test]
  fn numbers_world()
  {
    let db = get_sqlite_db_ecdc();

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
    let db = get_sqlite_db_ecdc();

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
    let db = get_sqlite_db_ecdc();

    let numbers = db.accumulated_numbers_world();
    // Vector of data must not be empty.
    assert!(!numbers.is_empty());
    // There should be more than 300 entries, ...
    assert!(numbers.len() > 300);
    // Check whether a specific value is in the vector.
    // 2020-04-03|1038420|53447
    let world_one_million = Numbers {
      date: String::from("2020-04-03"),
      cases: 1038420,
      deaths: 53447
    };
    let found = numbers.iter().find(|&n| n.date == "2020-04-03");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(world_one_million.date, found.date);
    assert_eq!(world_one_million.cases, found.cases);
    assert_eq!(world_one_million.deaths, found.deaths);
    // Check another value (2020-09-29|33483079|100283).
    let world_one_million_gone = Numbers {
      date: String::from("2020-09-29"),
      cases: 33483079,
      deaths: 1002883
    };
    let found = numbers.iter().find(|&n| n.date == "2020-09-29");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(world_one_million_gone.date, found.date);
    assert_eq!(world_one_million_gone.cases, found.cases);
    assert_eq!(world_one_million_gone.deaths, found.deaths);
  }

  #[test]
  fn incidence14()
  {
    let db = get_sqlite_db_ecdc();

    let incidences = db.incidence14(&76);
    // Vector of data must not be empty.
    assert!(!incidences.is_empty());
    // There should be more than 300 entries, ...
    assert!(incidences.len() > 300);
    // ... but less than 600, because vector has only data from one country.
    assert!(incidences.len() < 600);
    // Check whether a specific value is in the vector.
    let germany_2020_10_23 = Incidence14 {
      date: String::from("2020-10-23"),
      incidence_14d: 106.76 // 106.759624, rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2020-10-23");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_10_23.date, found.date);
    assert_eq!(germany_2020_10_23.incidence_14d, found.incidence_14d);
    // Check another value (2020-02-12|0.01325).
    let germany_2020_02_12 = Incidence14 {
      date: String::from("2020-02-12"),
      incidence_14d: 0.01 // 0.01325, rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2020-02-12");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_02_12.date, found.date);
    assert_eq!(germany_2020_02_12.incidence_14d, found.incidence_14d);
  }

  #[test]
  fn incidence14_negative_luxembourg()
  {
    let db = get_sqlite_db_ecdc();

    let incidences = db.incidence14(&118);
    // Vector of data must not be empty.
    assert!(!incidences.is_empty());
    // There should be more than 300 entries, ...
    assert!(incidences.len() > 300);
    // ... but less than 600, because vector has only data from one country.
    assert!(incidences.len() < 600);
    // Check whether a specific value is in the vector.
    // 118|2020-08-28|-1385|0|-134.38802138|6543|124
    let luxembourg_2020_08_28 = Incidence14 {
      date: String::from("2020-08-28"),
      incidence_14d: -134.39 // -134.38802138, rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2020-08-28");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(luxembourg_2020_08_28.date, found.date);
    assert_eq!(luxembourg_2020_08_28.incidence_14d, found.incidence_14d);
    // Check another value (2020-09-09|-140.74090967).
    let luxembourg_2020_09_09 = Incidence14 {
      date: String::from("2020-09-09"),
      incidence_14d: -140.74 // -140.74090967, rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2020-09-09");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(luxembourg_2020_09_09.date, found.date);
    assert_eq!(luxembourg_2020_09_09.incidence_14d, found.incidence_14d);
  }

  #[test]
  fn incidence7()
  {
    let db = get_sqlite_db_rki();

    // Country id 77 is Germany in the current DB.
    let incidences = db.incidence7(&77);
    // Vector of data must not be empty.
    assert!(!incidences.is_empty());
    // There should be more than 300 entries, ...
    assert!(incidences.len() > 300);
    // ... but less than 6000, because vector has only data from one country.
    assert!(incidences.len() < 6000);
    // Check whether a specific value is in the vector.
    let germany_2020_10_23 = Incidence7 {
      date: String::from("2020-10-23"),
      incidence_7d: 65.93 // 65.92931686789, rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2020-10-23");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_10_23.date, found.date);
    assert_eq!(germany_2020_10_23.incidence_7d, found.incidence_7d);
    // Check another value (2021-02-12|66.4713600693854).
    let germany_2021_02_12 = Incidence7 {
      date: String::from("2021-02-12"),
      incidence_7d: 66.47 // rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2021-02-12");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2021_02_12.date, found.date);
    assert_eq!(germany_2021_02_12.incidence_7d, found.incidence_7d);
  }

  #[test]
  fn incidence7_negative_luxembourg()
  {
    let db = get_sqlite_db_jhu();

    // Country id 121 is Luxembourg in the JHU DB.
    let incidences = db.incidence7(&121);
    // Vector of data must not be empty.
    assert!(!incidences.is_empty());
    // There should be more than 300 entries, ...
    assert!(incidences.len() > 300);
    // ... but less than 6000, because vector has only data from one country.
    assert!(incidences.len() < 6000);
    // Check whether a specific value is in the vector.
    // 120|2020-08-28|-1348|0|-134.388021384799|-183.093498226078|6580|124
    let luxembourg_2020_08_28 = Incidence7 {
      date: String::from("2020-08-28"),
      incidence_7d: -183.09 // rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2020-08-28");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(luxembourg_2020_08_28.date, found.date);
    assert_eq!(luxembourg_2020_08_28.incidence_7d, found.incidence_7d);
    // Check another value (2020-09-03|-181.95323622645|6811|124).
    let luxembourg_2020_09_03 = Incidence7 {
      date: String::from("2020-09-03"),
      incidence_7d: -181.95 // rounded to two decimals after the point
    };
    let found = incidences.iter().find(|&i| i.date == "2020-09-03");
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(luxembourg_2020_09_03.date, found.date);
    assert_eq!(luxembourg_2020_09_03.incidence_7d, found.incidence_7d);
  }

  #[test]
  fn incidence7_by_year()
  {
    let db = get_sqlite_db_rki();

    // Country id 77 is Germany in the current DB.
    let incidences = db.incidence7_by_year(&77);
    // Vector of data must not be empty.
    assert!(!incidences.is_empty());
    // Data for 2020 and 2021 should exist.
    println!("keys: {:?}", incidences.keys());
    assert!(incidences.contains_key(&2020));
    assert!(incidences.contains_key(&2021));
    // There should be more than 300 entries, ...
    assert!(incidences[&2020].len() > 300);
    assert!(incidences[&2021].len() > 300);
    // ... but less than 367, because a year can have only up to 366 days.
    assert!(incidences[&2020].len() < 367);
    assert!(incidences[&2021].len() < 367);
    // Check whether a specific value is in the vector.
    let germany_2020_10_23 = IncidenceWithDay {
      day_of_year: 297, // 2020-10-23
      incidence: 65.93 // 65.92931686789, rounded to two decimals after the point
    };
    let found = incidences[&2020].iter().find(|&i| i.day_of_year == 297);
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2020_10_23.day_of_year, found.day_of_year);
    assert_eq!(germany_2020_10_23.incidence, found.incidence);
    // Check another value (2021-02-12|66.4713600693854).
    let germany_2021_02_12 = IncidenceWithDay {
      day_of_year: 43, // 2021-02-12
      incidence: 66.47 // rounded to two decimals after the point
    };
    let found = incidences[&2021].iter().find(|&i| i.day_of_year == 43);
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(germany_2021_02_12.day_of_year, found.day_of_year);
    assert_eq!(germany_2021_02_12.incidence, found.incidence);
  }

  #[test]
  fn calculate_total_numbers_no_operation()
  {
    let db = get_sqlite_db_ecdc();

    // This is a no-op on the existing database, because it already has the
    // columns with the total numbers. However, this test checks that the
    // function works (i. e. returns true) in that case anyway.
    assert!(db.calculate_total_numbers(&true));
  }

  #[test]
  fn calculate_total_numbers_empty()
  {
    let path = std::env::temp_dir().join("test_calculate_totals_empty.db");
    // scope for db
    {
      let db = Database::create(&path.to_str().unwrap()).unwrap();
      // Update of structure should succeed.
      assert!(db.calculate_total_numbers(&true));
    }
    // clean up
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn calculate_total_numbers()
  {
    let path = std::env::temp_dir().join("test_calculate_totals.db");
    // scope for db
    {
      let db = Database::create(&path.to_str().unwrap()).unwrap();

      // Insert some test data.
      let sql = "INSERT INTO country (\
          countryId, name, population, geoId, countryCode, continent) VALUES \
          (1, 'Wonderland', 42, 'XX', 'WON', 'Utopia'),\
          (2, 'Neuland', 1337, 'ZZ', 'TBL', 'Internet');";
      assert!(db.batch(&sql));
      let sql = "INSERT INTO covid19 (\
          countryId, date, cases, deaths, incidence14, incidence7) VALUES \
          (1, '2020-10-01', 123, 1, 23.45, 12.3),\
          (1, '2020-10-02', 234, 2, 34.56, 17.3),\
          (2, '2020-10-01', 3, 0, 11.22, 5.4),\
          (2, '2020-10-02', 5, 1, 22.33, 12.3);";
      assert!(db.batch(&sql));

      // Update of structure should succeed.
      assert!(db.calculate_total_numbers(&true));

      // Check accumulated numbers of 1st country.
      let accumulated = db.accumulated_numbers(&1);
      assert_eq!(2, accumulated.len());
      assert_eq!(123, accumulated[0].cases);
      assert_eq!(1, accumulated[0].deaths);
      assert_eq!(357, accumulated[1].cases);
      assert_eq!(3, accumulated[1].deaths);
      // Check accumulated numbers of 2nd country.
      let accumulated = db.accumulated_numbers(&2);
      assert_eq!(2, accumulated.len());
      assert_eq!(3, accumulated[0].cases);
      assert_eq!(0, accumulated[0].deaths);
      assert_eq!(8, accumulated[1].cases);
      assert_eq!(1, accumulated[1].deaths);
    }
    // clean up
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn quote()
  {
    assert_eq!(Database::quote(""), "''");
    assert_eq!(Database::quote("'"), "''''");
    assert_eq!(Database::quote("foobar"), "'foobar'");
    assert_eq!(Database::quote("foo'bar"), "'foo''bar'");
    assert_eq!(Database::quote("''"), "''''''");
  }

  #[test]
  fn batch()
  {
    let path = std::env::temp_dir().join("test_batch_insert.db");
    // scope for db
    {
      let db = Database::create(&path.to_str().unwrap()).unwrap();

      let sql = "INSERT INTO country (\
          countryId, name, population, geoId, countryCode, continent) VALUES \
          (1, 'Wonderland', 42, 'XX', 'WON', 'Utopia'),\
          (2, 'Neuland', 1337, 'ZZ', 'TBL', 'Internet');";
      // Batch statement should execute successfully.
      assert!(db.batch(&sql));
      // Countries should exist.
      let countries = db.countries();
      let wonderland = Country {
        country_id: 1,
        geo_id: String::from("XX"),
        name: String::from("Wonderland"),
        population: 42,
        country_code: String::from("WON"),
        continent: String::from("Utopia")
      };
      let found = countries.iter().find(|&c| c.name == "Wonderland");
      assert!(found.is_some());
      let found = found.unwrap();
      assert_eq!(wonderland.country_id, found.country_id);
      assert_eq!(wonderland.name, found.name);
      assert_eq!(wonderland.population, found.population);
      assert_eq!(wonderland.geo_id, found.geo_id);
      assert_eq!(wonderland.country_code, found.country_code);
      assert_eq!(wonderland.continent, found.continent);
      let neu_land = Country {
        country_id: 2,
        geo_id: String::from("ZZ"),
        name: String::from("Neuland"),
        population: 1337,
        country_code: String::from("TBL"),
        continent: String::from("Internet")
      };
      let found = countries.iter().find(|&c| c.name == "Neuland");
      assert!(found.is_some());
      let found = found.unwrap();
      assert_eq!(neu_land.country_id, found.country_id);
      assert_eq!(neu_land.name, found.name);
      assert_eq!(neu_land.population, found.population);
      assert_eq!(neu_land.geo_id, found.geo_id);
      assert_eq!(neu_land.country_code, found.country_code);
      assert_eq!(neu_land.continent, found.continent);
    }
    // clean up
    assert!(std::fs::remove_file(path).is_ok());
  }

  #[test]
  fn insert_data()
  {
    let path = std::env::temp_dir().join("insert_data_test.db");
    // scope for db
    {
      let db = Database::create(path.to_str().unwrap());
      // Creation should be successful and file should exist.
      assert!(db.is_ok());
      let db = db.unwrap();
      assert!(db.calculate_total_numbers(&false));
      // Insert some country.
      let id = db.get_country_id_or_insert("XX", "Wonderland", &421337, "WON", "Utopia");
      // Id -1 means an error occurred.
      assert!(id != -1);
      let data = vec![
        NumbersAndIncidenceAndTotals {
          date: "2020-10-01".to_string(),
          cases: 12345,
          deaths: 543,
          incidence_14d: None,
          incidence_7d: None,
          total_cases: 12345,
          total_deaths: 543
        },
        NumbersAndIncidenceAndTotals {
          date: "2020-10-02".to_string(),
          cases: 54321,
          deaths: 1234,
          incidence_14d: Some(234.5),
          incidence_7d: Some(112.3),
          total_cases: 66666,
          total_deaths: 1777
        },
      ];
      // Insert should succeed.
      assert!(db.insert_data(&id, &data));
      // Inserted data should exist.
      let id = id as i32;
      let numbers = db.numbers(&id);
      assert_eq!(2, numbers.len());
      assert_eq!("2020-10-01", numbers[0].date);
      assert_eq!(12345, numbers[0].cases);
      assert_eq!(543, numbers[0].deaths);
      assert_eq!("2020-10-02", numbers[1].date);
      assert_eq!(54321, numbers[1].cases);
      assert_eq!(1234, numbers[1].deaths);
      // Incidence (14-day) should exist - but only for one value.
      let incidence = db.incidence14(&id);
      assert_eq!(1, incidence.len());
      assert_eq!("2020-10-02", incidence[0].date);
      assert_eq!(234.5, incidence[0].incidence_14d);
      // Incidence (7-day) should exist - but only for one value.
      let incidence = db.incidence7(&id);
      assert_eq!(1, incidence.len());
      assert_eq!("2020-10-02", incidence[0].date);
      assert_eq!(112.3, incidence[0].incidence_7d);
      // Check total numbers.
      let numbers = db.accumulated_numbers(&id);
      assert_eq!(2, numbers.len());
      assert_eq!("2020-10-01", numbers[0].date);
      assert_eq!(12345, numbers[0].cases);
      assert_eq!(543, numbers[0].deaths);
      assert_eq!("2020-10-02", numbers[1].date);
      assert_eq!(66666, numbers[1].cases);
      assert_eq!(1777, numbers[1].deaths);
    }
    // clean up
    assert!(std::fs::remove_file(path).is_ok());
  }
}
