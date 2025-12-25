# Redis-Rust

A Redis server implementation in Rust.

## Running the Server

```bash
cargo run
```

The server starts on `127.0.0.1:6379`.

## Testing

### Using the Test Script

```bash
# Make executable
chmod +x test_redis.sh

# Run tests (server must be running)
./test_redis.sh
```

### Using netcat (manual)

```bash
echo "PING" | nc localhost 6379
echo "SET mykey myvalue" | nc localhost 6379
echo "GET mykey" | nc localhost 6379
```

### Using redis-cli (if installed)

```bash
redis-cli -p 6379
> PING
> SET foo bar
> GET foo
```

## Supported Commands

| Category | Commands |
|----------|----------|
| **String** | SET, GET, DEL (with EX for TTL) |
| **List** | LPUSH, RPUSH, LPOP, RPOP, LRANGE |
| **Set** | SADD, SREM, SISMEMBER, SMEMBERS |
| **Sorted Set** | ZADD, ZREM, ZRANGE, ZSCORE |
| **Utility** | PING |

## Why not Bruno?

Bruno is designed for HTTP APIs. This Redis server uses raw TCP with the RESP protocol, just like real Redis. Use `netcat`, `redis-cli`, or the test script instead.
