// use std::thread;
// use std::time::Duration;

//FEARLESS CONCURRENCY - you don't have to fear of a bug when writting concurrency codes in RUST
/*
fn main() {
    /*
    Note that when the main thread of a Rust program completes, all spawned threads
       are shut down, whether or not they have finished running. The output from this
       program might be a little different every time, but it will look similar to the following:
     */

    /*
    he calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run. The threads will probably take turns, but that isn’t guaranteed:
      it depends on how your operating system schedules the threads. In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code.
      And even though we told the spawned thread to print until i is 9, it only got to 5 before the main thread shut down.
     */

    /*
    not only stops the spawned thread prematurely most of the time due to the main thread ending, but because there is no guarantee on the order in which threads run, we also can’t guarantee
    that the spawned thread will get to run at all!
     */
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {i} from the main thread");
    //     thread::sleep(Duration::from_millis(1));
    // }

    /*
    we can fix the problem of the spawned thread not running or ending prematurely be saving
      the return value of thread::spawn in a variable. the return type of thread::spawn is JoinHandle<T>.
      A JoinHandle<T> is an owned value that, when we call the join method on it, it will waits for its thread to finish.
     */
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join(); //gives us a result enum. so unwap it
    handle.join().unwrap();
    //the two threads continue alternating, but the main thread waits because of the call to handle.join()
    //   and does not end until the spawned thread is finished

    /*
        but let's see  what will happen when we instead move handle.join() before the for loop in main
       The main thread will wait for the spawned thread to finish and then run its for loop, so the output won’t be interleaved anymore.

        //like this
        use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }
         */
}
 */

/*
/////////////////////////////////////////////
/////////////////////////////////////////////
//USING MOVE CLOSURES WITH THREADS
/*
we often use the move keyword with closures passed to thread::spawn b/c the closre will then take
  ownership of the values it uses from the environment, thus transferring ownership of those values
  from one thread to another.

*/
fn main() {
    /*
    The closure uses v, so it will capture v and make it part of the closure’s environment. Because thread::spawn runs this closure in a new thread, we should be able to access v inside that new thread. But when we compile this example, we get the a compile-time error

     Rust infers how to capture v, and because println! only needs a reference to v, the closure tries to borrow v. However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know
     whether the reference to v will always be valid. so we should move the ownership to the spawned thread
     */

    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| println!("Here is a vector: {:#?}", v));

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || println!("Here is a vector: {:#?}", v));

    // we move the ownership of the v to other thread. so using v is not allowed here. RUST SAVE US HERE. other languages like C++, we can use it
    // so that we might create a problem. b/c getting the reference may endup pointing dangling reference b/c the main thread can endup first and cleared everything, but the spawn is still in execution -> the reference will point to nothing
    // Rust “saves us” by using its ownership and borrowing rules to ensure that when you spawn a thread, you cannot accidentally access invalid memory (dangling references) or cause data races. Instead of letting you shoot yourself in the foot like in C++, it refuses to compile unsafe patterns.
}

*/

