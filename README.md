# This is a scraper for sportshub, which also gets active links from the website

[![Rust](https://github.com/NikolaiSch/sportshub-scraper/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/NikolaiSch/sportshub-scraper/actions/workflows/rust.yml)

## Very much in progress

### This tools performance is very much based on the performance of the website, so it might be slow at times

### Especially during popular football games

- Todo

  - [x] Multiple tabs at same time
  - [x] Batch SQLite writes
  - [x] Fix advertisement causing delay
  - [x] Add RustDoc comments
  - [x] Create cli
  - [ ] Add tests
  - [x] Add CI
  - [x] remove `unwrap()`s
  - [ ] Store Timestamps instead of strings
  - [ ] Other sports

- to install use the following commands

  ```bash
  git clone https://github.com/NikolaiSch/sportshub-scraper
  cd sportshub-scraper
  cargo install --path .
  ```

- to run use the following command

  ```bash
  sportshub

  # to scrape use, where <T> is number of simultaneous tabs (defaults to 10)
  sportshub scrape <T>

  # to run the http api use
  sportshub server

  # to run on custom port use
  sportshub server -p <PORT>
  ```

- You can use crontab to refresh every 15 minutes,

  ```cron
  */15 * * * * sportshub parser 20
  ```

- server has the following urls:
- `/` - returns all scraped data
- `/active` - returns all links with active streams (with links to the streams)
- `/id/<id>` - returns the data for the given id
