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

use crate::data::Numbers;

/**
 * Request historical API of disease.sh for a single country.
 *
 * @param  geo_id  geo id (i. e. two letter country code) of a country
 * @return Returns a vector of Numbers containing the new daily cases.
 *         If an error occurred, an Err containing the error message is
 *         returned.
 */
pub fn request_historical_api(geo_id: &str) -> Result<Vec<Numbers>, String>
{
  use reqwest::StatusCode;
  use serde_json::Value;
  use std::convert::TryFrom;
  use std::collections::HashMap;
  use std::io::Read;

  let url = format!("https://corona.lmao.ninja/v3/covid-19/historical/{}", geo_id);
  let mut res = match reqwest::blocking::get(&url)
  {
    Ok(responded) => responded,
    Err(e) => return Err(format!("API request failed: {}", e))
  };
  let mut body = String::new();
  if let Err(e) = res.read_to_string(&mut body)
  {
    return Err(format!("Failed to read API response: {}", e));
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
    Err(e) => return Err(format!("Failed to deserialize JSON from API: {}", e))
  };
  let timeline = match json.get("timeline")
  {
    Some(Value::Object(map)) => map,
    None => return Err(String::from("JSON from API does not contain timeline!")),
    Some(_) => return Err(String::from("JSON from API contains timeline, but it is not an object!"))
  };
  let cases = match timeline.get("cases")
  {
    Some(Value::Object(map)) => map,
    None => return Err(String::from("JSON from API does not contain cases in timeline!")),
    Some(_) => return Err(String::from("JSON from API contains cases, but it is not an object!"))
  };
  let deaths = match timeline.get("deaths")
  {
    Some(Value::Object(map)) => map,
    None => return Err(String::from("JSON from API does not contain deaths in timeline!")),
    Some(_) => return Err(String::from("JSON from API contains deaths, but it is not an object!"))
  };
  // Date is something like e. g. "12/31/20" for 31st December 2020 or
  // "1/1/21" for 1st January 2021.
  let date_exp = regex::RegexBuilder::new("^([0-9]+)/([0-9]+)/([0-9]{2})$")
                .build()
                .unwrap();
  let mut numbers: HashMap<String, Numbers> = HashMap::new();
  for (date, num) in cases.iter()
  {
    let iso_date = match date_exp.captures(&date)
    {
      None => {
        println!("Found date '{}' which does not match the pattern. Skipping it.", date);
        continue;
      },
      Some(cap) => {
        format!("20{}-{:0>2}-{:0>2}", cap[3].to_string(), cap[1].to_string(), cap[2].to_string())
      }
    };
    let infections = match num.as_i64()
    {
      None => return Err(format!("JSON from API contains a non-number ('{}') as number of infections!", num)),
      Some(cases) => cases
    };
    let infections: i32 = match i32::try_from(infections)
    {
      Err(_) => return Err("Number of daily infections does not fit into i32. This pandemic is really out of hand!".to_string()),
      Ok(i) => i
    };
    numbers.insert(iso_date.clone(), Numbers { date: iso_date, cases: infections, deaths: -2147483648 });
  }
  for (date, num) in deaths.iter()
  {
    let iso_date = match date_exp.captures(&date)
    {
      None => {
        println!("Found date '{}' which does not match the pattern. Skipping it.", date);
        continue;
      },
      Some(cap) => {
        format!("20{}-{:0>2}-{:0>2}", cap[3].to_string(), cap[1].to_string(), cap[2].to_string())
      }
    };
    let deaths = match num.as_i64()
    {
      None => return Err(format!("JSON from API contains a non-number ('{}') as number of deaths!", num)),
      Some(cases) => cases
    };
    let deaths: i32 = match i32::try_from(deaths)
    {
      Err(_) => return Err("Number of daily deaths does not fit into i32. This pandemic is REALLY out of hand!".to_string()),
      Ok(i) => i
    };
    let mut found = match numbers.get_mut(&iso_date)
    {
      None => return Err(format!("Date '{}' is not present in both timelines!", iso_date)),
      Some(x) => x
    };
    found.deaths = deaths;
    // Since HashMap::get_mut() returns a reference, the value is changed in place
    // and does not need to be updated by another HashMap::insert() call.
  }
  // Get it out of the map ...
  let mut numbers: Vec<Numbers> = numbers.values().cloned().collect();
  // ... and sort it by date, because it was not sorted in the map.
  numbers.sort_unstable_by(|a, b| a.date.cmp(&b.date));
  // Rebuild with daily values calculated by differences.
  let mut numbers_diff = Vec::new();
  for idx  in 1..numbers.len()
  {
    numbers_diff.push(Numbers {
      date: numbers[idx].date.clone(),
      cases: numbers[idx].cases - numbers[idx-1].cases,
      deaths: numbers[idx].deaths - numbers[idx-1].deaths
    });
  }

  Ok(numbers_diff)
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn historical_api()
  {
    let numbers = request_historical_api("ES");
    assert!(numbers.is_ok());
    let numbers = numbers.unwrap();
    assert!(numbers.len() > 0);
    for idx in 1..numbers.len()
    {
      assert!(numbers[idx-1].date < numbers[idx].date)
    }
  }
}
