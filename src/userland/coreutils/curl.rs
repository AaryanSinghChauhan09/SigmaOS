//! # curl-compatible HTTP/HTTPS client
//!
//! Provides a programmatic HTTP client modelled after the `curl` command-line
//! tool.  Supports GET / POST / PUT / DELETE requests, custom headers, redirect
//! following, TLS certificate verification, a progress bar, and writing the
//! response body to a file (`--output`).
//!
//! ## Usage (shell invocation model)
//! ```text
//! curl -X POST https://example.com/api -H "Content-Type: application/json" \
//!      -d '{"key":"value"}' -o response.json
//! ```

#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;

extern crate alloc;

// ---------------------------------------------------------------------------
// HTTP method
// ---------------------------------------------------------------------------

/// HTTP request method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
        }
    }
}

// ---------------------------------------------------------------------------
// TLS configuration
// ---------------------------------------------------------------------------

/// TLS / certificate verification policy.
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Verify the server's TLS certificate chain (enabled by default).
    pub verify_peer: bool,
    /// Verify that the certificate's Common Name / SAN matches the host.
    pub verify_host: bool,
    /// Path to a CA bundle PEM file; `None` uses the system default.
    pub ca_bundle: Option<String>,
    /// Path to a client certificate (mTLS).
    pub client_cert: Option<String>,
    /// Path to the client private key for mTLS.
    pub client_key: Option<String>,
    /// Minimum TLS version to accept.
    pub min_tls_version: TlsVersion,
}

/// TLS protocol version floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TlsVersion {
    Tls10,
    Tls11,
    Tls12,
    Tls13,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            verify_peer: true,
            verify_host: true,
            ca_bundle: None,
            client_cert: None,
            client_key: None,
            min_tls_version: TlsVersion::Tls12,
        }
    }
}

// ---------------------------------------------------------------------------
// HTTP header
// ---------------------------------------------------------------------------

/// A single HTTP header field.
#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Progress callback
// ---------------------------------------------------------------------------

/// Callback invoked periodically during a transfer to report progress.
///
/// Arguments: `(downloaded_bytes, total_bytes_or_0_if_unknown)`.
pub type ProgressCallback = fn(u64, u64);

// ---------------------------------------------------------------------------
// Request builder
// ---------------------------------------------------------------------------

/// Builder for an HTTP request.
pub struct Request {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<Header>,
    pub body: Option<Vec<u8>>,
    /// Maximum number of redirects to follow (0 = do not follow).
    pub max_redirects: u32,
    pub tls: TlsConfig,
    /// Write the response body to this path instead of returning it in memory.
    pub output_file: Option<String>,
    pub progress_cb: Option<ProgressCallback>,
    /// Connection timeout in milliseconds.
    pub connect_timeout_ms: u64,
    /// Data transfer timeout in milliseconds (0 = unlimited).
    pub transfer_timeout_ms: u64,
    /// Send compressed `Accept-Encoding` and auto-decompress the response.
    pub compress: bool,
    /// Override the `User-Agent` header.
    pub user_agent: String,
}

impl Request {
    /// Constructs a GET request for the given URL.
    pub fn get(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Get, url)
    }

    /// Constructs a POST request for the given URL.
    pub fn post(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Post, url)
    }

    /// Constructs a PUT request for the given URL.
    pub fn put(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Put, url)
    }

    /// Constructs a DELETE request for the given URL.
    pub fn delete(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Delete, url)
    }

    fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: Vec::new(),
            body: None,
            max_redirects: 30,
            tls: TlsConfig::default(),
            output_file: None,
            progress_cb: None,
            connect_timeout_ms: 30_000,
            transfer_timeout_ms: 0,
            compress: true,
            user_agent: String::from("curl-sigma/0.1"),
        }
    }

    /// Appends a request header.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(Header::new(name, value));
        self
    }

    /// Sets the request body.
    pub fn body(mut self, data: Vec<u8>) -> Self {
        self.body = Some(data);
        self
    }

    /// Sets the request body from a string slice.
    pub fn body_str(self, s: &str) -> Self {
        self.body(s.as_bytes().to_vec())
    }

    /// Disables TLS certificate verification (`-k` / `--insecure`).
    pub fn insecure(mut self) -> Self {
        self.tls.verify_peer = false;
        self.tls.verify_host = false;
        self
    }

    /// Writes the response body to a file (`-o`).
    pub fn output(mut self, path: impl Into<String>) -> Self {
        self.output_file = Some(path.into());
        self
    }

    /// Registers a progress callback.
    pub fn on_progress(mut self, cb: ProgressCallback) -> Self {
        self.progress_cb = Some(cb);
        self
    }

    /// Sets the maximum number of redirects to follow.
    pub fn max_redirects(mut self, n: u32) -> Self {
        self.max_redirects = n;
        self
    }
}

