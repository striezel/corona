/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2020, 2021  Dirk Stolle
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

use super::configuration::HtmlConfiguration;
use crate::data::Country;
use crate::database::Database;
use crate::template::Template;

use std::fs; // for create_dir_all() and copy()
use std::path::Path;
use std::path::PathBuf;

#[cfg(not(target_family = "windows"))]
const MAIN_TEMPLATE: &str = include_str!("./templates/main.tpl");

#[cfg(target_family = "windows")]
const MAIN_TEMPLATE: &str = include_str!(".\\templates\\main.tpl");

pub struct Generator
{
  config: HtmlConfiguration
}

impl Generator
{
  /**
   * Creates a new Generator instance.
   *
   * @config   application configuration
   * @return   Returns a Result containing the Generator object, if successful.
   *           Returns a string with an error message, if the configuration
   *           seems to be invalid.
   */
  pub fn new(config: &HtmlConfiguration) -> Result<Generator, String>
  {
    if config.db_path.is_empty()
    {
      return Err("Path to SQLite database must not be an empty string!".to_string());
    }
    if config.output_directory.is_empty()
    {
      return Err("Path of output directory must be set to a non-empty string!".to_string());
    }
    if let Some(path) = &config.template_path
    {
      if !path.exists()
      {
        return Err(format!("Template file '{}' does not exist!", path.display()));
      }
    }

    Ok(Generator
    {
      config: HtmlConfiguration
      {
        db_path: config.db_path.clone(),
        output_directory: config.output_directory.clone(),
        template_path: config.template_path.clone()
      }
    })
  }

  /**
   * Generates the HTML files.
   *
   * @return Returns whether the generation was successful.
   */
  pub fn generate(&self) -> bool
  {
    let db = Database::new(&self.config.db_path);
    let db = match db
    {
      Ok(db) => db,
      Err(_) => {
        eprintln!("Error: Database file {} does not exist or is not readable!", self.config.db_path);
        return false;
      }
    };

    let success = fs::create_dir_all(&self.config.output_directory);
    if success.is_err()
    {
      eprintln!("Error: Could not create directory {}: {}",
                self.config.output_directory, success.unwrap_err());
      return false;
    }
    // Perform calculations for total numbers in database, if necessary.
    if !db.calculate_total_numbers()
    {
      eprintln!("Error: Database update failed. \
                 Calculations for accumulated numbers could not be performed!");
      return false;
    }
    // Handle each country.
    let countries = db.countries();
    if countries.is_empty()
    {
      // Something is wrong here, there is no data.
      eprintln!("Error: Could not find any countries in the database {}!",
                self.config.db_path);
      return false;
    }
    for country in countries.iter()
    {
      if !self.generate_country(&db, &country)
      {
        eprintln!("Error while generating file for {} ({})!", &country.name, &country.geo_id);
        return false;
      }
    }
    // Handle accumulated numbers worldwide.
    if !self.generate_world(&db)
    {
      eprintln!("Error while generating file for worldwide numbers!");
      return false;
    }
    // Generate graphs per continent (incidence only).
    if !self.generate_continents(&db)
    {
      eprintln!("Error while generating files for continents!");
      return false;
    }
    // Copy assets.
    if !self.create_assets()
    {
      return false;
    }
    // Site index comes last.
    self.create_index(&countries, &db.continents())
  }

  /**
   * Loads the template from a file or the internal string slice.
   * The decision is made depending on the configuration setting.
   *
   * @param tpl   the template instance into which the template shall be loaded
   * @return Returns whether the template was loaded successfully.
   */
  fn load_template(&self, tpl: &mut Template) -> bool
  {
    match &self.config.template_path
    {
      None =>
      {
        if !tpl.load_from_str(MAIN_TEMPLATE)
        {
          eprintln!("Error: Could not load main template!");
          return false;
        }
      },
      Some(path) =>
      {
        if !tpl.load_from_file(&path)
        {
          eprintln!("Error: Could not load main template file '{}'!", path.display());
          return false;
        }
      }
    }

    true
  }

