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
use crate::collect::api::Range;
use crate::data::Numbers;

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
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "LI" // Liechtenstein
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    use super::switzerland::Switzerland;
    // CSV data for Switzerland also contains data for Liechtenstein
    // (FL = "Fürstentum Liechtenstein"), so let's use that here, too.
    let vec = Switzerland::official_csv_data("FL");
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
    Ok(vec.drain(vec.len()-30..).collect())
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

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
      assert!(data[idx-1].date < data[idx].date)
    }
  }
}