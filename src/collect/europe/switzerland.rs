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
use crate::collect::{Collect, JsonCache};
use crate::data::Country;
use crate::data::Numbers;

pub struct Switzerland
{
}

struct Urls
{
  cases_url: String,
  deaths_url: String
}

struct CsvContent
{
  cases_csv: String,
  deaths_csv: String
}

impl Switzerland
{
  /**
   * Returns a new instance.
   */
  pub fn new() -> Switzerland
  {
    Switzerland { }
  }

  /**
   * Downloads and parses the official CSV data for Switzerland.
   *
   * @param  geo_region  abbreviation for the canton, or "CH" for all of Switzerland
   * @return Returns a vector of Numbers in case of success.
   *         Returns a string containing an error message, if an error occurred.
   */
  pub fn official_csv_data(geo_region: &str) -> Result<Vec<Numbers>, String>
  {
    let urls = Switzerland::get_official_csv_data_urls()?;
    let csv_content = Switzerland::get_official_csv_content(&urls)?;
    Switzerland::parse_csv_content(&csv_content, geo_region)
  }

  /**
   * Gets the URLs of the official CSV data for Switzerland.
   *
   * @return Returns a Urls struct containing the URLs in case of success.
   *         Returns a string containing an error message, if an error occurred.
   */
  fn get_official_csv_data_urls() -> Result<Urls, String>
  {
    use reqwest::StatusCode;
    use serde_json::Value;
    use std::io::Read;

    let mut res = match reqwest::blocking::get("https://www.covid19.admin.ch/api/data/context")
    {
      Ok(responded) => responded,
      Err(e) => return Err(format!("HTTP request failed: {}", e))
    };
    let mut body = String::new();
    if let Err(e) = res.read_to_string(&mut body)
    {
      return Err(format!("Failed to read JSON into string: {}", e));
    }

    if res.status() != StatusCode::OK
    {
      return Err(format!("HTTP request failed with unexpected status code: {}\n\
                        Headers:\n{:#?}\n\
                        Body:\n{}", res.status(), res.headers(), body));
    }
    let json: Value = match serde_json::from_str(&body)
    {
      Ok(v) => v,
      Err(e) =>
      {
        return Err(format!(
          "Failed to deserialize JSON from covid19.admin.ch/api/context: {}",
          e
        ))
      }
    };
    let json = match json.get("sources")
    {
      Some(Value::Object(map)) => map,
      None => return Err(String::from("JSON from API does not contain element 'sources'!")),
      Some(_) => return Err(String::from("JSON from API contains sources, but it is not an object!"))
    };
    let json = match json.get("individual")
    {
      Some(Value::Object(map)) => map,
      None => return Err(String::from("JSON from API does not contain element 'individual'!")),
      Some(_) => return Err(String::from("JSON from API contains individual, but it is not an object!"))
    };
    let json = match json.get("csv")
    {
      Some(Value::Object(map)) => map,
      None => return Err(String::from("JSON from API does not contain element 'csv'!")),
      Some(_) => return Err(String::from("JSON from API contains csv, but it is not an object!"))
    };
    let json = match json.get("daily")
    {
      Some(Value::Object(map)) => map,
      None => return Err(String::from("JSON from API does not contain element 'daily'!")),
      Some(_) => return Err(String::from("JSON from API contains daily, but it is not an object!"))
    };
    let cases = match json.get("cases")
    {
      Some(Value::String(s)) => s,
      None => return Err(String::from("JSON from API does not contain element 'cases'!")),
      Some(_) => return Err(String::from("JSON from API contains element 'cases', but it is not a string!"))
    };
    let death = match json.get("death")
    {
      Some(Value::String(s)) => s,
      None => return Err(String::from("JSON from API does not contain element 'death'!")),
      Some(_) => return Err(String::from("JSON from API contains element 'death', but it is not a string!"))
    };

    Ok(Urls {
      cases_url: cases.clone(),
      deaths_url: death.clone()
    })
  }

  /**
   * Gets the CSV data for Switzerland as plain text.
   *
   * @param  urls    the URLs where the CSV files can be downloaded
   * @return Returns a CsvContent struct containing the plain text CSV in case of success.
   *         Returns a string containing an error message, if an error occurred.
   */
  fn get_official_csv_content(urls: &Urls) -> Result<CsvContent, String>
  {
    use reqwest::StatusCode;
    use std::io::Read;
    // Retrieve CSV with case numbers.
    let mut res = match reqwest::blocking::get(&urls.cases_url)
    {
      Ok(responded) => responded,
      Err(e) => return Err(format!("HTTP request failed: {}", e))
    };
    let mut body_cases = String::new();
    if let Err(e) = res.read_to_string(&mut body_cases)
    {
      return Err(format!("Failed to read CSV into string: {}", e));
    }
    if res.status() != StatusCode::OK
    {
      return Err(format!("HTTP request failed with unexpected status code: {}\n\
                        Headers:\n{:#?}\n\
                        Body:\n{}", res.status(), res.headers(), body_cases));
    }
    // Retrieve CSV with numbers of deaths.
    let mut res = match reqwest::blocking::get(&urls.deaths_url)
    {
      Ok(responded) => responded,
      Err(e) => return Err(format!("HTTP request failed: {}", e))
    };
    let mut body_deaths = String::new();
    if let Err(e) = res.read_to_string(&mut body_deaths)
    {
      return Err(format!("Failed to read CSV into string: {}", e));
    }
    if res.status() != StatusCode::OK
    {
      return Err(format!("HTTP request failed with unexpected status code: {}\n\
                        Headers:\n{:#?}\n\
                        Body:\n{}", res.status(), res.headers(), body_deaths));
    }

    Ok(CsvContent {
      cases_csv: body_cases,
      deaths_csv: body_deaths
    })
  }

