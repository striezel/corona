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

class Database
{
  private $pdo;

  /**
   * Constructs a new database object with the given connection information.
   *
   * @param path   path to the SQLite3 database
   */
  public function __construct(string $path)
  {
    if (empty($path) || !is_readable($path))
      throw new ValueError('Database path is empty or not readable!');

    // create PDO
    $dsn = 'sqlite:' . $path;
    try {
      $this->pdo = new PDO($dsn);
    } catch (PDOException $e) {
      $this->pdo = null;
      // Logging to LOG_USER, because that is the only one available on Windows
      // systems and we want to be cross-platform.
      openlog('generator', LOG_CONS | LOG_PERROR | LOG_PID, LOG_USER);
      syslog(LOG_WARNING, 'Connection to database failed: ' . $e->getMessage());
      closelog();
    }
  }

  /**
   * Lists all countries in the database.
   *
   * @return Returns an array of arrays containing country data.
   */
  public function countries()
  {
    if (null === $this->pdo)
      throw new Exception('There is no database connection!');
    $sql = 'SELECT countryId, name, population, geoId, continent'
         . ' FROM country'
         . " WHERE geoId <> '' AND continent <> 'Other'"
         . ' ORDER BY name ASC;';
    $stmt = $this->pdo->query($sql);
    $data = array();
    while (false !== ($row = $stmt->fetch(PDO::FETCH_ASSOC)))
    {
      $data[] = array(
        'countryId' => $row['countryId'],
        'name' => $row['name'],
        'population' => $row['population'],
        'geoId' => $row['geoId'],
        'continent' => $row['continent']
      );
    }
    $stmt->closeCursor();
    unset($stmt);
    return $data;
  }

  /**
   * Get Covid-19 numbers for a specific country.
   *
   * @param countryId   id of the country
   * @return Returns an array of arrays containing the date, infections and deaths on that day.
   */
  public function numbers(int $countryId)
  {
    if (null === $this->pdo)
      throw new Exception('There is no database connection!');

    $stmt = $this->pdo->prepare(
           'SELECT date, cases, deaths FROM covid19'
         . ' WHERE countryId = :cid'
         . ' ORDER BY date ASC;');
    if (!$stmt->execute(array(':cid' => $countryId)))
    {
      throw new Exception('Could not execute prepared statement to get numbers for id ' . $countryId . '!');
    }
    $data = array();
    while (false !== ($row = $stmt->fetch(PDO::FETCH_ASSOC)))
    {
      $data[] = array(
        'date' => $row['date'],
        'cases' => intval($row['cases']),
        'deaths' => intval($row['deaths'])
      );
    }
    $stmt->closeCursor();
    unset($stmt);
    return $data;
  }

  /**
   * Get total Covid-19 numbers worldwide.
   *
   * @return Returns an array of arrays containing the date, infections and deaths of that date.
   */
  public function numbersWorld()
  {
    if (null === $this->pdo)
      throw new Exception('There is no database connection!');

    $stmt = $this->pdo->query(
           'SELECT date, SUM(cases), SUM(deaths) FROM covid19'
         . ' GROUP BY date'
         . ' ORDER BY date ASC;');
    $data = array();
    while (false !== ($row = $stmt->fetch(PDO::FETCH_NUM)))
    {
      $data[] = array(
        'date' => $row[0],
        'cases' => intval($row[1]),
        'deaths' => intval($row[2])
      );
    }
    $stmt->closeCursor();
    unset($stmt);
    return $data;
  }

  /**
   * Get accumulated Covid-19 numbers for a specific country.
   *
   * @param countryId   id of the country
   * @return Returns an array of arrays containing the date, total infections and total deaths until this date.
   */
  public function accumulatedNumbers(int $countryId)
  {
    if (null === $this->pdo)
      throw new Exception('There is no database connection!');

    $stmt = $this->pdo->prepare(
           'SELECT date, totalCases, totalDeaths FROM covid19'
         . ' WHERE countryId = :cid'
         . ' ORDER BY date ASC;');
    if (!$stmt->execute(array(':cid' => $countryId)))
    {
      throw new Exception('Could not execute prepared statement to get numbers for id ' . $countryId . '!');
    }
    $data = array();
    while (false !== ($row = $stmt->fetch(PDO::FETCH_ASSOC)))
    {
      $data[] = array(
        'date' => $row['date'],
        'cases' => intval($row['totalCases']),
        'deaths' => intval($row['totalDeaths'])
      );
    }
    $stmt->closeCursor();
    unset($stmt);
    return $data;
  }

