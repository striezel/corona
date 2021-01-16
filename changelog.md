# Version history of Corona command line application

_(Note: This changelog focuses on the major changes between the different
versions. Therefore, it may not contain all changes. Especially smaller fixes or
improvements may be omitted.)_

## Next version 0.x.y (2021-01-??)

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
