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
use crate::collect::JsonCache;
use crate::data::Country;
use crate::data::Numbers;

pub struct PuertoRico
{
}

impl PuertoRico
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> PuertoRico
  {
    PuertoRico { }
  }
}

impl Collect for PuertoRico
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 158,
      name: "Puerto Rico".to_string(),
      population: 2933404,
      geo_id: "PR".to_string(),
      country_code: "PRI".to_string(),
      continent: "America".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "PR" // Puerto Rico
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    disease_sh::request_historical_api_usa_counties("puerto%20rico", &range)
  }

  fn collect_cached(&self, range: &Range, _cache: &JsonCache) -> Result<Vec<Numbers>, String>
  {
    // No caching for US provinces yet.
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
    let data = PuertoRico::new().collect(&Range::Recent);
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
