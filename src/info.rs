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
use crate::collect::Collector;
use crate::data::calculate_incidence;
use crate::configuration::InfoConfiguration;

pub struct Info
{
  country_name: String
}

impl Info
{
  /**
   * Creates a new Info instance.
   *
   * @return Returns a Collector, if the configuration is OK.
   *         Returns an error message, if an error occurred.
   */
  pub fn new(config: &InfoConfiguration) -> Result<Info, String>
  {
    if config.country_name.is_empty()
    {
      return Err("Country name must not be an empty string!".to_string());
    }

    Ok(Info{ country_name: config.country_name.clone() })
  }

  pub fn run(&self) -> bool
  {
    let world = crate::world::World::new();
    let country = match world.find_by_geo_id(&self.country_name.to_uppercase())
    {
      None =>
      {
        match world.find_by_name(&self.country_name)
        {
          Some(c) => c,
          None =>
          {
            eprintln!("Error: Could not find a matching country for '{}'!",
                      &self.country_name);
            return false;
          }
        }
      },
      Some(c) => c
    };

    let all = Collector::all();
    let collector = match all.iter().find(|c|  c.geo_id() == country.geo_id)
    {
      Some(collect) => collect,
      None => {
        eprintln!("Error: Could not find a matching country for '{}'!",
                  &self.country_name);
        return false;
      }
    };

    let numbers = match collector.collect(&Range::Recent)
    {
      Ok(num) => num,
      Err(e) =>
      {
        eprintln!("Error: Could not get recent data for {}.\n{}", country.name, e);
        return false;
      }
    };

    let numbers = calculate_incidence(&numbers, &country.population);

    println!("Coronavirus cases in {} ({}):", country.name, country.geo_id);
    let mut has_incidence = false;
    for elem in numbers.iter().rev().take(10)
    {
      match elem.incidence_14d
      {
        Some(value) =>
        {
          println!("{}: {} cases, {} deaths, 14-day incidence: {}",
                   elem.date, elem.cases, elem.deaths, value);
          has_incidence = true;
        },
        None => println!("{}: {} cases, {} deaths",
                         elem.date, elem.cases, elem.deaths),
      }
    }

    if has_incidence
    {
      println!("\nThe 14-day incidence is the number of infections during \
                the last 14 days per\n100000 inhabitants. Note that some \
                authorities like e. g. Germany's Robert Koch\nInstitute use a \
                7-day incidence value instead, which is different.");
    }

    true
  }
}