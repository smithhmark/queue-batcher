#!/usr/bin/bash

docker run -d --rm --hostname rabbit -p 5672:5672 --name rabbit rabbitmq:latest > rabbit.container
