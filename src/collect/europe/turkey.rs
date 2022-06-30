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
use crate::data::Numbers;
use serde_json::Value;

pub struct Turkey
{
}

impl Turkey
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Turkey
  {
    Turkey { }
  }

  /**
   * Gets the JSON from the official website and parses it.
   *
   * @return Returns a vector of Number in case of success.
   *         Returns an error message in case of failure.
   */
  fn official_json_data() -> Result<Vec<Numbers>, String>
  {
    let json = Turkey::extract_official_json()?;
    Turkey::parse_json(&json)
  }

  /**
   * Extracts the JSON from the official website.
   *
   * @return Returns a parsed JSON value in case of success.
   *         Returns an error message in case of failure.
   */
  fn extract_official_json() -> Result<Value, String>
  {
    use reqwest::StatusCode;
    use std::io::Read;

    let mut res = match reqwest::blocking::get(
      "https://covid19.saglik.gov.tr/TR-66935/genel-koronavirus-tablosu.html"
    )
    {
      Ok(responded) => responded,
      Err(e) => return Err(format!("HTTP request failed: {}", e))
    };
    let mut body = String::new();
    if let Err(e) = res.read_to_string(&mut body)
    {
      return Err(format!("Failed to read HTML into string: {}", e));
    }

    if res.status() != StatusCode::OK
    {
      return Err(format!("HTTP request failed with unexpected status code: {}\n\
                        Headers:\n{:#?}\n\
                        Body:\n{}", res.status(), res.headers(), body));
    }
    // JSON is within a <script> block using assignment statement.
    let pattern = "var geneldurumjson = (.*);";
    // Use regular expression builder with pattern above.
    let re = regex::RegexBuilder::new(pattern)
      // Build it and unwrap it, because it does not error out with proper pattern.
      .build()
      .unwrap();
    return match re.captures(&body)
    {
      Some(cap) =>
      {
        let json: Value = match serde_json::from_str(&cap[1])
        {
          Ok(v) => v,
          Err(e) => return Err(format!("Failed to deserialize JSON from covid19.saglik.gov.tr: {}", e))
        };
        Ok(json)
      },
      None => Err("Regular expression did not match!".to_string())
    };
  }

  /**
   * Parses the JSON from the official website into numbers.
   *
   * @return Returns a vector of Numbers in case of success.
   *         Returns an error message in case of failure.
   */
  fn parse_json(json: &Value) -> Result<Vec<Numbers>, String>
  {
    let json = match json.as_array()
    {
      Some(vec) => vec,
      None => return Err("JSON is not an array!".to_string())
    };
    // Date format is something like "31.12.2020".
    let date_regex = regex::RegexBuilder::new("^([0-9]{2})\\.([0-9]{2})\\.([0-9]{4})$")
      .build()
      .unwrap();
    let mut result: Vec<Numbers> = Vec::new();
    for day in json.iter()
    {
      let date = match day.get("tarih")
      {
        Some(Value::String(s)) => s,
        _ => continue
      };
      let date: String = match date_regex.captures(date)
      {
        /* When .to_string() is removed as suggested by the clippy lint, then
           the build fails with:

           error[E0277]: the size for values of type `str` cannot be known at compilation time
           help: the trait `Sized` is not implemented for `str`

           Therefore, the lint cannot be followed.
         */
        #[allow(clippy::to_string_in_format_args)]
        Some(cap) => format!("{}-{}-{}", cap[3].to_string(), cap[2].to_string(), cap[1].to_string()),
        _ =>  continue
      };
      let cases: i32 = match day.get("gunluk_vaka")
      {
        Some(Value::String(s)) => match s.replace('.', "").parse()
        {
          Ok(i) => i,
          Err(_) => continue
        },
        _ => continue
      };
      let deaths: i32 = match day.get("gunluk_vefat")
      {
        Some(Value::String(s)) => match s.replace('.', "").parse()
        {
          Ok(i) => i,
          Err(_) => continue
        },
        _ => continue
      };
      result.push(Numbers { date, cases, deaths });
    }
    // Sort, because JSON data is in inverse order.
    result.sort_unstable_by(|a, b| a.date.cmp(&b.date));
    Ok(result)
  }
}

impl Collect for Turkey
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 196,
      name: "Turkey".to_string(),
      population: 82003882,
      geo_id: "TR".to_string(),
      country_code: "TUR".to_string(),
      continent: "Europe".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "TR" // Turkey
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    let vec = Turkey::official_json_data();
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
    // There is no meaningful way to cache the data. Fall back to collect.
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
    let data = Turkey::new().collect(&Range::Recent);
    assert!(data.is_ok());
    let data = data.unwrap();
    assert!(!data.is_empty());
    // Elements should be sorted by date.
    for idx in 1..data.len()
    {
      assert!(data[idx - 1].date < data[idx].date)
    }
  }
}
