use std::{iter, ops::Deref};

use crate::{consumer::ShmemConsumer, producer::ShmemProducer, ShmemSettings};

const QUEUESIZE: u64 = 1024 * 512;

fn settings() -> ShmemSettings {
    let mut name: Vec<u8> = iter::repeat_with(|| rand::random_range(97..123))
        .take(8)
        .collect();
    name.insert(0, b'/');
    let name = unsafe { String::from_utf8_unchecked(name) };
    ShmemSettings {
        name,
        size: QUEUESIZE as usize,
    }
}

fn init<T: Copy>() -> (ShmemProducer<T>, ShmemConsumer<T>) {
    let settings = settings();
    let consumer = ShmemConsumer::<T>::new(settings.clone());
    assert!(
        consumer
            .as_ref()
            .inspect_err(|err| eprintln!("{err}"))
            .is_ok(),
        "failed to initialize consumer"
    );
    let producer = ShmemProducer::<T>::new(settings);
    assert!(
        producer
            .as_ref()
            .inspect_err(|err| eprintln!("{err}"))
            .is_ok(),
        "failed to initialize producer"
    );
    (producer.unwrap(), consumer.unwrap())
}

#[test]
fn test_fixed_size() {
    const MSGSIZE: usize = 32;
    let (mut tx, mut rx) = init::<[u64; MSGSIZE]>();
    let consumer = move || {
        for i in 0..QUEUESIZE * 2 {
            let val = rx.consume();
            assert_eq!(
                val, [i; MSGSIZE],
                "consumed values don't match produced ones"
            )
        }
    };
    let producer = move || {
        for i in 0..QUEUESIZE * 2 {
            tx.produce([i; MSGSIZE]);
        }
    };

    let ch = std::thread::spawn(consumer);
    let ph = std::thread::spawn(producer);

    assert!(ch.join().is_ok(), "failed to run consumer");
    assert!(ph.join().is_ok(), "failed to run producer");
}

#[test]
fn test_variable_size() {
    const MSGSIZE: usize = 512;
    let (mut tx, mut rx) = init::<u8>();
    let consumer = move || {
        for i in 0..QUEUESIZE * 2 {
            let val = rx.consume_slice();
            assert_eq!(
                val.deref(),
                &[i as u8; MSGSIZE],
                "consumed values don't match produced ones"
            )
        }
    };
    let producer = move || {
        for i in 0..QUEUESIZE * 2 {
            tx.produce_slice([i as u8; MSGSIZE]);
        }
    };

    let ch = std::thread::spawn(consumer);
    let ph = std::thread::spawn(producer);

    assert!(ch.join().is_ok(), "failed to run consumer");
    assert!(ph.join().is_ok(), "failed to run producer");
}
