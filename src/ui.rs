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

#[cfg(test)]
mod test {
use super::*;

    #[test]
    fn evaluate_selection_empty() {
        let vector: Vec<usize> = Vec::new();
        let selection = Ok(Some(vector));

        assert!(
            matches!(
                evaluate_selection(selection),
                PromptOutcome::Empty
            ),
            "Expected output to be empty"
        )
    }

    #[test]
    fn evaluate_selection_error() {
        let err: std::io::Error = std::io::Error::other("terminal unavailable");

        assert!(
            matches!(
                evaluate_selection(Err(err.into())),
                PromptOutcome::Error(dialoguer::Error::IO(_))
            ),
            "Expected IO Error"
        )
    }

    #[test]
    fn evaluate_selection_cancelled() {
        let selection = Ok(None);

        assert!(
            matches!(
                evaluate_selection(selection),
                PromptOutcome::Cancelled
            ),
            "Expected prompt to be cancelled"
        )
    }

    #[test]
    fn evaluate_selection_non_empty() {
        let vector: Vec<usize> = vec![1, 2, 3];
        let selection = Ok(Some(vector));

        assert!(
            matches!(
                evaluate_selection(selection),
                PromptOutcome::Selected(vec) if vec == [1, 2, 3]
            ),
            "Expected prompt to succeed"
        )
    }
}
