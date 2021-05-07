# Version history of Corona command line application

_(Note: This changelog focuses on the major changes between the different
versions. Therefore, it may not contain all changes. Especially smaller fixes or
improvements may be omitted.)_

## Version NEXT (2021-05-??)

Adjust program to new CSV layout of Canadian. (One more column has been added to
the official CSV, so the program has to be aware of that.)

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

Adjust program to new CSV layout of Canadian. (One column has been added to the
official CSV, so the program has to be aware of that.)

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

Dependencies are updated to fix vulnerablities in them:

* update smallvec to version 1.6.1 to fix
  [RUSTSEC-2021-0003](https://rustsec.org/advisories/RUSTSEC-2021-0003)
* uncritical package updates (i .e. not fixing known security vulnerabilites):
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

Adjust program to new CSV layout of Canadian. (Three new columns have been added
to the official CSV, so the program has to be aware of that.)

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
