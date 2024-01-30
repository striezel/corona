/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2020, 2021, 2022  Dirk Stolle
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

/// struct to hold the case numbers and 14-day incidence as well as
/// 7-day incidence for a single day in a single country
pub struct NumbersAndIncidence
{
  pub date: String,
  pub cases: i32,
  pub deaths: i32,
  pub incidence_14d: Option<f64>,
  pub incidence_7d: Option<f64>
}

/// struct to hold the case numbers and 14-day incidence as well as 7-day
/// incidence and sum of cases and deaths so far for a single day in a single
/// country
pub struct NumbersAndIncidenceAndTotals
{
  pub date: String,
  pub cases: i32,
  pub deaths: i32,
  pub incidence_14d: Option<f64>,
  pub incidence_7d: Option<f64>,
  pub total_cases: i32,
  pub total_deaths: i32
}

/// struct to hold 7-day incidence value for a single day in a single country
pub struct Incidence7
{
  pub date: String,
  pub incidence_7d: f64
}

/// struct to hold 14-day incidence value for a single day in a single country
pub struct Incidence14
{
  pub date: String,
  pub incidence_14d: f64
}

/// struct to hold incidence value for a single day in a single country
pub struct IncidenceWithDay
{
  pub day_of_year: u16, // day of year, range [1;366]
  pub incidence: f64
}

/**
 * Calculates the 14-day incidence and 7-day incidence for a slice of Numbers
 * that are pre-sorted by date in ascending order.
 *
 * @param number   slice of numbers, has to be sorted by date in ascending order
 *                 without any gaps
 * @param population  number of inhabitants in the country
 * @return Returns the numbers with 14-day and 7-day incidence calculated.
 */
pub fn calculate_incidence(numbers: &[Numbers], population: &i32) -> Vec<NumbersAndIncidence>
{
  let len = numbers.len();
  let mut result: Vec<NumbersAndIncidence> = Vec::with_capacity(len);
  // If there is no valid population number, no incidence can be calculated.
  if population <= &0
  {
    for elem in numbers.iter()
    {
      result.push(NumbersAndIncidence {
        date: elem.date.clone(),
        cases: elem.cases,
        deaths: elem.deaths,
        incidence_14d: None,
        incidence_7d: None
      });
    }
    return result;
  }
  // Elements for the first six days can have no 7-day (or 14-day) incidence.
  for elem in numbers.iter().take(6)
  {
    result.push(NumbersAndIncidence {
      date: elem.date.clone(),
      cases: elem.cases,
      deaths: elem.deaths,
      incidence_14d: None,
      incidence_7d: None
    });
  }
  // If there is not enough data to ever get to seven days, then there can be
  // no 7-day (or 14-day) incidence.
  if len <= 6
  {
    return result;
  }

  // Calculate values for the 7th day.
  let mut sum7: i32 = 0;
  for elem in numbers.iter().take(7)
  {
    sum7 += elem.cases;
  }
  result.push(NumbersAndIncidence {
    date: numbers[6].date.clone(),
    cases: numbers[6].cases,
    deaths: numbers[6].deaths,
    incidence_14d: None,
    incidence_7d: Some(sum7 as f64 * 100_000.0 / *population as f64)
  });

  // Calculate values for days 8 till 13.
  for (idx, elem) in numbers.iter().enumerate().skip(7).take(6)
  {
    // Recalculate sum.
    sum7 = sum7 + elem.cases - numbers[idx - 7].cases;
    result.push(NumbersAndIncidence {
      date: elem.date.clone(),
      cases: elem.cases,
      deaths: elem.deaths,
      incidence_14d: None,
      incidence_7d: Some(sum7 as f64 * 100_000.0 / *population as f64)
    });
  }

  // If there is not enough data to ever get to 14 days, then there can be no
  // 14-day incidence.
  if len <= 13
  {
    return result;
  }

  // Calculate values for the 14th day.
  let mut sum14: i32 = 0;
  for elem in numbers.iter().take(14)
  {
    sum14 += elem.cases;
  }
  sum7 = sum7 + numbers[13].cases - numbers[6].cases;
  result.push(NumbersAndIncidence {
    date: numbers[13].date.clone(),
    cases: numbers[13].cases,
    deaths: numbers[13].deaths,
    incidence_14d: Some(sum14 as f64 * 100_000.0 / *population as f64),
    incidence_7d: Some(sum7 as f64 * 100_000.0 / *population as f64)
  });

  // Calculate values for days 15 onwards.
  for idx in 14..len
  {
    // Recalculate sum.
    sum14 = sum14 + numbers[idx].cases - numbers[idx - 14].cases;
    sum7 = sum7 + numbers[idx].cases - numbers[idx - 7].cases;
    result.push(NumbersAndIncidence {
      date: numbers[idx].date.clone(),
      cases: numbers[idx].cases,
      deaths: numbers[idx].deaths,
      incidence_14d: Some(sum14 as f64 * 100_000.0 / *population as f64),
      incidence_7d: Some(sum7 as f64 * 100_000.0 / *population as f64)
    });
  }

  result
}

