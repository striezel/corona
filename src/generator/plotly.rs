/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2020, 2021, 2022, 2023, 2024, 2025  Dirk Stolle

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

use std::path::Path;

pub struct Plotly
{
}

impl Plotly
{
  /// basic file name of the plotly-basic.min.js file
  pub const FILE_NAME: &'static str = "plotly-basic-3.0.0.min.js";

  /// relative path to plotly-basic.min.js
  pub const ASSET_PATH: &'static str = "./assets/plotly-basic-3.0.0.min.js";

  /// SHA256 digest of plotly-basic.min.js
  const SHA256: &'static str = "695dfe02adc7c5681d7c78a3711505059cf2a2037e22f2955fe6bb3520ec787e";

  /**
   * Checks whether the data has the expected hash.
   *
   * @return Returns true, if the hash matches. Returns false otherwise.
   */
  fn check_hash(data: &[u8]) -> bool
  {
    use sha2::Digest;
    let mut hash = sha2::Sha256::new();
    hash.update(data);
    let digest = hash.finalize();
    // Transform hash into hexadecimal string.
    let digest_string: String = digest[..].iter().fold(String::new(), |mut hash, x| {
      hash.push_str(&format!("{:02x}", x));
      hash
    });
    // Compare with expected value.
    digest_string == Plotly::SHA256
  }

  /**
   * Downloads the minified plotly.js from a CDN.
   *
   * @param destination  destination path for the .js file
   * @return Returns true, if file was downloaded successfully.
   */
  pub fn download(destination: &Path) -> bool
  {
    use std::io::Read;
    // Retrieve minified JS file.
    let url = format!("https://cdn.plot.ly/{}", Plotly::FILE_NAME);
    let res = match ureq::get(&url).call()
    {
      Ok(responded) => responded,
      Err(e) =>
        {
          eprintln!("Download of plotly.js failed: {}", e);
          return false;
        }
    };
    if res.status() != 200
    {
      let mut all_headers = std::collections::HashMap::new();
      let names = res.headers_names();
      for name in names
      {
        if let Some(value) = res.header(&name)
        {
          all_headers.insert(name, value);
        }
      }
      eprintln!("HTTP request failed with unexpected status code: {}\n\
                 Headers:\n{:#?}", res.status(), all_headers);
      return false;
    }
    let mut body: Vec<u8> = Vec::new();
    if let Err(e) = res.into_reader().read_to_end(&mut body)
    {
      eprintln!("Failed to read plotly.js into buffer: {}", e);
      return false;
    }

    // Check SHA256 hash of the file.
    if !Plotly::check_hash(&body)
    {
      eprintln!("Error: SHA256 hash of the downloaded plotly.js does not match \
                 the expected hash!");
      return false;
    }

    match std::fs::write(destination, &body)
    {
      Ok(()) => true,
      Err(e) =>
      {
        eprintln!("Error while writing plotly.js file: {}", e);
        false
      }
    }
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn constants_contain_same_version()
  {
    assert!(!Plotly::FILE_NAME.is_empty());
    assert!(Plotly::ASSET_PATH.contains(Plotly::FILE_NAME))
  }

  #[test]
  fn download_works()
  {
    let destination = std::env::temp_dir().join(Plotly::FILE_NAME);

    assert!(Plotly::download(&destination));
    assert!(std::fs::remove_file(&destination).is_ok());
  }
}
