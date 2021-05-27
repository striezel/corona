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

pub struct UnitedStatesOfAmerica
{
}

impl UnitedStatesOfAmerica
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> UnitedStatesOfAmerica
  {
    UnitedStatesOfAmerica { }
  }
}

impl Collect for UnitedStatesOfAmerica
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 203,
      name: "United States of America".to_string(),
      population: 329064917,
      geo_id: "US".to_string(),
      country_code: "USA".to_string(),
      continent: "America".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "US" // United States of America
  }

  // Uses the default implementation of collect(), which is to query the
  // disease.sh historical API.

  // Note: JHU numbers do not exactly match ECDC numbers, but they are close
  // enough to be good.
}
