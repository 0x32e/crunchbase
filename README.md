# crunchbase

Startup funding data analyzer

### Requirements
1. Set up a database instance with the schema in the schema.sql file.
2. Use the import command (i.e., see `/src/api/import.rs`) to ingest funding data into the db instance. (Feel free to ingest csv files in the /data directory)
3. (Optional) If you have an account on Crunchbase, download the data in csv format by clicking "EXPORT TO CSV" [here](https://www.crunchbase.com/discover/funding_rounds).

### Features
1. Querying pre-seed & seed startup funding data based on the following criteria: "Industry", "Last X days", & "Currency".
3. Importing a csv file into the db.
4. Asking questions for the fetched funding data in natural language.

### Usage

1. ```cargo build --release```
2. Import: ```./target/release/crunchbase import```
3. Query: ```./target/release/crunchbase query```

### TODO
1. Set up a central database for others to use upon approval.
2. Integrate an LLM-based agent with tools (e.g., search, access to vector store, etc.)
3. Add prompt templates
4. Add plot capability

### Note
- The quality of the answer is almost always better with GPT-4, but be aware of the cost.
