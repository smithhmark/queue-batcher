#!/usr/bin/bash
cat rabbit.container | xargs docker stop && rm rabbit.container

