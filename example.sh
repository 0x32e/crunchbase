#!/bin/bash

# Query from the database
cargo run query --days 30 --limit 10

# Import csv file into the database
cargo run import fundings_ai.csv