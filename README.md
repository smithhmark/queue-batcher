# queue-batcher
a toy worker process to create batches from a queue

## Testing
if you have a docker image for `rabbitmq:latest` then the following will confirm that you can run the tests:
1. start rabbitmq, eg `bash tests/run_rabbit.sh`
1. open two shells
   1. in the first, run `cargo run --bin hello_consumer`
   1. in the second, run `cargo run --bin hello_producer`
1. confirm in the first shell that the producer as printed that it recieved a message
1. when testing is complete:
   1. stop the rabbitmq container, eg run: `bash tests/stop_rabbit.sh`
   1. stop the hello_consumer by typing Ctrl-C in that shell

## References and Thanks
The hello producer and consumer come straight from the amiquip examples: [see here](https://github.com/jgallagher/amiquip/tree/main/examples)


