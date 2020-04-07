#!/bin/bash

createdb parking -U postgres
psql parking --username postgres -f db.sql
