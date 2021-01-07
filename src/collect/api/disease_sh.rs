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
use crate::data::Numbers;
use serde_json::Value;

/**
 * Request historical API of disease.sh for a single country.
 *
 * @param  geo_id  geo id (i. e. two letter country code) of a country
 * @param  range   whether to collect recent or all data
 * @return Returns a vector of Numbers containing the new daily cases.
 *         If an error occurred, an Err containing the error message is
 *         returned.
 */
pub fn request_historical_api(geo_id: &str, range: &Range) -> Result<Vec<Numbers>, String>
{
  let url = construct_historical_api_url(geo_id, "", &range);
  let json = perform_api_request(&url)?;
  parse_json_timeline(&json)
}

/**
 * Request historical API of disease.sh for a single province of a country.
 *
 * @param  geo_id  geo id (i. e. two letter country code) of a country
 * @param  province  name of the province as seen in the API response for the country
 * @param  range   whether to collect recent or all data
 * @return Returns a vector of Numbers containing the new daily cases.
 *         If an error occurred, an Err containing the error message is
 *         returned.
 */
pub fn request_historical_api_province(geo_id: &str, province: &str, range: &Range) -> Result<Vec<Numbers>, String>
{
  let url = construct_historical_api_url(geo_id, province, &range);
  let json = perform_api_request(&url)?;
  parse_json_timeline(&json)
}

/**
 * Request historical API of disease.sh for the 1st province (out of multiple) of a country.
 *
 * @param  geo_id  geo id (i. e. two letter country code) of a country
 * @param  provinces  names of the province as seen in the API response for
 *                    the country, separated by '|' (pipe character)
 * @param  range   whether to collect recent or all data
 * @return Returns a vector of Numbers containing the new daily cases for the first province.
 *         No guarantee is given about the order of the provinces, so the first province out
 *         of many could be any of the given provinces.
 *         If an error occurred, an Err containing the error message is
 *         returned.
 */
pub fn request_historical_api_first_of_multiple_provinces(geo_id: &str, provinces: &str, range: &Range) -> Result<Vec<Numbers>, String>
{
  let url = construct_historical_api_url(geo_id, provinces, &range);
  let json = perform_api_request(&url)?;
  let vec: Vec<Value> = match json
  {
    Value::Array(vector) => vector,
    _ => return Err("Error: Found invalid JSON format in request for multiple provinces.".to_string())
  };
  if vec.is_empty()
  {
    return Err("Error: Found empty JSON array in request for multiple provinces.".to_string())
  }
  parse_json_timeline(&vec[0])
}

/**
 * Request historical API of disease.sh for counties in the USA.
 *
 * @param  county  name of the county
 * @param  range   whether to collect recent or all data
 * @return Returns a vector of Numbers containing the new daily cases.
 *         If an error occurred, an Err containing the error message is
 *         returned.
 */
pub fn request_historical_api_usa_counties(county: &str, range: &Range) -> Result<Vec<Numbers>, String>
{
  let url = construct_historical_api_url_usa_counties(county, &range);
  let json = perform_api_request(&url)?;
  // The API for US counties returns an array, where each element within is the
  // data for a province within the area. To get the total numbers, those have
  // to be added up.
  let vec: Vec<Value> = match json
  {
    Value::Array(vector) => vector,
    _ => return Err("Error: Found invalid JSON format in request for USA counties.".to_string())
  };

  let mut may_need_sorting = false;
  let mut numbers: Vec<Numbers> = Vec::new();
  for elem in vec.iter()
  {
    let partial_numbers = parse_json_timeline(&elem)?;
    if numbers.is_empty()
    {
      numbers = partial_numbers;
    }
    else
    {
      // Add it up to existing numbers.
      for num in partial_numbers.iter()
      {
        let pos = numbers.iter().position(|x| x.date == num.date);
        if let Some(idx) = pos
        {
          numbers[idx].cases = numbers[idx].cases + num.cases;
          numbers[idx].deaths = numbers[idx].deaths + num.deaths;
        }
        else
        {
          numbers.push(Numbers {
            date: num.date.clone(),
            cases: num.cases,
            deaths: num.deaths });
          // Vector may need to be sorted, because we do know whether it is
          // still sorted after the push().
          may_need_sorting = true;
        }
      }
    }
  }

  if may_need_sorting
  {
    numbers.sort_unstable_by(|a, b| a.date.cmp(&b.date));
  }
  Ok(numbers)
}

/**
 * Performs a request to the API of disease.sh for a given URL and deserializes the JSOn.
 *
 * @param  url   URL of the endpoint
 * @return Returns a deserialized JSON value in case of success.
 *         If an error occurred, an Err containing the error message is
 *         returned.
 */
fn perform_api_request(url: &str) -> Result<serde_json::Value, String>
{
  use reqwest::StatusCode;
  use std::io::Read;

  let mut res = match reqwest::blocking::get(url)
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
  // Hint: The return cannot be moved out of the match expression, because the
  // JSON parsing fails in that case - probably because type deduction is needed
  // here by serde_json to properly deserialize the body.
  let json: Value = match serde_json::from_str(&body)
  {
    Ok(v) => v,
    Err(e) => return Err(format!("Failed to deserialize JSON from API: {}", e))
  };
  Ok(json)
}

/**
 * Constructs the URL for a request to the historical API of disease.sh.
 *
 * @param  geo_id  geo id (i. e. two letter country code) of a country
 * @param  province  name of the province as seen in the API response for the
           country; if empty, the numbers for the whole country are requested
 * @param  range   whether to collect recent or all data
 * @return Returns a string containing the URL.
 */
