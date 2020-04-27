# parking application


## How to run
First you'll need the database setup. This uses the default `postgres` user with a database `parking` (I know, so secure). To initialize the DB, just run this command once the postgres server is running:
```
psql parking --username postgres < db.sql
```

Then, you'll need rust nightly (I use some macros that wouldnt work in stable), which means you need rust installed (google it)
```
$ rustup toolchain install nightly
$ rustup default nightly
```

Then, assuming all goes well, you can just run
```
$ cargo run
```

And the webserver will be running on http://localhost:8000
