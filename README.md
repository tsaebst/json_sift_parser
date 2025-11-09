# JSON-Sift

**JSON-Sift** is a parser that works with weather data of civil air flights that come from APIs in JSON format.  
Such data contain various specific notations and a particular way of arrangement.  
This parser deals with recognizing embedded codes and transforming JSON into **CSV**,  
which is the most common format for working with data, processing, and analysis.  

I often work with data, and such a parser would make my work easier if, for example,  
I wanted to train a **model** on it or perform **EDA**.

---
> [!NOTE]
> Name selection

“Sift” in ukrainian means *просіювати*.  
Our data come in a very unclear format — sometimes presented just as a line of abbreviations and numbers,  
which is not visually understandable.  
My parser sifts this data through its filters and outputs data that can be worked with.  
That is why I named my project this way.

---
## Data Source
> [!NOTE]
> At the moment, the parser works with data from corresponding APIs.  
For demonstration purposes, the data are taken from the **AviationWeather (METAR)** API:  
[https://aviationweather.gov/help/data/#metar](https://aviationweather.gov/help/data/#metar)

---

## Purpose of the project

Currently, as I have already mentioned, the parser works with data from civil aviation flights.  
In general, the parser can be adapted to decode flight data of other flying devices such as **drones**,  
since this is a relevant topic in Ukraine.  

Since I don’t have access to real drone flight data, I use alternative data sources.  
In the future, if desired, the parser may include the possibility of configuration via a config file,  
in case the incoming data have a slightly different structure.

---
## Example of transformed data
Below is an example of how the raw aviation weather data looks **after being parsed and converted** into structured CSV format :

![Example of transformed data](example_transformed_data.png)
 --- 
## Getting Started

> [!TIP]
> To download the project use commands:
```
bash
git clone https://github.com/tsaebst/json_sift_parser.git
cd json_sift_parser
cargo build
cargo run

```

To start working, you need to install the project locally  
*(add detailed installation and run instructions here)*.

To begin, type:
```
make help
```

# Project files:

```
json_sift_parser/
├── Cargo.toml              # metadata and dependencies
├── Makefile                # CLI build + tests
├── README.md               # project doumentation
├── config.json             # parser patterns and rules config
├── src/
│   ├── grammar.pest        # Metar grammar defining
│   ├── lib.rs              # parsing and transformation logic
|   |── metar.rs            
│   └── main.rs             # cli entry point 
├── tests/
│   └── parser_tests.rs     # unit-tests for grammar (to be aaded for parsing logic)
├── result.csv              # outout CSV
└── test.json               # json input data
```


## grammar.pest
The METAR grammar describes how the parser recognizes weather observation strings.  
These strings typically consist of compact tokens( combinations of letters, digits, and abbreviations) — that encode different metrics

---
## About grammar

> [!IMPORTANT]
> typical input looks like this:
> UKBB 121200Z 18005KT 10SM FEW020 15/10 A2992 RMK TEST

## Each segment is a token representing a distinct type of weather information.  
The grammar processes them using `pest` rules as follows:

| Rule | Meaning | Example |
|------|----------|---------|
| `station` | 4-letter station code | `UKBB`, `KJFK`, `EGLL` |
| `time` | UTC timestamp in `HHMMSSZ` format | `121200Z` |
| `wind` | Wind direction, speed, optional gust, and units | `18005KT`, `25010G15KT` |
| `visibility` | Horizontal visibility with optional prefixes | `10SM`, `M1/2SM`, `P6SM` |
| `clouds` | Cloud layers or clear condition | `FEW020`, `BKN100`, `CLR` |
| `temp_dew` | Temperature / dew point pair | `15/10`, `M02/M05` |
| `pressure` | Atmospheric pressure (inHg) | `A2992` |
| `remarks` | Free-text remarks | `RMK AO2 SLP123` |
| `known_keyword` | Recognized control words | `COR`, `AUTO`, `NOSIG` |
| `uppercase_token` | Any unknown uppercase abbreviation | `VV`, `CB`, `TS` |
| `separator` | Whitespace or line breaks | `" "` or `"\n"` |
| `unknown_token` | Fallback for any unrecognized token | `XYZ123` |

---

## Tests

JSON-Sift includes a set of unit tests (written via cargo)to verify the correctness of the METAR grammar and the future parsing logic implemented in `lib.rs`.  

| Test Type | Description |
|------------|-------------|
| **Grammar tests (`parser_tests.rs`)** | Validate the grammar rules defined in `grammar.pest`. Each METAR component (station, time, wind, etc.) is parsed and checked for correctness. |
| **Parsing logic tests** *(planned)* | Will validate transformation from raw METAR strings into structured JSON or CSV. |
| **JSON/CSV conversion tests** *(future work)* | Ensure flattened JSON structure and correct CSV export. |


To run all unit tests:

make test

---
## Parsing architecture

The crate is split into two logical parts:

- `src/lib.rs` — public API for JSON → flat map → CSV.
- `src/metar.rs` — METAR grammar, token helpers, and decoding logic.

---

## `src/lib.rs`

* `ParseError`
Custom error type for JSON errors, structural issues, and (future) detector/pattern errors.

* `parse_json(&str) -> Result<Value, ParseError>`
Parses input string as JSON using `serde_json::from_str` and wraps failures into `ParseError::Json`.

* `convert_to_csv(&Value) -> Result<String, ParseError>`
Accepts a JSON object or array, flattens each entry, collects all keys as CSV headers, and writes rows via `csv::Writer` using stable sorted columns.

* `flatten(&Value, String, &mut HashMap<String, String>)`
Recursively walks nested JSON (objects, arrays, scalars), builds dotted/indexed keys, and delegates string values to `parse_scalar`.

* `parse_scalar(String, &str, &mut HashMap<String, String>)`
Normalizes a string, tries to decode it as METAR via `metar::decode_metar`, otherwise tokenizes and uses simple METAR-like patterns or falls back to `token_N` columns.

---

## `src/metar.rs`

* `SiftParser`
Pest-generated parser using `grammar.pest` rules for METAR reports.

* `decode_metar(&str) -> Option<HashMap<String, String>>`
Parses a full METAR string with `SiftParser`, walks the parse tree, and returns a flat map of normalized METAR fields, or `None` if nothing meaningful is found.

* `visit_metar(pair, &mut HashMap<String, String>)`
Traverses Pest parse pairs, matches rules (station, time, wind, etc.), and fills the output map by reusing `apply_pattern` where possible.

* `complex_key_value(&str) -> Vec<String>`
Splits a free-form string into tokens by whitespace and basic separators, used before pattern detection.

* `is_code_like_token(&str)` / `all_tokens_code_like(&[String])`
Detects whether tokens look like uppercase/number codes to decide if pattern parsing is safe.

* `SimplePattern`
Enum describing recognized token types like `TempDew`, `Wind`, `Pressure`, `Time`, `Visibility`, `Cloud`, `FlightCategory`.

* `holds_pattern_value(&str) -> Option<SimplePattern>`
Classifies a single token into one of the `SimplePattern` variants based on simple textual rules.

* `apply_pattern(&str, &str, SimplePattern, &mut HashMap<String, String>)`
Expands a recognized pattern token into one or more well-named columns (e.g. `wind_*`, `temp_c`, `cloud_cover`, `flight_category`), respecting optional key prefix.

* `norm(&str) -> String`
Normalizes raw text by trimming, cleaning trailing symbols, and collapsing whitespace for more robust matching.
---

## Error handling

- All library-facing functions return `Result<_, ParseError>` so callers get a single, typed error surface.
- JSON issues (invalid syntax, wrong encoding) become `ParseError::Json`.
- Structural problems (unsupported top-level type, CSV write failures, unexpected shapes) become `ParseError::Structure`.
- `Detector` / pattern-related variants are reserved for future, more detailed parsing diagnostics without changing the public API.


---
> [!WARNING]
> to be done

## MAIN file

The `main.rs` file defines the **CLI interface** and the high-level program flow.  
Its main goal is to connect logic from `lib.rs` with  user commands.

Before executing any of the commands, the program uses functions from `lib.rs`  to perform all data handling.

As a concept for now, but a plan for the future i might add: 
- Loads a configuration file (`config.json`) if available.



## How to run

You can interact with **JSON-Sift** directly from the terminal using Cargo or Make commands.

- **Run the program:**
``` 
bash
cargo run
```

- **Parse and save**
```
cargo run -- decode test.json --output result.csv
make decode FILE=test.json OUT=result.csv CONFIG=config.json
```

- **Credits**
```
cargo run -- credits

```

**Crates.io** – [check it out](https://crates.io/crates/json_sift_parser)

## Author
**Vladyslava Spitkovska** – [GitHub](https://github.com/tsaebst)
