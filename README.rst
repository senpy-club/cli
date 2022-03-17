:code:`senpy-cli`
=========

The Senpy Club CLI is a tool that provides quick access to The Senpy Club API
from the command-line!

Get data **fast** from The Senpy Club API either as stdout or to a supported
file format.

Quick links
^^^^^^^^^^^

.. raw:: html

  <p>
    <a href="https://discord.com/invite/yWKgRT6">
      <img src="https://img.shields.io/discord/246524734718738442"
           alt="Discord" />
    </a>
    <a href="https://saythanks.io/to/fuwnzy@gmail.com">
      <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg"
           alt="Say Thanks" />
    </a>
    <a href="LICENSE">
      <img src="https://img.shields.io/github/license/senpy-club/cli"
           alt="License" />
    </a>
    <a href="https://crates.io/crates/senpy-cli">
      <img src="https://img.shields.io/crates/v/senpy-cli.svg"
           alt="Crate" />
    </a>
    <a href="https://github.com/senpy-club/lci/actions/workflows/check.yaml">
      <img src="https://github.com/senpy-club/cli/actions/workflows/check.yaml/badge.svg?branch=main"
           alt="Build Status" />
    </a>
  </p>

Installation
^^^^^^^^^^^^

Install from crates.io
----------------------

.. code-block:: shell

  $ cargo install senpy-cli --force

Download from releases
----------------------

Alternatively, prebuilt binaries for x86_64-based Linux systems are available in
the `releases <https://github.com/senpy-club/cli/releases/latest>`_. If you are
using a different operating system or architecture such as macOS or Windows;
you'll have to build and install The Senpy Club CLI yourself!

Install from self-compile
-------------------------

.. code-block:: shell

  $ cargo install --git https://github.com/senpy-club/cli --branch main

If you are building and installing yourself; you must have
`Rust <https://www.rust-lang.org/>`_ installed!

Usage
^^^^^

The Senpy Club CLI allows you to export to stdout in the form of
ten-space-seperated columns or to a file in the JSON, YAML, or Dhall formats.

Examples
--------

.. code-block:: shell

  $ senpy languages    # Prints all available languages in a single column
  $ senpy languages -t # Prints all available languages in one column and the fetch time in another
  $ senpy languages -f languages.json  # Exports all available languages to a JSON file
  $ senpy languages -f languages.yaml  # Exports all available languages to a YAML file
  $ senpy languages -f languages.dhall # Exports all available languages to a Dhall file
  $ senpy random | awk '{ print $1 }'  # Prints the first column (language) from stdout on *nix-based environments

Feel free to explore the rest of the available commands with :code:`senpy help`!

License
^^^^^^^

`GNU General Public License v3.0 <https://github.com/senpy-club/api-worker/blob/main/LICENSE>`_
