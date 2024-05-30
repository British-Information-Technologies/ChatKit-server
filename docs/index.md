# Chat Kit

This is a self hosted, distributed shat system.
It derives a lot of ideas from Discord, IRC and RCS.

This repository contains a couple of crates.
- Protocol: The protocol message structures
- Foundation: Shared structures and functions utilised in the server and client crate.
- Server: The server that accepts client connections and manages state between them.
- Client: A basic terminal client, used for testing and will be unstable.