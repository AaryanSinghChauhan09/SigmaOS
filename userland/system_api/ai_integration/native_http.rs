// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/system_api/ai_integration/native_http.rs — Native HTTP Client
//
// Simple HTTP client implementation without external dependencies
// Supports basic GET/POST requests for AI model API calls
//
// Language: Rust (std)

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// HTTP response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status_code: u16) -> Self {
        Self {
            status_code,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    pub fn json<T: for<'de> serde::de::Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }
}

/// HTTP request method
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

/// Simple HTTP client
pub struct HttpClient {
    timeout: Duration,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Perform HTTP GET request
    pub fn get(&self, url: &str) -> Result<HttpResponse, String> {
        self.request(url, HttpMethod::Get, None, &[])
    }

    /// Perform HTTP POST request
    pub fn post(&self, url: &str, body: &[u8], content_type: &str) -> Result<HttpResponse, String> {
        self.request(url, HttpMethod::Post, Some(body), &[(content_type)])
    }

    /// Perform HTTP POST request with JSON
    pub fn post_json<T: serde::Serialize>(&self, url: &str, data: &T) -> Result<HttpResponse, String> {
        let body = serde_json::to_vec(data).map_err(|e| format!("Failed to serialize JSON: {}", e))?;
        self.post(url, &body, "application/json")
    }

    /// Generic HTTP request
    fn request(
        &self,
        url: &str,
        method: HttpMethod,
        body: Option<&[u8]>,
        content_types: &[&str],
    ) -> Result<HttpResponse, String> {
        // Parse URL
        let (host, port, path) = self.parse_url(url)?;

        // Connect to server
        let mut stream = TcpStream::connect(format!("{}:{}", host, port))
            .map_err(|e| format!("Failed to connect: {}", e))?;

        stream
            .set_read_timeout(Some(self.timeout))
            .map_err(|e| format!("Failed to set read timeout: {}", e))?;
        stream
            .set_write_timeout(Some(self.timeout))
            .map_err(|e| format!("Failed to set write timeout: {}", e))?;

        // Build request
        let method_str = match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        };

        let body_len = body.map(|b| b.len()).unwrap_or(0);
        let content_type = content_types.first().unwrap_or(&"text/plain");

        let request = format!(
            "{} {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n",
            method_str, path, host, content_type, body_len
        );

        // Send request
        stream
            .write_all(request.as_bytes())
            .map_err(|e| format!("Failed to write request: {}", e))?;

        if let Some(body_data) = body {
            stream
                .write_all(body_data)
                .map_err(|e| format!("Failed to write body: {}", e))?;
        }

        // Read response
        let mut reader = BufReader::new(stream);
        let mut response = HttpResponse::new(0);

        // Read status line
        let mut status_line = String::new();
        reader
            .read_line(&mut status_line)
            .map_err(|e| format!("Failed to read status line: {}", e))?;

        // Parse status code
        if let Some(status_part) = status_line.split(' ').nth(1) {
            response.status_code = status_part
                .parse()
                .unwrap_or(500);
        }

        // Read headers
        loop {
            let mut header_line = String::new();
            reader
                .read_line(&mut header_line)
                .map_err(|e| format!("Failed to read header: {}", e))?;

            if header_line == "\r\n" || header_line.is_empty() {
                break;
            }

            if let Some((key, value)) = header_line.split_once(':') {
                response.headers.push((
                    key.trim().to_string(),
                    value.trim().trim_end_matches('\r').to_string(),
                ));
            }
        }

        // Read body
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read body: {}", e))?;
        response.body = buffer;

        Ok(response)
    }

    /// Parse URL into host, port, and path
    fn parse_url(&self, url: &str) -> Result<(String, u16, String), String> {
        // Remove protocol
        let url = url.strip_prefix("http://")
            .or_else(|| url.strip_prefix("https://"))
            .ok_or("Invalid URL: missing protocol")?;

        // Split host and path
        let (host_part, path) = url.split_once('/').unwrap_or((url, ""));

        // Split host and port
        let (host, port) = host_part.split_once(':').unwrap_or((host_part, "80"));

        let port = port
            .parse()
            .map_err(|e| format!("Invalid port: {}", e))?;

        let path = if path.is_empty() { "/" } else { &format!("/{}", path) };

        Ok((host.to_string(), port, path.to_string()))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        let client = HttpClient::new();
        let (host, port, path) = client.parse_url("http://example.com/path").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 80);
        assert_eq!(path, "/path");

        let (host, port, path) = client.parse_url("http://example.com:8080").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 8080);
        assert_eq!(path, "/");
    }
}
