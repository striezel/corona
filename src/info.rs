/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2021, 2023  Dirk Stolle
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
use crate::configuration::InfoConfiguration;
use crate::data::calculate_incidence;

pub struct Info
{
  country_name: String
}

/**
 * Rounds a floating-point value to two decimals after the point.
 *
 * @return Returns the rounded value.
 */
fn round_to_2(f: &f64) -> f64
{
  (f * 100.0).round() / 100.0
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

  /**
   * Performs the info operation.
   *
   * @return Returns whether the operation was successful.
   */
  pub fn run(&self) -> bool
  {
    let world = crate::world::World::new();
    let country = match world.find_by_geo_id(&self.country_name.to_uppercase())
    {
      None => match world.find_by_name(&self.country_name)
      {
        Some(c) => c,
        None =>
        {
          eprintln!(
            "Error: Could not find a matching country for '{}'!",
            &self.country_name
          );
          return false;
        }
      },
      Some(c) => c
    };

    let all = Collector::all();
    let collector = match all.iter().find(|c| c.geo_id() == country.geo_id)
    {
      Some(collect) => collect,
      None =>
      {
        eprintln!(
          "Error: Could not find a matching country for '{}'!",
          &self.country_name
        );
        return false;
      }
    };

    let numbers = match collector.collect(&Range::Recent)
    {
      Ok(num) => num,
      Err(e) =>
      {
        eprintln!(
          "Error: Could not get recent data for {}.\n{}",
          country.name, e
        );
        return false;
      }
    };

    let numbers = calculate_incidence(&numbers, &country.population);

    println!(
      "Coronavirus cases in {} ({}):",
      country.name, country.geo_id
    );
    let mut has_incidence = false;
    for elem in numbers.iter().rev().take(10)
    {
      match elem.incidence_14d
      {
        Some(value) =>
        {
          println!("{}: {} infection(s), {} death(s), 14-day incidence¹: {}",
                   elem.date, elem.cases, elem.deaths, round_to_2(&value));
          has_incidence = true;
        },
        None => println!("{}: {} cases, {} deaths",
                         elem.date, elem.cases, elem.deaths),
      }
    }

    if has_incidence
    {
      println!("\n¹=The 14-day incidence is the number of infections during \
                the last 14 days per\n100000 inhabitants. Note that some \
                authorities like e. g. Germany's Robert Koch\nInstitute use a \
                7-day incidence value instead, which is different.");
    }

    // Add note about when JHU ceased data collection. However, Jersey still
    // uses another data source, so only show that when we are showing data for
    // other countries.
    if country.geo_id != "JE"
    {
      println!();
      println!("Note: The Johns Hopkins Coronavirus Resource Center ceased its \
                collecting and\nreporting of global COVID-19 data on 10th March \
                2023. Since this data is currently\nused by the program, newer \
                data cannot be shown.");
    }

    true
  }
}
