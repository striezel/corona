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

mod database;
mod template;
mod generator;
mod csv;

#[derive(Copy, Clone)]
pub enum Operation
{
  HtmlGeneration, // generate HTML files
  Csv,            // write data to CSV
  Version         // show version
}

pub struct Configuration
{
  pub db_path: String,
  pub output_directory: String,
  pub op: Operation
}

impl Configuration
{
  pub fn new(args: &[String]) -> Result<Configuration, String>
  {
    if args.len() < 2
    {
      return Err(String::from("Not enough command line parameters!"));
    }

    if args[1] == "csv"
    {
      // requires three parameters:
      // 1:   csv
      // 2:   /path/to/corona.db
      // 3:   /path/to/output.csv
      if args.len() < 4
      {
        return Err(String::from("Not enough command line parameters for CSV mode!"));
      }

      let db_path = args[2].clone();
      let output_directory = args[3].clone();
      return Ok(Configuration { db_path, output_directory, op: Operation::Csv });
    }

    if args[1] == "html"
    {
      // requires three parameters:
      // 1:   html
      // 2:   /path/to/corona.db
      // 3:   /path/to/output.csv
      if args.len() < 4
      {
        return Err(String::from("Not enough command line parameters for HTML generation!"));
      }

      let db_path = args[2].clone();
      let output_directory = args[3].clone();
      return Ok(Configuration { db_path, output_directory, op: Operation::HtmlGeneration });
    }

    if args[1] == "version"
    {
      return Ok(Configuration
      {
        db_path: String::new(),
        output_directory: String::new(),
        op: Operation::Version
      });
    }

    // invalid command line parameters
    Err(String::from("Invalid command line parameters have been specified!"))
  }
}

pub fn run(config: &Configuration) -> Result<(), String>
{
  match &config.op
  {
    Operation::HtmlGeneration => {
      use generator::Generator;
      let gen = Generator::new(&config)?;
      if !gen.generate()
      {
        return Err("Generation of HTML files failed!".to_string());
      }
      println!("Generation of HTML files was successful.");
      Ok(())
    },
    Operation::Csv => {
      use crate::csv::Csv;

      let csv = Csv::new(&config)?;
      if !csv.create_csv()
      {
        return Err("Failed to write CSV file!".to_string());
      }

      Ok(())
    },
    Operation::Version => {
      let version = match option_env!("CARGO_PKG_VERSION")
      {
        None => String::from("corona, unknown version (executable was not built with Cargo)"),
        Some(v) => format!("corona, version {}", v)
      };
      println!("{}", version);

      Ok(())
    }
  }
}
