# Tentacle

A command line tool for performing service discovery on local networks.

### Usage
```
tentacle SUBCOMMAND

SUBCOMMAND
    serve   --name|-n <name> [--verbose|-v] [--ping-delay <timeout-s>] [--serve-time <serve-s>]
        --name|-n <name>            Name of the service, required
        --verbose|-v                Verbose output
        --ping-delay <delay-s>      Time between pings (in seconds), defaults to 1s
        --serve-time <serve-s>      Time to broadcast service (in seconds), defaults to infinity

    find    [--verbose|-v] [--timeout|-t <timeout-s>] [--json]
        --verbose|-v                Verbose output
        --json                      Output values in JSON format
        --timeout|-t <timeout-s>    Set timeout for finding services (in seconds), defaults to infinity
```

Upon finding a service, `tentacle find ...` will print out the service
in the following format:

`<name>:<ip>`

e.g.

`my-service:192.168.0.1`

^C (Ctrl-C) can be used to cancel an action safely at any time.

### Example Usage
Let's say we want to start a service called "Kaden's Service". We start by
typing the following command into the terminal:

```
[user]> tentacle serve --name "Kaden's Service" --verbose
Service "Kaden's Service" started on 192.168.0.101
Listening...
```

This has started our service. On another computer, we use the following command
to find any available services on the network.

```
[user2]> tentacle find --json
Looking for services on local network...
```

Once the client has received a broadcast message from the service provider,
we get the following message:

```
...
Kaden's Service:192.168.0.101
```

At this point, both the server and the client may be safely closed.
