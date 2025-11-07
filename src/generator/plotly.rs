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
  pub const FILE_NAME: &'static str = "plotly-basic-3.2.0.min.js";

  /// relative path to plotly-basic.min.js
  pub const ASSET_PATH: &'static str = "./assets/plotly-basic-3.2.0.min.js";

  #[cfg(not(target_family = "windows"))]
  const PLOTLY_JS: &'static [u8] = include_bytes!("../assets/plotly-basic-3.2.0.min.js");

  #[cfg(target_family = "windows")]
  const PLOTLY_JS: &'static [u8] = include_bytes!("..\\assets\\plotly-basic-3.2.0.min.js");

  /// SHA256 digest of plotly-basic.min.js
  const SHA256: &'static str = "12d69f8b38d1109cdf6dde9723f944a8387f5782181174d6e440cfc9cdafef13";

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
      hash.push_str(&format!("{x:02x}"));
      hash
    });
    // Compare with expected value.
    digest_string == Plotly::SHA256
  }

  /**
   * Extracts the embedded minified plotly.js from the binary.
   *
   * @param destination  destination path for the .js file
   * @return Returns true, if file was written successfully.
   */
  pub fn extract(destination: &Path) -> bool
  {
    // Check SHA256 hash of the file.
    if !Plotly::check_hash(Self::PLOTLY_JS)
    {
      eprintln!("Error: SHA256 hash of the embedded plotly.js does not match \
                 the expected hash!");
      return false;
    }

    match std::fs::write(destination, Self::PLOTLY_JS)
    {
      Ok(()) => true,
      Err(e) =>
      {
        eprintln!("Error while writing plotly.js file: {e}");
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
  fn extract_works()
  {
    let destination = std::env::temp_dir().join(Plotly::FILE_NAME);

    assert!(Plotly::extract(&destination));
    assert!(std::fs::remove_file(&destination).is_ok());
  }
}
