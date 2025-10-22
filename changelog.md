# Version history of Corona command line application

_(Note: This changelog focuses on the major changes between the different
versions. Therefore, it may not contain all changes. Especially smaller fixes or
improvements may be omitted.)_

## Version 0.?.? (2025-10-??)

Dependency updates:

* update bitflags to 2.10.0
* update cfg-if to 1.0.4
* update csv to 1.4.0
* update csv-core to 0.1.13
* update generic-array to 0.14.9
* update libc to 0.2.177
* update regex to 1.12.2
* update regex-automata to 0.4.13
* update regex-syntax to 0.8.8
* update syn to 2.0.107
* update unicode-ident to 1.0.20

## Version 0.20.0 (2025-10-05)

The Minimum Supported Rust Version (MSRV) is bumped to 1.75.0. Rust 1.75.0 has
been released on 28th December 2023, more than a year ago, so it is probably
safe to update to that version. For the corresponding Rust release announcement
see <https://blog.rust-lang.org/2023/12/28/Rust-1.75.0/>.

The plotly.js library is updated from version 3.1.0 to version 3.1.1.

Dependency updates:

* update bitflags to 2.9.4
* update chrono to 0.4.42
* update csv to 1.3.1
* update libc to 0.2.176
* update memchr to 2.7.6
* update once_cell to 1.21.3
* update quote to 1.0.41
* update regex to 1.11.3
* update regex-automata to 0.4.11
* update regex-syntax to 0.8.6
* update serde to 1.0.228
* update typenum to 1.19.0
* update unicode-ident to 1.0.19
* update zerocopy to 0.8.27

## Version 0.19.3 (2025-08-20)

__[maintenance]__
The plotly.js library is updated from version 3.0.1 to version 3.1.0.

Dependency updates:

* update ahash to 0.8.12
* update autocfg to 1.5.0
* update bitflags to 2.9.2
* update cfg-if to 1.0.3
* update libc to 0.2.175
* update memchr to 2.7.5
* update proc-macro2 to 1.0.101
* update sha2 to 0.10.9
* update smallvec to 1.15.1
* update syn to 2.0.106
* update zerocopy to 0.8.26

## Version 0.19.2 (2025-04-29)

__[improvement]__
The `db` command will now skip some implausible data from the compact CSV format
used by Our World in Data. By implausible we mean data from dates in the future
and non-contiguous dates. Previously that data was included and could lead to
unexpected gaps in the data.

Dependency updates:

* update chrono to 0.4.41
* update libc to 0.2.172
* update proc-macro2 to 1.0.95
* update smallvec to 1.15.0
* update syn to 2.0.101

## Version 0.19.1 (2025-03-17)

__[improvement]__
The `db` command will now also extract the data for Kosovo from the compact CSV
format used by Our World in Data. Previously that data was skipped.

Dependency updates:

* update bitflags to 2.9.0
* update itoa to 1.0.15
* update libc to 0.2.171
* update pkg-config to 0.3.32
* update proc-macro2 to 1.0.94
* update quote to 1.0.40
* update ryu to 1.0.20
* update serde to 1.0.219
* update syn to 2.0.100
* update unicode-ident to 1.0.18

## Version 0.19.0 (2025-02-24)

__[improvement]__
It is no longer required to download the plotly.js file over the internet during
graph generation. The file is now embedded in the compiled executable. This
allows to drop a lot of other dependencies, resulting in a smaller executable
file despite the extra embedded file.

## Version 0.18.0 (2025-02-23)

__[feature]__
The program can now create a SQLite database from the data of a given CSV file
with daily Coronavirus data using the "compact" CSV format used by Our World in
Data at
<https://catalog.ourworldindata.org/garden/covid/latest/compact/compact.csv>.
The SQLite database creation uses the `db` subcommand, just like for the old
Our World In Data, ECDC and WHO formats. Assuming the file
<https://catalog.ourworldindata.org/garden/covid/latest/compact/compact.csv>
was downloaded to `/path/to/owid-compact.csv` then the database can be
created by invoking

    cargo run db /path/to/owid-compact.csv /path/to/sqlite.db

In that case, the database file will be created at `/path/to/sqlite.db`.

The `db` command still supports the ECDC and WHO formats as well as the older
pre-ETL format of Our World In Data. It will detect the format of a CSV file by
looking at its headers.

__[fix]__
A bug with the handling of the values for the country of Montserrat is fixed.

__[maintenance]__
The plotly.js library is updated from version 2.35.2 to version 3.0.1.

__[improvement]__
The graph for general incidence values of a country is now rendered even if
there only are either 7-day or 14-day incidence values. It is no longer required
that both values are present.

__[maintenance]__
Dependency updates:

* update ahash to 0.8.11
* update bitflags to 2.8.0
* update cc to 1.2.15
* update cpufeatures to 0.2.17
* update csv-core to 0.1.12
* update fallible-iterator to 0.3.0
* update flate2 to 1.0.35
* update hashbrown to 0.14.5
* update hashlink to 0.9.1
* update itoa to 1.0.14
* update libc to 0.2.170
* update libsqlite3-sys to 0.28.0
* update log to 0.4.26
* update miniz_oxide to 0.8.5
* update once_cell to 1.20.3
* update proc-macro2 to 1.0.93
* update quote to 1.0.38
* update ring to 0.17.11
* update rusqlite to 0.31.0
* update rustls to 0.23.19
* update rustls-pki-types to 1.11.0
* update ryu to 1.0.19
* update serde to 1.0.218
* update smallvec to 1.14.0
* update syn to 2.0.98
* update tinyvec to 1.8.1
* update typenum to 1.18.0
* update unicode-bidi to 0.3.18
* update unicode-ident to 1.0.17
* update url to 2.5.2
* update webpki-roots to 0.26.8

## Version 0.17.2 (2024-10-26)

The library for HTTP(S) requests is switched from `reqwest` to `ureq` to reduce
the number of overall dependencies.

The plotly.js library is updated from version 2.29.1 to version 2.35.2.

Dependency updates:

* update aho-corasick to 1.1.3
* update base64 to 0.22.1
* update bitflags to 2.6.0
* update cc to 1.1.31
* update cpufeatures to 0.2.14
* update crc32fast to 1.4.2
* update flate2 to 1.0.34
* update getrandom to 0.2.15
* update itoa to 1.0.11
* update libc to 0.2.161
* update log to 0.4.22
* update memchr to 2.7.4
* update miniz_oxide to 0.8.0
* update once_cell to 1.20.2
* update pkg-config to 0.3.31
* update proc-macro2 to 1.0.89
* update quote to 1.0.37
* update rustls to 0.23.15
* update rustls-pki-types to 1.10.0
* update rustls-webpki to 0.102.8
* update ryu to 1.0.18
* update serde to 1.0.213
* update smallvec to 1.13.2
* update subtle to 2.6.1
* update syn to 2.0.85
* update tinyvec to 1.8.0
* update unicode-bidi to 0.3.17
* update unicode-ident to 1.0.13
* update unicode-normalization to 0.1.24
* update ureq to 2.10.1
* update version_check to 0.9.5
* update webpki-roots to 0.26.6
* update windows-targets, windows_aarch64_gnullvm, windows_aarch64_msvc,
  windows_i686_gnu, windows_i686_msvc, windows_x86_64_gnu,
  windows_x86_64_gnullvm, windows_x86_64_msvc to 0.52.6
