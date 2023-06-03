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

use crate::data::{Numbers, NumbersAndIncidence, NumbersAndIncidenceAndTotals};
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
  actually_save_numbers_into_db(db, country_id, &enriched_data)
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
  actually_save_numbers_into_db(db, country_id, &enriched_data)
}

fn actually_save_numbers_into_db(db: &Database, country_id: &i64, numbers: &[NumbersAndIncidenceAndTotals]) -> bool
{
  // Build insert statement.
  let mut batch = String::from(
    "INSERT INTO covid19 (countryId, date, cases, deaths, incidence14, \
     incidence7, totalCases, totalDeaths) VALUES "
  );
  // Reserve 60 bytes for every data record to avoid frequent reallocation.
  batch.reserve(60 * numbers.len());
  let country_id = country_id.to_string();
  for elem in numbers.iter()
  {
    batch.push('(');
    batch.push_str(&country_id);
    batch.push_str(", ");
    batch.push_str(&Database::quote(&elem.date));
    batch.push_str(", ");
    batch.push_str(&elem.cases.to_string());
    batch.push_str(", ");
    batch.push_str(&elem.deaths.to_string());
    batch.push_str(", ");
    match elem.incidence_14d
    {
      Some(float) => batch.push_str(&float.to_string()),
      None => batch.push_str("NULL")
    }
    batch.push_str(", ");
    match elem.incidence_7d
    {
      Some(float) => batch.push_str(&float.to_string()),
      None => batch.push_str("NULL")
    }
    batch.push_str(", ");
    batch.push_str(&elem.total_cases.to_string());
    batch.push_str(", ");
    batch.push_str(&elem.total_deaths.to_string());
    batch.push_str("),");
  }

  // replace last ',' with ';' to make it valid SQL syntax
  batch.truncate(batch.len() - 1);
  batch.push(';');

  // Insert all data in one go.
  db.batch(&batch)
}
