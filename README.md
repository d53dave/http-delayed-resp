# Debug HTTP requests with http-delayed-resp

`http-delayed-resp` is a tiny application that can be used to debug HTTP
connections in scenarios where network infrastructure might be bugging you. It
allows the client to dictate how fast the server will respond, and can therefore
simulate slow servers, long-polling systems, and other similar situations.

## Running `http-delayed-resp`

There are 2 ways to run `http-delayed-resp`:

1. Use the [docker image](https://hub.docker.com/r/d53dave/http-delayed-resp)
2. [Download a precompiled binary](https://github.com/d53dave/http-delayed-resp/releases) (Linux x86_64 only) and execute `./http-delayed-resp_x86_64`.

## Configuration

Use an environment variable called `DELAYED_RESP_PORT` to specify the port. It
will bind to `0.0.0.0:<DELAYED_RESP_PORT>`. The default port is 3000.

## Client Usage

```shell
# simple request with a 10s delay

curl http://<host>:<port>/any/path/really?delay=10000

# or for 10 concurrent requests:

ab -n 10 -c 10 http://<host>:<port>/any/path/really?delay=10000
```