  /**
   * Generates the HTML file for a single country.
   *
   * @param db       reference to the Database instance
   * @param country  country data (id, name, etc.)
   * @return Returns whether the generation was successful.
   */
  fn generate_country(&self, db: &Database, country: &Country) -> bool
  {
    let mut tpl = Template::new();
    if !self.load_template(&mut tpl)
    {
      return false;
    }
    // scripts
    if !tpl.load_section("script")
    {
      return false;
    }
    tpl.tag("path", "./assets/plotly-1.58.3.min.js");
    let scripts = match tpl.generate()
    {
      Some(generated) => generated,
      None => return false
    };
    // header
    if !tpl.load_section("header")
    {
      return false;
    }
    tpl.integrate("scripts", &scripts);
    tpl.tag("title",
            &("Corona cases in ".to_owned() + &country.name
          + " (" + &country.geo_id + ")"));
    let header = match tpl.generate()
    {
      Some(generated) => generated,
      None => return false
    };
    // graph
    let graph = self.generate_graph(&db, &country, &mut tpl);
    let graph = match graph
    {
      Some(stringy) => stringy,
      None => return false
    };
    let graph_accu = self.generate_accumulated_graph(&db, &country, &mut tpl);
    let graph_accu = match graph_accu
    {
      Some(stringy) => stringy,
      None => return false
    };
    let mut graph = graph + "\n<br />\n" + &graph_accu;
    let graph_incidence = self.generate_incidence_graph(&db, &country, &mut tpl);
    let graph_incidence = match graph_incidence
    {
      Some(stringy) => stringy,
      None => return false
    };
    if !graph_incidence.is_empty()
    {
      graph = graph_incidence + "\n<br />\n" + &graph;
    }
    // full
    if !tpl.load_section("full")
    {
      return false;
    }
    tpl.integrate("header", &header);
    tpl.integrate("content", &graph);
    let full = match tpl.generate()
    {
      Some(generated) => generated,
      None => return false
    };
    // write it to a file
    let file = format!("{}/{}.html", self.config.output_directory, country.geo_id.to_lowercase());
    let written = fs::write(&file, &full.as_bytes());
    written.is_ok()
  }

  /**
   * Generates the HTML file for worldwide numbers.
   *
   * @param db       reference to the Database instance
   * @return Returns whether the generation was successful.
   */
  fn generate_world(&self, db: &Database) -> bool
  {
    let mut tpl = Template::new();
    if !self.load_template(&mut tpl)
    {
      return false;
    }
    // scripts
    if !tpl.load_section("script")
    {
      return false;
    }
    tpl.tag("path", "./assets/plotly-1.58.3.min.js");
    let scripts = match tpl.generate()
    {
      Some(stringy) => stringy,
      None => return false
    };
    // header
    if !tpl.load_section("header")
    {
      return false;
    }
    tpl.integrate("scripts", &scripts);
    tpl.tag("title", "Coronavirus cases worldwide");
    let header = match tpl.generate()
    {
      Some(generated) => generated,
      None => return false
    };
    // graph
    let graph = match self.generate_graph_world(&db, &mut tpl)
    {
      Some(generated) => generated,
      None => return false
    };
    let graph_accu = match self.generate_accumulated_graph_world(&db, &mut tpl)
    {
      Some(generated) => generated,
      None => return false
    };
    let graph = graph + "\n<br />\n" + &graph_accu;
    // full
    if !tpl.load_section("full")
    {
      return false;
    }
    tpl.integrate("header", &header);
    tpl.integrate("content", &graph);
    let full = match tpl.generate()
    {
      Some(generated) => generated,
      None => return false
    };
    // write it to a file
    let file = format!("{}/world.html", self.config.output_directory);
    let written = fs::write(&file, &full.as_bytes());
    written.is_ok()
  }

