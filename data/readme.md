# Data for the graph generation

The tool uses the data provided by the Center for Systems Science and
Engineering (CSSE) at Johns Hopkins University provided at
<https://github.com/CSSEGISandData/COVID-19> and queries the data via the API
provided by disease.sh at <https://disease.sh/docs/>.

Earlier versions of the tool used the data provided by the European Centre for
Disease Prevention and Control (ECDC) at
<https://data.europa.eu/euodp/data/dataset/covid-19-coronavirus-data>.
That data was mostly identical to the data from Johns Hopkins university, except
for some European countries where other official sources were used.

## Contents

Depending on whether you want the new data or the old data (kept here for
reference) you have two options:

### New data (mainly from Johns Hopkins University)

This directory contains an [SQLite](https://www.sqlite.org/) database containing
the case numbers for every country, `corona.db`. That database
file was created by the [corona application](https://gitlab.com/striezel/corona/),
a command-line utility to get numbers on the Coronavirus.

Data source is mainly the Johns Hopkins University, with a few notable
exceptions:

* **Canada**: _(only before 2022-07-23 / before v0.12.3)_
  The government of Canada provides case numbers at
  <https://health-infobase.canada.ca/>, e. g. as CSV data
  (<https://health-infobase.canada.ca/src/data/covidLive/covid19.csv>), and the
  program uses it to generate.

* **Germany**: _(only before 2021-11-25 / before v0.10.3)_
  The data for Germany is retrieved from the Robert Koch Institute,
  the foremost public institution for disease control and prevention in Germany.
  It is an agency of the federal government of Germany. Data source is the
  spreadsheet from this site, which seems to be available in German only:
  <https://www.rki.de/DE/Content/InfAZ/N/Neuartiges_Coronavirus/Daten/Fallzahlen_Kum_Tab.xlsx>.
  A more machine-readable/-friendly and open (non-proprietary) format would be
  better, but unfortunately that is as good as it gets for the moment.

* **Jersey**: The government of the Island of Jersey provides its official
  Coronavirus case numbers at
  <https://www.gov.je/Health/Coronavirus/Pages/CoronavirusCases.aspx>, and the
  program uses the corresponding JSON data to create the database / CSV file.

* **Switzerland** and **Liechtenstein**: Switzerland provides data as CSV (and
  JSON, too) at <https://www.covid19.admin.ch/> via an API entry point at
  <https://www.covid19.admin.ch/api/data/context>. Well done from a technical
  point of view. :) This data also contains numbers for Liechtenstein, not just
  Switzerland, so it is used for both countries.

* **Turkey**: _(only before 2022-08-28 / before v0.12.4)_
  The Turkish Ministry of Health provides official numbers at
  <https://covid19.saglik.gov.tr/TR-66935/genel-koronavirus-tablosu.html>, and
  the program uses those.

### RKI data

The German Robert Koch Institute (RKI) changed the update interval of their
provided data (the spreadsheet file) on 25th November 2021 from daily to weekly
updates. Therefore, this data source can no longer be used to get up to date
data for the current day.

The latest collected data that has been available from that source is available
in the files `corona-2021-11-23.db` or `corona-2021-11-23.csv`, respectively.

### ECDC data

_Note: This data is outdated, it shows the numbers as of 14th December 2020.
After that date the ECDC stopped publishing daily case numbers and daily updates
at <https://data.europa.eu/euodp/data/dataset/covid-19-coronavirus-data>, so
that is why there is no newer data._

This directory contains an [SQLite](https://www.sqlite.org/) database containing
the case numbers for every country, `corona-ecdc-2020-12-14.db`. That database
file was created by [botvinnik](https://gitlab.com/striezel/botvinnik/), a
Matrix chat bot that comes with various features - among them the ability to
show Coronavirus case numbers per country, if requested. Since the code already
existed, it was used to create the database file. The original (now removed) PHP
script prototype worked with that database file, and so did the Rust application
before it was able to create its own database using the same schema.

For reference, the corresponding CSV data from the ECDC is also provided as
`corona-daily-ecdc-2020-12-14.csv`.
