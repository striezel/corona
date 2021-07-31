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

use crate::collect::api::disease_sh;
use crate::collect::{Collect, Range};
use crate::data::{Country, Numbers};

pub struct CookIslands
{
}

impl CookIslands
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> CookIslands
  {
    CookIslands { }
  }
}

impl Collect for CookIslands
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 215,
      name: "Cook Islands".to_string(),
      population: 17_459,
      geo_id: "CK".to_string(),
      country_code: "COK".to_string(),
      continent: "Oceania".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "CK" // Cook Islands
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    disease_sh::request_historical_api_province("NZ", "cook%20islands", range)
  }

  /**
   * Returns the name of the country for which the data is collected as it
   * appears in the API data.
   */
  fn name_in_api(&self) -> String
  {
    String::from("New Zealand")
  }

  /**
   * Returns the name of the province for which the data is collected as it
   * appears in the API data. May be empty.
   */
  fn province_in_api(&self) -> &str
  {
    "cook islands"
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn has_data()
  {
    let data = CookIslands::new().collect(&Range::Recent);
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
