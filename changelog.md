# Version history of Corona command line application

_(Note: This changelog focuses on the major changes between the different
versions. Therefore, it may not contain all changes. Especially smaller fixes or
improvements may be omitted.)_

## Next version (2021-02-??)

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
