/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2020  Dirk Stolle
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

use super::Configuration;
use crate::database::Database;
use crate::database::Country;
use crate::template::Template;

use std::fs; // for create_dir_all() and copy()
use std::path::Path;
use std::path::PathBuf;

pub struct Generator
{
  config: Configuration
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
  pub fn new(config: &Configuration) -> Result<Generator, String>
  {
    if config.db_path.is_empty()
    {
      return Err("Path to SQLite database must not be an empty string!".to_string());
    }
    if config.output_directory.is_empty()
    {
      return Err("Path of output directory must be set to a non-empty string!".to_string());
    }

    Ok(Generator
    {
      config: Configuration
      {
        db_path: config.db_path.clone(),
        output_directory: config.output_directory.clone(),
        op: config.op.clone()
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
   * Gets the path of the main.tpl template file.
   *
   * @return Returns the (relative) path of the template file.
   */
  fn get_template_file_name() -> String
  {
    let db_path = Path::new(file!()) // current file: src/generator.rs
        .parent().unwrap() // parent: src/
        .join("..") // up one directory
        .join("src") // into directory src/
        .join("templates") // into directory src
        .join("main.tpl"); // and to the main.tpl file;
    db_path.to_str().unwrap().to_string()
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
    if !tpl.from_file(&Generator::get_template_file_name())
    {
      eprintln!("Error: Could not load main template file!");
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
    if !tpl.from_file(&Generator::get_template_file_name())
    {
      eprintln!("Error: Could not load main template file!");
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
    if !tpl.from_file(&Generator::get_template_file_name())
    {
      eprintln!("Error: Could not load main template file!");
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
    let mut dates: Vec<String> = Vec::new();
    let mut infections: Vec<String> = Vec::new();
    let mut deaths: Vec<String> = Vec::new();
    let data = db.numbers(&country.country_id);
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
    let mut dates: Vec<String> = Vec::new();
    let mut infections: Vec<String> = Vec::new();
    let mut deaths: Vec<String> = Vec::new();
    let data = db.numbers_world();
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
    let mut dates: Vec<String> = Vec::new();
    let mut infections: Vec<String> = Vec::new();
    let mut deaths: Vec<String> = Vec::new();
    let data = db.accumulated_numbers(&country.country_id);
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
    let mut dates: Vec<String> = Vec::new();
    let mut infections: Vec<String> = Vec::new();
    let mut deaths: Vec<String> = Vec::new();
    let data = db.accumulated_numbers_world();
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
    // prepare numbers
    let mut dates: Vec<String> = Vec::new();
    let mut incidence: Vec<String> = Vec::new();
    let data = db.incidence(&country.country_id);
    // May be an empty array, if there is no known incidence.
    if data.is_empty()
    {
      return Some(String::from(""));
    }
    tpl.tag("title", &("Coronavirus: 14-day incidence in ".to_owned()
                     + &country.name + " (" + &country.geo_id + ")"));
    tpl.tag("plotId", &("graph_incidence14_".to_owned() + &country.geo_id.to_lowercase()));
    for d in data.iter()
    {
      dates.push(d.date.clone());
      incidence.push(d.incidence.to_string());
    }
    // graph: date values
    // TODO: Use proper JSON library for encoding.
    let dates = match dates.is_empty()
    {
      false => "[\"".to_owned() + &dates.join("\",\"") + "\"]",
      true => "[]".to_string()
    };
    tpl.integrate("dates", &dates);
    // graph: incidence values
    let incidence = match incidence.is_empty()
    {
      false => "[".to_owned() + &incidence.join(",") + "]",
      true => "[]".to_string()
    };
    tpl.integrate("incidence", &incidence);
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
      let data = db.incidence(&country.country_id);
      // May be an empty array, if there is no known incidence.
      if data.is_empty()
      {
        continue;
      }
      // prepare data for plot
      let mut dates: Vec<String> = Vec::new();
      let mut incidence: Vec<String> = Vec::new();
      for d in data.iter()
      {
        dates.push(d.date.clone());
        incidence.push(d.incidence.to_string());
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
        .parent().unwrap() // parent: src/
        .join("..") // up one directory
        .join("src") // into directory src/
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
    // Note: Should be replaced by opendir() / readdir() / closedir() triad once
    // there are more files. Or use directory iterator instead.
    let plotly_origin = Generator::get_assets_path().join("plotly-1.58.3.min.js");
    let plotly_destination = path.join("plotly-1.58.3.min.js");
    let cp_success = fs::copy(&plotly_origin, &plotly_destination);
    match cp_success
    {
      Ok(_bytes_written) => true,
      Err(e) => {
        eprintln!("Error: Could not copy asset file {:?} to {:?}: {}",
                  plotly_origin, plotly_destination, e);
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
    if !tpl.from_file(&Generator::get_template_file_name())
    {
      eprintln!("Error: Could not load main template file!");
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
      Some(generated) => content+ "<br />\n" + &generated,
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
mod tests {
  use super::*;

  /**
   * Gets a database instance connected to the corona.db file in data directory.
   *
   * @return Returns an open database.
   */
  fn get_sqlite_db_path() -> String
  {
    let db_path = Path::new(file!()) // current file: src/generator.rs
        .parent().unwrap() // parent: src/
        .join("..") // up one directory
        .join("data") // into directory data/
        .join("corona.db"); // and to the corona.db file;
    db_path.to_str().unwrap().to_string()
  }

  #[test]
  fn successful_execution()
  {
    use std::env;
    use std::fs;

    let directory = env::temp_dir().join("test_generation_of_files");
    let config = Configuration {
      db_path: get_sqlite_db_path(),
      output_directory: directory.to_str().unwrap().to_string()
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
