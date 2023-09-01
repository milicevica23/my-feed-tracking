
## Thanks to Jeremy Chone <jeremy.chone@gmail.com>"
The code is based on his [youtube series](https://www.youtube.com/watch?v=3cA_mk4vdWY&t=1515s) 


## Docker & Rust
https://hub.docker.com/_/rust

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run the quick_dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```
