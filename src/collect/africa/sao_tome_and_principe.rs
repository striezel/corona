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

use crate::collect::{Collect, JsonCache};
use crate::collect::api::{disease_sh, Range};
use crate::data::{Country, Numbers};

pub struct SaoTomeAndPrincipe
{
}

impl SaoTomeAndPrincipe
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> SaoTomeAndPrincipe
  {
    SaoTomeAndPrincipe { }
  }
}

impl Collect for SaoTomeAndPrincipe
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 167,
      name: "Sao Tome and Principe".to_string(),
      population: 215048,
      geo_id: "ST".to_string(),
      country_code: "STP".to_string(),
      continent: "Africa".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "ST" // Sao Tome and Principe
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    // disease.sh historical API seems to be off by one day, so let's fix that.
    match disease_sh::request_historical_api(self.geo_id(), &range)
    {
      Ok(vector) => Ok(disease_sh::shift_one_day_later(&vector)),
      Err(e) => Err(e)
    }
  }

  fn collect_cached(&self, range: &Range, cache: &JsonCache) -> Result<Vec<Numbers>, String>
  {
    let json = cache.find_json(&self.name_in_api(), self.province_in_api());
    match json
    {
      Some(value) =>
      {
        match disease_sh::parse_json_timeline(value)
        {
          Ok(vector) => Ok(disease_sh::shift_one_day_later(&vector)),
          Err(e) => Err(e)
        }
      },
      None =>
        {
          println!("    Info: Could not find data for {} in cache, doing extra request.",
                   self.country().name);
          self.collect(&range)
        }
    }
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn has_data()
  {
    let data = SaoTomeAndPrincipe::new().collect(&Range::Recent);
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
