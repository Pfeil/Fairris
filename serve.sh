#!/usr/bin/env bash

# miniserve ./static --index index.html
cd ./static
simple-http-server . -i --cors --port 8080 --try-file index.html