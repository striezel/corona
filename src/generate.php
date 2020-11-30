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

define('GENERATOR_ROOT', __DIR__);
require GENERATOR_ROOT . '/classes/FileGenerator.php';

$hasArgs = ini_get('register_argc_argv');
if ($hasArgs != 1)
{
  die('Error: register_argc_argv is disabled!'."\n");
}

if ($argc != 3)
{
  $help = "usage:  php generate.php /path/to/corona.db /path/to/output/directory\n";
  die($help);
}

$generator = new FileGenerator($argv[1], $argv[2]);
if (!$generator->generate())
{
  echo "Generation of HTML files failed!\n";
}
else
{
  echo "Generation of HTML files was successful.\n";
}

?>
