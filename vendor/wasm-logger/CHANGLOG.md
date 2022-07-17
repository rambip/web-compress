# `wasm-logger` change log

## v0.2.0

* Add `impl Default for Config`.

Breaking changes:
* Replace constructor `with_prefix` by `Config::module_prefix`

Output changes:
* A log entry is now formated as: `$LEVEL $FILE_PATH:$LINE_NUMBER $message`

## v0.1.5

* Fix dependency: `log = { version="0.4", features=["std"] }`

## v0.1.4

* Allow to config on where messages are logged: On the same line or on its own line. Default to one-line.

## v0.1.3

Changes:
* Log `$LineNumber:$FilePath` instead of target

## v0.1.2

Changes
* `Level::Trace => console::debug_4` and `Level::Debug => console::log_4` (see https://gitlab.com/limira-rs/wasm-logger/merge_requests/2)

## v0.1.1

Changes
* Prettify log's entries (see https://gitlab.com/limira-rs/wasm-logger/merge_requests/1)
* Update README