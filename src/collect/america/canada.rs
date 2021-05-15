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

use crate::collect::api::Range;
use crate::collect::Collect;
use crate::data::Numbers;

pub struct Canada
{
}

impl Canada
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Canada
  {
    Canada { }
  }

  /// zero-based column index of province name ("prname")
  const INDEX_PRNAME: usize = 1;

  /// zero-based column index of date ("date")
  const INDEX_DATE: usize = 3;

  /// zero-based column index of daily new cases ("numtoday")
  const INDEX_CASES: usize = 15;

  /// zero-based column index of daily new deaths ("numdeathstoday")
  const INDEX_DEATHS: usize = 19;

  /**
   * Downloads and parses the official CSV data for Canada.
   *
   * @return Returns a vector of Numbers in case of success.
   *         Returns a string containing an error message, if an error occurred.
   */
  fn official_csv_data() -> Result<Vec<Numbers>, String>
  {
    use reqwest::StatusCode;
    use std::io::Read;

    let mut res = match reqwest::blocking::get("https://health-infobase.canada.ca/src/data/covidLive/covid19.csv")
    {
      Ok(responded) => responded,
      Err(e) => return Err(format!("HTTP request failed: {}", e))
    };
    let mut body = String::new();
    if let Err(e) = res.read_to_string(&mut body)
    {
      return Err(format!("Failed to read CSV into string: {}", e));
    }

    if res.status() != StatusCode::OK
    {
      return Err(format!("HTTP request failed with unexpected status code: {}\n\
                        Headers:\n{:#?}\n\
                        Body:\n{}", res.status(), res.headers(), body));
    }

    Canada::parse_official_csv(&body)
  }

  fn parse_official_csv(text: &str) -> Result<Vec<Numbers>, String>
  {
    use csv::Reader;

    let mut reader = Reader::from_reader(text.as_bytes());
    let check = match Canada::check_csv_headers(&mut reader)
    {
      Err(e) => return Err(e),
      Ok(b) => b
    };
    if !check
    {
      return Err("CSV headers do not match!".to_string());
    }

    let mut result: Vec<Numbers> = Vec::new();
    let mut record = csv::StringRecord::new();
    let date_regex = regex::RegexBuilder::new("^([0-9]{2})\\-([0-9]{2})\\-([0-9]{4})$")
                    .build().unwrap();
    loop
    {
      match reader.read_record(&mut record)
      {
        Ok(success) =>
        {
          if !success
          {
            // No more records to read.
            break;
          }
        },
        Err(e) =>
        {
          // Failed to read.
          return Err(format!("Failed to read CSV record! {}", e));
        }
      }

      // If "prname" (second column) is not "Canada", it's the data for one of
      // the provinces and it can be skipped, because we only want data for all
      // of Canada here.
      if record.get(Canada::INDEX_PRNAME).unwrap() != "Canada"
      {
        continue;
      }
      // Date is in fourth column named "date".
      let date = match record.get(Canada::INDEX_DATE)
      {
        Some(s) => s,
        None => continue
      };
      // Date has a format like "31-12-2020", but needs to be inverted to e. g.
      // "2020-12-31".
      if !date_regex.is_match(&date)
      {
        return Err(format!("Error: Date format does not match the DD-MM-YYYY pattern: '{}'.", date));
      }
      let date = format!("{}-{}-{}", &date[6..10], &date[3..5], &date[0..2]);
      // Daily new cases: "numtoday", index 15.
      let cases: i32 = record.get(Canada::INDEX_CASES).unwrap().parse().unwrap_or(-1);
      // Daily new deaths: "numdeathstoday", index 19.
      let deaths: i32 = record.get(Canada::INDEX_DEATHS).unwrap().parse().unwrap_or(-1);
      result.push(Numbers { date, cases, deaths });
    }

    // Sort result by date, because other methods return sorted, too.
    result.sort_unstable_by(|a, b| a.date.cmp(&b.date));
    // Done. :)
    Ok(result)
  }

  /**
   * Checks whether the CSV headers match the expected headers.
   *
   * @param reader    an opened CSV reader
   * @return Returns true, if the headers are correct. Returns false otherwise.
   */
  fn check_csv_headers(reader: &mut csv::Reader<&[u8]>) -> Result<bool, String>
  {
    let headers = match reader.headers()
    {
      Ok(head) => head,
      Err(e) => {
        return Err(format!("Error: Could not read header of CSV: {}", e));
      }
    };
    // Highest required column index is "numdeathstoday", so the CSV needs at
    // least INDEX_DEATHS + 1 columns.
    const REQUIRED_COLUMNS: usize = Canada::INDEX_DEATHS + 1;
    if headers.len() < REQUIRED_COLUMNS || &headers[Canada::INDEX_PRNAME] != "prname"
      || &headers[Canada::INDEX_DATE] != "date" || &headers[Canada::INDEX_CASES] != "numtoday"
      || &headers[Canada::INDEX_DEATHS] != "numdeathstoday"
    {
      eprintln!("Error: CSV headers do not match the expected headers. \
                 Found the following headers: {:?}", headers);
      return Ok(false);
    }
    // Headers match. :)
    Ok(true)
  }
}

impl Collect for Canada
{
  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "CA" // Canada
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    // Note: The JHU numbers seem to be a bit higher than the official numbers.
    // Official data: https://health-infobase.canada.ca/src/data/covidLive/covid19.csv
    let vec = Canada::official_csv_data();
    if vec.is_err() || range == &Range::All
    {
      return vec;
    }
    // Shorten to 30 elements, if necessary.
    let mut vec = vec.unwrap();
    if vec.len() <= 30
    {
      return Ok(vec);
    }
    Ok(vec.drain(vec.len()-30..).collect())
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn official_csv_data()
  {
    let data = Canada::official_csv_data();
    assert!(data.is_ok());
    let data = data.unwrap();
    assert!(data.len() >= 30);
    // Elements should be sorted by date.
    for idx in 1..data.len()
    {
      assert!(data[idx-1].date < data[idx].date)
    }
  }
}