// ---------------------------------------------------------------------------
// HTTP response
// ---------------------------------------------------------------------------

/// An HTTP response.
#[derive(Debug)]
pub struct Response {
    /// HTTP status code (e.g. 200, 404).
    pub status: u16,
    /// Response headers.
    pub headers: Vec<Header>,
    /// Response body bytes (empty if written directly to a file).
    pub body: Vec<u8>,
    /// URL of the final request after following any redirects.
    pub final_url: String,
    /// Total number of bytes transferred (including headers).
    pub bytes_transferred: u64,
}

impl Response {
    /// Returns `true` if the status code indicates success (200–299).
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    /// Parses the body as a UTF-8 string.
    pub fn text(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.body)
    }

    /// Returns the value of the first header with the given name
    /// (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .map(|h| h.value.as_str())
    }
}

// ---------------------------------------------------------------------------
// URL parser (minimal)
// ---------------------------------------------------------------------------

/// Components of a parsed URL.
#[derive(Debug, Clone)]
pub struct ParsedUrl {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: Option<String>,
    pub is_tls: bool,
}

impl ParsedUrl {
    /// Parses a URL string into its components.
    pub fn parse(url: &str) -> Result<Self, CurlError> {
        let (scheme, rest) = url.split_once("://").ok_or(CurlError::InvalidUrl)?;
        let is_tls = scheme.eq_ignore_ascii_case("https");
        let default_port = if is_tls { 443 } else { 80 };

        let (authority, path_and_query) = rest.split_once('/').unwrap_or((rest, ""));
        let path = alloc::format!("/{}", path_and_query);
        let (path, query) = match path.split_once('?') {
            Some((p, q)) => (p.into(), Some(q.into())),
            None => (path, None),
        };

        let (host_part, port) = match authority.split_once(':') {
            Some((h, p)) => {
                let port = p.parse::<u16>().map_err(|_| CurlError::InvalidUrl)?;
                (h, port)
            }
            None => (authority, default_port),
        };

        Ok(Self {
            scheme: scheme.into(),
            host: host_part.into(),
            port,
            path,
            query,
            is_tls,
        })
    }

