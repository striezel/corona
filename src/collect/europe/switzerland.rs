/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2021, 2023  Dirk Stolle
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
use crate::collect::api::SwissApi;
use crate::collect::{Collect, JsonCache};
use crate::data::Country;
use crate::data::Numbers;

pub struct Switzerland
{
}

impl Switzerland
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Switzerland
  {
    Switzerland { }
  }
}

impl Collect for Switzerland
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 187,
      name: "Switzerland".to_string(),
      population: 8544527,
      geo_id: "CH".to_string(),
      country_code: "CHE".to_string(),
      continent: "Europe".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "CH" // Switzerland
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    let vec = SwissApi::official_csv_data("CH");
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
    // Data comes from Swiss CSV data, so fall back to collect().
    // TODO: Cache Swiss CSV once it is downloaded. This would allow reuse for
    //       cached collect of Liechtenstein and save us three(?) HTTP requests.
    self.collect(range)
  }
}
