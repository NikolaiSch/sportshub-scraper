# This is a scraper for sportshub, which also gets active links from the website

[![Rust](https://github.com/NikolaiSch/sportshub-scraper/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/NikolaiSch/sportshub-scraper/actions/workflows/rust.yml)

## Very much in progress

### This tools performance is very much based on the performance of the website, so it might be slow at times, especially during popular football games

- Todo

  - [x] Multiple tabs at same time
  - [x] Batch SQLite writes
  - [x] Fix advertisement causing delay
  - [x] Add RustDoc comments
  - [x] Create cli
  - [x] Add tests
  - [x] Add CI
  - [x] remove `unwrap()`s
  - [x] Store Timestamps instead of strings
  - [x] Other sports

- to install use the following commands

  ```bash
  git clone https://github.com/NikolaiSch/sportshub-scraper
  cd sportshub-scraper
  cargo install --path .
  ```

- to run use the following command

  ```bash
  sportshub

  # to scrape all events, where <T> is number of simultaneous tabs use (defaults to 10)
  sportshub data scrape -t <T>

  # to scrape stream links (videos) use (use -H for headless)
  sportshub data update -t <T> -H

  # to run the http api use
  sportshub serve

  # to run on custom port use
  sportshub serve -p <PORT>

  # to run the server silently use
  sportshub serve -s

  # to do a full data refresh before serving use
  sportshub serve -F

  ```

- You can use crontab to refresh every 15 minutes,

  ```cron
  */15 * * * * sportshub parser 20
  ```

- server has the following urls:
- `/` - returns all scraped data
- `/active` - returns all links with active streams (with links to the streams)
- `/id/<id>` - returns the data for the given id
