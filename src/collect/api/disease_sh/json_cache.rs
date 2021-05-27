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

use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};

/// Caches JSON data of disease.sh API for all countries.
pub struct JsonCache
{
  done: AtomicBool,
  json: Vec<Value>
}

impl JsonCache
{
  /**
   * Provides a new instance of JsonCache.
   *
   * @return Returns an empty instance of JsonCache.
   */
  pub fn new() -> JsonCache
  {
    JsonCache {
      done: AtomicBool::new(false),
      json: Vec::new()
    }
  }

  /**
   * Loads data into the cache.
   *
   * @return Returns whether data could be loaded.
   */
  pub fn load(&mut self) -> bool
  {
    if self.done.load(Ordering::Relaxed)
    {
      return false;
    }

    let the_json = crate::collect::api::disease_sh::perform_api_request(
      "https://corona.lmao.ninja/v3/covid-19/historical/?lastdays=all");
    match the_json
    {
      Ok(serde_json::Value::Array(json)) =>
      {
        self.json = json;
        self.done = AtomicBool::new(true);
        true
      },
      _ =>
      {
        self.json = Vec::new();
        self.done = AtomicBool::new(true);
        false
      }
    }
  }

  /**
   * Finds the matching JSON value for a given country and province.
   *
   * @param country   name of the country, e. g. "France"
   * @param province  name of the province, e. g. "new caledonia";
   *                  use empty string for country without provinces or complete country
   * @return Returns the matching JSON with timeline, if a match was found.
   *         Returns None, if no matching JSON was found.
   */
  pub fn find_json(&self, country: &str, province: &str) -> Option<&Value>
  {
    if !self.done.load(Ordering::Relaxed) || self.json.is_empty()
    {
      return None;
    }

    let real_province = if province.is_empty() { serde_json::Value::Null }
                               else { serde_json::Value::String(province.to_string()) };
    for elem in self.json.iter()
    {
      // Country name should match element "country".
      match elem.get("country")
      {
        Some(Value::String(country_name)) =>
        {
          if country_name != country
          {
            continue;
          }
        },
        _ => continue
      }
      // Province name should match element "province".
      match elem.get("province")
      {
        Some(value) =>
        {
          if value == &real_province
          {
            return Some(elem);
          }
          continue;
        },
        _ => continue
      }
    }

    // No match found.
    None
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn cache()
  {
    let mut cache = JsonCache::new();
    // Load should succeed.
    assert!(cache.load());
    // Next load should fail, because cache only loads once.
    assert!(!cache.load());
    // Should find data for some countries.
    let spain = cache.find_json("Spain", "");
    assert!(spain.is_some());
    let isle_of_man = cache.find_json("UK", "isle of man");
    assert!(isle_of_man.is_some());
  }
}
