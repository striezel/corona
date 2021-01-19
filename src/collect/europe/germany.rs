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

use crate::collect::Collect;
use crate::collect::api::Range;
use crate::data::Numbers;
use std::path::{Path, PathBuf};

pub struct Germany
{
}

impl Germany
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Germany
  {
    Germany { }
  }

  /**
   * Gets the numbers for Germany from the Robert Koch Institute.
   *
   * @return Returns the vector of Numbers, if successful.
   *         Returns an error message otherwise.
   */
  fn rki_data() -> Result<Vec<Numbers>, String>
  {
    let xlsx_path = Germany::download_xlsx()?;
    let result = Germany::extract_from_file(&xlsx_path);
    if std::fs::remove_file(&xlsx_path).is_err()
    {
      println!("Info: Could not remove downloaded spreadsheet file {}!",
               xlsx_path.display());
    }
    result
  }

  /**
   * Downloads the spreadsheet with current data from the Robert Koch Institute.
   *
   * @return Returns a PathBuf containing the path of the downloaded file in
   *         case of success. Returns an error message otherwise.
   */
  fn download_xlsx() -> Result<PathBuf, String>
  {
    use reqwest::StatusCode;
    use std::io::Read;
    // Retrieve XLSX with case numbers.
    let mut res = match reqwest::blocking::get("https://www.rki.de/DE/Content/InfAZ/N/Neuartiges_Coronavirus/Daten/Fallzahlen_Kum_Tab.xlsx?__blob=publicationFile")
    {
      Ok(responded) => responded,
      Err(e) => return Err(format!("HTTP request failed: {}", e))
    };
    let mut body: Vec<u8> = Vec::new();
    if let Err(e) = res.read_to_end(&mut body)
    {
      return Err(format!("Failed to read XLSX into string: {}", e));
    }
    if res.status() != StatusCode::OK
    {
      return Err(format!("HTTP request failed with unexpected status code: {}\n\
                        Headers:\n{:#?}\n\
                        Body:\n{:?}", res.status(), res.headers(), body));
    }

    let path = std::env::temp_dir().join("de-dl.xlsx");
    match std::fs::write(&path, &body)
    {
      Ok(()) => Ok(path),
      Err(e) => Err(format!("Error while writing temporary file: {}", e))
    }
  }

  /**
   * Extracts the numbers from a RKI spreadsheet file (.xlsx).
   *
   * @param path   path to the spreadsheet file (.xlsx format)
   * @return Returns a vector containing the extracted numbers in case of
   *         success. Returns an error message otherwise.
   */
  fn extract_from_file(path: &Path) -> Result<Vec<Numbers>, String>
  {
    use calamine::{Reader, open_workbook, Xlsx, DataType};
    use chrono::prelude::*;
    use chrono::{Duration, Utc};

    let mut workbook: Xlsx<_> = match open_workbook(path)
    {
      Ok(x) => x,
      Err(e) => return Err(format!("Failed to open spreadsheet file: {}", e))
    };
    let range = match workbook.worksheet_range("Fälle-Todesfälle-gesamt")
    {
      Some(Ok(range)) => range,
      Some(Err(e)) => return Err(format!("Could not find matching worksheet! {}", e)),
      None => return Err("Could not find matching worksheet!".to_string())
    };
    if !Germany::check_headers(&range)
    {
      return Err("Column headers do not match!".to_string());
    }

    let date_regex = regex::RegexBuilder::new("^([0-9]{2})\\.([0-9]{2})\\.([0-9]{4})$")
      .build().unwrap();
    let excel_epoch_date = Utc.ymd(1899, 12, 30);
    let mut result: Vec<Numbers> = Vec::new();
    for row_idx in 3..range.end().unwrap_or((0, 0)).0
    {
      let date = match range.get_value((row_idx, 0))
      {
        Some(DataType::Float(f)) =>
        {
          let d = excel_epoch_date.checked_add_signed(Duration::days(*f as i64)).unwrap();
          format!("{}-{:0>2}-{:0>2}", d.year(), d.month(), d.day())
        },
        Some(DataType::String(s)) =>
        {
          match date_regex.captures(s)
          {
            // Transform date into proper form.
            Some(cap) => format!("{}-{}-{}", cap[3].to_string(), cap[2].to_string(), cap[1].to_string()),
            _ => return Err(format!("'{}' is not a valid date format!", s))
          }
        },
        _ => continue // Not a valid / recoverable date.
      };
      let cases = match range.get_value((row_idx, 3))
      {
        Some(DataType::Float(f)) => *f as i32,
        Some(DataType::Empty) => if date == "2020-02-25" { 16 } else { 0 },
        Some(DataType::Int(i)) => *i as i32,
        _ => continue
      };
      let deaths = match range.get_value((row_idx, 5))
      {
        Some(DataType::Float(f)) => *f as i32,
        Some(DataType::Empty) => 0,
        Some(DataType::Int(i)) => *i as i32,
        _ => continue
      };
      result.push(Numbers { date, cases, deaths });
    }

    result.sort_unstable_by(|a, b| a.date.cmp(&b.date));
    Ok(result)
  }

  /**
   * Checks the headers of a worksheet to match the expectations.
   *
   * @param range  range covering the current worksheet
   * @return Returns whether the headers match the expected headers.
   */
  fn check_headers(range: &calamine::Range<calamine::DataType>) -> bool
  {
    use calamine::DataType;

    if range.start() != Some((1, 0))
    {
      eprintln!("Worksheet layout changed, start does not fit!");
      return false;
    }
    // Check some known cell values.
    range.get_value((2, 0)) == Some(&DataType::String("Berichtsdatum".to_string()))
      && range.get_value((2, 3)) == Some(&DataType::String("Differenz Vortag Fälle".to_string()))
      && range.get_value((2, 5)) == Some(&DataType::String("Differenz Vortag Todesfälle".to_string()))
  }
}

impl Collect for Germany
{
  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "DE" // Germany
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    let vec = Germany::rki_data();
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
  fn has_data()
  {
    let data = Germany::new().collect(&Range::Recent);
    assert!(data.is_ok());
    let data = data.unwrap();
    assert!(!data.is_empty());
    // Elements should be sorted by date.
    for idx in 1..data.len()
    {
      assert!(data[idx-1].date < data[idx].date)
    }
  }
}
