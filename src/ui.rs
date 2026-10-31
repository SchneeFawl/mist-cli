use dialoguer::{MultiSelect, Error};

pub fn select(labels: &[String]) -> Result<Option<Vec<usize>>, Error> {
    MultiSelect::new()
        .with_prompt("Select labels:")
        .items(labels)
        .interact_opt()
}
