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
    // Perform calculations in database, if necessary.
    $db = new Database($this->dbFile);
    if (!$db->calculateTotalNumbers())
    {
      echo "Error: Database update failed. Calculations for accumulated numbers could not be performed!\n";
      return false;
    }
    // Handle each country.
    $countries = $db->countries();
    foreach($countries as $country)
    {
      if (!$this->generateCountry($db, $country))
      {
        echo 'Error while generating file for ' . $country['name'] . ' (' . $country['geoId'] . ")!\n";
        return false;
      }
    }
    // Handle accumulated numbers worldwide.
    if (!$this->generateWorld($db))
    {
      echo "Error while generating file for worldwide numbers!\n";
      return false;
    }
    // Generate graphs per continent (incidence only).
    if (!$this->generateContinents($db))
    {
      echo "Error while generating files for continents!\n";
      return false;
    }
    // Copy assets.
    if (!$this->createAssets())
      return false;
    // Site index comes last.
    return $this->createIndex($countries, $db->continents());
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
    if (!$tpl->fromFile(GENERATOR_ROOT . '/../src/templates/main.tpl'))
    {
      echo "Error: Could not load main template file!\n";
      return false;
    }
    // scripts
    if (!$tpl->loadSection('script'))
      return false;
    $tpl->tag('path', './assets/plotly-1.58.3.min.js');
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
    $graph_accu = $this->generateAccumulatedGraph($db, $country, $tpl);
    if ($graph_accu === false)
      return false;
    $graph = $graph . "\n<br />\n" . $graph_accu;
    unset($graph_accu);
    $graph_incidence = $this->generateIncidenceGraph($db, $country, $tpl);
    if ($graph_incidence === false)
      return false;
    if ($graph_incidence !== '')
    {
      $graph = $graph_incidence . "\n<br />\n" . $graph;
    }
    unset($graph_incidence);
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
   * Generates the HTML file for worldwide numbers.
   *
   * @param db       reference to the Database instance
   * @return Returns whether the generation was successful.
   */
  private function generateWorld(Database &$db)
  {
    $tpl = new Template();
    if (!$tpl->fromFile(GENERATOR_ROOT . '/../src/templates/main.tpl'))
    {
      echo "Error: Could not load main template file!\n";
      return false;
    }
    // scripts
    if (!$tpl->loadSection('script'))
      return false;
    $tpl->tag('path', './assets/plotly-1.58.3.min.js');
    $scripts = $tpl->generate();
    // header
    if (!$tpl->loadSection('header'))
      return false;
    $tpl->integrate('scripts', $scripts);
    $tpl->tag('title', 'Coronavirus cases worldwide');
    $header = $tpl->generate();
    // graph
    $graph = $this->generateGraphWorld($db, $tpl);
    if ($graph === false)
      return false;
    $graph_accu = $this->generateAccumulatedGraphWorld($db, $tpl);
    if ($graph_accu === false)
      return false;
    $graph = $graph . "\n<br />\n" . $graph_accu;
    unset($graph_accu);
    // full
    if (!$tpl->loadSection('full'))
      return false;
    $tpl->integrate('header', $header);
    $tpl->integrate('content', $graph);
    $full = $tpl->generate();
    // write it to a file
    $written = file_put_contents($this->outputDirectory . '/world.html', $full);
    return ($written !== false && ($written == strlen($full)));
  }

  /**
   * Generates the HTML files for different continents.
   *
   * @param db       reference to the Database instance
   * @return Returns whether the generation was successful.
   */
  private function generateContinents(Database &$db)
  {
    $tpl = new Template();
    if (!$tpl->fromFile(GENERATOR_ROOT . '/../src/templates/main.tpl'))
    {
      echo "Error: Could not load main template file!\n";
      return false;
    }

    $continents = $db->continents();
    foreach ($continents as $continent)
    {
      // template: scripts
      if (!$tpl->loadSection('script'))
        return false;
      $tpl->tag('path', './assets/plotly-1.58.3.min.js');
      $scripts = $tpl->generate();
      // template: header
      if (!$tpl->loadSection('header'))
        return false;
      $tpl->integrate('scripts', $scripts);
      $tpl->tag('title', 'Coronavirus incidence in ' . $continent);
      $header = $tpl->generate();
      // template: graph
      $graph = $this->generateGraphContinent($db, $continent, $tpl);
      if ($graph === false)
        return false;
      // template: full
      if (!$tpl->loadSection('full'))
        return false;
      $tpl->integrate('header', $header);
      $tpl->integrate('content', $graph);
      $full = $tpl->generate();
      // write it to a file
      $written = file_put_contents($this->outputDirectory . '/continent_' . strtolower($continent) . '.html', $full);
      if ($written === false || ($written != strlen($full)))
        return false;
    }
    // All is done here.
    return true;
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
    $tpl->tag('title', 'Coronavirus cases in ' . $country['name'] . ' (' . $country['geoId'] . ')');
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
   * Generates the HTML snippet containing the graph for worldwide data.
   *
   * @param db       reference to the Database instance
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns false, if an error occurred.
   */
  private function generateGraphWorld(Database &$db, &$tpl)
  {
    // load graph section
    if (!$tpl->loadSection('graph'))
      return false;
    $tpl->tag('title', 'Coronavirus cases worldwide');
    $tpl->tag('plotId', 'graph_world');
    // prepare numbers
    $dates = array();
    $infections = array();
    $deaths = array();
    $data = $db->numbersWorld();
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
   * Generates the HTML snippet containing the graph with accumulated numbers of a single country.
   *
   * @param db       reference to the Database instance
   * @param country  country data (id, name, etc.)
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns false, if an error occurred.
   */
  private function generateAccumulatedGraph(Database &$db, array $country, &$tpl)
  {
    // load graph section
    if (!$tpl->loadSection('graphAccumulated'))
      return false;
    $tpl->tag('title', 'Accumulated Coronavirus cases in ' . $country['name'] . ' (' . $country['geoId'] . ')');
    $tpl->tag('plotId', 'graph_accu_' . strtolower($country['geoId']));
    // prepare numbers
    $dates = array();
    $infections = array();
    $deaths = array();
    $data = $db->accumulatedNumbers($country['countryId']);
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
   * Generates the HTML snippet containing the graph with accumulated worldwide data.
   *
   * @param db       reference to the Database instance
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns false, if an error occurred.
   */
  private function generateAccumulatedGraphWorld(Database &$db, &$tpl)
  {
    // load graph section
    if (!$tpl->loadSection('graphAccumulated'))
      return false;
    $tpl->tag('title', 'Accumulated Coronavirus cases worldwide');
    $tpl->tag('plotId', 'graph_world_accu');
    // prepare numbers
    $dates = array();
    $infections = array();
    $deaths = array();
    $data = $db->accumulatedNumbersWorld();
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
   * Generates the HTML snippet containing the graph with 14-day incidence numbers of a single country.
   *
   * @param db       reference to the Database instance
   * @param country  country data (id, name, etc.)
   * @param tpl      loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns false, if an error occurred.
   */
  private function generateIncidenceGraph(Database &$db, array $country, &$tpl)
  {
    // load graph section
    if (!$tpl->loadSection('graphIncidence'))
      return false;
    // prepare numbers
    $dates = array();
    $incidence = array();
    $data = $db->incidence($country['countryId']);
    // May be an empty array, if there is no known incidence.
    if (empty($data))
    {
      return '';
    }
    $tpl->tag('title', 'Coronavirus: 14-day incidence in ' . $country['name'] . ' (' . $country['geoId'] . ')');
    $tpl->tag('plotId', 'graph_incidence14_' . strtolower($country['geoId']));
    foreach($data as $d)
    {
      $dates[] = $d['date'];
      $incidence[] = $d['incidence'];
    }
    // graph: date values
    $dates = json_encode($dates);
    if (false === $dates)
    {
      echo "Error: JSON encoding of date array failed!\n";
      return false;
    }
    $tpl->integrate('dates', $dates);
    // graph: indicence values
    $incidence = json_encode($incidence);
    if (false === $incidence)
    {
      echo "Error: JSON encoding of incidence array failed!\n";
      return false;
    }
    $tpl->integrate('incidence', $incidence);
    return $tpl->generate();
  }

  /**
   * Generates the HTML snippet containing the graph with 14-day incidence numbers of the continent.
   *
   * @param db         reference to the Database instance
   * @param continent  name of the continent
   * @param tpl        loaded template instance of main.tpl
   * @return Returns a string containing the HTML snippet, if the generation was successful.
   *         Returns false, if an error occurred.
   */
  private function generateGraphContinent(Database &$db, string $continent, &$tpl)
  {
    $traces = '';
    // load graph section
    if (!$tpl->loadSection('trace'))
      return false;
    // iterate over countries
    $countries = $db->countriesOfContinent($continent);
    foreach ($countries as $country)
    {
      $data = $db->incidence($country['countryId']);
      // May be an empty array, if there is no known incidence.
      if (empty($data))
      {
        continue;
      }
      // prepare data for plot
      $dates = array();
      $incidence = array();
      foreach($data as $d)
      {
        $dates[] = $d['date'];
        $incidence[] = $d['incidence'];
      }
      // graph: date values
      $dates = json_encode($dates);
      if (false === $dates)
      {
        echo "Error: JSON encoding of date array failed!\n";
        return false;
      }
      // graph: indicence values
      $incidence = json_encode($incidence);
      if (false === $incidence)
      {
        echo "Error: JSON encoding of incidence array failed!\n";
        return false;
      }
      // template generation for data
      $tpl->integrate('dates', $dates);
      $tpl->integrate('incidence', $incidence);
      $tpl->tag('name', $country['name']);
      $traces .= $tpl->generate();
    }
    // template: graph
    if (!$tpl->loadSection('graphContinent'))
      return false;
    $tpl->integrate('traces', $traces);
    $tpl->tag('plotId', 'continent_' . strtolower($continent));
    $tpl->tag('title', 'Coronavirus: 14-day incidence in ' . $continent);
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
    return copy(GENERATOR_ROOT . '/../src/assets/plotly-1.58.3.min.js', $this->outputDirectory . '/assets/plotly-1.58.3.min.js');
  }

  /**
   * Creates the index.hml in the output directory.
   *
   * @param countries   array containing names and ids of the countries
   * @param countries   array containing names of the continents
   * @return Returns whether the operation was successful.
   */
  private function createIndex(array $countries, array $continents)
  {
    $tpl = new Template();
    if (!$tpl->fromFile(GENERATOR_ROOT . '/../src/templates/main.tpl'))
    {
      echo "Error: Could not load main template file!\n";
      return false;
    }
    // links
    if (!$tpl->loadSection('indexLink'))
      return false;
    // worldwide links + country links
    $tpl->tag('url', './world.html');
    $tpl->tag('text', 'All countries accumulated');
    $links = $tpl->generate();
    foreach($countries as $country)
    {
      $tpl->tag('url', './' . strtolower($country['geoId']) . '.html');
      $tpl->tag('text', $country['name'] . ' (' . $country['geoId'] . ')');
      $links .= $tpl->generate();
    }
    // continent links
    $continentLinks = '';
    foreach ($continents as $continent)
    {
      $tpl->tag('url', './continent_' . strtolower($continent) . '.html');
      $tpl->tag('text', $continent);
      $continentLinks .= $tpl->generate();
    }
    // index template
    if (!$tpl->loadSection('index'))
      return false;
    $tpl->integrate('links', $links);
    $content = $tpl->generate();
    // continent index template
    if (!$tpl->loadSection('indexContinents'))
      return false;
    $tpl->integrate('links', $continentLinks);
    $content .= "<br />\n" . $tpl->generate();
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
