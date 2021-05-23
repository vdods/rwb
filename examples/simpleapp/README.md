# `simpleapp` -- An Example From `rwb`

This is a minimal web app that uses the [yew](https://github.com/yewstack/yew) to build a web app
that has Rust guts and HTML/JS externals.

For instructions on building, deploying, and serving, see the [README.md](../../README.md) from `rwb`.

For the impatient, to build and serve the app locally, from this project dir, run:

    export PATH=$PATH:<path/to/rwb/bin>
    rwb-local-build && rwb-local-serve-build latest

This will print a URL which you can open to use the app.  Note that you have to fill in `<path/to/rwb/bin>`
with the path specific to your system, don't just copy and paste.