* update zerocopy to 0.7.35
* update zeroize to 1.8.1

## Version 0.17.1 (2024-02-23)

The plotly.js library is updated from version 2.29.0 to version 2.29.1.

Dependency updates:

* update cc to 1.0.86
* update hermit-abi to 0.3.6
* update indexmap to 2.2.3
* update openssl to 0.10.64
* update openssl-sys to 0.9.101
* update pkg-config to 0.3.30
* update ryu to 1.0.17
* update serde to 1.0.197
* update serde_json to 1.0.114
* update syn to 2.0.50
* update unicode-normalization to 0.1.23
* update windows-targets, windows_aarch64_gnullvm, windows_aarch64_msvc,
  windows_i686_gnu, windows_i686_msvc, windows_x86_64_gnu,
  windows_x86_64_gnullvm, windows_x86_64_msvc to 0.52.3

## Version 0.17.0 (2024-02-06)

The program can now create a SQLite database from the data of a given CSV file
with daily Coronavirus data using the CSV format used by Our World in Data at
<https://covid.ourworldindata.org/data/owid-covid-data.csv>. The SQLite database
creation uses the `db` subcommand, just like for the ECDC and WHO formats.
Assuming the file <https://covid.ourworldindata.org/data/owid-covid-data.csv>
was downloaded to `/path/to/owid-covid-data.csv` then the database can be
created by invoking

    cargo run db /path/to/owid-covid-data.csv /path/to/sqlite.db

In that case, the database file will be created at `/path/to/sqlite.db`.

The `db` command still supports the ECDC and WHO formats. It will detect the
format of a CSV file by looking at its headers.

The plotly.js library is updated from version 2.28.0 to version 2.29.0.

Dependency updates:

* update hermit-abi to 0.3.5
* update indexmap to 2.2.2
* update js-sys to 0.3.68
* update libc to 0.2.153
* update miniz_oxide to 0.7.2
* update reqwest to 0.11.24
* update rustix to 0.38.31
* update tempfile to 3.10.0
* update tokio to 1.36.0
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.91
* update wasm-bindgen-futures to 0.4.41
* update web-sys to 0.3.68

## Version 0.16.0 (2024-01-30)

__[breaking change]__
The `collect` and `info` commands are removed.

Both of them relied on the disease.sh API to get their data, but the server
providing that data has been failing for a while now. This means that both of
those commands are practically useless, with the server effectively being gone.
This is a breaking change, but removing functionality that could not be used
due to the API server not working anymore is the right course of action here.

## Version 0.15.3 (2024-01-30)

The plotly.js library is updated from version 2.26.2 to version 2.28.0.

Dependency updates:

* update ahash to 0.8.7
* update base64 to 0.21.7
* update bitflags to 2.4.2
* update core-foundation to 0.9.4
* update core-foundation-sys to 0.8.6
* update cpufeatures to 0.2.12
* update errno to 0.3.8
* update form_urlencoded to 1.2.1
* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  futures-util to 0.3.30
* update getrandom to 0.2.11
* update gimli to 0.28.1
* update h2 to 0.3.24
* update hashbrown to 0.14.3
* update hashlink to 0.8.2
* update hermit-abi to 0.3.4
* update http to 0.2.11
* update http-body to 0.4.6
* update hyper to 0.14.28
* update idna to 0.5.0
* update indexmap to 2.2.1
* update ipnet to 2.9.0
* update itoa to 1.0.10
* update js-sys to 0.3.67
* update libc to 0.2.152
* update linux-raw-sys to 0.4.13
* update memchr to 2.7.1
* update mio to 0.8.10
* update object to 0.32.2
* update once_cell to 1.19.0
* update openssl to 0.10.63
* update openssl-sys to 0.9.99
* update percent-encoding to 2.3.1
* update pkg-config to 0.3.29
* update proc-macro2 to 1.0.78
* update quote to 1.0.35
* update redox_syscall to 0.4.1
* update reqwest to 0.11.23
* update rustix to 0.38.30
* update ryu to 1.0.16
* update schannel to 0.1.23
* update serde to 1.0.196
* update serde_json to 1.0.113
* update smallvec to 1.13.1
* update syn to 2.0.48
* update tempfile to 3.9.0
* update tokio to 1.35.1
* update tokio-util to 0.7.10
* update tracing to 0.1.40
* update tracing-core to 0.1.32
* update try-lock to 0.2.5
* update unicode-bidi to 0.3.15
* update url to 2.5.0
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.89
* update wasm-bindgen-futures to 0.4.40
* update web-sys to 0.3.67
* update zerocopy to 0.7.32

## Version 0.15.2 (2023-10-10)

The program does now use `https://disease.sh/` as base URL for API
requests again, because the alternative URL `https://corona.lmao.ninja/`
currently seems to have problems with redirects, redirecting to itself. That
causes all API requests to fail because of an infinite redirect loop. To work
around this issue, the canonical domain, `https://disease.sh/`, is now used in
all API requests. Earlier certificate problems with this domain seem to have
been fixed since the switch in version 0.15.1.

The plotly.js library is updated from version 2.26.0 to version 2.26.2.

Dependency updates:

* update aho-corasick to 1.1.2
* update csv to 1.3.0
* update csv-core to 0.1.11
* update errno to 0.3.5
* update fastrand to 2.0.1
* update hermit-abi to 0.3.3
* update libc to 0.2.149
* update linux-raw-sys to 0.4.10
* update memchr to 2.6.4
* update proc-macro2 to 1.0.69
* update regex to 1.9.6
* update regex-automata to 0.3.9
* update reqwest to 0.11.22
* update rustix to 0.38.18
* update sha2 to 0.10.8
* update smallvec to 1.11.1
* update syn to 2.0.38
* update tokio to 1.33.0
* update tokio-util to 0.7.9

## Version 0.15.1 (2023-09-16)

The program does now use `https://corona.lmao.ninja/` as base URL for API
requests, because the canonical URL `https://disease.sh/` currently seems to
have problems with Cloudflare (like 526 Invalid SSL certificate). That causes
all API requests to fail. To work around this issue, the alternative domain with
a valid certificate, `https://corona.lmao.ninja/`, is now used in all API
requests.

