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

use crate::collect::Collect;
use crate::data::Country;

pub struct Liechtenstein
{
}

impl Liechtenstein
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Liechtenstein
  {
    Liechtenstein { }
  }
}

impl Collect for Liechtenstein
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 116,
      name: "Liechtenstein".to_string(),
      population: 38378,
      geo_id: "LI".to_string(),
      country_code: "LIE".to_string(),
      continent: "Europe".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "LI" // Liechtenstein
  }

  // Uses the default implementation of collect(), which is to query the
  // disease.sh historical API.
}

#[cfg(test)]
mod tests
{
  use super::*;
  use crate::collect::api::Range;

  #[test]
  fn has_data()
  {
    let data = Liechtenstein::new().collect(&Range::Recent);
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
