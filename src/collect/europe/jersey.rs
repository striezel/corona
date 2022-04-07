/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2021, 2022  Dirk Stolle
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

use crate::collect::api::Range;
use crate::collect::{Collect, JsonCache};
use crate::data::Country;
use crate::data::{fill_missing_dates, Numbers};
use serde_json::value::Value;

pub struct Jersey
{
}

impl Jersey
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Jersey
  {
    Jersey { }
  }

  /**
   * Gets the numbers from the official JSON data.
   *
   * @return Returns a vector containing the Numbers, if successful.
   *         Returns an error message, if the operation failed.
   */
  fn official_json() -> Result<Vec<Numbers>, String>
  {
    let json = Jersey::get_official_json()?;
    let mut numbers = Jersey::parse_json(&json)?;
    Jersey::transform_vector(&mut numbers);
    fill_missing_dates(&mut numbers)?;
    Jersey::remove_oddity(&mut numbers);
    Jersey::add_old_data(&mut numbers);
    Ok(numbers)
  }

  /**
   * Retrieves the JSON data from the official Jersey government website.
   *
   * @return Returns serde_json::Value, if successful.
   *         Returns a String containing the error message otherwise.
   */
  fn get_official_json() -> Result<Value, String>
  {
    use reqwest::StatusCode;
    use std::io::Read;

    let mut res = match reqwest::blocking::get(
      "https://www.gov.je/Datasets/ListOpenData?ListName=COVID19&type=json"
    )
    {
      Ok(responded) => responded,
      Err(e) => return Err(format!("HTTP request failed: {}", e))
    };
    let mut body = String::new();
    if let Err(e) = res.read_to_string(&mut body)
    {
      return Err(format!("Failed to read CSV into string: {}", e));
    }
    if res.status() != StatusCode::OK
    {
      return Err(format!("HTTP request failed with unexpected status code: {}\n\
                        Headers:\n{:#?}\n\
                        Body:\n{}", res.status(), res.headers(), body));
    }

    let json: Value = match serde_json::from_str(&body)
    {
      Ok(v) => v,
      Err(e) => return Err(format!("Failed to deserialize JSON from www.gov.je/Datasets/ListOpenData: {}", e))
    };
    Ok(json)
  }

  /**
   * Transforms the given JSON value containing the official JSON data to numbers.
   *
   * @param json   parse JSON value from serde_json
   * @return Returns a vector containing the extracted Numbers in case of
   *         success. Returns an error message otherwise.
   */
  fn parse_json(json: &Value) -> Result<Vec<Numbers>, String>
  {
    let chart = match json.get("COVID19")
    {
      Some(value) => value,
      None => return Err("JSON does not contain element COVID19!".to_string())
    };
    let chart = match chart.as_array()
    {
      Some(vec) => vec,
      None => return Err("JSON element COVID19 is not an array!".to_string())
    };
    let date_exp = regex::RegexBuilder::new("^[0-9]{4}\\-[0-9]{2}\\-[0-9]{2}$")
      .build()
      .unwrap();
    let mut result: Vec<Numbers> = Vec::new();
    for elem in chart.iter()
    {
      let date = match elem.get("Date")
      {
        Some(value) => match value.as_str()
        {
          Some(s) => s.to_string(),
          None => return Err("Element 'Date' is not a string!".to_string())
        },
        None => return Err("Element 'Date' does not exist!".to_string())
      };
      if !date_exp.is_match(&date)
      {
        return Err(format!(
          "Date value '{}' does not match the YYYY-MM-DD format!",
          date
        ));
      }
      let cases = match elem.get("CasesDailyNewConfirmedCases")
      {
        Some(value) => match value.as_str()
        {
          Some(s) => s.to_string(),
          None => return Err("Element 'CasesDailyNewConfirmedCases' is not a string!".to_string())
        },
        None => return Err("Element 'CasesDailyNewConfirmedCases' does not exist!".to_string())
      };
      let cases = match cases.parse()
      {
        Ok(i) => i,
        Err(_) => i32::MIN
      };
      let deaths = match elem.get("MortalityTotalDeaths")
      {
        Some(value) => match value.as_str()
        {
          Some(s) => s.to_string(),
          None => return Err("Element 'MortalityTotalDeaths' is not a string!".to_string())
        },
        None => return Err("Element 'MortalityTotalDeaths' does not exist!".to_string())
      };
      let deaths = match deaths.parse()
      {
        Ok(i) => i,
        Err(_) => i32::MIN
      };
      result.push(Numbers { date, cases, deaths });
    }

    result.sort_unstable_by(|a, b| a.date.cmp(&b.date));
    Ok(result)
  }

  /**
   * Transforms a vector of Numbers retrieved from parse_json() into the real,
   * usable data by filling gaps and calculating missing data.
   *
   * @param numbers   vector of numbers, will be modified in place
   */
  fn transform_vector(numbers: &mut Vec<Numbers>)
  {
    let len = numbers.len();
    if len == 0
    {
      return;
    }
    if numbers[0].cases < 0
    {
      numbers[0].cases = 0;
    }
    if numbers[0].deaths < 0
    {
      numbers[0].deaths = 0;
    }
    // On some days (e. g. weekends) there is no testing and not numbers are
    // reported. In that case the JSON gives an empty string for those numbers.
    // Empty strings are transformed to i32::MIN by the parse method, so these
    // days can be identified easily.
    // In those cases the numbers from the previous day can be used instead.
    for idx in 0..len
    {
      if numbers[idx].cases == i32::MIN
      {
        // No new cases reported on that day.
        numbers[idx].cases = 0;
      }
      if numbers[idx].deaths == i32::MIN
      {
        numbers[idx].deaths = numbers[idx - 1].deaths;
      }
    }

    // On some other days the numbers are updated more than once and we get two
    // entries in the JSON data. In that case it has to be reduced to the later
    // one.
    let mut to_remove = Vec::new();
    for idx in 1..len
    {
      if numbers[idx].date == numbers[idx - 1].date
      {
        // Just use the maximum value of both and store it into the element
        // with the lower index, marking the higher index for removal.
        numbers[idx - 1].cases = numbers[idx].cases.max(numbers[idx - 1].cases);
        numbers[idx - 1].deaths = numbers[idx].deaths.max(numbers[idx - 1].deaths);
        to_remove.push(idx);
      }
    }
    // Remove duplicates.
    for element_index in to_remove.iter().rev()
    {
      numbers.remove(*element_index);
    }
    // Length may have changed.
    let len = numbers.len();

    // Numbers of deaths are total in the JSON, but the Numbers struct uses (or
    // expects) daily values. Therefore, those have to be transformed.
    let deaths: Vec<i32> = numbers.iter().map(|e| e.deaths).collect();
    for idx in 1..len
    {
      numbers[idx].deaths = deaths[idx] - deaths[idx - 1];
    }

    // Done.
  }

  /**
   * Removes an odd data point, where number of deaths is off.
   */
  fn remove_oddity(numbers: &mut [Numbers])
  {
    let april_8 = match numbers.iter().position(|elem| elem.date == "2021-04-08")
    {
      Some(idx) => idx,
      None => return
    };
    let april_9 = match numbers.iter().position(|elem| elem.date == "2021-04-09")
    {
      Some(idx) => idx,
      None => return
    };

    if numbers[april_8].deaths == -52 && numbers[april_9].deaths == 52
    {
      // Sum is zero anyway, so set both to zero.
      numbers[april_8].deaths = 0;
      numbers[april_9].deaths = 0;
    }
  }

  /**
   * Attempts to add older case data that may be missing in newer JSON.
   *
   * @param numbers   the case numbers for Jersey
   */
  fn add_old_data(numbers: &mut Vec<Numbers>)
  {
    let length_before = numbers.len();
    if length_before == 0
    {
      return;
    }
    // Hardcoded data form days before August 2020.
    let old_data = vec![
      Numbers { date: "2020-03-13".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-03-14".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-15".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-16".to_string(), cases: 3, deaths: 0 },
      Numbers { date: "2020-03-17".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-18".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-19".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-03-20".to_string(), cases: 3, deaths: 0 },
      Numbers { date: "2020-03-21".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-22".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-23".to_string(), cases: 3, deaths: 0 },
      Numbers { date: "2020-03-24".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-25".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-26".to_string(), cases: 30, deaths: 1 },
      Numbers { date: "2020-03-27".to_string(), cases: 9, deaths: 0 },
      Numbers { date: "2020-03-28".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-29".to_string(), cases: 0, deaths: 1 },
      Numbers { date: "2020-03-30".to_string(), cases: 29, deaths: 0 },
      Numbers { date: "2020-03-31".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-04-01".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-04-02".to_string(), cases: 14, deaths: 0 },
      Numbers { date: "2020-04-03".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-04-04".to_string(), cases: 29, deaths: 1 },
      Numbers { date: "2020-04-05".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-04-06".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-04-07".to_string(), cases: 24, deaths: 0 },
      Numbers { date: "2020-04-08".to_string(), cases: 4, deaths: 0 },
      Numbers { date: "2020-04-09".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-04-10".to_string(), cases: 36, deaths: 0 },
      Numbers { date: "2020-04-11".to_string(), cases: 18, deaths: 0 },
      Numbers { date: "2020-04-12".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-04-13".to_string(), cases: 5, deaths: 0 },
      Numbers { date: "2020-04-14".to_string(), cases: 3, deaths: 3 },
      Numbers { date: "2020-04-15".to_string(), cases: 0, deaths: 1 },
      Numbers { date: "2020-04-16".to_string(), cases: 6, deaths: 3 },
      Numbers { date: "2020-04-17".to_string(), cases: 17, deaths: 1 },
      Numbers { date: "2020-04-18".to_string(), cases: 5, deaths: 1 },
      Numbers { date: "2020-04-19".to_string(), cases: 4, deaths: 0 },
      Numbers { date: "2020-04-20".to_string(), cases: 0, deaths: 2 },
      Numbers { date: "2020-04-21".to_string(), cases: 5, deaths: 0 },
      Numbers { date: "2020-04-22".to_string(), cases: 5, deaths: 4 },
      Numbers { date: "2020-04-23".to_string(), cases: 10, deaths: 1 },
      Numbers { date: "2020-04-24".to_string(), cases: 9, deaths: 0 },
      Numbers { date: "2020-04-25".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-04-26".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-04-27".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-04-28".to_string(), cases: 1, deaths: 1 },
      Numbers { date: "2020-04-29".to_string(), cases: 2, deaths: 1 },
      Numbers { date: "2020-04-30".to_string(), cases: 0, deaths: 2 },
      Numbers { date: "2020-05-01".to_string(), cases: 0, deaths: 1 },
      Numbers { date: "2020-05-02".to_string(), cases: 5, deaths: 0 },
      Numbers { date: "2020-05-03".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-04".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-05-05".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-05-06".to_string(), cases: 0, deaths: 1 },
      Numbers { date: "2020-05-07".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-08".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-09".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-10".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-05-11".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-12".to_string(), cases: 1, deaths: 1 },
      Numbers { date: "2020-05-13".to_string(), cases: 1, deaths: 1 },
      Numbers { date: "2020-05-14".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-05-15".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-05-16".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-05-17".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-18".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-05-19".to_string(), cases: 1, deaths: 1 },
      Numbers { date: "2020-05-20".to_string(), cases: 2, deaths: 1 },
      Numbers { date: "2020-05-21".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-22".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-23".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-05-24".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-05-25".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-26".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-27".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-05-28".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-29".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-30".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-05-31".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-01".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-02".to_string(), cases: 0, deaths: 1 },
      Numbers { date: "2020-06-03".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-04".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-05".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-06-06".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-06-07".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-06-08".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-06-09".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-10".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-11".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-12".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-13".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-14".to_string(), cases: 3, deaths: 0 },
      Numbers { date: "2020-06-15".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-16".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-17".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-18".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-06-19".to_string(), cases: 0, deaths: 1 },
      Numbers { date: "2020-06-20".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-21".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-22".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-23".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-24".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-25".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-26".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-27".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-28".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-29".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-06-30".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-01".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-02".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-07-03".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-04".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-05".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-07-06".to_string(), cases: 3, deaths: 0 },
      Numbers { date: "2020-07-07".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-08".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-09".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-10".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-11".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-07-12".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-07-13".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-14".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-15".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-16".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-07-17".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-18".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-19".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-20".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-21".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-22".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-23".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-07-24".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-25".to_string(), cases: 2, deaths: 0 },
      Numbers { date: "2020-07-26".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-27".to_string(), cases: 1, deaths: 0 },
      Numbers { date: "2020-07-28".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-29".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-30".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-07-31".to_string(), cases: 0, deaths: 0 }
    ];

    // Add old data elements that do not exist in numbers.
    let oldest_existing_date = &numbers[0].date.clone();
    for elem in old_data.iter()
    {
      if oldest_existing_date > &elem.date
      {
        numbers.push(elem.clone());
      }
    }

    // Fix numbers in overlap.
    let overlap = old_data.iter().position(|x| &x.date == oldest_existing_date);
    if let Some(idx) = overlap
    {
      numbers[0].cases = old_data[idx].cases;
      numbers[0].deaths = old_data[idx].deaths;
    }

    let length_after = numbers.len();
    if length_before != length_after
    {
      // Rotating should be faster than sorting.
      numbers.rotate_left(length_before);
    }
  }
}

impl Collect for Jersey
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 103,
      name: "Jersey".to_string(),
      population: 107796,
      geo_id: "JE".to_string(),
      country_code: "JEY".to_string(),
      continent: "Europe".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "JE" // Jersey
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    let vec = Jersey::official_json();
    if vec.is_err() || range == &Range::All
    {
      return vec;
    }
    // Shorten to 30 elements, if necessary.
    let mut vec = vec.unwrap();
    if vec.len() <= 30
    {
      return Ok(vec);
    }
    Ok(vec.drain(vec.len() - 30..).collect())
  }

  /**
   * Collects new data of the specified time range, using the cache.
   * If there is no cached data, it may fallback to non-cached data collection.
   *
   * @param  range   the data range to collect
   * @param  cache   the cached JSON data
   * @return Returns a vector containing new daily numbers for cases + deaths.
   *         Returns an Err(), if no data could be retrieved.
   */
  fn collect_cached(&self, range: &Range, _cache: &JsonCache) -> Result<Vec<Numbers>, String>
  {
    // Data for Jersey is not in cache. Fall back to collect().
    self.collect(range)
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn has_data()
  {
    let data = Jersey::new().collect(&Range::All);
    assert!(data.is_ok());
    let data = data.unwrap();
    assert!(!data.is_empty());
    // Elements should be sorted by date.
    for idx in 1..data.len()
    {
      assert!(
        data[idx - 1].date < data[idx].date,
        "Date {} is not less than {}!",
        data[idx - 1].date,
        data[idx].date
      );
    }
  }

  #[test]
  fn odd_data_fixed()
  {
    let data = Jersey::new().collect(&Range::All);
    assert!(data.is_ok());
    let data = data.unwrap();

    // Deaths for 2021-04-08 are -52 (yes, negative), which cannot be true.
    let april_8 = data.iter().find(|elem| elem.date == "2021-04-08");
    assert!(april_8.is_some());
    let april_8 = april_8.unwrap();
    assert_ne!(april_8.deaths, -52);
    assert_eq!(april_8.deaths, 0);

    let april_9 = data.iter().find(|elem| elem.date == "2021-04-09");
    assert!(april_9.is_some());
    let april_9 = april_9.unwrap();
    assert_ne!(april_9.deaths, 52);
    assert_eq!(april_9.deaths, 0);
  }

  #[test]
  fn older_data_exists()
  {
    let data = Jersey::new().collect(&Range::All);
    assert!(data.is_ok());
    let data = data.unwrap();

    // Data should have elements before 30th July 2020.
    let old = data.iter().find(|elem| elem.date == "2020-03-30");
    assert!(old.is_some());

    let old_count = data
      .iter()
      .filter(|x| x.date < "2020-07-30".to_string())
      .count();
    assert!(old_count > 100);

    let mut accumulated_cases = 0;
    let mut accumulated_deaths = 0;
    data
      .iter()
      .filter(|x| x.date < "2021-05-19".to_string())
      .for_each(|x| {
        accumulated_cases += x.cases;
        accumulated_deaths += x.deaths;
      });

    // Case numbers should be approx. 3236 by the given date.
    println!("Cases: {}", accumulated_cases);
    assert!(accumulated_cases > 3200);
    assert!(accumulated_cases < 3300);
    // Approx. 69 people have died due to the virus by the given date.
    println!("Deaths: {}", accumulated_deaths);
    assert!(accumulated_deaths > 60);
    assert!(accumulated_deaths < 80);
  }
}