  /**
   * Get accumulated total Covid-19 numbers worldwide.
   *
   * @return Returns an array of arrays containing the date, infections and deaths up to that date.
   */
  public function accumulatedNumbersWorld()
  {
    if (null === $this->pdo)
      throw new Exception('There is no database connection!');

    $stmt = $this->pdo->query(
           'SELECT date, SUM(totalCases), SUM(totalDeaths) FROM covid19'
         . ' GROUP BY date'
         . ' ORDER BY date ASC;');
    $data = array();
    while (false !== ($row = $stmt->fetch(PDO::FETCH_NUM)))
    {
      $data[] = array(
        'date' => $row[0],
        'cases' => intval($row[1]),
        'deaths' => intval($row[2])
      );
    }
    $stmt->closeCursor();
    unset($stmt);
    return $data;
  }

  /**
   * Checks whether the table covid19 already has the columns totalCases and
   * totalDeaths, and creates them, if they are missing.
   *
   * @return Returns whether the operation was successful.
   */
  public function calculateTotalNumbers()
  {
    if (null === $this->pdo)
      throw new Exception('There is no database connection!');

    $hasTotalCases = false;
    $hasTotalDeaths = false;
    $stmt = $this->pdo->query('PRAGMA table_info(covid19);');
    while (false !== ($row = $stmt->fetch(PDO::FETCH_NUM)))
    {
      if ($row[1] == 'totalCases')
      {
        $hasTotalCases = true;
      }
      else if ($row[1] == 'totalDeaths')
      {
        $hasTotalDeaths = true;
      }
    }
    $stmt->closeCursor();
    unset($stmt);

    if (!$hasTotalCases)
    {
      if (!$this->calculateTotalCases())
        return false;
    }
    if (!$hasTotalDeaths)
    {
      if (!$this->calculateTotalDeaths())
        return false;
    }

    return true;
  }

  /**
   * Creates the column totalCases and calculates all required values for it.
   * This may take quite a while.
   *
   * @return Returns whether the operation was successful.
   */
  private function calculateTotalCases()
  {
    if (null === $this->pdo)
      throw new Exception('There is no database connection!');
    // add new column
    $affected = $this->pdo->exec('ALTER TABLE covid19 ADD COLUMN totalCases INTEGER;');
    if ($affected === false)
    {
      $error = $this->pdo->errorInfo();
      echo "Error: Could not add new column 'totalCases' to table!\n"
         . 'Error: ' . $error[2] . "\n";
      return false;
    }
    echo "Calculating accumulated number of cases for each day and country. "
       . "This may take a while...\n";
    $affected = $this->pdo->exec(
                'UPDATE covid19 AS c1'
              . ' SET totalCases=(SELECT SUM(cases) FROM covid19 AS c2'
              . ' WHERE c2.countryId = c1.countryId AND c2.date <= c1.date);');
    return false !== $affected;
  }

  /**
   * Creates the column totalCases and calculates all required values for it.
   * This may take quite a while.
   *
   * @return Returns whether the operation was successful.
   */
  private function calculateTotalDeaths()
  {
    if (null === $this->pdo)
      throw new Exception('There is no database connection!');
    // add new column
    $affected = $this->pdo->exec('ALTER TABLE covid19 ADD COLUMN totalDeaths INTEGER;');
    if ($affected === false)
    {
      $error = $this->pdo->errorInfo();
      echo "Error: Could not add new column 'totalDeaths' to table!\n"
         . 'Error: ' . $error[2] . "\n";
      return false;
    }
    // Update may take ca. two minutes.
    echo "Calculating accumulated number of deaths for each day and country. "
       . "This may take a while...\n";
    $affected = $this->pdo->exec(
                'UPDATE covid19 AS c1'
              . ' SET totalDeaths=(SELECT SUM(deaths) FROM covid19 AS c2'
              . ' WHERE c2.countryId = c1.countryId AND c2.date <= c1.date);');
    return false !== $affected;
  }
}
?>
