Stock Quote Generator Demo
===============================

This is a simple REST-ful Stock Quote Generator Demo.

## How to use the Stock Quote Generator

To generate a random quote, try:

```textmate
http://localhost:8080/
```

#### Sample Output:

```json
{
  "symbol": "VDBRZ",
  "exchange": "OTCBB",
  "last_sale": 54.66193728481041,
  "transaction_time": 1707430872052
}
```

To generate a quote with a known ticker but random pricing, try:

```textmate
http://localhost:8080/AAPL/NYSE
```

#### Sample Output:

```json
{
  "symbol": "AAPL",
  "exchange": "NYSE",
  "last_sale": 387.08284270417306,
  "transaction_time": 1707430883080
}
```

## Starting the service

```bash
cargo run
```