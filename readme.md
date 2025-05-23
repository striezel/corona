# Coronavirus case numbers per country

This repository contains a Rust command line application that can generate
graphs showing the Coronavirus (SARS-CoV-2, COVID-19) case numbers for various
countries.

It is still in an early stage of development and very simplistic. And so are the
generated graphs.

See the <https://striezel.github.io/corona/> for a snapshot of those graphs.

## Data source

The tool uses the compact COVID-19 dataset by Our World In Data, provided at
<https://catalog.ourworldindata.org/garden/covid/latest/compact/compact.csv>.

Earlier versions of the tool used an older dataset by Our World In Data,
provided at <https://ourworldindata.org/coronavirus> and available in CSV format
at <https://covid.ourworldindata.org/data/owid-covid-data.csv>.

Even earlier versions of the tool used the data provided by the World Health
Organization (WHO) at <https://covid19.who.int/data> in CSV format at
<https://covid19.who.int/WHO-COVID-19-global-data.csv>.

And even earlier versions of the tool used the data provided by the European
Centre for Disease Prevention and Control (ECDC) at
<https://data.europa.eu/euodp/data/dataset/covid-19-coronavirus-data>.

For more information see the [readme file in the data/ directory](./data/readme.md).

## Build status

* General:
[![GitHub CI Rust linting](https://github.com/striezel/corona/workflows/Clippy%20lints/badge.svg)](https://github.com/striezel/corona/actions)
[![GitHub CI Rust MSRV](https://github.com/striezel/corona/workflows/Minimum%20Supported%20Rust%20Version/badge.svg)](https://github.com/striezel/corona/actions)
[![GitLab pipeline status](https://gitlab.com/striezel/corona/badges/master/pipeline.svg)](https://gitlab.com/striezel/corona/)
* By OS:
[![GitHub CI tests](https://github.com/striezel/corona/workflows/Ubuntu%20LTS/badge.svg)](https://github.com/striezel/corona/actions)
[![GitHub CI Windows with GNU toolchain](https://github.com/striezel/corona/workflows/Windows/badge.svg)](https://github.com/striezel/corona/actions)
[![GitHub CI on MacOS](https://github.com/striezel/corona/workflows/MacOS/badge.svg)](https://github.com/striezel/corona/actions)

## Building the application from source

### Prerequisites

[![minimum rustc 1.63.0](https://img.shields.io/badge/minimum%20rustc-1.63.0-c18170?logo=rust&style=for-the-badge)](https://www.whatrustisit.com/)

To build the application you need the Rust compiler. The Minimum Supported Rust
Version (MSRV) is Rust 1.63.0. Furthermore, you need Cargo (Rust's package
manager), the development libraries for SQLite3 (version 3.26.0+) and OpenSSL,
and pkg-config.

It also helps to have Git, a distributed version control system, on your system
to get the latest source code directly from the Git repository.

All of that can usually be installed by typing

    # Debian-based Linux distribution
    apt-get install cargo git libsqlite3-dev libssl-dev pkg-config rustc

or

    # Rocky Linux 8 and similar distributions
    yum install cargo git rust sqlite-devel openssl-devel

or

    # Alpine
    apk add cargo git rust sqlite-dev openssl-dev pkgconfig

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

## Building with Docker

You can also use the provided `Dockerfile` to build the application inside of a
container. Docker 17.05 or later is required for this, since the Dockerfile does
a multi-stage build.

To start the build type

    docker build . -t corona

into your terminal. This will take a while. After that is finished, you can type

    docker run --rm -d -p 3210:80 corona

to start the container. It will bind to port 3210 of the local system, so the
HTML output can be viewed at <http://localhost:3210/> in a browser on the same
system.

## Using the application

Currently, the application supports three modes of operation:

* `html`: creating HTML files that contain graphs showing the Coronavirus
  (SARS-CoV-2, COVID-19) case numbers for various countries
* `csv`: creating a CSV file that contains the data from the SQLite database
* `db`: creating a SQLite database file that contains the data from a given CSV
  file, basically the reverse of the `csv` operation

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

Since version 0.4.2 you can also specify your own template file for the HTML
generation. Take a look at the [default template file](./src/templates/main.tpl)
to get an idea what such a file can look like. The path to the custom template
file has to be given after the output directory, e. g.:

    cargo run html /path/to/corona.db /path/to/new/output/directory /home/user/my.tpl

Replace `/home/user/my.tpl` with the path where your template file is located.
Note that there is no documentation for the template file syntax yet, and it
is currently unclear whether there will ever be such documentation. Don't count
on it.

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

### Use CSV file to create SQLite database (`db`)

Starting in the root directory of the source, you can invoke the following
command in a terminal to create a SQLite 3 database that contains the data from
a given CSV file:

    cargo run db /path/to/corona-daily.csv /path/to/sqlite.db

That's it. Cargo will build the executable and run it afterwards.

Replace `/path/to/corona-daily.csv` with the path to the existing CSV file. If
you do not have one ready, you can use the version provided in the `data/`
subdirectory of this Git repository. However, it may be slightly outdated,
because it is not updated on a regular schedule.

Furthermore, replace `/path/to/sqlite.db` with a path where you want the SQLite
database file to be located. Note that the file must not exist yet, because the
application will refuse to overwrite an existing database file.

## Copyright and Licensing

Copyright 2020, 2021, 2022, 2023, 2024, 2025  Dirk Stolle

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
