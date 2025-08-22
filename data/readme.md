# Data Directory

> **Note:** This directory contains all the data files required by the various DNS services in
> rdns.toys. Instructions and file requirements may change as the project is in an early stage of
> development.

## Files

### `cities15000.txt`

A comprehensive database of world cities with geographic coordinates, used by the timezone service
to determine the timezone for any location on Earth.

**Size:** ~7.3MB

**Source:** [Geonames](http://download.geonames.org/export/dump/)

**Format:** Tab-separated values with city information including coordinates, population, and
country codes

**How to set it up:**

1. Go to [Geonames export dump](http://download.geonames.org/export/dump/)
2. Download the `cities15000.zip` file
3. Unzip the file and place the `cities15000.txt` file in this directory

**Required for:** timezone service

### `excuses.txt`

A collection of humorous developer excuses, used by the excuse service to return random excuses when
developers need to explain why something isn't working.

**Size:** ~17KB

**Format:** One excuse per line, with comments starting with `#`

**Sources:**

- [michelegera/devexcuses-api](https://github.com/michelegera/devexcuses-api)
- [thecatontheflat/excuses](https://github.com/thecatontheflat/excuses)
- [afreeorange/developer-excuses](https://github.com/afreeorange/developer-excuses)
- [lnfnunes/WFH-excuses](https://github.com/lnfnunes/WFH-excuses)
- And others

**How to set it up:** This file is included in the repository and should be ready to use.

**Required for:** excuse service

### `vitamins.json`

A structured database of vitamin information including common names, scientific names, and food
sources, used by the vitamin service to provide nutritional information.

**Size:** ~4.1KB

**Format:** JSON with vitamin data including common names, scientific names, and food sources

**Source:** Data from [MedlinePlus](https://medlineplus.gov/ency/article/002399.htm)

**How to set it up:** This file is included in the repository and should be ready to use.

**Required for:** vitamin service

## Directories

### `ifsc/`

A collection of Indian Financial System Code (IFSC) data files for bank branch lookups, used by the
IFSC service to provide banking information for Indian banks.

**Usage:** Contains Indian Financial System Code (IFSC) data for bank branch lookups

**Format:** JSON files, one per bank (e.g., `SBIN.json`, `HDFC.json`)

**Source:**
[Razorpay IFSC releases](https://github.com/razorpay/ifsc/releases/download/latest/by-bank.tar.gz)

**How to set it up:** Run `scripts/fetch-ifsc.sh data/ifsc` to fetch the data

**Required for:** ifsc service

### `wordnet/`

The WordNet lexical database, a comprehensive English dictionary that groups words into sets of
cognitive synonyms (synsets), used by the dict service to provide word definitions and
relationships.

**Usage:** Contains WordNet dictionary data for the dict service

**Files:**

- `data.adj`, `data.adv`, `data.noun`, `data.verb` - Main dictionary data
- `index.adj`, `index.adv`, `index.noun`, `index.verb` - Index files
- `index.sense` - Sense index

**Source:** [WordNet database](https://wordnetcode.princeton.edu/3.0/WNdb-3.0.tar.gz)

**How to set it up:**

1. Download from [WordNet website](https://wordnetcode.princeton.edu/3.0/WNdb-3.0.tar.gz)
2. Extract the tarball
3. Rename the extracted directory to `wordnet`
4. The directory should contain files: `data.noun`, `data.adj`, `data.adv`, `data.verb`

**Required for:** dict service

## Expected Directory Structure

After setup, your data directory should look like:

```
data/
├── cities15000.txt
├── excuses.txt
├── vitamins.json
├── ifsc/
│   ├── SBIN.json
│   ├── HDFC.json
│   └── ... (many more bank files)
└── wordnet/
    ├── data.adj
    ├── data.adv
    ├── data.noun
    ├── data.verb
    ├── index.adj
    ├── index.adv
    ├── index.noun
    ├── index.verb
    └── index.sense
```

## Configuration

Make sure your `config.toml` has the correct paths:

```toml
[timezones]
geo_filepath = "data/cities15000.txt"

[dict]
wordnet_path = "data/wordnet"

[ifsc]
data_path = "data/ifsc"

[excuse]
file = "data/excuses.txt"

[vitamin]
file = "data/vitamins.json"
```
