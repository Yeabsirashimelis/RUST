//ASYNCHRONOUS RUST
// future - promise(in other languages)
/*
The Future trait in Rust is the abstraction that represents a value that will be available later.
It’s like a contract that says: “I’m not ready yet, but keep checking, and eventually I’ll give you a result.”
When you write an async fn, the compiler turns it into a type that implements Future.

So in essence:
A Future is a placeholder for a result that hasn’t been computed or received yet.
You get the actual value when you .await it.

*you can also implement it for you own data types with their own implementations of Future
*/

//now we can use the various pieces provided by trpl to write our first async program. we will build a little c
// command line tool that fetched two web pages, pulls the <title/> elemnet from each, and prints out the little of whichever page
// finishes that whole process first
// use std::{env, future::Future};
// use trpl::Html;

// async fn page_title(url: &str) -> Option<String> {
//     // let response = trpl::get(url).await;
//     // let response_text = response.text().await;
//     let response_text = trpl::get(url).await.text().await;
//     Html::parse(&response_text)
//         .select_first("title")
//         .map(|title_element| title_element.inner_html())
// }

/*
/*
when rust sees a block marked with the async keyword, it compiles it into unique, anonymous data type that implements the Future trait.
when rust sees a function marked with async, it compiles it into a non-async function whose body is an async block.
an async function return type is the type of anonymous data type the compiler creates that async block
THUS writing async function is equivalent to writing a function that returns a future of the return type. to the compiler, a function
  defination such as the async fn page_title is equivalent to a non-async function defined like this:-
*/
fn page_title(url: String) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(&url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    trpl::run(async {
        let url = args[1].clone();
        match page_title(url.clone()).await {
            Some(title) => println!("the title for the {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
 */
/*
////////////////////////////////////////////////////////
////////////////////////////////////////////////////////
//RACING OUR TWO URLS AGAINST EACH OTHER

use trpl::{Either, Html};

async fn page_title(url: String) -> (String, Option<String>) {
    let text = trpl::get(&url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(args[1].clone());
        let title_fut_2 = page_title(args[2].clone());

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(title) => title,
            Either::Right(title) => title,
        };

        println!("{} returned first", url);

        match maybe_title {
            Some(title) => println!("Its page title is: {}", title),
            None => println!("Its title could not be parsed"),
        }
    })
}
 */
//////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////
//APPLYING CONCURRENCY WITH ASYNC - (we use the TRPL package FROM THE RUST BOOK)
/*
  we saw concurrency with threads, in this section we'll focus on what is different between threads and futures

in many cases, the APIs for working with concurrency using async are very similar to those for using threads.
in other cases, they end up being quite different. Even when the APIs look similar between threads and async,
  they often have different behavior - and they nearly always have different performance characteristis
*/

use std::time::Duration;

//CREAITING A NEW TASK WITH SPAWN_TASK
fn main() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap(); //to finish the spawn thread
    });
}