    /// Returns the request target (path + optional query string).
    pub fn request_target(&self) -> String {
        match &self.query {
            Some(q) => alloc::format!("{}?{}", self.path, q),
            None => self.path.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

/// The HTTP client responsible for executing [`Request`]s.
pub struct Client {
    /// Default TLS configuration applied to all requests.
    pub tls: TlsConfig,
    /// Default connect timeout in milliseconds.
    pub connect_timeout_ms: u64,
}

impl Client {
    pub fn new() -> Self {
        Self {
            tls: TlsConfig::default(),
            connect_timeout_ms: 30_000,
        }
    }

    /// Executes the given request and returns the response.
    ///
    /// Internally this will:
    /// 1. Parse the URL.
    /// 2. Open a TCP connection (optionally wrapped in TLS).
    /// 3. Send the HTTP/1.1 request.
    /// 4. Read and parse the response status line and headers.
    /// 5. Stream the body, calling the progress callback if set.
    /// 6. Follow redirects up to `max_redirects`.
    pub fn execute(&self, req: Request) -> Result<Response, CurlError> {
        let _parsed = ParsedUrl::parse(&req.url)?;
        // TODO: open TCP socket (or TLS session) and perform HTTP/1.1 exchange
        //       Handle chunked transfer-encoding, Content-Length, gzip, etc.
        Err(CurlError::Unimplemented)
    }

    /// Convenience wrapper for a simple GET.
    pub fn get(&self, url: &str) -> Result<Response, CurlError> {
        self.execute(Request::get(url))
    }

    /// Convenience wrapper for a JSON POST.
    pub fn post_json(&self, url: &str, json: &str) -> Result<Response, CurlError> {
        self.execute(
            Request::post(url)
                .header("Content-Type", "application/json")
                .body_str(json),
        )
    }
}

// ---------------------------------------------------------------------------
// Progress bar renderer
// ---------------------------------------------------------------------------

/// Renders a text-mode progress bar similar to curl's output.
pub struct ProgressBar {
    pub total: u64,
    pub downloaded: u64,
    /// Width of the bar in characters.
    pub width: usize,
}

impl ProgressBar {
    pub fn new(total: u64, width: usize) -> Self {
        Self {
            total,
            downloaded: 0,
            width,
        }
    }

    /// Updates progress and writes the bar to the provided output slice.
    pub fn update(&mut self, downloaded: u64, buf: &mut String) {
        self.downloaded = downloaded;
        let pct = if self.total == 0 {
            0.0
        } else {
            self.downloaded as f64 / self.total as f64
        };
        let filled = ((pct * self.width as f64) as usize).min(self.width);
        buf.push('[');
        for i in 0..self.width {
            if i < filled {
                buf.push('#');
            } else {
                buf.push(' ');
            }
        }
        buf.push_str(&alloc::format!("] {:.1}%", pct * 100.0));
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors produced by the HTTP client.
#[derive(Debug)]
pub enum CurlError {
    /// The URL could not be parsed.
    InvalidUrl,
    /// DNS resolution failed.
    DnsResolutionFailed,
    /// TCP connection timed out or was refused.
    ConnectionFailed,
    /// TLS handshake or certificate verification failed.
    TlsError(String),
    /// The server returned an invalid or truncated HTTP response.
    ProtocolError,
    /// Too many redirects.
    TooManyRedirects,
    /// File I/O error writing the output file.
    IoError(String),
    /// Not yet implemented.
    Unimplemented,
}

// ---------------------------------------------------------------------------
// CLI entry point
// ---------------------------------------------------------------------------

/// Parses a `curl`-style argument list and executes the request.
///
/// Recognised flags:
/// `-X METHOD`, `-H "Name: Value"`, `-d body`, `-o output`, `-k`, `-L`,
/// `--max-redirs N`, `--output PATH`, `--compressed`.
pub fn run(args: &[&str]) -> Result<Response, CurlError> {
    let mut method = HttpMethod::Get;
    let mut url = String::new();
    let mut headers: Vec<Header> = Vec::new();
    let mut body: Option<Vec<u8>> = None;
    let mut output: Option<String> = None;
    let mut insecure = false;
    let mut max_redirects = 30u32;
    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "-X" | "--request" => {
                i += 1;
                method = match args.get(i).copied().unwrap_or("GET") {
                    "POST" => HttpMethod::Post,
                    "PUT" => HttpMethod::Put,
                    "DELETE" => HttpMethod::Delete,
                    "PATCH" => HttpMethod::Patch,
                    _ => HttpMethod::Get,
                };
            }
            "-H" | "--header" => {
                i += 1;
                if let Some(hdr) = args.get(i) {
                    if let Some((n, v)) = hdr.split_once(": ") {
                        headers.push(Header::new(n, v));
                    }
                }
            }
            "-d" | "--data" | "--data-raw" => {
                i += 1;
                if let Some(d) = args.get(i) {
                    body = Some(d.as_bytes().to_vec());
                    if method == HttpMethod::Get {
                        method = HttpMethod::Post;
                    }
                }
            }
            "-o" | "--output" => {
                i += 1;
                output = args.get(i).map(|s| String::from(*s));
            }
            "-k" | "--insecure" => {
                insecure = true;
            }
            "--max-redirs" => {
                i += 1;
                max_redirects = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(30);
            }
            arg if !arg.starts_with('-') => {
                url = arg.into();
            }
            _ => {}
        }
        i += 1;
    }
    let mut req = Request::new(method, url).max_redirects(max_redirects);
    for h in headers {
        req.headers.push(h);
    }
    if let Some(b) = body {
        req = req.body(b);
    }
    if let Some(o) = output {
        req = req.output(o);
    }
    if insecure {
        req = req.insecure();
    }
    Client::new().execute(req)
}
