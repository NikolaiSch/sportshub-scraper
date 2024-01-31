# This is a scraper for sportshub, which also gets active links from the website

## Very much in progress

- Todo

  - [x] Multiple tabs at same time
  - [x] Batch SQLite writes
  - [x] Fix advertisement causing delay
  - [x] Add RustDoc comments
  - [x] Create cli
  - [ ] Add tests
  - [x] Add CI
  - [ ] remove `unwrap()`s
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

- server has the following urls:
- `/` - returns all scraped data
- `/active` - returns all links with active streams (with links to the streams)
- `/id/<id>` - returns the data for the given id
