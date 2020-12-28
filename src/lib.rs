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

pub struct Configuration
{
  pub db_path: String,
  pub output_directory: String
}

impl Configuration
{
  pub fn new(args: &[String]) -> Result<Configuration, String>
  {
    if args.len() < 3
    {
      return Err(String::from("Not enough command line parameters!"));
    }

    let db_path = args[1].clone();
    let output_directory = args[2].clone();
    Ok(Configuration { db_path, output_directory })
  }
}

pub fn run(config: &Configuration) -> Result<(), String>
{
  use generator::Generator;
  let gen = Generator::new(&config)?;
  if !gen.generate()
  {
    return Err("Generation of HTML files failed!".to_string());
  }
  Ok(())
}
