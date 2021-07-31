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
use crate::collect::api::Range;
use crate::collect::Collect;
use crate::data::Country;
use crate::data::Numbers;

pub struct Aruba
{
}

impl Aruba
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Aruba
  {
    Aruba { }
  }
}

impl Collect for Aruba
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 10,
      name: "Aruba".to_string(),
      population: 106310,
      geo_id: "AW".to_string(),
      country_code: "ABW".to_string(),
      continent: "America".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "AW" // Aruba
  }

  /**
   * Returns the name of the country for which the data is collected as it
   * appears in the API data.
   */
  fn name_in_api(&self) -> String
  {
    String::from("Netherlands")
  }

  /**
   * Returns the name of the province for which the data is collected as it
   * appears in the API data. May be empty.
   */
  fn province_in_api(&self) -> &str
  {
    "aruba"
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    disease_sh::request_historical_api_province("NL", "Aruba", range)
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn has_data()
  {
    let data = Aruba::new().collect(&Range::Recent);
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
