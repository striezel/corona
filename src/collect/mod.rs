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

mod api;
mod europe;

use crate::collect::api::disease_sh;
use crate::collect::api::Range;
use crate::data::Numbers;
use europe::*;


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
   * @param  range   the data range to collect
   * @return Returns a vector containing new daily numbers for cases + deaths.
   *         Returns an Err(), if no data could be retrieved.
   */
  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    // Default implementation: Use disease.sh API.
    disease_sh::request_historical_api(self.geo_id(), &range)
  }
}

pub struct Collector
{
  elements: Vec<Box<dyn Collect>>
}

impl Collector
{
  /**
   * Creates a new Collector for all implemented countries.
   */
  pub fn new() -> Collector
  {
    Collector{ elements: vec![
      Box::new(Albania::new()),
      Box::new(Andorra::new()),
      Box::new(Armenia::new()),
      Box::new(Azerbaijan::new()),
      Box::new(Belarus::new()),
      Box::new(Bosnia::new()),
      Box::new(Bulgaria::new()),
      Box::new(Croatia::new()),
      Box::new(Cyprus::new()),
      Box::new(Czechia::new()),
      Box::new(Denmark::new()),
      Box::new(Estonia::new()),
      Box::new(FaroeIslands::new()),
      Box::new(Finland::new()),
      Box::new(Georgia::new()),
      Box::new(Gibraltar::new()),
      Box::new(Greece::new()),
      Box::new(Hungary::new()),
      Box::new(Iceland::new()),
      Box::new(Ireland::new()),
      Box::new(IsleOfMan::new()),
      Box::new(Italy::new()),
      Box::new(Kosovo::new()),
      Box::new(Latvia::new()),
      Box::new(Lithuania::new()),
      Box::new(Luxembourg::new()),
      Box::new(Malta::new()),
      Box::new(Moldova::new()),
      Box::new(Monaco::new()),
      Box::new(Montenegro::new()),
      Box::new(NorthMacedonia::new()),
      Box::new(Norway::new()),
      Box::new(Poland::new()),
      Box::new(Portugal::new()),
      Box::new(Romania::new()),
      Box::new(SanMarino::new()),
      Box::new(Serbia::new()),
      Box::new(Slovakia::new()),
      Box::new(Slovenia::new()),
      Box::new(Spain::new()),
      Box::new(Sweden::new()),
      Box::new(Ukraine::new()),
      Box::new(UnitedKingdom::new()),
    ] }
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
      let data = country.collect(&Range::Recent);
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