/*
/////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////
//USING MESSAGE PASSING TO TRANSFER DATA BETWEEN THREADS
/*
one increasingly popular approach to ensuring safe concurrency is message passing,
 where threads or actors communicate by sending each other messages containing data.
 Here is the idea in a slogan from GO language document
    ``DON'T COMMUNICATE BY SHARING MEMORY, INSTEAD SHARE MEMORY BY COMMUNICATING``
to accomplish message-sending concurrency, Rust's standard library provides an implementations
 of channels. a channel is a general programmig concept by which data is sent from one thread to another.
a channel has 2 halves. A TRANSMITTER AND RECEIVER
*/
use std::sync::mpsc;
//mpsc - stands for multiple producer, single consumer
/*
In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values
 but only one receiving end that consumes those values.

In short, the way Rust’s standard library implements channels means a channel can have multiple sending
 ends that produce values but only one receiving end that consumes those values. Imagine multiple streams flowing together into one big river
*/
fn main() {
    let (tx, rx) = mpsc::channel(); //returns a tuple(the sending end(transmitter) and the recieving end(reciever))

    thread::spawn(move || {
        let val = String::from("hi Yeabsira Shimeils");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got : {}", received);
    /*
    the receiver has 2 useful methods: recv and try_recv. we are using recv, short for recieve, which will block the main
    threads execution and wait until a value is sent down the channel. once a value is sent, recv will return it in a RESULT<T,E>.
    when the transmitter closed, recv will return an error to signal that no more values will be coming

    the try_recv method doesnot block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available
    and an Err value if there are not any messages this time. using try_recv is useful if this thread has other work to do while waiting
    for messages: we could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does the other work
    for a little while until checking again.

     */
}
 */
/*
////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////
//CHANNELS AND OWNERSHIP TRANSFERENCE
/*
the ownership rules play a vital role in message sending b/c they help you write safe, concurrent code.
preventing errors in concurrent programming is the advantage of thinking about ownership throughout your RUST programs.
let's do an experiment to show how channels and ownership work together to prevent problems
  we'll try to use a "val" value in the spawned thread after we've snet it down the channel
*/
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        //this line will result an COMPILE-TIME ERROR
        /*
        Here, we try to print val after we’ve sent it down the channel via tx.send.
         Allowing this would be a bad idea: once the value has been sent to another thread,
         that thread could modify or drop it before we try to use the value again. Potentially,
         the other thread’s modifications could cause errors or unexpected results due to inconsistent or nonexistent data.
         */
        println!("val is {val}")
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
*/
/*
/////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////
//SENDING MULTIPLE VALUES AND SEEING THE RECIEVER WAITING
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /*
    In the main thread, we’re not calling the recv function explicitly anymore: instead,
    we’re treating rx as an iterator. For each value received, we’re printing it. When the
    channel is closed, iteration will end.
     */

    for received in rx {
        println!("Got : {}", received);
    }
}
 */
/*
///////////////////////////////////////////////////////
////////////////////////////////////////////////////////
//CREATE MULTIPLE PRODCUERS BY CLONING THE TRANSMITTER
/*
earlier we mentioned that mpsc was an acronym for multiple producer, single consumer.
let's put mpsc to use and expand the code above to create multiple threads that all
send values to the same receiver. we can do so by cloning the transmitter.
*/

use std::sync::mpsc;
use std::time::Duration;
use std::{thread, vec};

/*
thread::sleep - to sleep one thread for a certain amount of time makes the os to switch to other available threads

*/
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
 */

//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//SHARED STATE CONCURRENCY
/*
message passing is a fine way to handle concurrency, but it is not the only way.
another method wouldbe for multiple threads to access the same shared data. it violates the sogan of go? yes it violates but still works✔
-unlike message passing it is multiple ownership. multiple threads can accss the same memory location at the same time
-smart pointers made multiple ownership possible

example, let’s look at mutexes, one of the more common concurrency primitives for shared memory.

*/
//USING MUTEXES TO ALLOW ACCESS TO DATA FROM ONE THREAD AT A TIME
/*
MUTEX - is an abbreviation for mutual exclusion. as in mutex allows only one thread to access some data at any given time.
-to access the data in a mutex, a thread must first signal that it wants to access by asking to acquire the mutex's lock.
-the lock is a data structure that is part of the mutex that keeps track of who has currently exclusive access to the data.
  1, you must attempt to acquire the lock before using the data
  2, when you are done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock

//management of mutexes can be incredibly tricky to get right, which is hwy so many people are enthusiastic about channels.
 HOWEVER, thanks to Rust's type system and ownership rules, you can't get locking and unlocking wrong
*/
/*
Mutex is a smart pointer
*/
// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     //as an example of how to use a mutex, et's start by using a mutex in a single-threaded context

//     let m = Mutex::new(5);

//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//         //we did not release the lock, but we can access it out of this scope.
//         //b/c as num (the locker) goes out of scope, it is no more locking. so the data is accessible outside. RUST💗💗💗
//     }

//     println!("m = {:?}", m);
// }

//THIS WHOLE MAIN FUNCTION WILL CREATE AN ERROR THAT  - "Rc<Mutex<i32>>` cannot be sent between threads safely."
// THE COMPILER SAYS THE TRATI "SEND" IS NOT IMPLEMENTED FOR Rc<mutext<i32>>
/*
When Rc<T> manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped.
 But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread.
 This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we’re done with it.
 What we need is a type that is exactly like Rc<T> but one that makes changes to the reference count in a thread-safe way.
*/
// fn main() {
//     //let's try to share a value between multiple threads using Mutex<T>.
//     //we use smartp pointer "Rc - reference count" - for multiple ownership
//     //        - if not the mutex will lose when it get to the spawn thread, it loses the ownership,
//     //          so we can't use it in the main thread
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     //10 threads created using loop
//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result : {}", *counter.lock().unwrap());
// }
/*
//SO NOW CHANGE THE REFERENCE COUNTIN A THREAD-SAFE WAY
//Atom reference counting with Arc<T> - ATOMIC REFERENCE COUNTING
/*
Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations. The a stands for atomic, meaning it’s an atomically reference-counted type.
*/
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    //10 threads created using loop
    //each threads increments the counter and push to the handles vector
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

    println!("Result : {}", *counter.lock().unwrap());
}
*/
/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
//EXTENSIBLE CONCURRENCY WITH SEND AND SYNC TRAITS

//Allowing transference of ownership betwen threads with send
/*
the send marker trait indicates that ownership of values of the type implementing
send can be tranferred between threads. almost every Rust type is Send, but
there are some exceptions, including Rc<T>: this cannot implement Send because
if you cloned an Rc<T> value and tried to transfer ownership of the clone to
another thread, both threads might update the reference count at the same time.
for these reason, Rc<T> is implemented for use in single-threaded where you don't
want to pay the thread-safe performace penality

Therefore, Rust’s type system and trait bounds ensure that you can never accidentally
send an Rc<T> value across threads unsafely. When we tried to do this in Listing 16-14,
we got the error the trait Send is not implemented for Rc<Mutex<i32>>. When we switched to Arc<T>,
 which does implement Send, the code compiled.
*/
////////////////////////////////////////////////////
////////////////////////////////////////////////////
//ALLOWING ACCESS FROM MULTIPLE THREADS WITH SYNC
/*
the sync maker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
in other words, any type T implements Sync if &T (an immutable reference to T) implements Send, meaning the reference can be snet safely to another thread.
similar to another thread, similar to Send, primitive types all implement Sync, and types composed entirely of types that implement Sync also implement Sync.

the smart pointer Rc<T> also doesn't implement Sync for the same reasons that it doesnot implement Send.
the RefCell<T> types and the family of related Cell<T> don't implement Sync. the implementation of borrow checking that RefCell<T> does is not thread safe.
the smart pointer Mutex<T> implements Sync and can be used to share access with multiple threads as

*/
//IMPLEMENTING SEND SYNC MANUALY IS UNSAFE

/*
The Rust standard library provides channels for message passing and smart pointer types, such as Mutex<T> and Arc<T>,
that are safe to use in concurrent contexts. The type system and the borrow checker ensure that the code using these
solutions won’t end up with data races or invalid references. Once you get your code to compile, you can rest assured that
it will happily run on multiple threads without the kinds of hard-to-track-down bugs common in other languages. Concurrent programming
is no longer a concept to be afraid of: go forth and make your programs concurrent, fearlessly!


*/
fn main() {}
