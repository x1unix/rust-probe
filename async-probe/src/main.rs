use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    // let rsp = trpl::get(url).await;
    // let text = rsp.text().await;
    let text = trpl::get(url).await.text().await;
    Html::parse(&text)
        .select_first("title")
        .map(|e| e.inner_html())
}

async fn page_title_with_url(url: &str) -> (&str, Option<String>) {
    let r = page_title(url).await;
    (url, r)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).take(2).collect();
    trpl::run(async {
        match args.as_slice() {
            [url] => run_once(url).await,
            [a, b] => run_race(a, b).await,
            _ => println!("missing url(s)"),
        };
    });
}

async fn run_race(url1: &str, url2: &str) {
    let t_fut_1 = page_title_with_url(url1);
    let t_fut_2 = page_title_with_url(url2);

    let (url, mb_title) = match trpl::race(t_fut_1, t_fut_2).await {
        trpl::Either::Left(l) => l,
        trpl::Either::Right(r) => r,
    };

    println!("{url} returned first");
    match mb_title {
        Some(title) => println!("Title: {title}"),
        None => println!("{url}: No title"),
    }
}

async fn run_once(url: &str) {
    println!(":: curl {url}");
    match page_title(url).await {
        Some(title) => println!("Title: {title}"),
        None => println!("{url}: No title"),
    }
}
