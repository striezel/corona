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

use std::path::Path;

pub struct Plotly
{
}

impl Plotly
{
  /// basic file name of the plotly.js file
  pub const FILE_NAME: &'static str = "plotly-basic-1.58.5.min.js";

  /// relative path to plotly.js
  pub const ASSET_PATH: &'static str = "./assets/plotly-basic-1.58.5.min.js";

  /// SHA256 digest of plotly.js
  const SHA256: &'static str = "75e92469b4c54da6c7ed5286841d69ffe47bbfb4ded1624d2e1e2afa0596362d";

  /**
   * Checks whether the data has the expected hash.
   *
   * @return Returns true, if the hash matches. Returns false otherwise.
   */
  fn check_hash(data: &[u8]) -> bool
  {
    use sha2::Digest;
    let mut hash = sha2::Sha256::new();
    hash.update(&data);
    let digest = hash.finalize();
    // Transform hash into hexadecimal string.
    let digest_string: String = digest[..].iter().map(|&x| format!("{:02x}", x)).collect();
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
    use reqwest::StatusCode;
    use std::io::Read;
    // Retrieve minified JS file.
    let url = format!("https://cdn.plot.ly/{}", Plotly::FILE_NAME);
    let mut res = match reqwest::blocking::get(&url)
    {
      Ok(responded) => responded,
      Err(e) =>
        {
          eprintln!("Download of plotly.js failed: {}", e);
          return false;
        }
    };
    let mut body: Vec<u8> = Vec::new();
    if let Err(e) = res.read_to_end(&mut body)
    {
      eprintln!("Failed to read plotly.js into buffer: {}", e);
      return false;
    }
    if res.status() != StatusCode::OK
    {
      eprintln!("HTTP request failed with unexpected status code: {}\n\
                 Headers:\n{:#?}\n\
                 Body:\n{:?}", res.status(), res.headers(), body);
      return false;
    }

    // Check SHA256 hash of the file.
    if !Plotly::check_hash(&body)
    {
      eprintln!("Error: SHA256 hash of the downloaded plotly.js does not match \
                 the expected hash!");
      return false;
    }

    match std::fs::write(&destination, &body)
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