The plotly.js library is updated from version 2.24.3 to version 2.26.0.

Dependency updates:

* update addr2line to 0.21.0
* update aho-corasick to 1.0.5
* update backtrace to 0.3.69
* update base64 to 0.21.4
* update bitflags to 2.4.0
* update bumpalo to 3.14.0
* update bytes to 1.5.0
* update cc to 1.0.83
* update encoding_rs to 0.8.33
* update errno to 0.3.3
* update gimli to 0.28.0
* update h2 to 0.3.21
* update httpdate to 1.0.3
* update libc to 0.2.148
* update linux-raw-sys to 0.4.7
* update log to 0.4.20
* update memchr to 2.6.3
* update object to 0.32.1
* update openssl to 0.10.57
* update openssl-sys to 0.9.93
* update pin-project-lite to 0.2.13
* update proc-macro2 to 1.0.67
* update quote to 1.0.33
* update regex to 1.9.5
* update regex-automata to 0.3.8
* update regex-syntax to 0.7.5
* update reqwest to 0.11.20
* update rustix to 0.38.13
* update security-framework to 2.9.2
* update security-framework-sys to 2.9.1
* update serde to 1.0.188
* update serde_json to 1.0.107
* update slab to 0.4.9
* update socket2 to 0.5.4
* update syn to 2.0.35
* update tempfile to 3.8.0
* update tokio to 1.32.0
* update typenum to 1.17.0
* update unicode-ident to 1.0.12
* update url to 2.4.1
* update windows-targets, windows_aarch64_gnullvm, windows_aarch64_msvc,
  windows_i686_gnu, windows_i686_msvc, windows_x86_64_gnu,
  windows_x86_64_gnullvm and windows_x86_64_msvc to 0.48.5
* update winreg to 0.50.0

## Version 0.15.0 (2023-07-21)

The Minimum Supported Rust Version (MSRV) is bumped to 1.63.0. Rust 1.63.0 has
been released on 11th August 2022, almost a year ago, so it is probably safe to
update to that version. For the corresponding Rust release announcement see
<https://blog.rust-lang.org/2022/08/11/Rust-1.63.0.html>.

The plotly.js library is updated from version 2.24.0 to version 2.24.3.

Dependency updates:

* update addr2line to 0.20.0
* update aho-corasick to 1.0.2
* update backtrace to 0.3.68
* update base64 to 0.21.2
* update bumpalo to 3.13.0
* update csv to 1.2.2
* update cpufeatures to 0.2.9
* update fastrand to 2.0.0
* update h2 to 0.3.20
* update hermit-abi to 0.3.2
* update hyper to 0.14.27
* update ipnet to 2.8.0
* update itoa to 1.0.9
* update js-sys to 0.3.64
* update libc to 0.2.147
* update linux-raw-sys to 0.4.3
* update log to 0.4.19
* update miniz_oxide to 0.7.1
* update num_cpus to 1.16.0
* update object to 0.31.1
* update once_cell to 1.18.0
* update openssl to 0.10.55
* update openssl-sys to 0.9.90
* update pin-project-lite to 0.2.10
* update proc-macro2 to 1.0.66
* update quote to 1.0.31
* update regex to 1.9.1
* update regex-syntax to 0.7.4
* update reqwest to 0.11.18
* update rusqlite to 0.29.0
* update rustix to 0.38.4
* update ryu to 1.0.15
* update schannel to 0.1.22
* update security-framework to 2.9.1
* update serde to 1.0.173
* update serde_json to 1.0.103
* update sha2 to 0.10.7
* update smallvec to 1.11.0
* update syn to 2.0.27
* update tempfile to 3.7.0
* update time to 0.3.20
* update tokio to 1.29.1
* update unicode-ident to 1.0.11
* update want to 0.3.1
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.87
* update wasm-bindgen-futures to 0.4.37
* update web-sys to 0.3.64
* update windows-targets to 0.48.1

## Version 0.14.3 (2023-06-06)

The performance of the `collect` operation is slightly improved by organizing
the database insert operations into larger batches.

A workaround for database operation failure on older SQLite versions is added.

The plotly.js library is updated from version 2.23.1 to version 2.24.0.

Dependency updates:

* update form_urlencoded to 1.2.0
* update getrandom to 0.2.10
* update idna to 0.4.0
* update libc to 0.2.146
* update percent-encoding to 2.3.0
* update url to 2.4.0

## Version 0.14.2 (2023-06-03)

This version fixes a mistake in a test case.

## Version 0.14.1 (2023-06-03)

A mistake in an error message for the `db` command is fixed.

When parsing a WHO CSV file into a database, the Kosovo will now be correctly
named "Kosovo" instead of "Kosovo[1]". This is a quirk in the WHO data, and the
application will now work around it.

The part of the `db` command that handles the ECDC variant of CSV files is
refactored and also creates the total numbers directly. The new version is
faster and reduces the overhead when running the `html` command after the
database creation.

The `collect` and `info` commands can now return data for Guernsey, too.

Dependency updates:

* update digest to 0.10.7
* update io-lifetimes to 1.0.11
* update linux-raw-sys to 0.3.8
* update log to 0.4.18
* update mio to 0.8.8
* update once_cell to 1.17.2
* update openssl to 0.10.54
* update openssl-sys to 0.9.88
* update proc-macro2 to 1.0.59
* update quote to 1.0.28
* update security-framework to 2.8.1
* update syn to 2.0.18
* update tokio to 1.28.2
* update tokio-util to 0.7.8
* update unicode-ident to 1.0.9

## Version 0.14.0 (2023-05-18)

The Minimum Supported Rust Version (MSRV) is bumped to 1.56.1. Rust 1.56.1 has
been released on 1st November 2021, a bit more than a year ago, so it is probably
safe to update to that version. For the Rust release announcement see
<https://blog.rust-lang.org/2021/11/01/Rust-1.56.1.html>.

The plotly.js library is updated from version 2.22.0 to version 2.23.1.

Dependency updates:

* update bumpalo to 3.12.0
* update h2 to 0.3.19
* update http to 0.2.9
* update hyper to 0.14.26
* update indexmap to 1.9.3
* update io-lifetimes to 1.0.10
* update js-sys to 0.3.63
* update libsqlite3-sys to 0.24.2
* update mio to 0.8.6
* update num_cpus to 1.15.0
* update once_cell to 1.17.1
* update openssl to 0.10.52
* update openssl-macros to 0.1.1
* update proc-macro2 to 1.0.58
* update rusqlite to 0.27.0
* update rustix to 0.37.19
* update security-framework to 2.7.0
* update security-framework-sys to 2.9.0
* update time to 0.3.9
* update tokio to 1.28.1
* update tokio-native-tls to 0.3.1
* update tracing-attributes to 0.1.24
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-futures,
  wasm-bindgen-macro, wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.86
