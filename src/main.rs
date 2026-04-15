use std::future::Future;
use trpl::Html;

fn main() {
    println!("Hello, world!");
}

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title_element| title_element.inner_html())
    }
}
