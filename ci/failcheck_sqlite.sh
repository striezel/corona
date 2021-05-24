#!/usr/bin/env bash

# This script checks whether SQLite3-related checks fail when using an old
# version of SQLite. It needs SQLite3 before 3.26.0 but after 3.6.7 to work
# correctly.

# version should still work.
cargo run -- version
if [ $? -eq 0 ]
then
  echo Version operation succeeded as expected.
else
  echo Version operation failed, but is should succeed!
  exit 1
fi

# info should still work.
cargo run -- info France
if [ $? -eq 0 ]
then
  echo Info operation succeeded as expected.
else
  echo Info operation failed, but is should succeed!
  exit 1
fi

# csv should still work, it only gives a warning.
cargo run -- csv data/corona.db /tmp/csv_check.csv
if [ $? -eq 0 ]
then
  echo Csv operation succeeded as expected.
  echo "The first few lines of the CSV file are:"
  head /tmp/csv_check.csv
else
  echo Csv operation failed, but is should not!
  exit 1
fi

# html should still work, it only gives a warning.
cargo run -- html data/corona.db /tmp/html_check
if [ $? -eq 0 ]
then
  echo Html operation succeeded as expected.
else
  echo Html operation failed, but is should not!
  exit 1
fi

# collect should fail.
cargo run -- collect /tmp/collect.db
if [ $? -ne 0 ]
then
  echo Collect operation failed as expected.
else
  echo Collect operation succeeded, but is should not!
  exit 1
fi

# db should fail.
cargo run -- db data/corona.csv /tmp/db_check.db
if [ $? -ne 0 ]
then
  echo Db operation failed as expected.
else
  echo Db operation succeeded, but is should not!
  exit 1
fi

exit 0
