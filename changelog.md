# Version history of Corona command line application

_(Note: This changelog focuses on the major changes between the different
versions. Therefore, it may not contain all changes. Especially smaller fixes or
improvements may be omitted.)_

## Version 0.5.0 (2021-01-23)

The program can now also just display the latest case numbers for a given
country on the standard output. To do this, the first argument passed to the
program has to be `info`, followed by either the name of the country or its ISO
3166 two letter code. E. g. both

    cargo run info France

and

    cargo run info FR

will show the latest numbers for France. The same works with other countries.

## Version 0.4.2 (2021-01-20)

Users can now specify an optional fourth argument to the HTML creation mode to
use a custom template file for the HTML generation process. For example, if the
template file is located at `/home/user/my.tpl`, the program can be invoked by
this command to use that file:

    cargo run html /path/to/corona.db /path/to/new/output/directory /home/user/my.tpl

Of course, the previous way of invoking the HTML generation without an explicit
template file is still available:

    cargo run html /path/to/corona.db /path/to/new/output/directory

In that case the program will just use the original template.

## Version 0.4.1 (2021-01-17)

* Remove unnecessary date shift by one day in collected data of some countries.
* Add data for COVID-19 cases on "Diamond Princess" as hard-coded vector to the
  data collection. Since these numbers are known and will not change in the
  future, it is safe to have them hard-coded.

## Version 0.4.0 (2021-01-17)

The program can now generate a SQLite database by querying an API or similar
sources to get the case numbers. This is started when the first command line
argument is `collect`, e. g. by:

    cargo run collect /path/to/sqlite.db

## Version 0.3.1 (2020-12-31)

The program can now generate a SQLite database from a given CSV file with daily
Coronavirus data. The SQLite database creation is started when the first command
line argument is `db`, e. g. by:

    cargo run db /path/to/corona-daily.csv /path/to/sqlite.db

## Version 0.3.0 (2020-12-29)

The program can now also generate a CSV file containing all the data from the
SQLite database. The first command line argument will now determine whether the
HTML file generation or the CSV generation will start. CSV generation is run
when the first argument is `csv`, like this:

    cargo run csv /path/to/corona.db /path/to/output_file.csv

HTML generation is run when the first argument is `html`, e. g.:

    cargo run html /path/to/corona.db /path/to/new/output/directory

## Version 0.2.0 (2020-12-29)

The generation of HTML files is now fully feature-complete and a full, working
replacement for the PHP prototype.

## Version 0.1.0 (2020-12-23 to 2020-12-28)

Version 0.1.0 is not a real version but bascially just the first, incomplete
implementation of the application. Do not use that version anymore, because the
generation of the HTML files may not work or it may be incomplete.
