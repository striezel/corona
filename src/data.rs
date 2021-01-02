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

/// struct that contains data of a single country
pub struct Country
{
  pub country_id: i32,
  pub name: String,
  pub population: i32,
  pub geo_id: String,
  pub country_code: String,
  pub continent: String
}

/// struct to hold the case numbers for a single day in a single country
pub struct Numbers
{
  pub date: String,
  pub cases: i32,
  pub deaths: i32
}

/// struct to hold the case numbers and 14-day incidence for a single day in a single country
pub struct NumbersAndIncidence
{
  pub date: String,
  pub cases: i32,
  pub deaths: i32,
  pub incidence_14d: Option<f64>
}

/// struct to hold 14-day incidence value for a single day in a single country
pub struct Incidence14
{
  pub date: String,
  pub incidence: f64
}
