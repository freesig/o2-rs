socket_finder [![Build Status](https://travis-ci.org/freesig/socket_finder.svg?branch=master)](https://travis-ci.org/freesig/socket_finder) [![Crates.io](https://img.shields.io/crates/v/socket_finder.svg)](https://crates.io/crates/socket_finder) [![Crates.io](https://img.shields.io/crates/l/socket_finder.svg)](https://github.com/freesig/socket_finder/blob/master/LICENSE) [![docs.rs](https://docs.rs/socket_finder/badge.svg)](https://docs.rs/socket_finder/)
=============

A simple crate for finding a socket on a network using a UTF-8 `String`.

This simplifies the process of establishing a one-way connection between two
machines by using a common `String`.

The crate consists of two types:

- **Beacon**: Used for broadcasting a `SocketAddrV4` alongside a UTF-8 string identifier.
- **Finder**: Finds the `SocketAddrV4` with the associated string identifier.
