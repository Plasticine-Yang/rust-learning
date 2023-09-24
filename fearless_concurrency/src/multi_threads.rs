//! Using Threads to Run Code Simultaneously
//!
//! ## 创建线程
//!
//! 使用 `thread::spawn` 函数创建线程，传入一个闭包函数作为要在新线程中执行的代码
//!
//! ## 等待线程结束
//!
//! 在多线程编程中，"join" 是一种用于等待线程执行完成的操作。当一个线程调用另一个线程的 "join" 方法时，它会等待被调用线程执行完成，然后再继续执行后续代码。这个等待的过程被称为 "join"。
//!
//! "join" 可以理解为"加入"或"汇合"。当一个线程调用另一个线程的 "join" 方法时，它会等待被调用线程的执行，就像它要"加入"或"汇合"到被调用线程的执行流中。在这个过程中，调用线程暂停执行，直到被调用线程完成为止。相当于调用线程加入到被调用线程的执行中，等待它完成。
//!
//! "join" 的作用是确保在主线程（或其他线程）中等待其他线程执行完成，以便协调线程之间的执行顺序和数据同步。通过使用 "join"，可以等待其他线程执行完成后再进行后续的操作，以保证线程间的正确性和一致性。
//!
//! 需要注意的是，"join" 方法的调用通常会导致当前线程阻塞，直到被调用线程执行完成。因此，在使用 "join" 时要注意处理可能的超时或异常情况，以避免程序无法继续执行。
//!

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    #[test]
    fn spawn_thread() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn spawn_with_join_handle() {
        let join_handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        join_handle.join().unwrap();
    }

    // 下面这段代码无法通过编译，原因如下：
    // - 在子线程闭包中使用了主线程中的数据，但是由于无法保证子线程的执行顺序，也就是说无法保证子线程执行时，主线程中的 data 仍然存在，因此不会通过编译
    // #[test]
    // fn using_data_in_main_thread() {
    //     let data = vec![1, 2, 3];

    //     let join_handle = thread::spawn(|| {
    //       println!("data in main thread: {:?}", data);
    //     });

    //     drop(data);
    //     join_handle.join().unwrap();
    // }

    // 改进措施是使用 move 关键字，强行将主线程中的 data 的所有权转移到子线程中，但依然无法通过编译，因为主线程已经没有了对 data 的所有权
    // 但其仍然尝试使用 `drop(data)` 将其销毁
    // #[test]
    // fn using_data_in_main_thread() {
    //     let data = vec![1, 2, 3];

    //     let join_handle = thread::spawn(move || {
    //         println!("data in main thread: {:?}", data);
    //     });

    //     drop(data);
    //     join_handle.join().unwrap();
    // }
}
