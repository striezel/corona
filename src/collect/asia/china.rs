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

use crate::collect::{Collect, JsonCache, Range};
use crate::data::{Country, Numbers};

pub struct China
{
}

impl China
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> China
  {
    China { }
  }
}

impl Collect for China
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 43,
      name: "China".to_string(),
      population: 1433783692,
      geo_id: "CN".to_string(),
      country_code: "CHN".to_string(),
      continent: "Asia".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "CN" // China
  }

  // Uses the default implementation of collect(), which is to query the
  // disease.sh historical API.

  // Note: JHU numbers seem slightly higher than ECDC numbers.

  fn collect_cached(&self, range: &Range, _cache: &JsonCache) -> Result<Vec<Numbers>, String>
  {
    // Data for complete China is not in cache. Use normal collect().
    self.collect(range)
  }
}
