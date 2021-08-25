# SUHA - a cross platform terminal file manager

[![Linux build](https://github.com/justincremer/suha/actions/workflows/linux-main.yml/badge.svg)](https://github.com/justincremer/suha/actions/workflows/linux-main.yml)

[![MacOS build](https://github.com/justincremer/suha/actions/workflows/mac-main.yml/badge.svg)](https://github.com/justincremer/suha/actions/workflows/mac-main.yml)

[![MacOS build](https://github.com/justincremer/suha/actions/workflows/windows-main.yml/badge.svg)](https://github.com/justincremer/suha/actions/workflows/windows-main.yml)

As much as I enjoy lf and as little a need for this project as there is, it seems like fun.
Much of the core is based on Jeff Zhao's (kamiyaa) project, Joshuto, but I'm using crossterm
as a backend instead of termion, for compatibility with non-ANSI terminals (TTY, Windows).
Another goal of mine is to provide dead simple apis for purposes of extensibility and source
code hackability.

Currently, the application will build out a cache of the directory structure you provide as a path
and nothing else.  I've moved the cache display rendering inside of the tui renderer (wow so cool),
but it still only does that and is very much under development.

## Todo

- [x] Core filesystem functionality
- [x] Implement display methods for directories and files
- [ ] User interface
  - [ ] Widget hierarchy and standardized design
  - [ ] Abstracted frame buffer and rendering api
  - [ ] Standardize a way to write unit and integration tests for views
- [ ] Event handling, including key and mouse events
  - [x] Create non-blocking workers with tokio
  - [ ] Navigation + context updating
  - [ ] Standard commands
- [x] Ensure smooth error handling and a clean exit (`cleanup` method in run.rs)
- [x] Configuration
  - [x] External config structure
  - [x] Deserialize into context with serde
- [x] Strutopt for arg parsing
- [ ] Logging

## Contributions

Please help me.  You're welcome to contact me on Github or over email.

## Bugs/Features

Please create an issue