* update web-sys to 0.3.63

## Version 0.13.0 (2023-05-18)

The program can now create a SQLite database from the data of a given CSV file
with daily Coronavirus data using the CSV format used by the World Health
Organization (WHO) at <https://covid19.who.int/WHO-COVID-19-global-data.csv>.
The SQLite database creation uses the `db` subcommand, just like for the ECDC
format. Assuming the file <https://covid19.who.int/WHO-COVID-19-global-data.csv>
was downloaded to `/path/to/WHO-COVID-19-global-data.csv` then the database can
be created by invoking

    cargo run db /path/to/WHO-COVID-19-global-data.csv /path/to/sqlite.db

In that case, the database file will be created at `/path/to/sqlite.db`.

The `db` command still supports the ECDC format. It will detect the format of a
CSV file by looking at its headers.

Dependency updates:

* update libc to 0.2.144
* update proc-macro2 to 1.0.57
* update quote to 1.0.27
* update serde to 1.0.163
* update tracing-core to 0.1.31

## Version 0.12.11 (2023-05-05)

The plotly.js library is updated from version 2.20.0 to version 2.22.0.

Dependency updates:

* update linux-raw-sys to 0.3.7
* update openssl-sys to 0.9.87
* update pkg-config to 0.3.27

## Version 0.12.10 (2023-04-22)

The Johns Hopkins Coronavirus Resource Center, currently the main source for
data of this program, ceased its collecting and reporting of global COVID-19
data on 10th March 2023. Therefore, newer data can currently not be displayed
by this program and the `info` command will now show a note that explains this.

Dependency updates:

* update cpufeatures to 0.2.7
* update errno to 0.3.1
* update getrandom to 0.2.9
* update libc to 0.2.142
* update linux-raw-sys to 0.3.3
* update openssl to 0.10.50
* update openssl-sys to 0.9.86
* update serde to 1.0.160
* update serde_json to 1.0.96

## Version 0.12.9 (2023-04-05)

The data collection for Liechtenstein and Switzerland is switched to the
disease.sh API. The previous data source, data provided via an API by the Swiss
government, has not received data updates after 1st January 2023, and therefore
it lacks more recent data.

The plotly.js library is updated from version 2.18.1 to version 2.20.0.

Dependency updates:

* update block-buffer to 0.10.4
* update core-foundation-sys to 0.8.4
* update cpufeatures to 0.2.6
* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  futures-util to 0.3.28
* update generic-array to 0.14.7
* update hyper to 0.14.25
* update ipnet to 2.7.2
* update itoa to 1.0.6
* update io-lifetimes to 1.0.6
* update libc to 0.2.141
* update linux-raw-sys to 0.3.1
* update mime to 0.3.17
* update openssl to 0.10.49
* update openssl-sys to 0.9.84
* update proc-macro2 to 1.0.56
* update quote to 1.0.26
* update regex to 1.7.3
* update regex-syntax to 0.6.29
* update rustix to 0.37.7
* update ryu to 1.0.13
* update serde to 1.0.159
* update serde_json to 1.0.95
* update slab to 0.4.8
* update socket2 to 0.4.9
* update syn to 1.0.109
* update tempfile to 3.5.0
* update unicode-bidi to 0.3.13
* update unicode-ident to 1.0.8
* update windows-targets, windows_aarch64_gnullvm, windows_aarch64_msvc,
  windows_i686_gnu, windows_i686_msvc, windows_x86_64_gnu,
  windows_x86_64_gnullvm, windows_x86_64_msvc to 0.42.2

## Version 0.12.8 (2023-02-17)

In previous versions, the data for Liechtenstein showed the number of deaths for
Switzerland instead of the numbers for Liechtenstein. This is now fixed.

Dependency updates:

* update fastrand to 1.9.0
* update proc-macro2 to 1.0.51
* update serde_json to 1.0.93
* update tinyvec_macros to 0.1.1

## Version 0.12.7 (2023-02-03)

The plotly.js library is updated from version 2.12.1 to version 2.18.1.

Dependency updates:

* update aho-corasick to 0.7.20
* update base64 to 0.13.1
* update block-buffer to 0.10.3
* update bytes to 1.4.0
* update cc to 1.0.79
* update cpufeatures to 0.2.5
* update digest to 0.10.6
* update encoding_rs to 0.8.32
* update form_urlencoded to 1.1.0
* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  futures-util to 0.3.26
* update getrandom to 0.2.8
* update httparse to 1.8.0
* update hyper to 0.14.24
* update idna to 0.3.0
* update ipnet to 2.7.1
* update itoa to 1.0.5
* update js-sys to 0.3.61
* update libc to 0.2.139
* update native-tls to 0.2.11
* update num_cpus to 1.14.0
* update once_cell to 1.14.0
* update openssl to 0.10.45
* update openssl-sys to 0.9.80
* update percent-encoding to 2.2.0
* update pkg-config to 0.3.26
* update proc-macro2 to 1.0.50
* update quote to 1.0.23
* update reqwest to 0.11.13
* update regex to 1.7.1
* update regex-syntax to 0.6.28
* update ryu to 1.0.12
* update schannel to 0.1.21
* update serde to 1.0.152
* update serde_json to 1.0.91
* update sha2 to 0.10.6
* update smallvec to 1.10.0
* update socket2 to 0.4.7
* update syn to 1.0.107
* update tracing to 0.1.37
* update tracing-attributes to 0.1.23
* update tracing-core to 0.1.30
* update try-lock to 0.2.4
* update typenum to 1.16.0
* update unicode-bidi to 0.3.10
* update unicode-ident to 1.0.6
* update unicode-normalization to 0.1.22
* update url to 2.3.1
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.84
* update wasm-bindgen-futures to 0.4.34
* update web-sys to 0.3.61
* update windows-sys to 0.42.0
* update windows_aarch64_gnullvm, windows_aarch64_msvc, windows_i686_gnu,
  windows_i686_msvc, windows_x86_64_gnu, windows_x86_64_gnullvm,
  windows_x86_64_msvc to 0.42.1

## Version 0.12.6 (2022-08-28)

Downgrade `security-framework` dependency to version 2.6.1 stay true to the
current MSRV 1.48.

## Version 0.12.5 (2022-08-28)

Revert update of `bumpalo` dependency to stay true to the current MSRV 1.48.

## Version 0.12.4 (2022-08-28)

The generated HTML files do now contain a `<meta>` element to indicate that the
used character encoding is UTF-8. This avoids problems with non-ASCII characters
in some browsers.

