# `rwb` - Rust Web Builder

By Victor Dods - Copyright 2021

This is a utility (currently a set of shell scripts) for easing building and deploying wasm-pack-based
Rust crates.  For example, you write the guts of an application as a Rust library crate, which gets built
as a wasm target, then write the UI as HTML and JS which uses an interface to the wasm lib.  The example(s)
use [yew](https://github.com/yewstack/yew) to do this Rust <-> HTML/JS binding.

This utility is just a simple thing for a simple build and deployment context, and isn't meant to do more.
There probably exist "real" tools that do this sort of thing better and more intelligently.

Ideally the shell scripts would be replaced by a single Rust-based CLI tool that can be installed via
cargo install and not require any external CLI tools, but for now, whateva.

## Releases

No guarantees for backward compatibility for now.

|Version|Notes|
|-------|-----|
|`v0.1.1`|A fix to holoversion.|
|`v0.1.0`|Improvements to portability of builds regarding location, layout of the build dirs, symmetry between local and remote build dirs, and other general usability improvements.|
|`v0.0.0`|Initial release.|

## Dependencies

Required tools:
-   `toml-cli` commandline tool.  This is used for extracting the package name from `Cargo.toml`.

        cargo install toml-cli

-   `sha256sum` commandline tool, which is part of the GNU coreutils package.  This is used for generating
    a build version string that takes into account local modifications.  On Ubuntu:

        sudo apt-get install coreutils

-   `git` commandline tool, which is only used for generating the build version string via `git describe`.
    This is a standard tool that you almost certainly have used to get this repo.
-   If you want to use the script for local serving of builds (e.g. for testing purposes during development),
    you'll need `python3` and the `http` package, which as far as I can tell, is included in a Python distribution.

## How To Use

For now, the easiest way to use this tool is to add the `rwb/bin` dir to your path.  Alternatively you could
copy it somewhere, but then you'd have to copy again to get any updates.  Later, this tool might get ported
to Rust and become a single binary, installable via cargo, but for now, this is how it works.

Look at the [simpleapp](examples/simpleapp) example.  You can locally build and serve the examples as is, but
in order to remotely deploy and serve, you'll have to specify values for the `RWB_REMOTE_*` env vars in
`rwb-package-config.env`

You'll need a cargo crate (which can be part of a cargo
workspace) that has a file `rwb-package-config.env` in it which defines your project-specific `rwb` configuration.
See the comments in that file in the example for what everything should be.

## Building and Deploying using `rwb`

### Local Operations

#### Locally Building

From within your project root dir (e.g. in `rwb/examples/simpleapp`), run

    rwb-local-build

This will create a complete build package under the `$RWB_LOCAL_BUILDS_DIR/<your-package-name>` directory (where
`RWB_LOCAL_BUILDS_DIR` is defined in your project's `rwb-package-config.env` file).  The build will be a
directory whose name is the "holoversion" of the project.  This contains:
-   Your package name (as defined in its `Cargo.toml` file).
-   A string from `git describe`, which ideally contains a version tag, a git commit hash, a `modified`
    marker if the repo's working state is modified.
-   A sha256 hash of the contents of the project dir.  The current implementation of this hash doesn't
    guarantee to agree with `.gitignore`, but is only meant to capture local modifications, and therefore
    produce distinctly named builds.

A successful build will cause a `latest` symlink to appear under the `$RWB_LOCAL_BUILDS_DIR/<your-package-name>`
directory, and is sort of like a docker tag.  Symlinks under `$RWB_LOCAL_BUILDS_DIR/<your-package-name>`
are considered tags.

#### Serving a Local Build

From within your project root dir (e.g. in `rwb/examples/simpleapp`), run

    rwb-local-serve-build <build-name>

where `<build-name>` is a full build name or a tag name (e.g. `latest`).  This will spin up a Python-based
http server on the port configured in your project's `rwd-package-config.env` file, and for your
convenience will print the local dir the build is located and the local URL at which your build is
served.  You can open this URL in your browser.

#### Tagging a Local Build

From within your project root dir (e.g. in `rwb/examples/simpleapp`), run

    rwb-local-serve-build <build-name> <tag-name>

where `<build-name>` is a full build name or a tag name (e.g. `latest`).  This will create a symlink
with the given tag name pointing at the full build name indicated by `<build-name>` (it will dereference
it if it's a tag).

#### Listing Local Builds

From within your project root dir (e.g. in `rwb/examples/simpleapp`), run

    rwb-local-list-builds

This is self-explanatory.  This will start an interactive `less` process, showing the time-ordered
builds, ordered from newest to oldest.

### Remote Operations

First, ensure that you've properly configured your project's `rwb-package-config.env` and set the
env vars `RWB_REMOTE_*` to values appropriate to `ssh` into your web host and the root HTML content dir.

#### Deploying a Remote Build

From within your project root dir (e.g. in `rwb/examples/simpleapp`), run

    rwb-remote-deploy-build <build-name>

where `<build-name>` is a full build name or a tag name (e.g. `latest`).  This will copy the specified
full build (i.e. dereferencing `<build-name>` if it's a tag) via `ssh` to the web host into the specified
location.  Then if `<build-name>` is a tag, it will copy that tag as well, overwriting any existing
tag with that name.  For your convenience, it will print the remote dir the build is located at and the
remote URL at which your build is served.  You can open this URL in your browser.

#### Listing Remote Builds

From within your project root dir (e.g. in `rwb/examples/simpleapp`), run

    rwb-remote-list-builds

This is self-explanatory.  This will start an interactive `less` process, showing the time-ordered
builds, ordered from newest to oldest.  The difference here is that this is done over `ssh`.

## Notes

In order for a build to have a reasonable holoversion, such as

    simpleapp-v0.0.0-2-g9afadcf

you should use annotated git tags on release commits, e.g.

    git tag v1.0.8 -m ""

The `-m` option provides a message (which can be left as the empty string) and causes it to be an annotated tag
(as opposed to a lightweight tag).  From `man git-tag`:

    Annotated tags are meant for release while lightweight tags are meant for private or temporary
    object labels. For this reason, some git commands for naming objects (like git describe) will
    ignore lightweight tags by default.

While building:
-   If you get an error message like

        fatal: No names found, cannot describe anything.

    then you don't have any version tags in your git repo, which causes minor problems for holoversion.
-   Ignore error messages like

        tar: Removing leading `../' from member names

    they don't negatively impact anything.
-   If you get an error message like

        tar: ../Cargo.lock: Cannot stat: No such file or directory

    then the value of `RWB_CRATE_IS_INSIDE_CARGO_WORKSPACE` in your project's `rwb-package-config.env`
    is wrong, it should be `false` instead of `true`.

## To-dos

-   Make `rwb-hacky-holoversion` into a function instead of a script.
-   Ideally, make each script into a function, and then have a single shell script with subcommands.
-   Figure out correct usage of terms 'crate' and 'package' in various places.  It's not clear what the
    distinction is, especially when in the context of a cargo workspace.
-   Is there a way to do this using cargo build processes?  I don't really know, I'm a relative Rust noob.

## License

The `rwb` tool itself, as well as all [examples](examples), is licensed under Apache License 2.0 -- see the
[`LICENSE`](LICENSE) file for details.

The code generated by `rwb` (a single, auto-generated Rust source file containing the version of the built package)
is not subject to this license and can be used for whatever purpose by the user of `rwb`.  There is a notice
to this effect in the generated source file itself.
