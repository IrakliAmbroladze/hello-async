use trpl::Html;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:#?}", args);
}

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title_element| title_element.inner_html())
    }
}
