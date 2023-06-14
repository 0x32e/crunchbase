#!/bin/bash

# Query from the database
cargo run query --days 60 --limit 10 --currency USD --industry "Machine"

# Import csv file into the database
cargo run import fundings_ai.csv
