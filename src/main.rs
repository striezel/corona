/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2020, 2021, 2024, 2025  Dirk Stolle

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

use std::env; // for std::env::args
use std::process; // for std::process::exit

fn main()
{
  let args: Vec<String> = env::args().collect();

  let config = corona::configuration::parse_args(&args).unwrap_or_else(|err| {
    if !err.is_empty()
    {
      eprintln!("Error: {err}\n");
    }
    let basename = match std::path::Path::new(&args[0]).file_name()
    {
      // This conversion is getting really nasty here. Can Rust do better?
      Some(name) => name.to_string_lossy().into_owned(),
      None => args[0].clone()
    };
    eprintln!(
      "Usage: {basename} html /path/to/corona.db /path/to/output/directory [/path/to/main.tpl]"
    );
    eprintln!("           or");
    eprintln!("Usage: {basename} csv /path/to/corona.db /path/to/output.csv");
    eprintln!("           or");
    eprintln!("Usage: {basename} db /path/to/input.csv /path/to/output.db");
    eprintln!("           or");
    eprintln!("Usage: {basename} version");
    process::exit(1);
  });

  if let Err(e) = corona::run(&config)
  {
    eprintln!("An error occurred: {e}");
    process::exit(1);
  }
}
