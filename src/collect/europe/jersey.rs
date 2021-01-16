/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2021  Dirk Stolle
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

use crate::collect::Collect;
use crate::collect::api::Range;
use crate::data::Numbers;
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
    Ok(numbers)
  }

  // The constant value i32::MIN only exists in Rust 1.43 and later. However,
  // the minimum supported Rust version (MSRV) for this program is 1.40, so
  // this constant has to be declared manually.
  const I32_MIN: i32 = -2147483648;

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

    let mut res = match reqwest::blocking::get("https://www.gov.je/Datasets/ListOpenData?ListName=COVID19CasesChart&type=json")
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
    let chart = match json.get("COVID19CasesChart")
    {
      Some(value) => value,
      None => return Err("JSON does not contain element COVID19CasesChart!".to_string())
    };
    let chart = match chart.as_array()
    {
      Some(vec) => vec,
      None => return Err("JSON element COVID19CasesChart is not an array!".to_string())
    };
    let date_exp = regex::RegexBuilder::new("^[0-9]{4}\\-[0-9]{2}\\-[0-9]{2}$")
      .build()
      .unwrap();
    let mut result: Vec<Numbers> = Vec::new();
    for elem in chart.iter()
    {
      let date = match elem.get("Date")
      {
        Some(value) =>
        {
          match value.as_str()
          {
            Some(s) => s.to_string(),
            None => return Err("Element 'Date' is not a string!".to_string())
          }
        },
        None => return Err("Element 'Date' does not exist!".to_string())
      };
      if !date_exp.is_match(&date)
      {
        return Err(format!("Date value '{}' does not match the YYYY-MM-DD format!", date));
      }
      let cases = match elem.get("Newcasesreported")
      {
        Some(value) =>
        {
          match value.as_str()
          {
            Some(s) => s.to_string(),
            None => return Err("Element 'Newcasesreported' is not a string!".to_string())
          }
        },
        None => return Err("Element 'Newcasesreported' does not exist!".to_string())
      };
      let cases = match cases.parse()
      {
        Ok(i) => i,
        Err(_) => Jersey::I32_MIN
      };
      let deaths = match elem.get("Deaths")
      {
        Some(value) =>
        {
          match value.as_str()
          {
            Some(s) => s.to_string(),
            None => return Err("Element 'Deaths' is not a string!".to_string())
          }
        },
        None => return Err("Element 'Deaths' does not exist!".to_string())
      };
      let deaths = match deaths.parse()
      {
        Ok(i) => i,
        Err(_) => Jersey::I32_MIN
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
    // Empty strings are transformed to Jersey::I32_MIN by the parse method, so
    // these days can be identified easily.
    // In those cases the numbers from the previous day can be used instead.
    for idx in 0..len
    {
      if numbers[idx].cases == Jersey::I32_MIN
      {
        // No new cases reported on that day.
        numbers[idx].cases = 0;
      }
      if numbers[idx].deaths == Jersey::I32_MIN
      {
        numbers[idx].deaths = numbers[idx-1].deaths;
      }
    }

    // On some other days the numbers are updated more than once and we get two
    // entries in the JSON data. In that case it has to be reduced to the later
    // one.
    let mut to_remove = Vec::new();
    for idx in 1..len
    {
      if numbers[idx].date == numbers[idx-1].date
      {
        // Just use the maximum value of both and store it into the element
        // with the lower index, marking the higher index for removal.
        numbers[idx-1].cases = numbers[idx].cases.max(numbers[idx-1].cases);
        numbers[idx-1].deaths = numbers[idx].deaths.max(numbers[idx-1].deaths);
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
      numbers[idx].deaths = deaths[idx] - deaths[idx-1];
    }

    // Done.
  }
}

impl Collect for Jersey
{
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
    Ok(vec.drain(vec.len()-30..).collect())
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
      assert!(data[idx-1].date < data[idx].date);
    }
  }
}
