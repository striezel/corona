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

use crate::collect::api::Range;
use crate::collect::Collect;
use crate::collect::JsonCache;
use crate::data::Country;
use crate::data::Numbers;

pub struct CasesOnAnInternationalConveyance
{
}

impl CasesOnAnInternationalConveyance
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> CasesOnAnInternationalConveyance
  {
    CasesOnAnInternationalConveyance { }
  }
}

impl Collect for CasesOnAnInternationalConveyance
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 38,
      name: "Cases on an international conveyance Japan".to_string(),
      population: -1,
      geo_id: "JPG11668".to_string(),
      country_code: "".to_string(),
      continent: "Other".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "JPG11668" // Cases on an international conveyance Japan
  }

  fn collect(&self, _range: &Range) -> Result<Vec<Numbers>, String>
  {
    // Data is hard-coded here, because it is short and will not change.
    Ok(vec![
      Numbers { date: "2019-12-31".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-01".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-02".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-03".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-04".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-05".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-06".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-07".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-08".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-09".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-10".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-11".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-12".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-13".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-14".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-15".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-16".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-17".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-18".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-19".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-20".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-21".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-22".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-23".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-24".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-25".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-26".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-27".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-28".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-29".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-30".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-01-31".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-01".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-02".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-03".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-04".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-05".to_string(), cases: 10, deaths: 0 },
      Numbers { date: "2020-02-06".to_string(), cases: 10, deaths: 0 },
      Numbers { date: "2020-02-07".to_string(), cases: 41, deaths: 0 },
      Numbers { date: "2020-02-08".to_string(), cases: 3, deaths: 0 },
      Numbers { date: "2020-02-09".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-10".to_string(), cases: 6, deaths: 0 },
      Numbers { date: "2020-02-11".to_string(), cases: 65, deaths: 0 },
      Numbers { date: "2020-02-12".to_string(), cases: 39, deaths: 0 },
      Numbers { date: "2020-02-13".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-14".to_string(), cases: 47, deaths: 0 },
      Numbers { date: "2020-02-15".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-16".to_string(), cases: 134, deaths: 0 },
      Numbers { date: "2020-02-17".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-18".to_string(), cases: 99, deaths: 0 },
      Numbers { date: "2020-02-19".to_string(), cases: 88, deaths: 0 },
      Numbers { date: "2020-02-20".to_string(), cases: 79, deaths: 2 },
      Numbers { date: "2020-02-21".to_string(), cases: 13, deaths: 0 },
      Numbers { date: "2020-02-22".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-23".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-24".to_string(), cases: 57, deaths: 1 },
      Numbers { date: "2020-02-25".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-26".to_string(), cases: 0, deaths: 1 },
      Numbers { date: "2020-02-27".to_string(), cases: 14, deaths: 0 },
      Numbers { date: "2020-02-28".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-02-29".to_string(), cases: 0, deaths: 2 },
      Numbers { date: "2020-03-01".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-02".to_string(), cases: 0, deaths: 0 },
      Numbers { date: "2020-03-10".to_string(), cases: -9, deaths: 1 },
    ])
  }

  fn collect_cached(&self, range: &Range, _cache: &JsonCache) -> Result<Vec<Numbers>, String>
  {
    // Use hardcoded values, that is even faster than caching.
    self.collect(range)
  }
}