  /**
   * Generates the HTML files for different continents.
   *
   * @param db       reference to the Database instance
   * @return Returns whether the generation was successful.
   */
  fn generate_continents(&self, db: &Database) -> bool
  {
    let mut tpl = Template::new();
    if !self.load_template(&mut tpl)
    {
      return false;
    }

    let continents = db.continents();
    for continent in continents.iter()
    {
      // template: scripts
      if !tpl.load_section("script")
      {
        return false;
      }
      tpl.tag("path", "./assets/plotly-1.58.3.min.js");
      let scripts = match tpl.generate()
      {
        Some(generated) => generated,
        None => return false
      };
      // template: header
      if !tpl.load_section("header")
      {
        return false;
      }
      tpl.integrate("scripts", &scripts);
      tpl.tag("title", &("Coronavirus incidence in ".to_owned() + &continent));
      let header = match tpl.generate()
      {
        Some(generated) => generated,
        None => return false
      };
      // template: graph
      let graph = self.generate_graph_continent(&db, &continent, &mut tpl);
      let graph = match graph
      {
        Some(g) => g,
        None => return false
      };
      // template: full
      if !tpl.load_section("full")
      {
        return false;
      }
      tpl.integrate("header", &header);
      tpl.integrate("content", &graph);
      let full = match tpl.generate()
      {
        Some(stuff) => stuff,
        None => return false
      };
      // write it to a file
      let file = format!("{}/continent_{}.html", self.config.output_directory, &continent.to_lowercase());
      let written = fs::write(&file, &full.as_bytes());
      if written.is_err()
      {
        return false;
      }
    }
    // All is done here.
    true
  }

