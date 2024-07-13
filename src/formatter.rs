use crate::processor::Item;

pub fn format_data(items: Vec<Item>) -> String {
    items
        .iter()
        .map(|item| {
            format!(
                "Name: {}\nLink: {}\nPrice: {}\n",
                item.name, item.link, item.price
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
