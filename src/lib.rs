use anyhow::{Result};

pub struct App {
}

impl App {

    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_returns_success() {
        let app = App {};

        assert_eq!(app.run().is_ok(), true);
    }
}
