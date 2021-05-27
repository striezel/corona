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
use crate::collect::api::disease_sh;
use crate::collect::api::Range;
use crate::data::Numbers;

pub struct Anguilla
{
}

impl Anguilla
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Anguilla
  {
    Anguilla { }
  }
}

impl Collect for Anguilla
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 6,
      name: "Anguilla".to_string(),
      population: 14872,
      geo_id: "AI".to_string(),
      country_code: "AIA".to_string(),
      continent: "America".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "AI" // Anguilla
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    // Anguilla is a overseas territory of the UK.
    disease_sh::request_historical_api_province("UK", "Anguilla", &range)
  }
}
