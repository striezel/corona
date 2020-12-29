# Coronavirus case numbers per country

This repository contains a Rust command line application that can generate
graphs showing the Coronavirus (SARS-CoV-2, COVID-19) case numbers for various
countries.

It is still in an early stage of development and very simplistic. And so are the
generated graphs.

See the <https://striezel.github.io/corona/> for a snapshot of those graphs.

## Data source

The tool uses the data provided by the European Centre for Disease Prevention
and Control (ECDC) at
<https://data.europa.eu/euodp/data/dataset/covid-19-coronavirus-data>.

## Status

* GitHub:
[![GitHub CI Rust tests](https://github.com/striezel/corona/workflows/Rust%20application%20tests/badge.svg)](https://github.com/striezel/corona/actions)
[![GitHub CI Rust linting](https://github.com/striezel/corona/workflows/Clippy%20lints/badge.svg)](https://github.com/striezel/corona/actions)
* GitLab:
[![GitLab pipeline status](https://gitlab.com/striezel/corona/badges/master/pipeline.svg)](https://gitlab.com/striezel/corona/)

## Building the application from source

### Prerequisites

To build the application you need the Rust compiler (1.30 or later should do),
Cargo (Rust's package manager) and the development libraries for SQLite3.

It also helps to have Git, a distributed version control system, on your system
to get the latest source code directly from the Git repository.

All of that can usually be installed be typing

    # Debian-based Linux distribution
    apt-get install cargo git libsqlite3-dev rustc

or

    # CentOS 8
    yum install cargo git rust sqlite-devel

or

    # Alpine
    apk add cargo git rust sqlite-dev

into a root terminal.

### Getting the source code

Get the source directly from Git by cloning the Git repository and change to
the directory after the repository is completely cloned:

    git clone https://gitlab.com/striezel/corona.git
    cd corona

That's it, you should now have the current source code on your machine.

### Build process

The build process is relatively easy, because Cargo can handle that for you.
Starting in the root directory of the source, you can invoke the following
command in a terminal to build the application:

    cargo build

Or, if you want the optimized release version, type

    cargo build --release

instead.

That's it. It may take a minute for Cargo to download the dependencies and
compile them, but after that you are ready to start using the application.

## Using the application

Currently, the application supports two modes of operation:

* `html`: creating HTML files that contain graphs showing the Coronavirus
  (SARS-CoV-2, COVID-19) case numbers for various countries
* `csv`: creating a CSV file that contains the data from the SQLite database

The mode is passed as the first command line argument to the application.
Only one mode of operation can be active during the application invocation.
Of course, you can invoke the application several times and change the mode as
you like.

### HTML file generation process (`html`)

The process is relatively easy, because you just have to pass the correct
parameters / paths to Cargo.
Starting in the root directory of the source, you can invoke the following
command in a terminal to start the process:

    cargo run html /path/to/corona.db /path/to/new/output/directory

That's it. Cargo will build the executable and run it afterwards.

Replace `/path/to/corona.db` with the path to the database. If you do not have
one ready, you can use the version provided in the `data/` subdirectory of this
Git repository. However, it may be slightly outdated, because it is not updated
on a regular schedule.

Furthermore, replace `/path/to/new/output/directory` with a path where you want
the created files to be located. Note that the directory must not exist yet,
because the application may overwrite existing files.

After that, open the `index.html` file in that directory with the browser of
your choice to get a list of available graphs by country.

### Dump database content into CSV file (`csv`)

Starting in the root directory of the source, you can invoke the following
command in a terminal to create a CSV file that contains the data from the
SQLite 3 database:

    cargo run csv /path/to/corona.db /path/to/file.csv

That's it. Cargo will build the executable and run it afterwards.

Replace `/path/to/corona.db` with the path to the database. If you do not have
one ready, you can use the version provided in the `data/` subdirectory of this
Git repository. However, it may be slightly outdated, because it is not updated
on a regular schedule.

Furthermore, replace `/path/to/file.csv` with a path where you want the CSV file
to be located. Note that the file must not exist yet, because the application
will refuse to overwrite an existing CSV file.

## Older PHP variant

A PHP variant of the code is available in the `php/` subdirectory. These PHP
scripts were the prototype of the application before it was implemented in Rust.
You probably do not want to use those scripts, they are just there to have the
archived scripts ready in case that I should ever need them again.

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
