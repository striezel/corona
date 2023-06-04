/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2023  Dirk Stolle
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

use crate::data::{Numbers, NumbersAndIncidence};
use crate::database::Database;
use crate::data;

/**
 * Writes case numbers of one country it into the database.
 *
 * @param db          an open SQLite database with existing tables
 * @param country_id  id of the country in the database
 * @param population  population of the country; or -1 if unknown
 * @param numbers     case numbers for that country
 * @return Returns true, if all data was written to the database successfully.
 *         Returns false otherwise.
 */
pub fn numbers_into_db(db: &Database, country_id: &i64, population: &i32, numbers: &mut [Numbers]) -> bool
{
  if numbers.is_empty()
  {
    return true;
  }
  numbers.sort_unstable_by(|a, b| a.date.cmp(&b.date));
  let enriched_data = data::calculate_incidence(numbers, population);
  let enriched_data = data::calculate_totals(&enriched_data);
  db.insert_data(country_id, &enriched_data)
}

/**
 * Writes case numbers of one country it into the database.
 *
 * @param db          an open SQLite database with existing tables
 * @param country_id  id of the country in the database
 * @param numbers     case numbers for that country
 * @return Returns true, if all data was written to the database successfully.
 *         Returns false otherwise.
 */
pub fn numbers_and_incidence_into_db(db: &Database, country_id: &i64, numbers: &mut [NumbersAndIncidence]) -> bool
{
  if numbers.is_empty()
  {
    return true;
  }
  numbers.sort_unstable_by(|a, b| a.date.cmp(&b.date));
  let enriched_data = data::calculate_totals(numbers);
  db.insert_data(country_id, &enriched_data)
}
