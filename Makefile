.PHONY: all decode inspect test

all: test

help:
	@echo "Makefile commands:"
	@echo "  make decode FILE=<input.json>        - Decode Metar from JSON file"
	@echo "  make inspect FILE=<input.json>       - Inspect structure of JSON"
	@echo "  make save FILE=<input.json> OUT=<output.csv> - Parse and save to CSV"
	@echo "  make test                            - Run tests" 
	@echo "  make proj_info FILE=<input.json>  - Show project genral info"
	@echo "  make fmt                             - Format the code viarustfmt"
	@echo "  make clippy                          - Run clippy linter on the code"

proj_info:
	@echo "Project general info:"
	@cargo read-manifest | jq -r '"Name: \(.name)\nVersion: \(.version)\nEdition: \(.edition)\nDescription: \(.description)\nLicense: \(.license)\nAuthor(s): \(.authors | join(", "))"'


# decode for convertion json through metar rules to csv
decode:
	@echo "Decoding METAR from JSON..."
	cargo run -- convert $(FILE)

# json structure
inspect:
	@echo "Inspecting structure of JSON file..."
	cargo run -- inspect $(FILE)

test:
	cargo test
	
# parse and save to CSV
# make save FILE=test.json OUT=out.csv
save:
	@echo "Parsing and saving to CSV..."
	cargo run -- decode $(FILE) --output $(OUT)

#also as you required:
fmt:
	@echo "Formatting code..."
	cargo fmt --all

clippy:
	@echo "Running clippy..."
	cargo clippy -- -D warnings