The data collection for Turkey is switched to the disease.sh API. The previous
data source, JSON data of the Turkish Health Ministry, has changed its update
interval from daily to weekly updates, and therefore it cannot be used to get up
to date data for the current day anymore.

Dependency updates:

* update bumpalo to 3.11.0 (breaks MSRV in theory, but in practice this only
  affects WebAssembly builds, and we do not build for WebAssembly targets at the
  moment, because it's not supported for this project yet)
* update bytes to 1.2.1
* update cpufeatures to 0.2.4
* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  and futures-util to 0.3.23
* update generic-array to 0.14.6
* update itoa to 1.0.3
* update js-sys to 0.3.59
* update libc to 0.2.132
* update once_cell to 1.13.1
* update proc-macro2 to 1.0.43
* update quote to 1.0.21
* update redox_syscall to 0.2.16
* update ryu to 1.0.11
* update security-framework to 2.7.0
* update serde to 1.0.144
* update serde_json to 1.0.85
* update socket2 to 0.4.6
* update syn to 1.0.99
* update tracing to 0.1.36
* update tracing-core to 0.1.29
* update unicode-ident to 1.0.3
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.82
* update wasm-bindgen-futures to 0.4.32
* update web-sys to 0.3.59

## Version 0.12.3 (2022-07-24)

The data collection for Canada is switched to the disease.sh API. The previous
data source, a CSV file of the Public Health Agency of Canada, has changed its
update interval from daily to weekly updates, and therefore it cannot be used
to get up to date data for the current day anymore.

Dependency updates:

* update bytes to 1.2.0
* update crypto-common to 0.1.6
* update fastrand to 1.7.0
* update hyper to 0.14.20
* update once_cell to 1.13.0
* update openssl to 0.10.41
* update openssl-sys to 0.9.75
* update redox_syscall to 0.2.15
* update regex to 1.6.0
* update regex-syntax to 0.6.27
* update serde to 1.0.140
* update slab to 0.4.7
* update tracing-attributes to 0.1.22
* update unicode-ident to 1.0.2
* update unicode-normalization to 0.1.21

## Version 0.12.2 (2022-06-30)

Ignore JSON elements with empty dates returned by Jersey's API.
(It seems the last element contains completely empty data at the moment, i. e.
not only the date but also case numbers etc. are just empty strings. This is
obviously an error, so it is just ignored.)

The regular expression for retrieval of JSON data for Turkey is adjusted to
match the new, current website.

Dependency updates:

* update bumpalo to 3.10.0
* update getrandom to 0.2.7
* update http-body to 0.4.5
* update hyper to 0.14.19
* update indexmap to 1.8.2
* update js-sys to 0.3.58
* update once_cell to 1.12.0
* update openssl-sys to 0.9.74
* update proc-macro2 to 1.0.40
* update quote to 1.0.20
* update regex to 1.5.6
* update regex-syntax to 0.6.26
* update reqwest to 0.11.11
* update schannel to 0.1.20
* update serde_json to 1.0.82
* update smallvec to 1.9.0
* update syn to 1.0.98
* update tower-service to 0.3.2
* update tracing to 0.1.35
* update tracing-core to 0.1.28
* update unicode-ident to 1.0.1
* update unicode-normalization to 0.1.20
* update wasi to 0.11.0+wasi-snapshot-preview1
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.81
* update wasm-bindgen-futures to 0.4.31
* update web-sys to 0.3.58

## Version 0.12.1 (2022-05-17)

The certificate validation for requesting data from health-infobase.canada.ca
for Canada has been disabled, because the current certificate chain is
incomplete. However, this is the official source for Canadian COVID numbers, so
there is no better workaround at the moment, if we want to get data from there.

Furthermore, the plotly.js library is updated from version 1.58.5 to version
2.12.1. This is a breaking change, because it removes support for Internet
Explorer 9 and 10. Those browsers have reached end of life (IE 9 on 14th January
2020 and IE 10 on 31st January 2020) and nobody should be using those anymore.
Therefore it's reasonably safe to drop support for them and update plotly.js.

Dependency updates:

* update cc to 1.0.73
* update cpufeatures to 0.2.2
* update crypto-common to 0.1.3
* update digest to 0.10.3
* update encoding_rs to 0.8.31
* update getrandom to 0.2.6
* update h2 to 0.3.13
* update httparse to 1.7.1
* update hyper to 0.14.18
* update indexmap to 1.8.1
* update ipnet to 2.5.0
* update itoa to 1.0.2
* update js-sys to 0.3.57
* update libc to 0.2.126
* update log to 0.4.17
* update memchr to 2.5.0
* update native-tls to 0.2.10
* update once_cell to 1.10.0
* update openssl to 0.10.40
* update openssl-sys to 0.9.73
* update pin-project-lite to 0.2.9
* update pkg-config to 0.3.25
* update proc-macro2 to 1.0.39
* update quote to 1.0.18
* update redox_syscall to 0.2.13
* update regex to 1.5.5
* update reqwest to 0.11.10
* update ryu to 1.0.10
* update serde to 1.0.137
* update serde_json to 1.0.81
* update sha2 to 0.10.2
* update slab to 0.4.6
* update syn to 1.0.95
* update tokio-util to 0.7.2
* update tracing to 0.1.34
* update tracing-attributes to 0.1.21
* update tracing-core to 0.1.26
* update tinyvec to 1.6.0
* update unicode-bidi to 0.3.8
* update unicode-xid to 0.2.3
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.80
* update wasm-bindgen-futures to 0.4.30
* update web-sys to 0.3.57
* update winreg to 0.10.1

## Version 0.12.0 (2022-02-16)

The Minimum Supported Rust Version (MSRV) is bumped to 1.48.0. Rust 1.48.0 has
been released on 19th November 2020, a bit more than a year ago, so it is probably
safe to update to that version. For the Rust release announcement see
<https://blog.rust-lang.org/2020/11/19/Rust-1.48.html>.

The `chrono` dependency is removed in favour of a newer version of the `time`
dependency. That way two vulnerabilities in those dependencies are fixed.
[RUSTSEC-2020-0071](https://rustsec.org/advisories/RUSTSEC-2020-0071) /
[CVE-2020-26235](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-26235)
is fixed by upgrading the `time` crate.
[RUSTSEC-2020-0159](https://rustsec.org/advisories/RUSTSEC-2020-0159) is fixed
by removing the dependency on the `chrono` crate.

Dependency updates:

* update autocfg to 1.1.0
* update block-buffer to 0.10.2
* remove chrono
* update core-foundation to 0.9.3
* update crypto-common to 0.1.2
* update digest to 0.10.2
* update fastrand to 1.7.0
* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  and futures-util to 0.3.21
* update h2 to 0.3.11
* update httparse to 1.6.0
* update hyper to 0.14.17
* update libc to 0.2.118
* update ntapi to 0.3.7
* update quote to 1.0.15
* update security-framework and security-framework-sys to 2.6.1
* update serde to 1.0.136
* update serde_json to 1.0.79
* update socket2 to 0.4.4
* update time to 0.3.2
* update tokio to 1.16.1
* update tracing to 0.1.30
* update tracing-core to 0.1.22

## Version 0.11.1 (2022-01-21)

The countries American Samoa, Kiribati, Micronesia (i. e. Federated States of
Micronesia), Samoa, Palau and Tonga are added to the program. Numbers for those
countries can now be queried via the `info` sub command, and data collection
will now include those countries, too.

Dependency updates:

* update js-sys to 0.3.56
* update libc to 0.2.113
* update security-framework + security-framework-sys to 2.5.0
* update serde to 1.0.134
* update serde_json to 1.0.75
* update serde_urlencoded to 0.7.1
* update socket2 to 0.4.3
* update syn to 1.0.86
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.79
* update wasm-bindgen-futures to 0.4.29
* update web-sys to 0.3.56

## Version 0.11.0 (2022-01-16)

A new plot is added to the graphs for each country: a comparison of the
incidence values by year. This should provide an easier way to see how the cases
are changing over the years. Note that this plot is only available for those
countries where 7-day incidence values are available.

Dependency updates:

* update bumpalo to 3.9.1
* update getrandom to 0.2.4
* update h2 to 0.3.10
* update indexmap to 1.8.0
* update openssl-probe to 0.1.5
* update reqwest to 0.11.9
* update sha2 to 0.10.1
* update smallvec to 1.8.0
* update syn to 1.0.85
* update tempfile to 3.3.0

## Version 0.10.5 (2022-01-07)

The database library for SQLite, rusqlite, is updated to version 0.25.4. It
contains some minor breaking changes and API changes, but in the context of this
application it does not break database operations.

Dependency updates:

* update ahash to 0.7.6
* remove hashbrown 0.9.1
* update hashlink to 0.7.0
* update libsqlite3-sys to 0.22.2
* update rusqlite to 0.25.4

## Version 0.10.4 (2022-01-06)

The minified plotly.js library is replaced by the minified plotly-basic.js file.
That version contains all plot types that the application needs, but it is
significantly smaller than the full library (3.3 MB vs. 0.9 MB).

Furthermore, the plotly.js library is updated from version 1.58.3 to version
1.58.5.

Dependency updates:

* update encoding_rs to 0.8.30
* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  and futures-util to 0.3.19
* update generic-array to 0.14.5
* update h2 to 0.3.9
* update http to 0.2.6
* update hyper to 0.14.16
* update libc to 0.2.112
* update num_cpus to 1.13.1
* update once_cell to 1.9.0
* update openssl-sys to 0.9.72
* update pin-project-lite to 0.2.8
* update pkg-config to 0.3.24
* update ppv-lite86 to 0.2.16
* update proc-macro2 to 1.0.36
* update quote to 1.0.14
* update reqwest to 0.11.8
* update ryu to 1.0.9
* update serde to 1.0.133
* update serde_json to 1.0.74
* update syn to 1.0.84
* update tokio to 1.15.0
* update typenum to 1.15.0
* update version_check to 0.9.4

## Version 0.10.3 (2021-11-26)

The data collection for Germany is switched to the disease.sh API. The previous
data source, a spreadsheet of the Robert Koch Institute, has changed its update
interval from daily to weekly updates, and therefore it cannot be used to get
up to date data for the current day anymore.

Dependency updates:

* update bumpalo to 3.8.0
* update cc to 1.0.72
* update crc32fast to 1.2.2
* update encoding_rs to 0.8.29
* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  and futures-util to 0.3.18
* update h2 to 0.3.7
* update http-body to 0.4.4
* update httpdate to 1.0.2
* update hyper to 0.14.15
* update libc to 0.2.108
* update mio to 0.7.14
* update openssl to 0.10.38
* update openssl-sys to 0.9.71
* update pkg-config to 0.3.22
* update ppv-lite86 to 0.2.15
* update proc-macro2 to 1.0.32
* update reqwest to 0.11.6
* update serde_json to 1.0.72
* update syn to 1.0.82
* update tinyvec to 1.5.1
* update tokio to 1.14.0
* update tokio-util to 0.6.9

## Version 0.10.2 (2021-10-16)

Switch to canonical URL of disease.sh API.

The program does now use `https://disease.sh/` as base URL for API requests.
This seems to be more reliable than the previously used alternative base URL
`https://corona.lmao.ninja/` which had problems with Cloudflare (like 502 Bad
Gateway, etc.) rather often, so that requests sometimes failed due to outages
of it or problems with Cloudflare.

Dependency updates:

* update cc to 1.0.71
* update core-foundation to 0.9.2
* update core-foundation-sys to 0.8.3
* update h2 to 0.3.6
* update libc to 0.2.104
* update ppv-lite86 to 0.2.14
* update proc-macro2 to 1.0.30
* update quote to 1.0.10
* update reqwest to 0.11.5
* update slab to 0.4.5
* update smallvec to 1.7.0
* update syn to 1.0.80
* update thiserror and thiserror-impl to 1.0.30
* update tracing to 0.1.28
* update tracing-core to 0.1.21
* update unicode-bidi to 0.3.7

## Version 0.10.1 (2021-09-27)

The reqwest library has been updated to 0.11.x, fixing some vulnerabilities in
indirect dependencies and updating and / or removing some indirect dependencies.

Fixed vulnerabilities:
* RUSTSEC-2021-0078 in hyper
  (see <https://rustsec.org/advisories/RUSTSEC-2021-0078>)
* RUSTSEC-2021-0079 in hyper
  (see <https://rustsec.org/advisories/RUSTSEC-2021-0079>)
* RUSTSEC-2020-0016 in net2, dependency is removed
  (see <https://rustsec.org/advisories/RUSTSEC-2020-0016>)

Furthermore, rusqlite, the crate for SQLite database handling, has been updated
to 0.24.2, bringing SQLite 3.33.0 when using the bundled SQLite version, e. g.
on Windows builds.

Dependency updates:

* remove fuchsia-zircon, fuchsia-zircon-sys
* update h2 to 0.3.4
* update http-body to 0.4.3
* update httpdate to 1.0.1
* update hyper to 0.14.13
* update hyper-tls to 0.5.0
* remove iovec
* update libsqlite3-sys to 0.20.1
* remove linked-hash-map, lru-cache, kernel32-sys, mime_guess
* update mio to 0.7.13
* update miow to 0.3.7
* remove net2, pin-project and pin-project-internal
* update reqwest to 0.11.4
* update rusqlite to 0.24.2
* update socket2 to 0.4.2
* update tokio to 1.12.0
* move from tokio-tls to tokio-native-tls 0.3.0
* update tokio-util to 0.6.8
* remove tracing-futures, unicase, winapi-build, ws2_32-sys

## Version 0.10.0 (2021-09-27)

The Minimum Supported Rust Version (MSRV) is bumped to 1.46.0. Rust 1.46.0 has
been released on 27th August 2020, a bit more than a year ago, so it is probably
safe to update to that version. For the Rust release announcement see
<https://blog.rust-lang.org/2020/08/27/Rust-1.46.0.html>.

Also, the version pinnings for the crates bitflags, tracing and tracing-code
have been removed.

Dependency updates:

* update bitflags to 1.3.2
* update http to 0.2.5
* update security-framework to 2.4.2
* update tracing to 0.1.28
* update tracing-core to 0.1.20

## Version 0.9.2 (2021-09-26)

This version brings various dependency updates, but no significant new features.

Dependency updates:

* update bstr to 0.2.17
* update bumpalo to 3.7.1
* update bytes to 1.1.0
* update cc to 1.0.70
* update cpufeatures to 0.2.1
* update flate2 to 1.0.22
* update futures-channel, futures-core, futures-io, futures-sink, futures-task
  and futures-util to 0.3.17
* update hashbrown to 0.11.2
* update httparse to 1.5.1
* update indexmap to 1.7.0
* update itoa to 0.4.8
* update js-sys to 0.3.55
* update libc to 0.2.102
* update matches to 0.1.9
* update memchr to 2.4.1
* update native-tls to 0.2.8
* update openssl to 0.10.36
* update openssl-sys to 0.9.67
* update pkg-config to 0.3.20
* update pin-project + pin-project-internal to 1.0.8
* update proc-macro2 to 1.0.29
* update redox_syscall to 0.2.10
* update security-framework-sys to 2.4.2
* update serde to 1.0.130
* update serde_json to 1.0.68
* update sha2 to 0.9.8
* update slab to 0.4.4
* update syn to 1.0.77
* update thiserror and thiserror-impl to 1.0.29
* update tinyvec to 1.5.0
* update typenum to 1.14.0
* update unicode-bidi to 0.3.6
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.78
* update wasm-bindgen-futures to 0.4.28
* update web-sys to 0.3.55

## Version 0.9.1 (2021-06-28)

Case numbers for the Cook Islands are no longer included in the case numbers of
New Zealand. The Cook Islands are now listed as a separate country.

Dependency updates:

* update bumpalo to 3.7.0
* update cpufeatures to 0.1.5
* update hermit-abi to 0.1.19
* update ipnet to 2.3.1
* update libc to 0.2.97
* update once_cell to 1.8.0
* update openssl to 0.10.35
* update openssl-sys to 0.9.65
* update pin-project-lite to 0.2.7
* update rand to 0.8.4
* update rand_chacha to 0.3.1
* update rand_core to 0.6.3
* update rand_hc to 0.3.1
* update redox_syscall to 0.2.9
* update regex-automata to 0.1.10
* update security-framework to version 2.3.1
* update security-framework-sys to version 2.3.0
* update syn to 1.0.73
* update unicode-normalization to 0.1.19
* update vcpkg to 0.2.15
* update zip to 0.5.13

## Version 0.9.0 (2021-05-29)

* The `collect` operation does now directly generate the accumulated case
  numbers. That way they do not have to be generated by the first invocation of the
  `html` operation by updating every row in the covid19 table. While this may
  slightly slow down the `collect`, it gives a huge boost to the `html`
  operation.

  In other words: When you do a `collect` followed by an `html` operation on the
  database created during `collect`, the overall procedure is now much faster,
  because SQLite can handle batch inserts of several thousand records faster
  than several thousands of single row update statements. (This should be no
  surprise to anyone who knows a bit about SQL performance, but I just haven't
  gotten around to implement it the faster way before.)

* Furthermore, the `collect` operation will now try to use less HTTP requests,
  resulting in a speedup of ca. 50 % relative to version 0.8.5. Your numbers may
  vary, depending on network, I/O and processor performance. (Before the change,
  it took ca. 140 seconds to do a collect operation on my laptop, now it only
  takes ca. 62 seconds.)

* The `version` command does now show the version of SQLite, too.

* All database-related operations will now check the used version of SQLite and
  abort, if the SQLite version is too old.

* Checks for the header of the RKI spreadsheet containing data for Germany has
  been relaxed to cope with the slightly modified content.

* Fix off-by-one error when parsing RKI spreadsheet.

* Dependency updates:

  * update cc to version 1.0.68
  * update getrandom to 0.2.3
  * update libc to 0.2.95
  * update thiserror and thiserror-impl to 1.0.25
  * update vcpkg to 0.2.13

## Version 0.8.5 (2021-05-20)

The data for Jersey is improved in the following ways:

* Older data from dates before 2020-08-01 is re-added, if it is missing.
  (The official JSON seems to lack those data sometimes.)
* Data for 8th and 9th April 2021 is fixed, if the numbers are -52 and 52 on
  those days. This is an odd datapoint in the series, where basically both days
  have zero deaths.

Dependency updates:

* update proc_macro2 to 1.0.27

## Version 0.8.4 (2021-05-20)

The minified plotly.js file is now downloaded from a CDN, if it is missing. That
ensures that the HTML file generation also works when the binary is not invoked
via `cargo` or not from the repository's root directory.

## Version 0.8.3 (2021-05-15)

Adjust program to new CSV layout of Canadian data.

Dependency updates:

* update serde to 1.0.126

## Version 0.8.2 (2021-05-11)

Update JSON parsing of Jersey, because the official JSON data has changed its
format / layout.

Dependency updates:

* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  futures-util to 0.3.15
* update httparse to 1.4.1
* update js-sys to 0.3.51
* update openssl-probe to 0.1.4
* update url to 2.2.2
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-futures,
  wasm-bindgen-macro, wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.74
* update web-sys to 0.3.51

## Version 0.8.1 (2021-05-07)

Adjust program to new CSV layout of Canadian data. (One more column has been
added to the official CSV, so the program has to be aware of that.)

Dependency updates:

* update regex to 1.5.4
* update openssl-sys to 0.9.63

## Version 0.8.0 (2021-05-05)

The incidence plots for the countries do now show the 7-day incidence values,
too. These are shown in addition to 14-day incidence values, so the viewers can
choose whichever incidence plots they prefer.

Dependency updates:

* update redox_syscall to 0.2.8

## Version 0.7.0 (2021-05-04)

* The date format of the CSV output is changed to match the ISO 8601 date
  format. To give an example, the date "23/09/2020" is now represented as
  "2020-09-23".
* Negative incidence values (due to correctional subtractions) are shown in
  plots, too.

Dependency updates:

* update aho-corasick to 0.7.18
* update bstr to 0.2.16
* update memchr to 2.4.0
* update regex to 1.5.3
* update regex-syntax to 0.6.25
* update syn to 1.0.72

## Version 0.6.0 (2021-04-30)

Fix handling of negative 14-day incidences in CSV creation.


Dependency updates:

* update openssl to 0.10.34
* update openssl-sys to 0.9.61
* update redox_syscall to 0.2.7
* update unicode-xid to 0.2.2

## Version 0.5.7 (2021-04-28)

Adjust program to new CSV layout of Canadian data. (One column has been added to
the official CSV, so the program has to be aware of that.)

Dependency updates:

* switch from adler32 (version 1.2.0) to adler (version 1.0.2) as part of the
  update for miniz_oxide
* update flate2 to 1.0.20
* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  futures-util to 0.3.14
* update http to 0.2.4
* update httparse to 1.4.0
* update idna to 0.2.3
* update libc to 0.2.94
* update miniz_oxide to 0.4.4 (also switches crate adler32 for adler)
* update pin-project + pin-project-internal to 1.0.7
* update redox_syscall to 0.2.6
* update regex to 1.4.6
* update slab to 0.4.3
* update syn to 1.0.71
* update tinyvec to 1.2.0
* update unicode-bidi to 0.3.5
* update vcpkg to 0.2.12
* update zip to 0.5.12

## Version 0.5.6 (2021-04-04)

In some cases there were "gaps" in the data collected for Jersey. These gaps are
now filled by supplying records with zero cases and deaths for the missing dates
in those gaps. That way the incidence values around and after the previous gaps
are also correct again.

Dependency updates:

* update byteorder to 1.4.3
* update csv to 1.1.6
* update indexmap t0 1.6.2
* update js-sys to 0.3.50
* update libc to 0.2.92
* update once_cell to 1.7.2
* update openssl to 0.10.33
* update openssl-sys to 0.9.61
* update pin-project and pin-project-internal to 1.0.6
* update pin-project-lite to 0.1.12 or 0.2.6 respectively
* update proc-macro2 to 1.0.26
* update regex to 1.4.5
* update regex-syntax to 0.6.23
* update security-framework and security-framework-sys to 2.2.0
* update serde to 1.0.125
* update serde_json to 1.0.64
* update syn to 1.0.68
* drop thread_local as part of regex update
* update version_check to 0.9.3
* update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
  wasm-bindgen-macro-support, wasm-bindgen-shared to 0.2.73
* update wasm-bindgend-futures to 0.4.23
* update web-sys to 0.3.50
* update zip to 0.5.11

## Version 0.5.5 (2021-02-25)

An omission in date parsing for the Excel file of Germany's Robert Koch
Institute is fixed, so that all dates are extracted. This omission was
introduced in version 0.5.4 while updating the library that reads Excel files
(calamine) to its newest version, because dates get parsed different in the
newer version.

Dependency updates:

* update once_cell to 1.7.0
* update security-framework and security-framework-sys to 2.1.1
* update serde_json to 1.0.62

## Version 0.5.4 (2021-02-23)

Dependency updates:

* update bumpalo to 3.6.1
* update calamine to 0.18.0 to fix
  [RUSTSEC-2021-0015](https://rustsec.org/advisories/RUSTSEC-2021-0015)
* update cc to 1.0.67
* update form_urlencoded to 1.0.1
* update futures-channel, futures-core, futures-io, futures-sink, futures-task,
  futures-util to 0.3.13
* update idna to 0.2.1
* update once_cell to 1.6.0
* update quote to 1.0.9
* update rand_core to 0.6.2
* update redox_syscall to 0.2.5
* update thiserror and thiserror-impl to 1.0.24
* update tracing to 0.1.25
* update tracing-futures to 0.2.5
* update unicode-normalization to 0.1.17
* update url to 2.2.1
* update zip to 0.5.10

## Version 0.5.3 (2021-02-09)

If data collection fails for some countries, those countries are listed at the
end of the output now. Output of collect is now also a bit nicer.

Dependencies are updated:

* update bstr to 0.2.15
* update hyper to 0.13.10
* update idna to 0.2.1
* update libc to 0.2.86
* update serde_json to 1.0.62

## Version 0.5.2 (2021-02-05)

Dependencies are updated to fix vulnerabilities in them:

* update smallvec to version 1.6.1 to fix
  [RUSTSEC-2021-0003](https://rustsec.org/advisories/RUSTSEC-2021-0003)
* uncritical package updates (i .e. not fixing known security vulnerabilities):
  * update bumpalo to version 3.6.0
  * update byteorder to version 1.4.2
  * update encoding_rs to version 0.8.28
  * update futures-channel, futures-core, futures-io, futures-sink, futures-task
    and futures-util to version 0.3.12
  * update getrandom to version 0.2.2
  * update hermit-abi to version 0.1.18
  * update http to version 0.2.3
  * update httparse to version 1.3.5
  * update libc to version 0.2.85
  * update linked-hash-map to version 0.5.3
  * update log to version 0.4.14
  * update js-sys to version 0.3.47
  * update pin-project, pin-project-internal from version 1.0.2 to 1.0.5
  * update pin-project-lite from 0.2.0 to 0.2.4
  * update rand to version 0.8.3
  * update rand_chacha to version 0.3.0
  * update rand_core to version 0.6.1
  * update rand_hc to version 0.3.0
  * update redox_syscall to version 0.2.4
  * update regex to version 1.4.3
  * update regex-syntax to version 0.6.22
  * update serde to version 1.0.123
  * update syn to version 1.0.60
  * update tempfile to version 3.2.0
  * downgrade time to version 0.1.43
  * update thread_local to version 1.1.3
  * update tinyvec to version 1.1.1
  * update tokio to version 0.2.25
  * update tower-service to version 0.3.1
  * update tracing to version 0.1.23
  * update wasi to version v0.10.2+wasi-snapshot-preview1
  * update wasm-bindgen, wasm-bindgen-backend, wasm-bindgen-macro,
    wasm-bindgen-macro-support, wasm-bindgen-shared to version 0.2.70
  * update wasm-bindgen-futures to version 0.4.20

## Version 0.5.1 (2021-02-03)

Adjust program to new CSV layout of Canadian data. (Three new columns have been
added to the official CSV, so the program has to be aware of that.)

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

Version 0.1.0 is not a real version but basically just the first, incomplete
implementation of the application. Do not use that version anymore, because the
generation of the HTML files may not work or it may be incomplete.
