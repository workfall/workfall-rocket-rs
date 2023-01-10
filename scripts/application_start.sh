#!/bin/bash

# give permission to the files inside /secure_docs directory

sudo chmod -R 777 /home/ubuntu/workfall-rs

sudo docker build . -t workfall-rs --no-cache

sudo docker run -d workfall-rs -p8000:8000 --name workfall-rs-i