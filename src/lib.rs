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
pub mod configuration;

use crate::configuration::*;

pub fn run(op: &Operation) -> Result<(), String>
{
  match &op
  {
    Operation::Html(config) => {
      use generator::Generator;
      let gen = Generator::new(&config)?;
      if !gen.generate()
      {
        return Err("Generation of HTML files failed!".to_string());
      }
      println!("Generation of HTML files was successful.");
      Ok(())
    },
    Operation::Csv(config) => {
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