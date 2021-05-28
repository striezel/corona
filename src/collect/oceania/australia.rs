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

use crate::collect::{Collect, Range, JsonCache};
use crate::data::{Country, Numbers};

pub struct Australia
{
}

impl Australia
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Australia
  {
    Australia { }
  }
}

impl Collect for Australia
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 11,
      name: "Australia".to_string(),
      population: 25203200,
      geo_id: "AU".to_string(),
      country_code: "AUS".to_string(),
      continent: "Oceania".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "AU" // Australia
  }

  // Uses the default implementation of collect(), which is to query the
  // disease.sh historical API.


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
    // Data for complete Australia is not in cache. Fall back to collect().
    self.collect(range)
  }
}
