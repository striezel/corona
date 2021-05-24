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

mod africa;
mod america;
pub mod api;
mod asia;
mod europe;
mod oceania;
mod other;

use crate::collect::api::disease_sh;
use crate::collect::api::Range;
use crate::data::Numbers;
use africa::*;
use america::*;
use asia::*;
use europe::*;
use oceania::*;
use other::*;
use crate::configuration::CollectConfiguration;

/// common trait / interface for collecting new data
pub trait Collect
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
  config: CollectConfiguration,
  elements: Vec<Box<dyn Collect>>
}

impl Collector
{
  /**
   * Creates a new Collector for all implemented countries.
   *
   * @return Returns a Collector, if the configuration is OK.
   *         Returns an error message, if an error occurred.
   */
  pub fn new(config: &CollectConfiguration) -> Result<Collector, String>
  {
    if config.db_path.is_empty()
    {
      return Err("Path for SQLite database must not be an empty string!".to_string());
    }

    Ok(Collector {
      config: CollectConfiguration
      {
        db_path: config.db_path.clone()
      },
      elements: Collector::all()
    })
  }

  /**
   * Gets a vector of all available structs that implement the Collect trait.
   *
   * @return Returns a vector of Collect implementations.
   */
  pub fn all() -> Vec<Box<dyn Collect>>
  {
    vec![
      Box::new(Afghanistan::new()),
      Box::new(Albania::new()),
      Box::new(Algeria::new()),
      Box::new(Andorra::new()),
      Box::new(Angola::new()),
      Box::new(Anguilla::new()),
      Box::new(AntiguaAndBarbuda::new()),
      Box::new(Argentina::new()),
      Box::new(Armenia::new()),
      Box::new(Aruba::new()),
      Box::new(Australia::new()),
      Box::new(Austria::new()),
      Box::new(Azerbaijan::new()),
      Box::new(Bahamas::new()),
      Box::new(Bahrain::new()),
      Box::new(Bangladesh::new()),
      Box::new(Barbados::new()),
      Box::new(Belarus::new()),
      Box::new(Belgium::new()),
      Box::new(Belize::new()),
      Box::new(Benin::new()),
      Box::new(Bermuda::new()),
      Box::new(Bhutan::new()),
      Box::new(Bolivia::new()),
      Box::new(BonaireSintEustatiusSaba::new()),
      Box::new(Bosnia::new()),
      Box::new(Bulgaria::new()),
      Box::new(Botswana::new()),
      Box::new(Brazil::new()),
      Box::new(BritishVirginIslands::new()),
      Box::new(BruneiDarussalam::new()),
      Box::new(BurkinaFaso::new()),
      Box::new(Burundi::new()),
      Box::new(Cambodia::new()),
      Box::new(Cameroon::new()),
      Box::new(Canada::new()),
      Box::new(CapeVerde::new()),
      Box::new(CasesOnAnInternationalConveyance::new()),
      Box::new(CaymanIslands::new()),
      Box::new(CentralAfricanRepublic::new()),
      Box::new(Chad::new()),
      Box::new(Chile::new()),
      Box::new(China::new()),
      Box::new(Colombia::new()),
      Box::new(Comoros::new()),
      Box::new(Congo::new()),
      Box::new(CostaRica::new()),
      Box::new(IvoryCoast::new()), // Cote d'Ivore
      Box::new(Croatia::new()),
      Box::new(Cuba::new()),
      Box::new(Curacao::new()),
      Box::new(Cyprus::new()),
      Box::new(Czechia::new()),
      Box::new(DemocraticRepublicOfTheCongo::new()),
      Box::new(Denmark::new()),
      Box::new(Djibouti::new()),
      Box::new(Dominica::new()),
      Box::new(DominicanRepublic::new()),
      Box::new(Ecuador::new()),
      Box::new(Egypt::new()),
      Box::new(ElSalvador::new()),
      Box::new(EquatorialGuinea::new()),
      Box::new(Eritrea::new()),
      Box::new(Estonia::new()),
      Box::new(Eswatini::new()),
      Box::new(Ethiopia::new()),
      Box::new(FaroeIslands::new()),
      Box::new(FalklandIslands::new()),
      Box::new(Fiji::new()),
      Box::new(Finland::new()),
      Box::new(France::new()),
      Box::new(FrenchPolynesia::new()),
      Box::new(Gabon::new()),
      Box::new(Gambia::new()),
      Box::new(Georgia::new()),
      Box::new(Germany::new()),
      Box::new(Ghana::new()),
      Box::new(Gibraltar::new()),
      Box::new(Greece::new()),
      Box::new(Greenland::new()),
      Box::new(Grenada::new()),
      Box::new(Guam::new()),
      Box::new(Guatemala::new()),
      Box::new(Guinea::new()),
      Box::new(GuineaBissau::new()),
      Box::new(Guyana::new()),
      Box::new(Haiti::new()),
      Box::new(HolySee::new()),
      Box::new(Honduras::new()),
      Box::new(Hungary::new()),
      Box::new(Iceland::new()),
      Box::new(India::new()),
      Box::new(Indonesia::new()),
      Box::new(Iran::new()),
      Box::new(Iraq::new()),
      Box::new(Ireland::new()),
      Box::new(IsleOfMan::new()),
      Box::new(Israel::new()),
      Box::new(Italy::new()),
      Box::new(Jamaica::new()),
      Box::new(Japan::new()),
      Box::new(Jersey::new()),
      Box::new(Jordan::new()),
      Box::new(Kazakhstan::new()),
      Box::new(Kenya::new()),
      Box::new(Kosovo::new()),
      Box::new(Kuwait::new()),
      Box::new(Kyrgyzstan::new()),
      Box::new(Laos::new()),
      Box::new(Latvia::new()),
      Box::new(Lebanon::new()),
      Box::new(Lesotho::new()),
      Box::new(Liberia::new()),
      Box::new(Libya::new()),
      Box::new(Liechtenstein::new()),
      Box::new(Lithuania::new()),
      Box::new(Luxembourg::new()),
      Box::new(Madagascar::new()),
      Box::new(Malawi::new()),
      Box::new(Malaysia::new()),
      Box::new(Maldives::new()),
      Box::new(Mali::new()),
      Box::new(Malta::new()),
      Box::new(MarshallIslands::new()),
      Box::new(Mauritania::new()),
      Box::new(Mauritius::new()),
      Box::new(Mexico::new()),
      Box::new(Moldova::new()),
      Box::new(Monaco::new()),
      Box::new(Mongolia::new()),
      Box::new(Montenegro::new()),
      Box::new(Montserrat::new()),
      Box::new(Morocco::new()),
      Box::new(Mozambique::new()),
      Box::new(Myanmar::new()),
      Box::new(Namibia::new()),
      Box::new(Netherlands::new()),
      Box::new(Nepal::new()),
      Box::new(NewCaledonia::new()),
      Box::new(NewZealand::new()),
      Box::new(Nicaragua::new()),
      Box::new(Niger::new()),
      Box::new(Nigeria::new()),
      Box::new(NorthernMarianaIslands::new()),
      Box::new(NorthMacedonia::new()),
      Box::new(Norway::new()),
      Box::new(Oman::new()),
      Box::new(Pakistan::new()),
      Box::new(Palestine::new()),
      Box::new(Panama::new()),
      Box::new(PapuaNewGuinea::new()),
      Box::new(Paraguay::new()),
      Box::new(Peru::new()),
      Box::new(Philippines::new()),
      Box::new(Poland::new()),
      Box::new(Portugal::new()),
      Box::new(PuertoRico::new()),
      Box::new(Qatar::new()),
      Box::new(Romania::new()),
      Box::new(Russia::new()),
      Box::new(Rwanda::new()),
      Box::new(SaintKittsAndNevis::new()),
      Box::new(SaintLucia::new()),
      Box::new(SaintVincentAndTheGrenadines::new()),
      Box::new(SanMarino::new()),
      Box::new(SaoTomeAndPrincipe::new()),
      Box::new(SaudiArabia::new()),
      Box::new(Senegal::new()),
      Box::new(Serbia::new()),
      Box::new(Seychelles::new()),
      Box::new(SierraLeone::new()),
      Box::new(Singapore::new()),
      Box::new(SintMaarten::new()),
      Box::new(Slovakia::new()),
      Box::new(Slovenia::new()),
      Box::new(SolomonIslands::new()),
      Box::new(Somalia::new()),
      Box::new(SouthAfrica::new()),
      Box::new(SouthKorea::new()),
      Box::new(SouthSudan::new()),
      Box::new(Spain::new()),
      Box::new(SriLanka::new()),
      Box::new(Sudan::new()),
      Box::new(Suriname::new()),
      Box::new(Sweden::new()),
      Box::new(Switzerland::new()),
      Box::new(Syria::new()),
      Box::new(Taiwan::new()),
      Box::new(Tajikistan::new()),
      Box::new(Tanzania::new()),
      Box::new(Thailand::new()),
      Box::new(TimorLeste::new()),
      Box::new(Togo::new()),
      Box::new(TrinidadAndTobago::new()),
      Box::new(Tunisia::new()),
      Box::new(Turkey::new()),
      Box::new(TurksAndCaicosIslands::new()),
      Box::new(Uganda::new()),
      Box::new(Ukraine::new()),
      Box::new(UnitedArabEmirates::new()),
      Box::new(UnitedKingdom::new()),
      Box::new(UnitedStatesOfAmerica::new()),
      Box::new(UnitedStatesVirginIslands::new()),
      Box::new(Uruguay::new()),
      Box::new(Uzbekistan::new()),
      Box::new(Vanuatu::new()),
      Box::new(Venezuela::new()),
      Box::new(Vietnam::new()),
      Box::new(WallisAndFutuna::new()),
      Box::new(Yemen::new()),
      Box::new(Zambia::new()),
      Box::new(Zimbabwe::new()),
    ]
  }

