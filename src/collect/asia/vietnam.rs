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

use crate::collect::Collect;
use crate::data::Country;

pub struct Vietnam
{
}

impl Vietnam
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Vietnam
  {
    Vietnam { }
  }
}

impl Collect for Vietnam
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 209,
      name: "Vietnam".to_string(),
      population: 96462108,
      geo_id: "VN".to_string(),
      country_code: "VNM".to_string(),
      continent: "Asia".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "VN" // Vietnam
  }

  // Uses the default implementation of collect(), which is to query the
  // disease.sh historical API.
}
