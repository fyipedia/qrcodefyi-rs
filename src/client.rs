use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://qrcodefyi.com/api";

/// Async client for the QRCodeFYI API.
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

impl Client {
    /// Creates a new client with the default base URL.
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Creates a new client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: reqwest::Client::new(),
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, QrCodeFyiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(QrCodeFyiError::Api {
                status: resp.status().as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }

    /// Search across QR code types, encodings, and glossary terms.
    pub async fn search(&self, query: &str) -> Result<SearchResult, QrCodeFyiError> {
        let encoded = urlencoding(query);
        self.get(&format!("/search/?q={}", encoded)).await
    }

    /// Get details for a QR code type by slug.
    pub async fn qr_type(&self, slug: &str) -> Result<QrTypeDetail, QrCodeFyiError> {
        self.get(&format!("/qr-type/{}/", slug)).await
    }

    /// Get details for a QR code version by number.
    pub async fn version(&self, version: u32) -> Result<VersionDetail, QrCodeFyiError> {
        self.get(&format!("/version/{}/", version)).await
    }

    /// Get details for a QR code component by slug.
    pub async fn component(&self, slug: &str) -> Result<ComponentDetail, QrCodeFyiError> {
        self.get(&format!("/component/{}/", slug)).await
    }

    /// Get details for an encoding mode by slug.
    pub async fn encoding(&self, slug: &str) -> Result<EncodingDetail, QrCodeFyiError> {
        self.get(&format!("/encoding/{}/", slug)).await
    }

    /// Get details for a standard by slug.
    pub async fn standard(&self, slug: &str) -> Result<StandardDetail, QrCodeFyiError> {
        self.get(&format!("/standard/{}/", slug)).await
    }

    /// Get details for a use case by slug.
    pub async fn use_case(&self, slug: &str) -> Result<UseCaseDetail, QrCodeFyiError> {
        self.get(&format!("/use-case/{}/", slug)).await
    }

    /// Get a glossary term by slug.
    pub async fn glossary_term(&self, slug: &str) -> Result<GlossaryTerm, QrCodeFyiError> {
        self.get(&format!("/glossary/{}/", slug)).await
    }

    /// Compare two QR code types.
    pub async fn compare(&self, slug_a: &str, slug_b: &str) -> Result<CompareResult, QrCodeFyiError> {
        self.get(&format!("/compare/?a={}&b={}", slug_a, slug_b)).await
    }

    /// Get a random QR code type.
    pub async fn random(&self) -> Result<QrTypeDetail, QrCodeFyiError> {
        self.get("/random/").await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