fn construct_historical_api_url(geo_id: &str, province: &str, range: &Range) -> String
{
  if province.is_empty()
  {
    match range
    {
      Range::Recent => format!("https://corona.lmao.ninja/v3/covid-19/historical/{}", geo_id),
      Range::All => format!("https://corona.lmao.ninja/v3/covid-19/historical/{}?lastdays=all", geo_id)
    }
  }
  else
  {
    match range
    {
      Range::Recent => format!("https://corona.lmao.ninja/v3/covid-19/historical/{}/{}", geo_id, province),
      Range::All => format!("https://corona.lmao.ninja/v3/covid-19/historical/{}/{}?lastdays=all", geo_id, province)
    }
  }
}

/**
 * Constructs the URL for a request to the historical API of disease.sh for an county of the USA.
 *
 * @param  county  name of the county as seen in the API
 * @param  range   whether to collect recent or all data
 * @return Returns a string containing the URL.
 */
fn construct_historical_api_url_usa_counties(county: &str, range: &Range) -> String
{
  match range
  {
    Range::Recent => format!("https://corona.lmao.ninja/v3/covid-19/historical/usacounties/{}", county),
    Range::All => format!("https://corona.lmao.ninja/v3/covid-19/historical/usacounties/{}?lastdays=all", county)
  }
}

/**
 * Parses JSON from historical API of disease.sh.
 *
 * @param  json  the parsed JSON object from serde_json
 * @return Returns a vector of Numbers containing the new daily cases.
 *         If an error occurred, an Err containing the error message is
 *         returned.
 */
fn parse_json_timeline(json: &Value) -> Result<Vec<Numbers>, String>
{
  use std::collections::HashMap;
  use std::convert::TryFrom;

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

/**
 * Shifts data in the vector to the next day, assuming the vector is sorted.
 *
 * @param numbers  the vector with the numbers to shift
 * @return Returns a vector that is one element shorter than `numbers`, or, if
 *         that is empty, an empty vector. In a non-empty vector the cases and
 *         deaths are the same, but the date value is moved to the next day.
 */
pub fn shift_one_day_later(numbers: &[Numbers]) -> Vec<Numbers>
{
  if numbers.is_empty()
  {
    return vec![];
  }
  let mut new_numbers = Vec::with_capacity(numbers.len() - 1);
  for idx in 0..numbers.len() - 1
  {
    new_numbers.push(Numbers {
      date: numbers[idx + 1].date.clone(),
      cases: numbers[idx].cases,
      deaths: numbers[idx].deaths
    });
  }

  new_numbers
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn historical_api()
  {
    let numbers = request_historical_api("ES", &Range::Recent);
    assert!(numbers.is_ok());
    let numbers = numbers.unwrap();
    assert!(numbers.len() > 0);
    for idx in 1..numbers.len()
    {
      assert!(numbers[idx-1].date < numbers[idx].date)
    }

    let all_numbers = request_historical_api("ES", &Range::All);
    assert!(all_numbers.is_ok());
    let all_numbers = all_numbers.unwrap();
    assert!(all_numbers.len() > 0);
    for idx in 1..all_numbers.len()
    {
      assert!(all_numbers[idx-1].date < all_numbers[idx].date)
    }

    // All data should have more entries than recent data.
    assert!(all_numbers.len() > numbers.len());
  }

  #[test]
  fn historical_api_province()
  {
    let numbers = request_historical_api_province("DK", "mainland", &Range::Recent);
    assert!(numbers.is_ok());
    let numbers = numbers.unwrap();
    assert!(numbers.len() > 0);
    for idx in 1..numbers.len()
    {
      assert!(numbers[idx-1].date < numbers[idx].date)
    }

    let all_numbers = request_historical_api_province("DK", "mainland", &Range::All);
    assert!(all_numbers.is_ok());
    let all_numbers = all_numbers.unwrap();
    assert!(all_numbers.len() > 0);
    for idx in 1..all_numbers.len()
    {
      assert!(all_numbers[idx-1].date < all_numbers[idx].date)
    }

    // All data should have more entries than recent data.
    assert!(all_numbers.len() > numbers.len());
  }

  #[test]
  fn historical_api_usa_counties()
  {
    let numbers = request_historical_api_usa_counties("guam", &Range::Recent);
    assert!(numbers.is_ok());
    let numbers = numbers.unwrap();
    assert!(numbers.len() > 0);
    for idx in 1..numbers.len()
    {
      assert!(numbers[idx-1].date < numbers[idx].date)
    }

    let all_numbers = request_historical_api_usa_counties("guam", &Range::All);
    assert!(all_numbers.is_ok());
    let all_numbers = all_numbers.unwrap();
    assert!(all_numbers.len() > 0);
    for idx in 1..all_numbers.len()
    {
      assert!(all_numbers[idx-1].date < all_numbers[idx].date)
    }

    // All data should have more entries than recent data.
    assert!(all_numbers.len() > numbers.len());
  }

  #[test]
  fn shift_one_day()
  {
    let old: Vec<Numbers> = vec![
       Numbers { date: "2020-01-01".to_string(), cases: 12, deaths: 0},
       Numbers { date: "2020-01-02".to_string(), cases: 17, deaths: 1},
       Numbers { date: "2020-01-03".to_string(), cases: 28, deaths: 2}
    ];
    let shifted = shift_one_day_later(&old);
    assert_eq!(2, shifted.len());
    assert_eq!(shifted[0].date, old[1].date, );
    assert_eq!(shifted[0].cases, old[0].cases);
    assert_eq!(shifted[0].deaths, old[0].deaths);
    assert_eq!(shifted[1].date, old[2].date);
    assert_eq!(shifted[1].cases, old[1].cases,);
    assert_eq!(shifted[1].deaths, old[1].deaths);
  }
}
