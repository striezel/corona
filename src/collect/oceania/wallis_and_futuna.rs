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
use crate::collect::api::disease_sh;
use crate::collect::api::Range;
use crate::data::Numbers;

pub struct WallisAndFutuna
{
}

impl WallisAndFutuna
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> WallisAndFutuna
  {
    WallisAndFutuna { }
  }
}

impl Collect for WallisAndFutuna
{
  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "WF" // Wallis and Futuna
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    disease_sh::request_historical_api_province("FR", "wallis%20and%20futuna", &range)

    // Note: Numbers seem to be a bit off / start later than ECDC numbers.
  }
}