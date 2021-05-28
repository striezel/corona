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

pub struct SintMaarten
{
}

impl SintMaarten
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> SintMaarten
  {
    SintMaarten { }
  }
}

impl Collect for SintMaarten
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 174,
      name: "Sint Maarten".to_string(),
      population: 42389,
      geo_id: "SX".to_string(),
      country_code: "SXM".to_string(),
      continent: "America".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "SX" // Sint Maarten
  }

  /**
   * Returns the name of the country for which the data is collected as it
   * appears in the API data.
   */
  fn name_in_api(&self) -> String {
    String::from("Netherlands")
  }

  /**
   * Returns the name of the province for which the data is collected as it
   * appears in the API data. May be empty.
   */
  fn province_in_api(&self) -> &str {
    "sint maarten"
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    disease_sh::request_historical_api_province("NL", "sint%20maarten", &range)
  }
}
