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

/// enum to hold the status of a check
pub enum Status
{
  Ok,
  Warn(String),
  Error(String)
}

/**
 * Checks the used SQLite version for compatibility.
 *
 * @return Returns the status of the .
 */
pub fn sqlite_check() -> Status
{
  use crate::checks::Status::Error;

  let sqlite_version = rusqlite::version_number();
  // match expression with ranges is still experimental, so we have to use ifs.
  if sqlite_version < 3_006_008
  {
    return Error(format!(
      "The SQLite version you are using ({}) is too old for this program. \
       At least SQLite 3.6.8 is required.",
      rusqlite::version()
    ));
  }
  if sqlite_version < 3_026_000
  {
    use crate::checks::Status::Warn;
    return Warn(format!(
      "The SQLite version you are using ({}) may be too old to use all \
       features of this program. Some features may not work. Update to \
       SQLite 3.26.0 or later to avoid that warning.",
      rusqlite::version()
    ));
  }

  crate::checks::Status::Ok
}
