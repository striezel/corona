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

pub struct Sweden
{
}

impl Sweden
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Sweden
  {
    Sweden { }
  }
}

impl Collect for Sweden
{
  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "SE" // Sweden
  }

  // Sweden uses the default implementation of collect(), which is to query the
  // disease.sh historical API.

  // Note: JHU numbers seem to be a bit off compared to ECDC's numbers on some
  // days.
}
