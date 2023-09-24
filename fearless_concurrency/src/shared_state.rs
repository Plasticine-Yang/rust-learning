//! Shared-State Concurrency
//!

#[cfg(test)]
mod tests {
    use std::{sync::{Mutex, Arc}, thread};

    #[test]
    fn use_mutex() {
        let m = Mutex::new(666);

        {
            let mut value = m.lock().unwrap();
            *value = 888
        }

        assert_eq!(888, *m.lock().unwrap());
    }

    #[test]
    fn share_mutex_value_between_multiple_threads() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
          let counter = Arc::clone(&counter);
          let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
          });

          handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(10, *counter.lock().unwrap());
    }
}
