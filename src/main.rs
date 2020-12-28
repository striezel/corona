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

use std::env; // for std::env::args
use std::process; // for std::process::exit

fn main()
{
  let args: Vec<String> = env::args().collect();

  let config = corona::Configuration::new(&args).unwrap_or_else(|_err| {
    eprintln!("Usage: {} /path/to/corona.db /path/to/output/directory", args[0]);
    process::exit(1);
  });

  if let Err(e) = corona::run(&config)
  {
    eprintln!("An error occurred: {}", e);
    process::exit(1);
  }

  println!("Generation of HTML files was successful.");
}
