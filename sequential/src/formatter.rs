use crate::processor::Item;

pub fn format_data(items: Vec<Item>) -> String {
    items
        .iter()
        .map(|item| {
            let reviews = item
                .reviews
                .iter()
                .map(|review| format!(" - {}", review))
                .collect::<Vec<String>>()
                .join("\n");

            format!(
                "Name: {}\nLink: {}\nPrice: {}\nReviews:\n{}\n",
                item.name, item.link, item.price, reviews
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
