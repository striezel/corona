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
#[derive(Clone)]
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

/**
 * Calculates the 14-day incidence for a slice of Numbers that are pre-sorted by
 * date in ascending order.
 *
 * @param number   slice of numbers, has to be sorted by date in ascending order
 *                 without any gaps
 * @param population  number of inhabitants in the country
 * @return Returns the numbers with 14-day incidence calculated.
 */
pub fn calculate_incidence(numbers: &[Numbers], population: &i32) -> Vec<NumbersAndIncidence>
{
  let len = numbers.len();
  let mut result: Vec<NumbersAndIncidence> = Vec::with_capacity(len);
  for elem in numbers.iter().take(13)
  {
    result.push(NumbersAndIncidence {
      date: elem.date.clone(),
      cases: elem.cases,
      deaths: elem.deaths,
      incidence_14d: None,
    });
  }
  // If there is not enough data to ever get to 14 days, then there can be no
  // 14-day incidence.
  if len <= 13
  {
    return result;
  }
  // If there is no valid population number, no incidence can be calculated.
  if population <= &0
  {
    for elem in numbers.iter().skip(13)
    {
      result.push(NumbersAndIncidence {
        date: elem.date.clone(),
        cases: elem.cases,
        deaths: elem.deaths,
        incidence_14d: None,
      });
    }
    return result;
  }

  let mut sum: i32 = 0;
  for elem in numbers.iter().take(14)
  {
    sum += elem.cases;
  }
  result.push(NumbersAndIncidence {
    date: numbers[13].date.clone(),
    cases: numbers[13].cases,
    deaths: numbers[13].deaths,
    incidence_14d: Some(sum as f64 * 100_000.0 / *population as f64),
  });

  for idx in 14..len
  {
    // Recalculate sum.
    sum = sum + numbers[idx].cases - numbers[idx - 14].cases;
    result.push(NumbersAndIncidence {
      date: numbers[idx].date.clone(),
      cases: numbers[idx].cases,
      deaths: numbers[idx].deaths,
      incidence_14d: Some(sum as f64 * 100_000.0 / *population as f64),
    });
  }

  result
}

/**
 * Adds missing dates to a vector of Numbers that are pre-sorted by date in
 * ascending order.
 *
 * @param number   vector of numbers, has to be sorted by date in ascending order
 *                 with gaps not larger than 100 days
 * @return Returns OK(()), if gaps could be filled.
 *         Returns an Err() containing an error message otherwise.
 */
