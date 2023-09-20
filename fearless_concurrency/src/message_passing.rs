//! Using Message Passing to Transfer Data Between Threads
//!
//! 通过管道的概念在多个线程之间传递消息，比如多个子线程作为管道的发送方，主线程作为管道的接收方，发送方不断地往管道中发送信息，接收方则不停地
//! 消费信息，也就是一个「多生产者，单消费者」的场景。
//!

#[cfg(test)]
mod tests {
    // mpsc - multiple producer, single consumer
    use std::{sync::mpsc, thread, time::Duration};

    /// 阻塞调用
    #[test]
    fn mpsc_blocking() {
        // tx - transmitter 发送方 | rx - receiver 接收方
        // 这种命名是历史原因导致的
        let (tx, rx) = mpsc::channel();

        // 子线程通过闭包捕获主线程上下文中的 tx 发送消息
        thread::spawn(move || {
            let value = String::from("[spawn thread] value");

            for _ in 0..3 {
                println!("[spawn thread] loading...");
                thread::sleep(Duration::from_millis(300));
            }

            tx.send(value).unwrap();
        });

        // 主线程通过 rx 接收子线程传来的消息
        let received_value = rx.recv().unwrap();
        println!("[main thread] receive value: {}", received_value);
    }

    /// 非阻塞调用
    #[test]
    fn mpsc_non_blocking() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let value = String::from("[spawn thread] value");

            for _ in 0..3 {
                println!("[spawn thread] loading...");
                thread::sleep(Duration::from_millis(300));
            }

            tx.send(value).unwrap();
        });

        for _ in 0..5 {
            println!("[main thread] waiting for spawned thread message...");
            thread::sleep(Duration::from_millis(500));

            // 繁琐一点的写法
            // match rx.try_recv() {
            //     Ok(received_value) => {
            //         println!("[main thread] receive value: {}", received_value);
            //         break;
            //     }
            //     Err(_) => {
            //         println!("[main thread] not receive value yet.",);
            //     }
            // }

            // 更简洁的写法
            if let Ok(received_value) = rx.try_recv() {
                println!("[main thread] receive value: {}", received_value);
                break;
            } else {
                println!("[main thread] not receive value yet.",);
            }
        }
    }

    /// 以 iterator 的方式消费多个到来的消息
    #[test]
    fn consume_message_with_iterator() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let values = vec![
                String::from("many"),
                String::from("messages"),
                String::from("is"),
                String::from("coming"),
            ];

            for value in values {
                tx.send(value).unwrap();
                thread::sleep(Duration::from_millis(300));
            }
        });

        for received_value in rx {
            println!("[main thread] receive message: {}", received_value)
        }
    }

    #[test]
    fn multiple_producers() {
        let (tx, rx) = mpsc::channel();

        let tx_cloned = tx.clone();

        thread::spawn(move || {
            let values = vec![
                String::from("many"),
                String::from("messages"),
                String::from("is"),
                String::from("coming"),
            ];

            for value in values {
                tx.send(value).unwrap();
                thread::sleep(Duration::from_millis(300));
            }
        });

        thread::spawn(move || {
            let values = vec![
                String::from("and"),
                String::from("more"),
                String::from("messages"),
            ];

            for value in values {
                tx_cloned.send(value).unwrap();
                thread::sleep(Duration::from_millis(300));
            }
        });

        for received_value in rx {
            println!("[main thread] receive message: {}", received_value)
        }
    }
}
