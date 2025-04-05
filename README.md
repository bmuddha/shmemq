
# ShmemQ

ShmemQ is a Rust library for implementing a shared memory queue designed for inter-process communication (IPC). It provides a single-producer, single-consumer channel to efficiently pass messages between different processes or threads. While it can be used for inter-thread communication within the same process, there are other alternatives like Crossbeam or Flume channels that offer greater throughput and lower latency.

## Features

- **Single Producer, Single Consumer**: Ensures data integrity with one producer and one consumer.
- **Shared Memory**: Utilizes shared memory for fast IPC.
- **Platform Support**: Compatible with Linux and other Unix-like systems.

## Usage

To utilize ShmemQ, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
shmemq = "0.1.0"
```

Here's a basic example of how to create and use a shared memory queue:

```rust
use shmemq::{ShmemProducer, ShmemConsumer, ShmemSettings};

fn main() {
    let settings = ShmemSettings {
        name: "/my_shm_queue".into(),
        size: 1024,
    };

    let mut producer = ShmemProducer::new(settings.clone()).expect("Failed to create producer");
    let mut consumer = ShmemConsumer::new(settings).expect("Failed to create consumer");

    // Producing a message
    producer.produce(42u64);

    // Consuming a message
    let message = consumer.consume();
    println!("Consumed message: {}", message);
}
```

## Safety

The library relies on the user ensuring there is only one consumer and one producer. Failure to do so may result in undefined behavior. Operations on the queue are inherently unsafe due to shared memory access without Rust's typical ownership guarantees.

## License

ShmemQ is licensed under the MIT License. See [LICENSE](LICENSE) for more details.