pub fn fill_missing_dates(numbers: &mut Vec<Numbers>) -> Result<(), String>
{
  use chrono::NaiveDate;

  let len = numbers.len();
  if len <= 1
  {
    return Ok(());
  }

  let mut needs_sort = false;
  let mut previous = match NaiveDate::parse_from_str(&numbers[0].date, "%Y-%m-%d")
  {
    Ok(d) => d,
    Err(_e) => return Err(format!("Could not parse date '{}'!", &numbers[0].date))
  };
  for idx in 1..len
  {
    let current = match NaiveDate::parse_from_str(&numbers[idx].date, "%Y-%m-%d")
    {
      Ok(d) => d,
      Err(_e) => return Err(format!("Could not parse date '{}'!", &numbers[idx].date))
    };
    previous = previous.succ();
    let mut inserts: usize = 0; // counter guard against prolonged loops
    while previous != current && inserts <= 100
    {
      numbers.push(Numbers { date: previous.format("%Y-%m-%d").to_string(), cases: 0, deaths: 0 });
      previous = previous.succ();
      inserts += 1;
      needs_sort = true;
    }
    if inserts > 100
    {
      return Err("Dates are far from sorted or contiguous, because \
          at least 100 dates are missing between two consecutive dates!".to_string())
    }
    previous = current;
  }

  // Do we need to sort here?
  if needs_sort
  {
    numbers.sort_unstable_by(|a, b| a.date.cmp(&b.date));
  }

  // Done.
  Ok(())
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn calculate_incidence_few_elements()
  {
    let mut numbers = Vec::new();
    for i in 1..10
    {
      numbers.push(Numbers {
        date: format!("2020-01-{:0>2}", i),
        cases: i,
        deaths: 0
      });
    }
    let incidence = calculate_incidence(&numbers, &123_456_789);
    assert_eq!(numbers.len(), incidence.len());
    for idx in 0..numbers.len()
    {
      // Numbers should be equal.
      assert_eq!(numbers[idx].date, incidence[idx].date);
      assert_eq!(numbers[idx].cases, incidence[idx].cases);
      assert_eq!(numbers[idx].deaths, incidence[idx].deaths);
      // Incidence values should not be set.
      assert!(incidence[idx].incidence_14d.is_none());
    }
  }

  #[test]
  fn calculate_incidence_14_elements()
  {
    let numbers = vec![
      Numbers { date: "2020-12-01".to_string(), cases: 272, deaths: 11 },
      Numbers { date: "2020-12-02".to_string(), cases: 400, deaths: 48 },
      Numbers { date: "2020-12-03".to_string(), cases: 202, deaths: 19 },
      Numbers { date: "2020-12-04".to_string(), cases: 119, deaths: 5 },
      Numbers { date: "2020-12-05".to_string(), cases: 235, deaths: 18 },
      Numbers { date: "2020-12-06".to_string(), cases: 234, deaths: 10 },
      Numbers { date: "2020-12-07".to_string(), cases: 210, deaths: 26 },
      Numbers { date: "2020-12-08".to_string(), cases: 200, deaths: 6 },
      Numbers { date: "2020-12-09".to_string(), cases: 135, deaths: 13 },
      Numbers { date: "2020-12-10".to_string(), cases: 202, deaths: 16 },
      Numbers { date: "2020-12-11".to_string(), cases:  63, deaths: 10 },
      Numbers { date: "2020-12-12".to_string(), cases: 113, deaths: 11 },
      Numbers { date: "2020-12-13".to_string(), cases: 298, deaths: 9 },
      Numbers { date: "2020-12-14".to_string(), cases: 746, deaths: 6 },
    ];

    let incidence = calculate_incidence(&numbers, &38_041_757);
    assert_eq!(numbers.len(), incidence.len());
    for idx in 0..numbers.len()
    {
      // Numbers should be equal.
      assert_eq!(numbers[idx].date, incidence[idx].date);
      assert_eq!(numbers[idx].cases, incidence[idx].cases);
      assert_eq!(numbers[idx].deaths, incidence[idx].deaths);
    }
    // Check incidence.
    for idx in 0..13
    {
      // Incidence values should not be set.
      assert!(incidence[idx].incidence_14d.is_none());
    }
    // Incidence for last entry should be set.
    assert!(incidence[13].incidence_14d.is_some());
    // Incidence should be roughly 9.01377925.
    assert!(incidence[13].incidence_14d.unwrap() > 9.013779);
    assert!(incidence[13].incidence_14d.unwrap() < 9.013780);
  }

  #[test]
  fn calculate_incidence_more_than_14_elements()
  {
    let numbers = vec![
      Numbers { date: "2020-10-31".to_string(), cases: 19059, deaths: 103 },
      Numbers { date: "2020-11-01".to_string(), cases: 14177, deaths: 29 },
      Numbers { date: "2020-11-02".to_string(), cases: 12097, deaths: 49 },
      Numbers { date: "2020-11-03".to_string(), cases: 15352, deaths: 131 },
      Numbers { date: "2020-11-04".to_string(), cases: 17214, deaths: 151 },
      Numbers { date: "2020-11-05".to_string(), cases: 19990, deaths: 118 },
      Numbers { date: "2020-11-06".to_string(), cases: 21506, deaths: 166 },
      Numbers { date: "2020-11-07".to_string(), cases: 23399, deaths: 130 },
      Numbers { date: "2020-11-08".to_string(), cases: 16017, deaths: 63 },
      Numbers { date: "2020-11-09".to_string(), cases: 13363, deaths: 63 },
      Numbers { date: "2020-11-10".to_string(), cases: 15332, deaths: 154 },
      Numbers { date: "2020-11-11".to_string(), cases: 18487, deaths: 261 },
      Numbers { date: "2020-11-12".to_string(), cases: 21866, deaths: 215 },
      Numbers { date: "2020-11-13".to_string(), cases: 23542, deaths: 218 },
      Numbers { date: "2020-11-14".to_string(), cases: 22461, deaths: 178 },
      Numbers { date: "2020-11-15".to_string(), cases: 16947, deaths: 107 },
      Numbers { date: "2020-11-16".to_string(), cases: 10824, deaths: 62 },
      Numbers { date: "2020-11-17".to_string(), cases: 14419, deaths: 267 },
      Numbers { date: "2020-11-18".to_string(), cases: 17561, deaths: 305 },
      Numbers { date: "2020-11-19".to_string(), cases: 22609, deaths: 251 },
      Numbers { date: "2020-11-20".to_string(), cases: 23648, deaths: 260 },
      Numbers { date: "2020-11-21".to_string(), cases: 22964, deaths: 254 },
      Numbers { date: "2020-11-22".to_string(), cases: 15741, deaths: 138 },
      Numbers { date: "2020-11-23".to_string(), cases: 10864, deaths: 90 },
      Numbers { date: "2020-11-24".to_string(), cases: 13554, deaths: 249 },
      Numbers { date: "2020-11-25".to_string(), cases: 18633, deaths: 410 },
      Numbers { date: "2020-11-26".to_string(), cases: 22268, deaths: 389 },
      Numbers { date: "2020-11-27".to_string(), cases: 22806, deaths: 426 },
      Numbers { date: "2020-11-28".to_string(), cases: 21695, deaths: 379 },
      Numbers { date: "2020-11-29".to_string(), cases: 14611, deaths: 158 },
      Numbers { date: "2020-11-30".to_string(), cases: 11169, deaths: 125 },
    ];

    let incidence = calculate_incidence(&numbers, &83_019_213);
    assert_eq!(numbers.len(), incidence.len());
    for idx in 0..numbers.len()
    {
      // Numbers should be equal.
      assert_eq!(numbers[idx].date, incidence[idx].date);
      assert_eq!(numbers[idx].cases, incidence[idx].cases);
      assert_eq!(numbers[idx].deaths, incidence[idx].deaths);
    }
    // Check incidence.
    for idx in 0..13
    {
      // Incidence values should not be set.
      assert!(incidence[idx].incidence_14d.is_none());
    }
    for idx in 13..numbers.len()
    {
      // Incidence values should be set.
      assert!(incidence[idx].incidence_14d.is_some());
    }
    // 13th: 302.82267311
    assert!(incidence[13].incidence_14d.unwrap() > 302.822673);
    assert!(incidence[13].incidence_14d.unwrap() < 302.822674);
    // 14th: 306.92051971
    assert!(incidence[14].incidence_14d.unwrap() > 306.920519);
    assert!(incidence[14].incidence_14d.unwrap() < 306.920520);
    // 15th: 310.25709675
    assert!(incidence[15].incidence_14d.unwrap() > 310.257096);
    assert!(incidence[15].incidence_14d.unwrap() < 310.257097);
    // 16th: 308.72371676
    assert!(incidence[16].incidence_14d.unwrap() > 308.723716);
    assert!(incidence[16].incidence_14d.unwrap() < 308.723717);
    // 17th: 307.59988052
    assert!(incidence[17].incidence_14d.unwrap() > 307.599880);
    assert!(incidence[17].incidence_14d.unwrap() < 307.599881);
    // 18th: 308.01785606
    assert!(incidence[18].incidence_14d.unwrap() > 308.017856);
    assert!(incidence[18].incidence_14d.unwrap() < 308.017857);
    // 19th: 311.17254749
    assert!(incidence[19].incidence_14d.unwrap() > 311.172547);
    assert!(incidence[19].incidence_14d.unwrap() < 311.172548);
    // 20th: 313.75267313
    assert!(incidence[20].incidence_14d.unwrap() > 313.752673);
    assert!(incidence[20].incidence_14d.unwrap() < 313.752674);
    // 21st: 313.22869804
    assert!(incidence[21].incidence_14d.unwrap() > 313.228698);
    assert!(incidence[21].incidence_14d.unwrap() < 313.228699);
    // 22nd: 312.89624487
    assert!(incidence[22].incidence_14d.unwrap() > 312.896244);
    assert!(incidence[22].incidence_14d.unwrap() < 312.896245);
    // 23rd: 309.88609829
    assert!(incidence[23].incidence_14d.unwrap() > 309.886098);
    assert!(incidence[23].incidence_14d.unwrap() < 309.886099);
    // 24th: 307.74442538
    assert!(incidence[24].incidence_14d.unwrap() > 307.744425);
    assert!(incidence[24].incidence_14d.unwrap() < 307.744426);
    // 25th: 307.92028828
    assert!(incidence[25].incidence_14d.unwrap() > 307.920288);
    assert!(incidence[25].incidence_14d.unwrap() < 307.920289);
    // 26th: 308.40451354
    assert!(incidence[26].incidence_14d.unwrap() > 308.404513);
    assert!(incidence[26].incidence_14d.unwrap() < 308.404514);
    // 27th: 307.51797177
    assert!(incidence[27].incidence_14d.unwrap() > 307.517971);
    assert!(incidence[27].incidence_14d.unwrap() < 307.517972);
    // 28th: 306.59529379
    assert!(incidence[28].incidence_14d.unwrap() > 306.595293);
    assert!(incidence[28].incidence_14d.unwrap() < 306.595294);
    // 29th: 303.7814873
    assert!(incidence[29].incidence_14d.unwrap() > 303.781487);
    assert!(incidence[29].incidence_14d.unwrap() < 303.781488);
    // 30th: 304.19705376
    assert!(incidence[30].incidence_14d.unwrap() > 304.197053);
    assert!(incidence[30].incidence_14d.unwrap() < 304.197054);
  }

  #[test]
  fn calculate_incidence_no_population()
  {
    let numbers = vec![
      Numbers { date: "2020-10-31".to_string(), cases: 19059, deaths: 103 },
      Numbers { date: "2020-11-01".to_string(), cases: 14177, deaths: 29 },
      Numbers { date: "2020-11-02".to_string(), cases: 12097, deaths: 49 },
      Numbers { date: "2020-11-03".to_string(), cases: 15352, deaths: 131 },
      Numbers { date: "2020-11-04".to_string(), cases: 17214, deaths: 151 },
      Numbers { date: "2020-11-05".to_string(), cases: 19990, deaths: 118 },
      Numbers { date: "2020-11-06".to_string(), cases: 21506, deaths: 166 },
      Numbers { date: "2020-11-07".to_string(), cases: 23399, deaths: 130 },
      Numbers { date: "2020-11-08".to_string(), cases: 16017, deaths: 63 },
      Numbers { date: "2020-11-09".to_string(), cases: 13363, deaths: 63 },
      Numbers { date: "2020-11-10".to_string(), cases: 15332, deaths: 154 },
      Numbers { date: "2020-11-11".to_string(), cases: 18487, deaths: 261 },
      Numbers { date: "2020-11-12".to_string(), cases: 21866, deaths: 215 },
      Numbers { date: "2020-11-13".to_string(), cases: 23542, deaths: 218 },
      Numbers { date: "2020-11-14".to_string(), cases: 22461, deaths: 178 },
      Numbers { date: "2020-11-15".to_string(), cases: 16947, deaths: 107 },
      Numbers { date: "2020-11-16".to_string(), cases: 10824, deaths: 62 },
      Numbers { date: "2020-11-17".to_string(), cases: 14419, deaths: 267 },
      Numbers { date: "2020-11-18".to_string(), cases: 17561, deaths: 305 },
      Numbers { date: "2020-11-19".to_string(), cases: 22609, deaths: 251 },
      Numbers { date: "2020-11-20".to_string(), cases: 23648, deaths: 260 },
      Numbers { date: "2020-11-21".to_string(), cases: 22964, deaths: 254 },
      Numbers { date: "2020-11-22".to_string(), cases: 15741, deaths: 138 },
      Numbers { date: "2020-11-23".to_string(), cases: 10864, deaths: 90 },
      Numbers { date: "2020-11-24".to_string(), cases: 13554, deaths: 249 },
      Numbers { date: "2020-11-25".to_string(), cases: 18633, deaths: 410 },
      Numbers { date: "2020-11-26".to_string(), cases: 22268, deaths: 389 },
      Numbers { date: "2020-11-27".to_string(), cases: 22806, deaths: 426 },
      Numbers { date: "2020-11-28".to_string(), cases: 21695, deaths: 379 },
      Numbers { date: "2020-11-29".to_string(), cases: 14611, deaths: 158 },
      Numbers { date: "2020-11-30".to_string(), cases: 11169, deaths: 125 },
    ];

    let incidence = calculate_incidence(&numbers, &0);
    assert_eq!(numbers.len(), incidence.len());
    for idx in 0..numbers.len()
    {
      // Numbers should be equal.
      assert_eq!(numbers[idx].date, incidence[idx].date);
      assert_eq!(numbers[idx].cases, incidence[idx].cases);
      assert_eq!(numbers[idx].deaths, incidence[idx].deaths);
      // Incidence values should not be set.
      assert!(incidence[idx].incidence_14d.is_none());
    }
    // Same game for negative population.
    let incidence = calculate_incidence(&numbers, &-1);
    assert_eq!(numbers.len(), incidence.len());
    for idx in 0..numbers.len()
    {
      // Numbers should be equal.
      assert_eq!(numbers[idx].date, incidence[idx].date);
      assert_eq!(numbers[idx].cases, incidence[idx].cases);
      assert_eq!(numbers[idx].deaths, incidence[idx].deaths);
      // Incidence values should not be set.
      assert!(incidence[idx].incidence_14d.is_none());
    }
  }

  #[test]
  fn fill_missing_dates_no_dates_missing()
  {
    let mut numbers = vec![
      Numbers { date: "2020-10-31".to_string(), cases: 19059, deaths: 103 },
      Numbers { date: "2020-11-01".to_string(), cases: 14177, deaths: 29 },
      Numbers { date: "2020-11-02".to_string(), cases: 12097, deaths: 49 }
    ];

    let fill = fill_missing_dates(&mut numbers);
    // Function should succeed.
    assert!(fill.is_ok());
    // ... and vector should still have three elements.
    assert_eq!(3, numbers.len());
  }

  #[test]
  fn fill_missing_dates_few_dates_missing()
  {
    let mut numbers = vec![
      Numbers { date: "2020-10-31".to_string(), cases: 19059, deaths: 103 },
      Numbers { date: "2020-11-05".to_string(), cases: 19990, deaths: 118 }
    ];

    let fill = fill_missing_dates(&mut numbers);
    // Function should succeed.
    assert!(fill.is_ok());
    // ... and vector should have six elements now.
    assert_eq!(6, numbers.len());
    assert_eq!("2020-10-31".to_string(), numbers[0].date);
    assert_eq!(19059, numbers[0].cases);
    assert_eq!(103, numbers[0].deaths);
    assert_eq!("2020-11-01".to_string(), numbers[1].date);
    assert_eq!(0, numbers[1].cases);
    assert_eq!(0, numbers[1].deaths);
    assert_eq!("2020-11-02".to_string(), numbers[2].date);
    assert_eq!(0, numbers[2].cases);
    assert_eq!(0, numbers[2].deaths);
    assert_eq!("2020-11-03".to_string(), numbers[3].date);
    assert_eq!(0, numbers[3].cases);
    assert_eq!(0, numbers[3].deaths);
    assert_eq!("2020-11-04".to_string(), numbers[4].date);
    assert_eq!(0, numbers[4].cases);
    assert_eq!(0, numbers[4].deaths);
    assert_eq!("2020-11-05".to_string(), numbers[5].date);
    assert_eq!(19990, numbers[5].cases);
    assert_eq!(118, numbers[5].deaths);
  }

  #[test]
  fn fill_missing_dates_too_much_dates_missing()
  {
    let mut numbers = vec![
      Numbers { date: "2020-10-31".to_string(), cases: 19059, deaths: 103 },
      Numbers { date: "2020-11-01".to_string(), cases: 14177, deaths: 29 },
      Numbers { date: "2021-04-01".to_string(), cases: 1, deaths: 23 }
    ];

    let fill = fill_missing_dates(&mut numbers);
    // Function should NOT succeed.
    assert!(fill.is_err());
  }
}