  /**
   * Generates the HTML snippet containing the graph of a single country.
   *
   * @param db       reference to the Database instance
   * @param country  country data (id, name, etc.)
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns None, if an error occurred.
   */
  fn generate_graph(&self, db: &Database, country: &Country, tpl: &mut Template) -> Option<String>
  {
    // load graph section
    if !tpl.load_section("graph")
    {
      return None;
    }
    tpl.tag("title", &("Coronavirus cases in ".to_owned() + &country.name
                   + " (" + &country.geo_id + ")"));
    tpl.tag("plotId", &("graph_".to_owned() + &country.geo_id.to_lowercase()));
    // prepare numbers
    let data = db.numbers(&country.country_id);
    let capacity = data.len();
    let mut dates: Vec<String> = Vec::with_capacity(capacity);
    let mut infections: Vec<String> = Vec::with_capacity(capacity);
    let mut deaths: Vec<String> = Vec::with_capacity(capacity);
    for d in data.iter()
    {
      dates.push(d.date.clone());
      infections.push(d.cases.to_string());
      deaths.push(d.deaths.to_string());
    }
    // graph: date values
    // -- Do some handmade "JSON-encoding". Since all of them are strings, we
    //    can just join them with '", "' and add the '['  and ']' at the begin
    //    and end and are done with it.
    // TODO: Use proper JSON library for encoding.
    let dates = match dates.is_empty()
    {
      false => "[\"".to_owned() + &dates.join("\",\"") + "\"]",
      true => "[]".to_string()
    };
    tpl.integrate("dates", &dates);
    // graph: infection values
    let infections = match infections.is_empty()
    {
      false => "[".to_owned() + &infections.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("infections", &infections);
    // graph: deaths
    let deaths = match deaths.is_empty()
    {
      false => "[".to_owned() + &deaths.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("deaths", &deaths);
    tpl.generate()
  }

  /**
   * Generates the HTML snippet containing the graph for worldwide data.
   *
   * @param db       reference to the Database instance
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns None, if an error occurred.
   */
  fn generate_graph_world(&self, db: &Database, tpl: &mut Template) -> Option<String>
  {
    // load graph section
    if !tpl.load_section("graph")
    {
      return None;
    }
    tpl.tag("title", "Coronavirus cases worldwide");
    tpl.tag("plotId", "graph_world");
    // prepare numbers
    let data = db.numbers_world();
    let capacity = data.len();
    let mut dates: Vec<String> = Vec::with_capacity(capacity);
    let mut infections: Vec<String> = Vec::with_capacity(capacity);
    let mut deaths: Vec<String> = Vec::with_capacity(capacity);
    for d in data.iter()
    {
      dates.push(d.date.clone());
      infections.push(d.cases.to_string());
      deaths.push(d.deaths.to_string());
    }
    // graph: date values
    // TODO: Use proper JSON library for encoding.
    let dates = match dates.is_empty()
    {
      false => "[\"".to_owned() + &dates.join("\",\"") + "\"]",
      true => "[]".to_string()
    };
    tpl.integrate("dates", &dates);
    // graph: infection values
    let infections = match infections.is_empty()
    {
      false => "[".to_owned() + &infections.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("infections", &infections);
    // graph: deaths
    let deaths = match deaths.is_empty()
    {
      false => "[".to_owned() + &deaths.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("deaths", &deaths);
    tpl.generate()
  }

  /**
   * Generates the HTML snippet containing the graph with accumulated numbers of a single country.
   *
   * @param db       reference to the Database instance
   * @param country  country data (id, name, etc.)
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns None, if an error occurred.
   */
  fn generate_accumulated_graph(&self, db: &Database, country: &Country, tpl: &mut Template) -> Option<String>
  {
    // load graph section
    if !tpl.load_section("graphAccumulated")
    {
      return None;
    }
    tpl.tag("title", &("Accumulated Coronavirus cases in ".to_owned()
                     + &country.name + " (" + &country.geo_id + ")"));
    tpl.tag("plotId", &("graph_accu_".to_owned() + &country.geo_id.to_lowercase()));
    // prepare numbers
    let data = db.accumulated_numbers(&country.country_id);
    let capacity = data.len();
    let mut dates: Vec<String> = Vec::with_capacity(capacity);
    let mut infections: Vec<String> = Vec::with_capacity(capacity);
    let mut deaths: Vec<String> = Vec::with_capacity(capacity);
    for d in data.iter()
    {
      dates.push(d.date.clone());
      infections.push(d.cases.to_string());
      deaths.push(d.deaths.to_string());
    }
    // graph: date values
    // TODO: Use proper JSON library for encoding.
    let dates = match dates.is_empty()
    {
      false => "[\"".to_owned() + &dates.join("\",\"") + "\"]",
      true => "[]".to_string()
    };
    tpl.integrate("dates", &dates);
    // graph: infection values
    let infections = match infections.is_empty()
    {
      false => "[".to_owned() + &infections.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("infections", &infections);
    // graph: deaths
    let deaths = match deaths.is_empty()
    {
      false => "[".to_owned() + &deaths.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("deaths", &deaths);
    tpl.generate()
  }

  /**
   * Generates the HTML snippet containing the graph with accumulated worldwide data.
   *
   * @param db       reference to the Database instance
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns None, if an error occurred.
   */
  fn generate_accumulated_graph_world(&self, db: &Database, tpl: &mut Template) -> Option<String>
  {
    // load graph section
    if !tpl.load_section("graphAccumulated")
    {
      return None;
    }
    tpl.tag("title", "Accumulated Coronavirus cases worldwide");
    tpl.tag("plotId", "graph_world_accu");
    // prepare numbers
    let data = db.accumulated_numbers_world();
    let capacity = data.len();
    let mut dates: Vec<String> = Vec::with_capacity(capacity);
    let mut infections: Vec<String> = Vec::with_capacity(capacity);
    let mut deaths: Vec<String> = Vec::with_capacity(capacity);
    for d in data.iter()
    {
      dates.push(d.date.clone());
      infections.push(d.cases.to_string());
      deaths.push(d.deaths.to_string());
    }
    // Sometimes the values for the latest day are lower than the values for the
    // previous day, because not all countries have numbers for the latest day
    // yet. To avoid that, remove the latest day, if necessary.
    // Sometimes it also affects more latest days instead of just one.
    let max = data.iter().rev().take(5).max_by(|x, y| x.cases.cmp(&y.cases).then(x.deaths.cmp(&y.deaths)));
    let pos = data.iter().rev().take(5).position(|elem| elem.cases == max.unwrap().cases);
    if let Some(idx) = pos
    {
      // Remove the last elements.
      for _i in 0..idx
      {
        dates.pop();
        infections.pop();
        deaths.pop();
      }
    }
    // graph: date values
    // TODO: Use proper JSON library for encoding.
    let dates = match dates.is_empty()
    {
      false => "[\"".to_owned() + &dates.join("\",\"") + "\"]",
      true => "[]".to_string()
    };
    tpl.integrate("dates", &dates);
    // graph: infection values
    let infections = match infections.is_empty()
    {
      false => "[".to_owned() + &infections.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("infections", &infections);
    // graph: deaths
    let deaths = match deaths.is_empty()
    {
      false => "[".to_owned() + &deaths.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("deaths", &deaths);
    tpl.generate()
  }

  /**
   * Generates the HTML snippet containing the graph with 14-day incidence numbers of a single country.
   *
   * @param db       reference to the Database instance
   * @param country  country data (id, name, etc.)
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns None, if an error occurred.
   */
  fn generate_incidence_graph(&self, db: &Database, country: &Country, tpl: &mut Template) -> Option<String>
  {
    // load graph section
    if !tpl.load_section("graphIncidence")
    {
      return None;
    }
    let data14 = db.incidence14(&country.country_id);
    // May be an empty array, if there is no known incidence.
    if data14.is_empty()
    {
      return Some(String::from(""));
    }
    tpl.tag("title", &("Coronavirus: incidences in ".to_owned()
                     + &country.name + " (" + &country.geo_id + ")"));
    tpl.tag("plotId", &("graph_incidence14_".to_owned() + &country.geo_id.to_lowercase()));
    // prepare numbers
    let capacity = data14.len();
    let mut dates: Vec<String> = Vec::with_capacity(capacity);
    let mut incidence: Vec<String> = Vec::with_capacity(capacity);
    for d in data14.iter()
    {
      dates.push(d.date.clone());
      incidence.push(d.incidence_14d.to_string());
    }
    // graph: date values
    // TODO: Use proper JSON library for encoding.
    let dates = match dates.is_empty()
    {
      false => "[\"".to_owned() + &dates.join("\",\"") + "\"]",
      true => "[]".to_string()
    };
    tpl.integrate("dates14", &dates);
    // graph: incidence values
    let incidence = match incidence.is_empty()
    {
      false => "[".to_owned() + &incidence.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("incidence14", &incidence);

    let data7 = db.incidence7(&country.country_id);
    // May be an empty array, if there is no known incidence.
    if data7.is_empty()
    {
      return Some(String::from(""));
    }
    // prepare numbers
    let capacity = data7.len();
    let mut dates: Vec<String> = Vec::with_capacity(capacity);
    let mut incidence: Vec<String> = Vec::with_capacity(capacity);
    for d in data7.iter()
    {
      dates.push(d.date.clone());
      incidence.push(d.incidence_7d.to_string());
    }
    // graph: date values
    // TODO: Use proper JSON library for encoding.
    let dates = match dates.is_empty()
    {
      false => "[\"".to_owned() + &dates.join("\",\"") + "\"]",
      true => "[]".to_string()
    };
    tpl.integrate("dates7", &dates);
    // graph: incidence values
    let incidence = match incidence.is_empty()
    {
      false => "[".to_owned() + &incidence.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("incidence7", &incidence);

    tpl.generate()
  }

  /**
   * Generates the HTML snippet containing the graph with 14-day incidence numbers of the continent.
   *
   * @param db         reference to the Database instance
   * @param continent  name of the continent
   * @param tpl        loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns None, if an error occurred.
   */
  fn generate_graph_continent(&self, db: &Database, continent: &str, tpl: &mut Template) -> Option<String>
  {
    // load graph section
    if !tpl.load_section("trace")
    {
      return None;
    }
    let mut traces = String::new();
    // iterate over countries
    let countries = db.countries_of_continent(&continent);
    for country in countries.iter()
    {
      let data = db.incidence14(&country.country_id);
      // May be an empty array, if there is no known incidence.
      if data.is_empty()
      {
        continue;
      }
      // prepare data for plot
      let capacity = data.len();
      let mut dates: Vec<String> = Vec::with_capacity(capacity);
      let mut incidence: Vec<String> = Vec::with_capacity(capacity);
      for d in data.iter()
      {
        dates.push(d.date.clone());
        incidence.push(d.incidence_14d.to_string());
      }
      // graph: date values
      // TODO: Use proper JSON library for encoding.
      let dates = match dates.is_empty()
      {
        false => "[\"".to_owned() + &dates.join("\",\"") + "\"]",
        true => "[]".to_string()
      };
      // graph: indicence values
      let incidence = match incidence.is_empty()
      {
        false => "[".to_owned() + &incidence.join(",") + "]",
        true => "[]".to_string()
      };
      // template generation for data
      tpl.integrate("dates", &dates);
      tpl.integrate("incidence", &incidence);
      tpl.tag("name", &country.name);
      traces = match tpl.generate()
      {
        Some(generated) => traces + &generated,
        None => return None
      };
    }
    // template: graph
    if !tpl.load_section("graphContinent")
    {
      return None;
    }
    tpl.integrate("traces", &traces);
    tpl.tag("plotId", &("continent_".to_owned() + &continent.to_lowercase()));
    tpl.tag("title", &("Coronavirus: 14-day incidence in ".to_owned() + continent));
    tpl.generate()
  }

  /**
   * Gets the path of the assets directory.
   *
   * @return Returns the Path of the assets directory.
   */
  fn get_assets_path() -> PathBuf
  {
    Path::new(file!()) // current file: src/generator.rs
      .parent()
      .unwrap() // parent: src/
      .join("assets") // into directory assets
  }

  /**
   * Creates any assets (i. e. library files) in the output directory.
   *
   * @return Returns whether the operation was successful.
   */
  fn create_assets(&self) -> bool
  {
    let path = Path::new(&self.config.output_directory).join("assets");
    let created = fs::create_dir_all(&path);
    if created.is_err()
    {
      eprintln!("Error: Could not create directory {:?}: {}",
                path, created.unwrap_err());
      return false;
    }

    self.copy_or_download_plotly_js(&path)
  }

  /**
   * Either copies or downloads the minified plotly.js file to the destination
   * directory.
   *
   * @param assets_destination  destination directory for assets
   * @return Returns true, if file was created successfully.
   */
  fn copy_or_download_plotly_js(&self, assets_destination: &Path) -> bool
  {
    let plotly_origin = Generator::get_assets_path().join("plotly-1.58.3.min.js");
    let plotly_destination = assets_destination.join("plotly-1.58.3.min.js");
    if plotly_origin.exists()
    {
      println!("File {:?} does exist.", plotly_origin);
      let cp_success = fs::copy(&plotly_origin, &plotly_destination);
      return match cp_success
      {
        Ok(_bytes_written) => true,
        Err(e) => {
          eprintln!("Error: Could not copy asset file {:?} to {:?}: {}",
                    plotly_origin, plotly_destination, e);
          false
        }
      }
    }

    // File does not exist, so download it from CDN.
    use reqwest::StatusCode;
    use std::io::Read;
    // Retrieve minified JS file.
    let mut res = match reqwest::blocking::get("https://cdn.plot.ly/plotly-1.58.3.min.js")
    {
      Ok(responded) => responded,
      Err(e) => {
        eprintln!("Download of plotly.js failed: {}", e);
        return false;
      }
    };
    let mut body: Vec<u8> = Vec::new();
    if let Err(e) = res.read_to_end(&mut body)
    {
      eprintln!("Failed to read plotly.js into buffer: {}", e);
      return false;
    }
    if res.status() != StatusCode::OK
    {
      eprintln!("HTTP request failed with unexpected status code: {}\n\
                 Headers:\n{:#?}\n\
                 Body:\n{:?}", res.status(), res.headers(), body);
      return false;
    }

    match std::fs::write(&plotly_destination, &body)
    {
      Ok(()) => true,
      Err(e) => {
        eprintln!("Error while writing plotly.js file: {}", e);
        false
      }
    }
  }

  /**
   * Creates the index.html in the output directory.
   *
   * @param countries   array containing names and ids of the countries
   * @param countries   array containing names of the continents
   * @return Returns whether the operation was successful.
   */
  fn create_index(&self, countries: &[Country], continents: &[String]) -> bool
  {
    let mut tpl = Template::new();
    if !self.load_template(&mut tpl)
    {
      return false;
    }
    // links
    if !tpl.load_section("indexLink")
    {
      return false;
    }
    // worldwide links + country links
    tpl.tag("url", "./world.html");
    tpl.tag("text", "All countries accumulated");
    let mut links = match tpl.generate()
    {
      Some(generated) => generated,
      None => return false
    };
    for country in countries.iter()
    {
      tpl.tag("url", &("./".to_owned() + &country.geo_id.to_lowercase() + ".html"));
      tpl.tag("text", &(country.name.clone() + " (" + &country.geo_id + ")"));
      links = match tpl.generate()
      {
        Some(generated) => links + &generated,
        None => return false
      };
    }
    // continent links
    let mut continent_links = String::new();
    for continent in continents.iter()
    {
      tpl.tag("url", &("./continent_".to_owned() + &continent.to_lowercase() + ".html"));
      tpl.tag("text", &continent);
      continent_links = match tpl.generate()
      {
        Some(generated) => continent_links + &generated,
        None => return false
      };
    }
    // index template
    if !tpl.load_section("index")
    {
      return false;
    }
    tpl.integrate("links", &links);
    let mut content = match tpl.generate()
    {
      Some(generated) => generated,
      None => return false
    };
    // continent index template
    if !tpl.load_section("indexContinents")
    {
      return false;
    }
    tpl.integrate("links", &continent_links);
    content = match tpl.generate()
    {
      Some(generated) => content + "<br />\n" + &generated,
      None => return false
    };
    // main page template
    // -- header
    if !tpl.load_section("header")
    {
      return false;
    }
    tpl.integrate("scripts", "");
    tpl.tag("title", "Corona worldwide");
    let header = match tpl.generate()
    {
      Some(stuff) => stuff,
      None => return false
    };
    // -- full template
    if !tpl.load_section("full")
    {
      return false;
    }
    tpl.integrate("header", &header);
    tpl.integrate("content", &content);
    let full = match tpl.generate()
    {
      Some(generated) => generated,
      None => return false
    };
    // write it to a file
    let file = format!("{}/index.html", self.config.output_directory);
    let written = fs::write(&file, &full.as_bytes());
    written.is_ok()
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  /**
   * Gets the path to the corona.db file in data directory.
   *
   * @return Returns path of the SQLite database.
   */
  fn get_sqlite_db_path() -> String
  {
    let db_path = Path::new(file!()) // current file: src/generator.rs
      .parent()
      .unwrap() // parent: src/
      .join("..") // up one directory
      .join("data") // into directory data/
      .join("corona-ecdc-2020-12-14.db"); // and to the corona.db file;
    db_path.to_str().unwrap().to_string()
  }

  #[test]
  fn successful_execution()
  {
    use std::env;
    use std::fs;

    let directory = env::temp_dir().join("test_generation_of_files");
    let config = HtmlConfiguration {
      db_path: get_sqlite_db_path(),
      output_directory: directory.to_str().unwrap().to_string(),
      template_path: None
    };
    let gen = Generator::new(&config).unwrap();
    assert!(gen.generate());
    // Check that some paths exists.
    assert!(directory.join("index.html").exists());
    assert!(directory.join("world.html").exists());
    assert!(directory.join("continent_asia.html").exists());
    assert!(directory.join("cn.html").exists());
    assert!(directory.join("de.html").exists());
    assert!(directory.join("el.html").exists());
    assert!(directory.join("ke.html").exists());
    assert!(directory.join("us.html").exists());
    // clean up
    assert!(fs::remove_dir_all(directory).is_ok());
  }
}