  pub fn run(&self) -> bool
  {
    match crate::checks::sqlite_check()
    {
      crate::checks::Status::Error(msg) =>
      {
        eprintln!("Error: {}", msg);
        return false;
      },
      crate::checks::Status::Warn(msg) =>
      {
        eprintln!("Warning: {}", msg);
        return false;
      },
      _ => ()
    }

    use crate::database::Database;

    let db = Database::create(&self.config.db_path);
    if db.is_err()
    {
      eprintln!("Error during database creation: {}", db.err().unwrap());
      return false;
    }
    let db = db.unwrap();
    if !db.calculate_total_numbers(&false)
    {
      eprintln!("Error: Could not add columns for accumulated numbers to database table!");
      return false;
    }

    println!("Collecting data for {} {} ...", self.elements.len(),
             if self.elements.len() != 1 { "countries" } else { "country "}
    );
    let world = crate::world::World::new();
    let mut count_ok: u32 = 0;
    let mut errors: Vec<String> = Vec::new();
    for country in self.elements.iter()
    {
      // Find matching country data.
      let country_data = match world.find_by_geo_id(&country.geo_id())
      {
        Some(c) => c,
        None =>
        {
          errors.push(country.geo_id().to_string());
          eprintln!("Error: Could not find country data for geo id '{}'!",
                    &country.geo_id());
          continue;
        }
      };

      println!("Collecting data for {} ...", &country_data.name);
      let data = country.collect(&Range::All);
      match data
      {
        Ok(vector) =>
        {
          // Insert country into database.
          let country_id = db.get_country_id_or_insert(&country.geo_id(),
                                                       &country_data.name,
                                                       &(country_data.population as i64),
                                                       &country_data.country_code,
                                                       &country_data.continent);
          if country_id <= 0
          {
            errors.push(format!("{} ({})", &country.geo_id(), &country_data.name));
            eprintln!("Error: Could not insert country data for geo id '{}' ({}) into database!",
                      &country.geo_id(), &country_data.name);
            continue;
          }
          let with_incidence = crate::data::calculate_incidence(&vector, &country_data.population);
          let with_incidence_and_totals = crate::data::calculate_totals(&with_incidence);
          let inserted = db.insert_data(&(country_id as i32), &with_incidence_and_totals);
          if !inserted
          {
            errors.push(format!("{} ({})", &country.geo_id(), &country_data.name));
            eprintln!("Error: Could not insert numbers for {} ({}) into database!",
                      &country_data.name, &country.geo_id());
          }
          else
          {
            count_ok += 1;
            println!("✓ OK");
          }
        },
        Err(error) =>
        {
          eprintln!("Error while collecting data for {} ({}): {}",
                    &country.geo_id(), &country_data.name, error);
          errors.push(format!("{} ({})", &country.geo_id(), &country_data.name));
        }
      }
    }

    println!("✓ Successfully collected data for {} of {} {}.", count_ok,
             self.elements.len(), if self.elements.len() != 1 { "countries" } else { "country "});
    if !errors.is_empty()
    {
      println!("❌ Failed to collect data for {} of {} {}.", errors.len(),
               self.elements.len(), if self.elements.len() != 1 { "countries" } else { "country "});
      for elem in errors.iter()
      {
        println!("    Collection failed for {}.", &elem);
      }
    }
    errors.is_empty()
  }
}