  fn parse_csv_content(csv_content: &CsvContent, geo_region: &str) -> Result<Vec<Numbers>, String>
  {
    use csv::Reader;
    // Parse CSV with cases.
    let mut reader = Reader::from_reader(csv_content.cases_csv.as_bytes());
    let check = match Switzerland::check_csv_headers(&mut reader)
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
    let date_regex = regex::RegexBuilder::new("^([0-9]{4})\\-([0-9]{2})\\-([0-9]{2})$")
      .build()
      .unwrap();
    // potential endless loop
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

      // If "geoRegion" (first column) does not match (i. e. it's not "CH"),
      // it's the data for one of the other cantons (provinces) and it can be
      // skipped, because we only want data for the correct region of
      // Switzerland here.
      if record.get(0).unwrap() != geo_region
      {
        continue;
      }
      // Date is in second column named "datum".
      let date = match record.get(1)
      {
        Some(s) => s,
        None => continue
      };
      // Date has a format like "2020-12-31", i. e. it fits already.
      if !date_regex.is_match(&date)
      {
        return Err(format!(
          "Error: Date format does not match the YYYY-MM-DD pattern: '{}'.",
          date
        ));
      }
      // Daily new cases: "entries", idx 2.
      let cases: i32 = record.get(2).unwrap().parse().unwrap_or(-1);
      result.push(Numbers { date: date.to_string(), cases, deaths: 0 });
    }

    // Parse CSV with number of deaths.
    let mut reader = Reader::from_reader(csv_content.deaths_csv.as_bytes());
    let check = match Switzerland::check_csv_headers(&mut reader)
    {
      Err(e) => return Err(e),
      Ok(b) => b
    };
    if !check
    {
      return Err("CSV headers do not match!".to_string());
    }
    let mut may_need_sorting = false;
    // potential endless loop # 2
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

      // If "geoRegion" (first column) is not "CH", it's the data for one of
      // the cantons (provinces) and it can be skipped, because we only want
      // data for all of Switzerland here.
      if record.get(0).unwrap() != "CH"
      {
        continue;
      }
      // Date is in second column named "datum".
      let date = match record.get(1)
      {
        Some(s) => s,
        None => continue
      };
      // Date has a format like "2020-12-31", i. e. it fits already.
      if !date_regex.is_match(&date)
      {
        return Err(format!(
          "Error: Date format does not match the YYYY-MM-DD pattern: '{}'.",
          date
        ));
      }
      // Daily new deaths: "entries", idx 2.
      let deaths: i32 = record.get(2).unwrap().parse().unwrap_or(-1);
      let pos = result.iter().position(|x| x.date == date);
      if let Some(idx) = pos
      {
        result[idx].deaths = deaths;
      }
      else
      {
        result.push(Numbers { date: date.to_string(), cases: 0, deaths });
        // Vector may need to be sorted, because we do know whether it is
        // still sorted after the push().
        may_need_sorting = true;
      }
    }

    if may_need_sorting
    {
      result.sort_unstable_by(|a, b| a.date.cmp(&b.date));
    }

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
      Err(e) =>
      {
        return Err(format!("Error: Could not read header of CSV: {}", e));
      }
    };
    let expected_headers = vec!["geoRegion", "datum", "entries"];
    if headers.len() < 3
    {
      eprintln!("Error: CSV headers do not have enough columns. \
                 Found the following headers: {:?}", headers);
      return Ok(false);
    }
    let actual_headers = vec![
      headers.get(0).unwrap_or_default(),
      headers.get(1).unwrap_or_default(),
      headers.get(2).unwrap_or_default(),
    ];
    if actual_headers != expected_headers
    {
      eprintln!("Error: CSV headers do not match the expected headers. \
                 Found the following headers: {:?}", headers);
      return Ok(false);
    }
    // Headers match. :)
    Ok(true)
  }
}

impl Collect for Switzerland
{
  /**
   * Returns the country associated with the Collect trait implementation.
   */
  fn country(&self) -> Country
  {
    Country {
      country_id: 187,
      name: "Switzerland".to_string(),
      population: 8544527,
      geo_id: "CH".to_string(),
      country_code: "CHE".to_string(),
      continent: "Europe".to_string()
    }
  }

  /**
   * Returns the geo id (two-letter code) of the country for which the data
   * is collected.
   */
  fn geo_id(&self) -> &str
  {
    "CH" // Switzerland
  }

  fn collect(&self, range: &Range) -> Result<Vec<Numbers>, String>
  {
    let vec = Switzerland::official_csv_data("CH");
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
    Ok(vec.drain(vec.len() - 30..).collect())
  }

  /**
   * Collects new data of the specified time range, using the cache.
   * If there is no cached data, it may fallback to non-cached data collection.
   *
   * @param  range   the data range to collect
   * @param  cache   the cached JSON data
   * @return Returns a vector containing new daily numbers for cases + deaths.
   *         Returns an Err(), if no data could be retrieved.
   */
  fn collect_cached(&self, range: &Range, _cache: &JsonCache) -> Result<Vec<Numbers>, String>
  {
    // Data comes from Swiss CSV data, so fall back to collect().
    // TODO: Cache Swiss CSV once it is downloaded. This would allow reuse for
    //       cached collect of Liechtenstein and save us three(?) HTTP requests.
    self.collect(range)
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn official_csv_data()
  {
    let data = Switzerland::official_csv_data("CH");
    assert!(data.is_ok());
    let data = data.unwrap();
    assert!(data.len() >= 30);
    // Elements should be sorted by date.
    for idx in 1..data.len()
    {
      assert!(data[idx - 1].date < data[idx].date)
    }
  }
}
