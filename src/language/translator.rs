use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Translator: Send + Sync {
    /// Translate a single text from source language to target language
    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String>;

    /// Translate multiple texts in a batch
    async fn translate_batch(&self, texts: &[String], from: &str, to: &str) -> Result<Vec<String>> {
        let mut results = Vec::new();
        for text in texts {
            results.push(self.translate(text, from, to).await?);
        }
        Ok(results)
    }
}
