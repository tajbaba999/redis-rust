#!/bin/bash

# Redis-Rust Test Script
# Tests all implemented Redis commands

HOST="localhost"
PORT="6379"

echo "=== Redis-Rust Test Suite ==="
echo ""

# Helper function to send command
send_cmd() {
    echo ">>> $1"
    echo "$1" | nc -w 1 $HOST $PORT
    echo ""
}

echo "--- String Commands ---"
send_cmd "PING"
send_cmd "SET testkey testvalue"
send_cmd "GET testkey"
send_cmd "SET expkey expvalue EX 60"
send_cmd "GET expkey"
send_cmd "DEL testkey"
send_cmd "GET testkey"

echo "--- List Commands ---"
send_cmd "LPUSH mylist first"
send_cmd "RPUSH mylist second"
send_cmd "LPUSH mylist zeroth"
send_cmd "LRANGE mylist 0 10"
send_cmd "LPOP mylist"
send_cmd "RPOP mylist"
send_cmd "LRANGE mylist 0 10"

echo "--- Set Commands ---"
send_cmd "SADD myset member1"
send_cmd "SADD myset member2"
send_cmd "SADD myset member1"
send_cmd "SISMEMBER myset member1"
send_cmd "SISMEMBER myset nonexistent"
send_cmd "SMEMBERS myset"
send_cmd "SREM myset member1"
send_cmd "SMEMBERS myset"

echo "--- Sorted Set Commands ---"
send_cmd "ZADD scores 100 alice"
send_cmd "ZADD scores 85 bob"
send_cmd "ZADD scores 95 charlie"
send_cmd "ZRANGE scores 0 10"
send_cmd "ZSCORE scores alice"
send_cmd "ZREM scores bob"
send_cmd "ZRANGE scores 0 10"

echo "=== All tests completed! ==="
