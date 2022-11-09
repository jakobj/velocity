# velocity
Make static html go zoom.

A simple (~50sloc) application build on [hyper](https://hyper.rs/) to serve http content which does not change all that often.
It avoids IO during run time by including all assets in the binary at compile time.
Due to the way routing is implemented, it may only by useful for serving a handful of assets.

## Usage
- copy your assets to the `assets/` directory
- edit the [binary](velocity/src/main.rs) to define your routes
- compile and run (or use the included Dockerfile to create a container)
