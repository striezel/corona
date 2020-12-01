<?php
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

require 'Database.php';
require 'Template.php';

/** Generator for HTML files. */
class FileGenerator
{
  private $dbFile;
  private $outputDirectory;

  /**
   * Constructs a new generator.
   *
   * @param path_db       path to the SQLite3 database
   * @param path_output   path to the output directory
   */
  public function __construct(string $path_db, string $path_output)
  {
    if (empty($path_db))
    {
      throw new Exception('Path to SQLite database must not be an empty string!');
    }
    if (empty($path_output))
    {
      throw new Exception('Path of output directory must be set to a non-empty string!');
    }

    $this->dbFile = $path_db;
    $this->outputDirectory = $path_output;
  }

  /**
   * Generates the HTML files.
   *
   * @return Returns whether the generation was successful.
   */
  public function generate()
  {
    if (!is_readable($this->dbFile))
    {
      throw new Exception('Error: Database file ' . $this->dbFile . ' does not exist or is not readable!');
    }
    if (!mkdir($this->outputDirectory, 0755, true))
    {
      throw new Exception('Could not create output directory ' . $this->outputDirectory . '!');
    }
    // Handle each country.
    $db = new Database($this->dbFile);
    $countries = $db->countries();
    foreach($countries as $country)
    {
      if (!$this->generateCountry($db, $country))
      {
        echo 'Error while generating file for ' . $country['name'] . ' (' . $country['geoId'] . ")!\n";
        return false;
      }
    }
    // Copy assets.
    if (!$this->createAssets())
      return false;
    // Site index comes last.
    return $this->createIndex($countries);
  }

  /**
   * Generates the HTML file for a single country.
   *
   * @param db       reference to the Database instance
   * @param country  country data (id, name, etc.)
   * @return Returns whether the generation was successful.
   */
  private function generateCountry(Database &$db, array $country)
  {
    $tpl = new Template();
    if (!$tpl->fromFile(GENERATOR_ROOT . '/templates/main.tpl'))
    {
      echo "Error: Could not load main template file!\n";
      return false;
    }
    // scripts
    if (!$tpl->loadSection('script'))
      return false;
    $tpl->tag('path', './assets/plotly-1.57.1.min.js');
    $scripts = $tpl->generate();
    // header
    if (!$tpl->loadSection('header'))
          return false;
    $tpl->integrate('scripts', $scripts);
    $tpl->tag('title', 'Corona cases in ' . $country['name'] . ' (' . $country['geoId'] . ')');
    $header = $tpl->generate();
    // graph
    $graph = $this->generateGraph($db, $country, $tpl);
    if ($graph === false)
      return false;
    // full
    if (!$tpl->loadSection('full'))
      return false;
    $tpl->integrate('header', $header);
    $tpl->integrate('content', $graph);
    $full = $tpl->generate();
    // write it to a file
    $written = file_put_contents($this->outputDirectory . '/' . strtolower($country['geoId']) . '.html', $full);
    return ($written !== false && ($written == strlen($full)));
  }

  /**
   * Generates the HTML snippet containing the graph of a single country.
   *
   * @param db       reference to the Database instance
   * @param country  country data (id, name, etc.)
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns false, if an error occurred.
   */
  private function generateGraph(Database &$db, array $country, &$tpl)
  {
    // load graph section
    if (!$tpl->loadSection('graph'))
      return false;
    $tpl->tag('title', 'Corona cases in ' . $country['name'] . ' (' . $country['geoId'] . ')');
    $tpl->tag('plotId', 'graph_' . strtolower($country['geoId']));
    // prepare numbers
    $dates = array();
    $infections = array();
    $deaths = array();
    $data = $db->numbers($country['countryId']);
    foreach($data as $d)
    {
      $dates[] = $d['date'];
      $infections[] = $d['cases'];
      $deaths[] = $d['deaths'];
    }
    // graph: date values
    $dates = json_encode($dates);
    if (false === $dates)
    {
      echo "Error: JSON encoding of date array failed!\n";
      return false;
    }
    $tpl->integrate('dates', $dates);
    // graph: infection values
    $infections = json_encode($infections);
    if (false === $infections)
    {
      echo "Error: JSON encoding of cases array failed!\n";
      return false;
    }
    $tpl->integrate('infections', $infections);
    // graph: deaths
    $deaths = json_encode($deaths);
    if (false === $deaths)
    {
      echo "Error: JSON encoding of deaths array failed!\n";
      return false;
    }
    $tpl->integrate('deaths', $deaths);
    return $tpl->generate();
  }

  /**
   * Creates any assets (i. e. library files) in the output directory.
   *
   * @return Returns whether the operation was successful.
   */
  private function createAssets()
  {
    // Note: Should be replaced by opendir() / readdir() / closedir() triad once
    // there are more files. Or use directory iterator instead.
    if (!mkdir($this->outputDirectory . '/assets'))
      return false;
    return copy(GENERATOR_ROOT . '/assets/plotly-1.57.1.min.js', $this->outputDirectory . '/assets/plotly-1.57.1.min.js');
  }

  /**
   * Creates the index.hml in the output directory.
   *
   * @param countries   array containing names and ids of the countries
   * @return Returns whether the operation was successful.
   */
  private function createIndex(array $countries)
  {
    $tpl = new Template();
    if (!$tpl->fromFile(GENERATOR_ROOT . '/templates/main.tpl'))
    {
      echo "Error: Could not load main template file!\n";
      return false;
    }
    // links
    $links = '';
    if (!$tpl->loadSection('indexLink'))
      return false;
    foreach($countries as $country)
    {
      $tpl->tag('url', './' . strtolower($country['geoId']) . '.html');
      $tpl->tag('text', $country['name'] . ' (' . $country['geoId'] . ')');
      $links .= $tpl->generate();
    }
    // index template
    if (!$tpl->loadSection('index'))
      return false;
    $tpl->integrate('links', $links);
    $content = $tpl->generate();
    // main page template
    // -- header
    if (!$tpl->loadSection('header'))
      return false;
    $tpl->integrate('scripts', '');
    $tpl->tag('title', 'Corona worldwide');
    $header = $tpl->generate();
    // -- full template
    if (!$tpl->loadSection('full'))
      return false;
    $tpl->integrate('header', $header);
    $tpl->integrate('content', $content);
    $full = $tpl->generate();
    // write it to a file
    $written = file_put_contents($this->outputDirectory . '/index.html', $full);
    return ($written !== false && ($written == strlen($full)));
  }
}
?>
