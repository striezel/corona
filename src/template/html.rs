/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2020  Dirk Stolle
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

/**
 * Replaces all HTML characters that have a "special" meaning by their safe,
 * encoded versions. Think of it like PHP's htmlspecialchars() function.
 *
 * @value   the value that needs to be escaped
 * @return Returns the value with proper escape sequences for the special characters.
 */
pub fn special_chars(value: &str) -> String
{
  value
    .to_string()
    .replace('&', "&amp;")
    .replace('"', "&quot;")
    .replace('\'', "&#39;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn html_tag()
  {
    assert_eq!("&lt;b&gt;hello&lt;/b&gt;", special_chars("<b>hello</b>"));
    assert_eq!("&lt;br /&gt;", special_chars("<br />"));
  }

  #[test]
  fn double_quotes()
  {
    assert_eq!(
      "&lt;a href=&quot;hello.html&quot;&gt;there&lt;/a&gt;",
      special_chars("<a href=\"hello.html\">there</a>")
    );
    assert_eq!(
      "&quot;Indeed!&quot;, he said.",
      special_chars("\"Indeed!\", he said.")
    );
  }

  #[test]
  fn single_quotes()
  {
    assert_eq!(
      "&lt;a href=&#39;hello.html&#39;&gt;there&lt;/a&gt;",
      special_chars("<a href='hello.html'>there</a>")
    );
    assert_eq!(
      "&#39;Indeed!&#39;, he said.",
      special_chars("'Indeed!', he said.")
    );
  }

  #[test]
  fn amp_it_up()
  {
    assert_eq!("Rust &amp; Co.", special_chars("Rust & Co."));
    assert_eq!("&amp;amp;", special_chars("&amp;"));
  }
}