/**
 * Calculates the total cases and death numbers for a slice of NumbersAndIncidence
 * that are pre-sorted by date in ascending order.
 *
 * @param number   slice of numbers, has to be sorted by date in ascending order
 *                 without any gaps
 * @return Returns the numbers with totals calculated.
 */
pub fn calculate_totals(numbers: &[NumbersAndIncidence]) -> Vec<NumbersAndIncidenceAndTotals>
{
  let len = numbers.len();
  let mut result: Vec<NumbersAndIncidenceAndTotals> = Vec::with_capacity(len);
  if len == 0
  {
    return result;
  }

  result.push(NumbersAndIncidenceAndTotals {
    date: numbers[0].date.clone(),
    cases: numbers[0].cases,
    deaths: numbers[0].deaths,
    incidence_14d: numbers[0].incidence_14d,
    incidence_7d: numbers[0].incidence_7d,
    total_cases: numbers[0].cases,
    total_deaths: numbers[0].deaths
  });
  for idx in 1..len
  {
    result.push(NumbersAndIncidenceAndTotals {
      date: numbers[idx].date.clone(),
      cases: numbers[idx].cases,
      deaths: numbers[idx].deaths,
      incidence_14d: numbers[idx].incidence_14d,
      incidence_7d: numbers[idx].incidence_7d,
      total_cases: result[idx - 1].total_cases + numbers[idx].cases,
      total_deaths: result[idx - 1].total_deaths + numbers[idx].deaths
    });
  }

  result
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn calculate_incidence_7d_few_elements()
  {
    let mut numbers = Vec::new();
    for i in 1..5
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
      assert!(incidence[idx].incidence_7d.is_none());
      assert!(incidence[idx].incidence_14d.is_none());
    }
  }

  #[test]
  fn calculate_incidence_14d_few_elements()
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
  fn calculate_incidence_7d_seven_elements()
  {
    let numbers = vec![
      Numbers { date: "2020-12-01".to_string(), cases: 272, deaths: 11 },
      Numbers { date: "2020-12-02".to_string(), cases: 400, deaths: 48 },
      Numbers { date: "2020-12-03".to_string(), cases: 202, deaths: 19 },
      Numbers { date: "2020-12-04".to_string(), cases: 119, deaths: 5 },
      Numbers { date: "2020-12-05".to_string(), cases: 235, deaths: 18 },
      Numbers { date: "2020-12-06".to_string(), cases: 234, deaths: 10 },
      Numbers { date: "2020-12-07".to_string(), cases: 210, deaths: 26 },
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
    for idx in 0..6
    {
      // Incidence values should not be set.
      assert!(incidence[idx].incidence_7d.is_none());
      assert!(incidence[idx].incidence_14d.is_none());
    }
    // Incidence for last entry should be set, but only 7d incidence.
    assert!(incidence[6].incidence_7d.is_some());
    assert!(incidence[6].incidence_14d.is_none());
    // Incidence should be roughly 4.39517029.
    assert!(incidence[6].incidence_7d.unwrap() > 4.3951702);
    assert!(incidence[6].incidence_7d.unwrap() < 4.3951703);
  }

  /* This test checks calculations when there are enough elements for 7-day
      incidence, but not yet enough for 14-day incidence values. */
  #[test]
  fn calculate_incidence_7d_ten_elements()
  {
    let numbers = vec![
      Numbers { date: "2020-12-01".to_string(), cases: 272, deaths: 11 },
      Numbers { date: "2020-12-02".to_string(), cases: 400, deaths: 48 },
      Numbers { date: "2020-12-03".to_string(), cases: 202, deaths: 19 },
      Numbers { date: "2020-12-04".to_string(), cases: 119, deaths: 5 },
      Numbers { date: "2020-12-05".to_string(), cases: 235, deaths: 18 },
      Numbers { date: "2020-12-06".to_string(), cases: 234, deaths: 10 },
      Numbers { date: "2020-12-07".to_string(), cases: 210, deaths: 26 },
      Numbers { date: "2020-12-08".to_string(), cases: 200, deaths: 26 },
      Numbers { date: "2020-12-09".to_string(), cases: 135, deaths: 13 },
      Numbers { date: "2020-12-10".to_string(), cases: 202, deaths: 16 },
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
    for idx in 0..6
    {
      // Incidence values should not be set.
      assert!(incidence[idx].incidence_7d.is_none());
      assert!(incidence[idx].incidence_14d.is_none());
    }
    // Incidence for other entries should be set, but only 7d incidence.
    for idx in 6..incidence.len()
    {
      assert!(incidence[idx].incidence_7d.is_some());
      assert!(incidence[idx].incidence_14d.is_none());
    }
    // Incidence on 7th day should be roughly 4.39517029.
    assert!(incidence[6].incidence_7d.unwrap() > 4.3951702);
    assert!(incidence[6].incidence_7d.unwrap() < 4.3951703);
    // Incidence on 8th day should be roughly 4.20590458.
    assert!(incidence[7].incidence_7d.unwrap() > 4.2059045);
    assert!(incidence[7].incidence_7d.unwrap() < 4.2059046);
    // Incidence on 9th day should be roughly 3.509301634.
    assert!(incidence[8].incidence_7d.unwrap() > 3.5093016);
    assert!(incidence[8].incidence_7d.unwrap() < 3.5093017);
    // Incidence on 10th day should be roughly 3.509301634, too.
    assert!(incidence[9].incidence_7d.unwrap() > 3.5093016);
    assert!(incidence[9].incidence_7d.unwrap() < 3.5093017);
  }

  #[test]
  fn calculate_incidence_14d_14_elements()
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
  fn calculate_incidence_7d_more_than_14_elements()
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
    for idx in 0..6
    {
      // Incidence values should not be set.
      assert!(incidence[idx].incidence_14d.is_none());
      assert!(incidence[idx].incidence_7d.is_none());
    }
    for idx in 7..13
    {
      // Incidence values should not be set only for 7-day incidence.
      assert!(incidence[idx].incidence_14d.is_none());
      assert!(incidence[idx].incidence_7d.is_some());
    }
    for idx in 13..numbers.len()
    {
      // Incidence values should be set for both.
      assert!(incidence[idx].incidence_14d.is_some());
      assert!(incidence[idx].incidence_7d.is_some());
    }

    // 6th: 143.81610676
    assert!(incidence[6].incidence_7d.unwrap() > 143.816106);
    assert!(incidence[6].incidence_7d.unwrap() < 143.816107);
    // 7th: 149.04381230
    assert!(incidence[7].incidence_7d.unwrap() > 149.043812);
    assert!(incidence[7].incidence_7d.unwrap() < 149.043813);
    // 8th: 151.26016672
    assert!(incidence[8].incidence_7d.unwrap() > 151.260166);
    assert!(incidence[8].incidence_7d.unwrap() < 151.260167);
    // 9th: 152.78511493
    assert!(incidence[9].incidence_7d.unwrap() > 152.785114);
    assert!(incidence[9].incidence_7d.unwrap() < 152.785115);
    // 10th: 152.76102412
    assert!(incidence[10].incidence_7d.unwrap() > 152.761024);
    assert!(incidence[10].incidence_7d.unwrap() < 152.761025);
    // 11th: 154.29440411
    assert!(incidence[11].incidence_7d.unwrap() > 154.294404);
    assert!(incidence[11].incidence_7d.unwrap() < 154.294405);
    // 12th: 156.55412199
    assert!(incidence[12].incidence_7d.unwrap() > 156.554121);
    assert!(incidence[12].incidence_7d.unwrap() < 156.554122);
    // 13th: 159.00656634
    assert!(incidence[13].incidence_7d.unwrap() > 159.006566);
    assert!(incidence[13].incidence_7d.unwrap() < 159.006567);
    // 14th: 157.87670740
    assert!(incidence[14].incidence_7d.unwrap() > 157.876707);
    assert!(incidence[14].incidence_7d.unwrap() < 157.876708);
    // 15th: 158.99693002
    assert!(incidence[15].incidence_7d.unwrap() > 158.996930);
    assert!(incidence[15].incidence_7d.unwrap() < 158.996931);
    // 16th: 155.93860182
    assert!(incidence[16].incidence_7d.unwrap() > 155.938601);
    assert!(incidence[16].incidence_7d.unwrap() < 155.938602);
    // 17th: 154.83885639
    assert!(incidence[17].incidence_7d.unwrap() > 154.838856);
    assert!(incidence[17].incidence_7d.unwrap() < 154.838857);
    // 18th: 153.72345194
    assert!(incidence[18].incidence_7d.unwrap() > 153.723451);
    assert!(incidence[18].incidence_7d.unwrap() < 153.723452);
    // 19th: 154.61842549
    assert!(incidence[19].incidence_7d.unwrap() > 154.618425);
    assert!(incidence[19].incidence_7d.unwrap() < 154.618426);
    // 20th: 154.74610678
    assert!(incidence[20].incidence_7d.unwrap() > 154.746106);
    assert!(incidence[20].incidence_7d.unwrap() < 154.746107);
    // 21st: 155.35199062
    assert!(incidence[21].incidence_7d.unwrap() > 155.351990);
    assert!(incidence[21].incidence_7d.unwrap() < 155.351991);
    // 22nd: 153.89931484
    assert!(incidence[22].incidence_7d.unwrap() > 153.899314);
    assert!(incidence[22].incidence_7d.unwrap() < 153.899315);
    // 23rd: 153.94749646
    assert!(incidence[23].incidence_7d.unwrap() > 153.947496);
    assert!(incidence[23].incidence_7d.unwrap() < 153.947497);
    // 24th: 152.90556897
    assert!(incidence[24].incidence_7d.unwrap() > 152.905568);
    assert!(incidence[24].incidence_7d.unwrap() < 152.905569);
    // 25th: 154.19683633
    assert!(incidence[25].incidence_7d.unwrap() > 154.196836);
    assert!(incidence[25].incidence_7d.unwrap() < 154.196837);
    // 26th: 153.78608804
    assert!(incidence[26].incidence_7d.unwrap() > 153.786088);
    assert!(incidence[26].incidence_7d.unwrap() < 153.786089);
    // 27th: 152.77186498
    assert!(incidence[27].incidence_7d.unwrap() > 152.771864);
    assert!(incidence[27].incidence_7d.unwrap() < 152.771865);
    // 28th: 151.24330316
    assert!(incidence[28].incidence_7d.unwrap() > 151.243303);
    assert!(incidence[28].incidence_7d.unwrap() < 151.243304);
    // 29th: 149.8821724
    assert!(incidence[29].incidence_7d.unwrap() > 149.882172);
    assert!(incidence[29].incidence_7d.unwrap() < 149.882173);
    // 30th: 150.24955729
    assert!(incidence[30].incidence_7d.unwrap() > 150.249557);
    assert!(incidence[30].incidence_7d.unwrap() < 150.249558);
  }

  #[test]
  fn calculate_incidence_14d_more_than_14_elements()
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
      assert!(incidence[idx].incidence_7d.is_none());
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
      assert!(incidence[idx].incidence_7d.is_none());
    }
  }

  #[test]
  fn calculate_totals_zero_elements()
  {
    let numbers = Vec::new();
    let totals = calculate_totals(&numbers);

    assert_eq!(totals.len(), 0);
  }

  #[test]
  fn calculate_totals_one_element()
  {
    let numbers = vec![NumbersAndIncidence {
      date: "2020-10-31".to_string(),
      cases: 19059,
      deaths: 103,
      incidence_7d: Some(123.45),
      incidence_14d: None
    }];
    let totals = calculate_totals(&numbers);

    assert_eq!(totals.len(), 1);
    // Check values of first element.
    assert_eq!(numbers[0].date, totals[0].date);
    assert_eq!(numbers[0].cases, totals[0].cases);
    assert_eq!(numbers[0].deaths, totals[0].deaths);
    assert_eq!(numbers[0].incidence_7d, totals[0].incidence_7d);
    assert_eq!(numbers[0].incidence_14d, totals[0].incidence_14d);
    // Totals should equal the numbers of the first day.
    assert_eq!(numbers[0].cases, totals[0].total_cases);
    assert_eq!(numbers[0].deaths, totals[0].total_deaths);
  }

  #[test]
  fn calculate_totals_more_elements()
  {
    let numbers = vec![
      NumbersAndIncidence { date: "2020-10-31".to_string(), cases: 518753, deaths: 10452,
        incidence_14d: None, incidence_7d: None },
      NumbersAndIncidence { date: "2020-11-01".to_string(), cases: 14177, deaths: 29,
        incidence_14d: None, incidence_7d: None },
      NumbersAndIncidence { date: "2020-11-02".to_string(), cases: 12097, deaths: 49,
        incidence_14d: None, incidence_7d: None },
      NumbersAndIncidence { date: "2020-11-03".to_string(), cases: 15352, deaths: 131,
        incidence_14d: None, incidence_7d: None },
      NumbersAndIncidence { date: "2020-11-04".to_string(), cases: 17214, deaths: 151,
        incidence_14d: None, incidence_7d: None },
      NumbersAndIncidence { date: "2020-11-05".to_string(), cases: 19990, deaths: 118,
        incidence_14d: None, incidence_7d: None },
      NumbersAndIncidence { date: "2020-11-06".to_string(), cases: 21506, deaths: 166,
        incidence_14d: None, incidence_7d: Some(143.81610676) },
      NumbersAndIncidence { date: "2020-11-07".to_string(), cases: 23399, deaths: 130,
        incidence_14d: None, incidence_7d: Some(149.04381230) },
      NumbersAndIncidence { date: "2020-11-08".to_string(), cases: 16017, deaths: 63,
        incidence_14d: None, incidence_7d: Some(151.26016672) },
      NumbersAndIncidence { date: "2020-11-09".to_string(), cases: 13363, deaths: 63,
        incidence_14d: None, incidence_7d: Some(152.78511493) },
      NumbersAndIncidence { date: "2020-11-10".to_string(), cases: 15332, deaths: 154,
        incidence_14d: None, incidence_7d: Some(152.76102412) },
      NumbersAndIncidence { date: "2020-11-11".to_string(), cases: 18487, deaths: 261,
        incidence_14d: None, incidence_7d: Some(154.29440411) },
      NumbersAndIncidence { date: "2020-11-12".to_string(), cases: 21866, deaths: 215,
        incidence_14d: None, incidence_7d: Some(156.55412199) },
      NumbersAndIncidence { date: "2020-11-13".to_string(), cases: 23542, deaths: 218,
        incidence_14d: Some(302.82267311), incidence_7d: Some(159.00656634) },
      NumbersAndIncidence { date: "2020-11-14".to_string(), cases: 22461, deaths: 178,
        incidence_14d: Some(306.92051971), incidence_7d: Some(157.87670740) },
      NumbersAndIncidence { date: "2020-11-15".to_string(), cases: 16947, deaths: 107,
        incidence_14d: Some(310.25709675), incidence_7d: Some(158.99693002) },
      NumbersAndIncidence { date: "2020-11-16".to_string(), cases: 10824, deaths: 62,
        incidence_14d: Some(308.72371676), incidence_7d: Some(155.93860182) },
      NumbersAndIncidence { date: "2020-11-17".to_string(), cases: 14419, deaths: 267,
        incidence_14d: Some(307.59988052), incidence_7d: Some(154.83885639) },
      NumbersAndIncidence { date: "2020-11-18".to_string(), cases: 17561, deaths: 305,
        incidence_14d: Some(308.01785606), incidence_7d: Some(153.72345194) },
      NumbersAndIncidence { date: "2020-11-19".to_string(), cases: 22609, deaths: 251,
        incidence_14d: Some(311.17254749), incidence_7d: Some(154.61842549) },
      NumbersAndIncidence { date: "2020-11-20".to_string(), cases: 23648, deaths: 260,
        incidence_14d: Some(313.75267313), incidence_7d: Some(155.35199062) },
      NumbersAndIncidence { date: "2020-11-21".to_string(), cases: 22964, deaths: 254,
        incidence_14d: Some(313.22869804), incidence_7d: Some(155.35199062) },
      NumbersAndIncidence { date: "2020-11-22".to_string(), cases: 15741, deaths: 138,
        incidence_14d: Some(312.89624487), incidence_7d: Some(153.89931484) },
      NumbersAndIncidence { date: "2020-11-23".to_string(), cases: 10864, deaths: 90,
        incidence_14d: Some(309.88609829), incidence_7d: Some(153.94749646) },
      NumbersAndIncidence { date: "2020-11-24".to_string(), cases: 13554, deaths: 249,
        incidence_14d: Some(307.74442538), incidence_7d: Some(152.90556897) },
      NumbersAndIncidence { date: "2020-11-25".to_string(), cases: 18633, deaths: 410,
        incidence_14d: Some(307.92028828), incidence_7d: Some(154.19683633) },
      NumbersAndIncidence { date: "2020-11-26".to_string(), cases: 22268, deaths: 389,
        incidence_14d: Some(308.40451354), incidence_7d: Some(153.78608804) },
      NumbersAndIncidence { date: "2020-11-27".to_string(), cases: 22806, deaths: 426,
        incidence_14d: Some(307.51797177), incidence_7d: Some(152.77186498) },
      NumbersAndIncidence { date: "2020-11-28".to_string(), cases: 21695, deaths: 379,
        incidence_14d: Some(306.59529379), incidence_7d: Some(151.24330316) },
      NumbersAndIncidence { date: "2020-11-29".to_string(), cases: 14611, deaths: 158,
        incidence_14d: Some(303.7814873), incidence_7d: Some(149.8821724) },
      NumbersAndIncidence { date: "2020-11-30".to_string(), cases: 11169, deaths: 125,
        incidence_14d: Some(304.19705376), incidence_7d: Some(150.24955729) },
    ];

    let totals = calculate_totals(&numbers);
    assert_eq!(numbers.len(), totals.len());
    for idx in 0..numbers.len()
    {
      // Numbers and incidence should be equal.
      assert_eq!(numbers[idx].date, totals[idx].date);
      assert_eq!(numbers[idx].cases, totals[idx].cases);
      assert_eq!(numbers[idx].deaths, totals[idx].deaths);
      assert_eq!(numbers[idx].incidence_14d, totals[idx].incidence_14d);
      assert_eq!(numbers[idx].incidence_7d, totals[idx].incidence_7d);
    }

    // Check total case numbers.
    assert_eq!(totals[0].total_cases, 518753);
    assert_eq!(totals[1].total_cases, 532930);
    assert_eq!(totals[2].total_cases, 545027);
    assert_eq!(totals[3].total_cases, 560379);
    assert_eq!(totals[4].total_cases, 577593);
    assert_eq!(totals[5].total_cases, 597583);
    assert_eq!(totals[6].total_cases, 619089);
    assert_eq!(totals[7].total_cases, 642488);
    assert_eq!(totals[8].total_cases, 658505);
    assert_eq!(totals[9].total_cases, 671868);
    assert_eq!(totals[10].total_cases, 687200);
    assert_eq!(totals[11].total_cases, 705687);
    assert_eq!(totals[12].total_cases, 727553);
    assert_eq!(totals[13].total_cases, 751095);
    assert_eq!(totals[14].total_cases, 773556);
    assert_eq!(totals[15].total_cases, 790503);
    assert_eq!(totals[16].total_cases, 801327);
    assert_eq!(totals[17].total_cases, 815746);
    assert_eq!(totals[18].total_cases, 833307);
    assert_eq!(totals[19].total_cases, 855916);
    assert_eq!(totals[20].total_cases, 879564);
    assert_eq!(totals[21].total_cases, 902528);
    assert_eq!(totals[22].total_cases, 918269);
    assert_eq!(totals[23].total_cases, 929133);
    assert_eq!(totals[24].total_cases, 942687);
    assert_eq!(totals[25].total_cases, 961320);
    assert_eq!(totals[26].total_cases, 983588);
    assert_eq!(totals[27].total_cases, 1006394);
    assert_eq!(totals[28].total_cases, 1028089);
    assert_eq!(totals[29].total_cases, 1042700);
    assert_eq!(totals[30].total_cases, 1053869);

    // Check accumulated number of deaths.
    assert_eq!(totals[0].total_deaths, 10452);
    assert_eq!(totals[1].total_deaths, 10481);
    assert_eq!(totals[2].total_deaths, 10530);
    assert_eq!(totals[3].total_deaths, 10661);
    assert_eq!(totals[4].total_deaths, 10812);
    assert_eq!(totals[5].total_deaths, 10930);
    assert_eq!(totals[6].total_deaths, 11096);
    assert_eq!(totals[7].total_deaths, 11226);
    assert_eq!(totals[8].total_deaths, 11289);
    assert_eq!(totals[9].total_deaths, 11352);
    assert_eq!(totals[10].total_deaths, 11506);
    assert_eq!(totals[11].total_deaths, 11767);
    assert_eq!(totals[12].total_deaths, 11982);
    assert_eq!(totals[13].total_deaths, 12200);
    assert_eq!(totals[14].total_deaths, 12378);
    assert_eq!(totals[15].total_deaths, 12485);
    assert_eq!(totals[16].total_deaths, 12547);
    assert_eq!(totals[17].total_deaths, 12814);
    assert_eq!(totals[18].total_deaths, 13119);
    assert_eq!(totals[19].total_deaths, 13370);
    assert_eq!(totals[20].total_deaths, 13630);
    assert_eq!(totals[21].total_deaths, 13884);
    assert_eq!(totals[22].total_deaths, 14022);
    assert_eq!(totals[23].total_deaths, 14112);
    assert_eq!(totals[24].total_deaths, 14361);
    assert_eq!(totals[25].total_deaths, 14771);
    assert_eq!(totals[26].total_deaths, 15160);
    assert_eq!(totals[27].total_deaths, 15586);
    assert_eq!(totals[28].total_deaths, 15965);
    assert_eq!(totals[29].total_deaths, 16123);
    assert_eq!(totals[30].total_deaths, 16248);
  }
}
