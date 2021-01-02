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

mod europe;

use crate::data::Numbers;
use europe::Spain;

/// common trait / interface for collecting new data
trait Collect
{
  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str;

  /**
   * Collects new data of an unspecified time range.
   *
   * @return Returns a vector containing new daily numbers for cases + deaths.
   *         Returns an Err(), if no data could be retrieved.
   */
  fn collect(&self) -> Result<Vec<Numbers>, String>;
}

pub struct Collector
{
  elements: Vec<Spain>
}

impl Collector
{
  /**
   * Creates a new Collector for all implemented countries.
   */
  pub fn new() -> Collector
  {
    Collector{ elements: vec![Spain::new()] }
  }

  pub fn run(&self) -> bool
  {
    let mut success = true;
    println!("Collecting data for {} {} ...", self.elements.len(),
             if self.elements.len() != 1 { "countries" } else { "country "}
    );
    for country in self.elements.iter()
    {
      println!("Processing {} ...", &country.geo_id());
      let data = country.collect();
      match data
      {
        Ok(vector) => {
          for num in vector.iter()
          {
            println!("{}: infections = {}, deaths = {}", &num.date, &num.cases,
                     &num.deaths);
          }
        },
        Err(error) => {
          eprintln!("Error while collecting data for {}: {}", &country.geo_id(),
                    error);
          success = false;
        }
      }
    }

    success
  }
}