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

//#[derive(Copy, Clone)]
pub enum Operation
{
  Html(HtmlConfiguration), // generate HTML files
  Csv(CsvConfiguration),   // write data to CSV
  Db(DbConfiguration),     // extract CSV data and write to DB
  Collect,                 // collects new data
  Version                  // show version
}

pub struct HtmlConfiguration
{
  pub db_path: String,
  pub output_directory: String
}

pub struct CsvConfiguration
{
  pub db_path: String,
  pub csv_output_file: String
}

pub struct DbConfiguration
{
  pub csv_input_file: String,
  pub db_path: String
}

pub fn parse_args(args: &[String]) -> Result<Operation, String>
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
    let csv_output_file = args[3].clone();
    return Ok(Operation::Csv(CsvConfiguration { db_path, csv_output_file}));
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
    return Ok(Operation::Html(HtmlConfiguration{ db_path, output_directory }));
  }

  if args[1] == "db"
  {
    // requires three parameters:
    // 1:   db
    // 2:   /path/to/input.csv
    // 3:   /path/to/corona.db
    if args.len() < 4
    {
      return Err(String::from("Not enough command line parameters for DB operation!"));
    }

    let csv_input_file = args[2].clone();
    let db_path = args[3].clone();
    return Ok(Operation::Db(DbConfiguration { csv_input_file, db_path }));
  }

  if args[1] == "collect"
  {
    // requires one parameter:
    // 1:   collect
    return Ok(Operation::Collect);
  }

  if args[1] == "version"
  {
    return Ok(Operation::Version);
  }

  // invalid command line parameters
  Err(String::from("Invalid command line parameters have been specified!"))
}
