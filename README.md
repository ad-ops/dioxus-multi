# Dioxus Multi Project
Trying out build a trivial Dioxus app, but targeting different UIs. Tried using Bootstrap 5 and it works very well!

## Develop
`/app` contains all code, while the respective UI (see below) are for running it.

All different UIs run the same app and you can see if there are any difference.

## Run
UI|Path|Run Command|Prerequisits
---|---|---|---
Desktop|`.`|cargo run --bin desktop|
SSR|`.`|cargo run --bin ssr|
Web|`./web`|trunk serve --watch . --watch ../app|[Trunk](https://trunkrs.dev)

## Issues
#### Form Submit
On desktop a submit does nothing but the event, while on web it will trigger a `POST` (which might be what you want). This might be unintended, but still I guess we can easily supress the default behavior.