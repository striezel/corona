# Data for the graph generation

The tool uses the dataset provided by Our World In Data, provided at
<https://ourworldindata.org/coronavirus> and available in CSV format at
<https://covid.ourworldindata.org/data/owid-covid-data.csv>.

This dataset by Our World In Data has been collected, aggregated, and documented
by Edouard Mathieu, Hannah Ritchie, Lucas Rodés-Guirao, Cameron Appel, Daniel
Gavrilov, Charlie Giattino, Joe Hasell, Bobbie Macdonald, Saloni Dattani, Diana
Beltekian, Esteban Ortiz-Ospina, and Max Roser.

Alternatively, the (now removed) `collect` subcommand used the data provided by
the Center for Systems Science and Engineering (CSSE) at Johns Hopkins
University provided at <https://github.com/CSSEGISandData/COVID-19> and queried
the data via the API provided by disease.sh at <https://disease.sh/docs/>.

Earlier versions of the tool used the data provided by the World Health
Organization (WHO) at <https://covid19.who.int/data> in CSV format at
<https://covid19.who.int/WHO-COVID-19-global-data.csv>.

Even earlier versions of the tool used the data provided by the European Centre
for Disease Prevention and Control (ECDC) at
<https://data.europa.eu/euodp/data/dataset/covid-19-coronavirus-data>.
That data was mostly identical to the data from Johns Hopkins university, except
for some European countries where other official sources were used.

## OWID data

This directory contains an [SQLite](https://www.sqlite.org/) database containing
the case numbers for every country, `corona.db`. That database file was created
using the OWID data. The file `corona.csv` contains the corresponding data in
CSV format, but (and that's where it might get confusing) it uses the ECDC
format despite containing the Our World In Data dataset.

## Data from removed `collect` subcommand (mainly from Johns Hopkins University)

This directory also contains an [SQLite](https://www.sqlite.org/) database
containing the case numbers for every country provided by Johns Hopkins
University. The corresponding files are `corona-jhu-2023-03-09.db` or
`corona-jhu-2023-03-09.csv`, respectively, the later providing a CSV version
of the data.

Data source is mostly the Johns Hopkins University, with a few notable
exceptions:

* **Jersey**: The government of the Island of Jersey provides its official
  Coronavirus case numbers at
  <https://www.gov.je/Health/Coronavirus/Pages/CoronavirusCases.aspx>, and the
  program uses the corresponding JSON data to create the database / CSV file.

### Data sources used in previous versions

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

* **Switzerland** and **Liechtenstein**: _(only before 2023-04-05 / before v0.12.9)_
  Switzerland provides data as CSV (and JSON, too) at
  <https://www.covid19.admin.ch/> via an API entry point at
  <https://www.covid19.admin.ch/api/data/context>. Well done from a technical
  point of view. :) This data also contains numbers for Liechtenstein, not just
  Switzerland, so it is used for both countries.

* **Turkey**: _(only before 2022-08-28 / before v0.12.4)_
  The Turkish Ministry of Health provides official numbers at
  <https://covid19.saglik.gov.tr/TR-66935/genel-koronavirus-tablosu.html>, and
  the program uses those.

## Historical data sources

### WHO data

The WHO switched the update interval of their data from daily to weekly near the
end of the year 2023. Therefore, this data source can no longer be used to get
up to date data for the current day.

The latest collected data that has been available from that source is available
in the files `corona-who-2023-12-19.db` or `corona-who-2023-12-19.csv`,
respectively.

Note that the CSV file uses the ECDC format, despite containing the WHO data.

### JHU data

The Coronavirus Resource Center at Johns Hopkins University ceased its
collecting and reporting of global COVID-19 data on 10th March 2023. Therefore,
newer data must be collected from other sources.

The latest collected data that has been available from that source (via the
disease.sh API) is available in the files `corona-jhu-2023-03-09.db` or
`corona-jhu-2023-03-09.csv`, respectively.

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
