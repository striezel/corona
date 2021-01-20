# Data for the graph generation

The tool uses the data provided by the European Centre for Disease Prevention
and Control (ECDC) at
<https://data.europa.eu/euodp/data/dataset/covid-19-coronavirus-data>.

## Contents

This directory contains an [SQLite](https://www.sqlite.org/) database containing
the case numbers for every country, `corona-ecdc-2020-12-14.db`. That database
file was created by [botvinnik](https://gitlab.com/striezel/botvinnik/), a
Matrix chat bot that comes with various features - among them the ability to
show Coronavirus case numbers per country, if requested. Since the code already
existed, it was used to create the database file. The original PHP scripts in
this repository worked with that database file, and so did the Rust application
before it was able to create its own database using the same schema.

For reference, the original CSV data from the ECDC is also provided as
`corona-daily-ecdc-2020-12-14.csv`.
