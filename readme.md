# Coronavirus case numbers per country

This repository contains a few PHP scripts that can generate graphs showing the
Coronavirus (SARS-CoV-2, COVID-19) case numbers for various countries.

It is still in an early stage of development and very simplistic. And so are the
generated graphs.

See the <https://striezel.github.io/corona/> for a snapshot of those graphs.

## Data source

The tool uses the data provided by the European Centre for Disease Prevention
and Control (ECDC) at
<https://data.europa.eu/euodp/data/dataset/covid-19-coronavirus-data>.

## Status

GitHub:
[![GitHub CI PHP status](https://github.com/striezel/corona/workflows/PHP%20syntax%20check/badge.svg)](https://github.com/striezel/corona/actions)

## Generating the HTML files

### Prerequisites

To generate the HTML files containing the graphs for the Coronavirus case
numbers you need PHP 7 and the SQLite3 extension for PHP.

It also helps to have Git, a distributed version control system, on your system
to get the latest source code directly from the Git repository.

All of that can usually be installed be typing

    # Debian-based Linux distribution
    apt-get install git php-cli php-json php-sqlite3

or

    # CentOS 8
    yum install git php-cli php-json php-pdo

or

    # Alpine
    apk add git php-cli php-json php-pdo_sqlite

into a root terminal.

### Getting the source code

Get the source directly from Git by cloning the Git repository and change to
the directory after the repository is completely cloned:

    git clone https://gitlab.com/striezel/corona.git
    cd corona

That's it, you should now have the current source code on your machine.

### File generation process

The process is relatively easy, because you just have to pass the correct
parameters / paths to the script.
Starting in the root directory of the source, you can invoke the following
command in a terminal to start the process:

    php src/generate.php /path/to/corona.db /path/to/new/output/directory

That's it.

Replace `/path/to/corona.db` with the path to the database. If you do not have
one ready, you can use the version provided in the `data/` subdirectory of this
Git repository. However, it may be slightly outdated, because it is not updated
on a regular schedule.

Furthermore, replace `/path/to/new/output/directory` with a path where you want
the created files to be located. Note that the directory must not exist yet,
because the script will try to create it.

After that, open the `index.html` file in that directory with the browser of
your choice to get a list of available graphs by country.

## Copyright and Licensing

Copyright 2020  Dirk Stolle

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
