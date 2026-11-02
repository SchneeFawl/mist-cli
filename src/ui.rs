use dialoguer::{MultiSelect};

pub fn select(labels: &[String]) -> Result<Option<Vec<usize>>, dialoguer::Error> {
    MultiSelect::new()
        .with_prompt("Select labels:")
        .items(labels)
        .interact_opt()
}

pub fn evaluate_selection(
    selection: Result<Option<Vec<usize>>, dialoguer::Error>
) -> PromptOutcome {
    match selection {
        Ok(Some(indices)) => {
            if indices.is_empty() {
                PromptOutcome::Empty
            } else {
                PromptOutcome::Selected(indices)
            }
        },
        Ok(None) => PromptOutcome::Cancelled,
        Err(err) => PromptOutcome::Error(err)
    }
}

#[derive(Debug)]
pub enum PromptOutcome {
    Error(dialoguer::Error),
    Cancelled,
    Empty,
    Selected(Vec<usize>)
}
