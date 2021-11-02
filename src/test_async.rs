use std::{collections::HashMap, pin::Pin};

use futures::Future;

type BoxFieldFuture<'a> = Pin<Box<dyn Future<Output = i32> + 'a + Send>>;

struct Container<'a> {
    future_map: HashMap<String, BoxFieldFuture<'a>>,
}

async fn ta1() -> i32 {
    i32::from(1)
}

async fn ta2() -> i32 {
    i32::from(2)
}

// 引数にasync関数を渡す
async fn test<'a>(f: BoxFieldFuture<'a>) {
    let value = f.await;

    println!("{}", value);
}

#[cfg(test)]
mod tests {
    use super::{ta1, test};

    #[tokio::test]
    async fn it_works() {
        let f = Box::pin(async move { ta1().await });
        test(f).await;
    }
}
