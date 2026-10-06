#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(dead_code)]

// SigmaOS Hypnotix IPTV Player - Linux Mint Hypnotix-inspired IPTV Streaming
// Modern IPTV streaming application with support for live TV, movies, and series

use std::collections::HashMap;

/// IPTV provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    M3UUrl,
    XtreamApi,
    LocalM3U,
}

/// Channel category
#[derive(Debug, Clone)]
pub struct ChannelCategory {
    pub name: String,
    pub country: String,
    pub language: String,
}

/// Channel
#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub logo: Option<String>,
    pub url: String,
    pub categories: Vec<ChannelCategory>,
    pub is_favorite: bool,
    pub is_custom: bool,
}

/// IPTV provider
#[derive(Debug, Clone)]
pub struct IptvProvider {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub channels: Vec<Channel>,
    pub movies: Vec<Channel>,
    pub series: Vec<Channel>,
}

/// Stream info
#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub codec: String,
    pub resolution: String,
    pub bitrate: u32,
    pub fps: f64,
}

/// Playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Buffering,
    Playing,
    Paused,
    Error,
}

/// Hypnotix configuration
#[derive(Debug, Clone)]
pub struct HypnotixConfig {
    pub hardware_acceleration: bool,
    pub default_provider: Option<String>,
    pub auto_play: bool,
    pub continue_from_last: bool,
    pub enable_favorites: bool,
    pub enable_custom_channels: bool,
}

impl Default for HypnotixConfig {
    fn default() -> Self {
        Self {
            hardware_acceleration: true,
            default_provider: None,
            auto_play: true,
            continue_from_last: true,
            enable_favorites: true,
            enable_custom_channels: true,
        }
    }
}

/// Hypnotix IPTV player
#[derive(Debug, Clone)]
pub struct HypnotixPlayer {
    config: HypnotixConfig,
    providers: HashMap<String, IptvProvider>,
    current_provider: Option<String>,
    current_channel: Option<String>,
    playback_state: PlaybackState,
    stream_info: Option<StreamInfo>,
    volume: f64,
    is_fullscreen: bool,
}

impl HypnotixPlayer {
    pub fn new(config: HypnotixConfig) -> Self {
        Self {
            config,
            providers: HashMap::new(),
            current_provider: None,
            current_channel: None,
            playback_state: PlaybackState::Stopped,
            stream_info: None,
            volume: 1.0,
            is_fullscreen: false,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(HypnotixConfig::default())
    }

    /// Add IPTV provider
    pub fn add_provider(&mut self, provider: IptvProvider) -> Result<(), String> {
        self.providers.insert(provider.id.clone(), provider);
        Ok(())
    }

    /// Remove provider
    pub fn remove_provider(&mut self, id: &str) -> Result<(), String> {
        self.providers.remove(id);
        if self.current_provider.as_ref() == Some(&id.to_string()) {
            self.current_provider = None;
            self.current_channel = None;
        }
        Ok(())
    }

    /// Get provider
    pub fn get_provider(&self, id: &str) -> Option<&IptvProvider> {
        self.providers.get(id)
    }

    /// Get all providers
    pub fn get_providers(&self) -> Vec<&IptvProvider> {
        self.providers.values().collect()
    }

    /// Set current provider
    pub fn set_provider(&mut self, id: &str) -> Result<(), String> {
        if self.providers.contains_key(id) {
            self.current_provider = Some(id.to_string());
            self.current_channel = None;
            Ok(())
        } else {
            Err(format!("Provider {} not found", id))
        }
    }

    /// Get current provider
    pub fn get_current_provider(&self) -> Option<&IptvProvider> {
        self.current_provider.as_ref().and_then(|id| self.providers.get(id))
    }

    /// Play channel
    pub fn play_channel(&mut self, channel_id: &str) -> Result<(), String> {
        if let Some(provider_id) = &self.current_provider {
            if let Some(provider) = self.providers.get(provider_id) {
                let channel = provider.channels.iter().find(|c| c.id == channel_id);
                if let Some(ch) = channel {
                    self.current_channel = Some(channel_id.to_string());
                    self.playback_state = PlaybackState::Buffering;
                    self.stream_info = Some(StreamInfo {
                        codec: "h264".to_string(),
                        resolution: "1920x1080".to_string(),
                        bitrate: 5000,
                        fps: 30.0,
                    });
                    self.playback_state = PlaybackState::Playing;
                    Ok(())
                } else {
                    Err(format!("Channel {} not found", channel_id))
                }
            } else {
                Err("Provider not found".to_string())
            }
        } else {
            Err("No provider selected".to_string())
        }
    }

    /// Play movie
    pub fn play_movie(&mut self, movie_id: &str) -> Result<(), String> {
        if let Some(provider_id) = &self.current_provider {
            if let Some(provider) = self.providers.get(provider_id) {
                let movie = provider.movies.iter().find(|m| m.id == movie_id);
                if let Some(m) = movie {
                    self.current_channel = Some(movie_id.to_string());
                    self.playback_state = PlaybackState::Buffering;
                    self.stream_info = Some(StreamInfo {
                        codec: "h264".to_string(),
                        resolution: "1920x1080".to_string(),
                        bitrate: 8000,
                        fps: 24.0,
                    });
                    self.playback_state = PlaybackState::Playing;
                    Ok(())
                } else {
                    Err(format!("Movie {} not found", movie_id))
                }
            } else {
                Err("Provider not found".to_string())
            }
        } else {
            Err("No provider selected".to_string())
        }
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        self.stream_info = None;
    }

    /// Pause playback
    pub fn pause(&mut self) {
        if self.playback_state == PlaybackState::Playing {
            self.playback_state = PlaybackState::Paused;
        }
    }

    /// Resume playback
    pub fn resume(&mut self) {
        if self.playback_state == PlaybackState::Paused {
            self.playback_state = PlaybackState::Playing;
        }
    }

    /// Get playback state
    pub fn get_playback_state(&self) -> PlaybackState {
        self.playback_state
    }

    /// Get stream info
    pub fn get_stream_info(&self) -> Option<&StreamInfo> {
        self.stream_info.as_ref()
    }

    /// Set volume
    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Get volume
    pub fn get_volume(&self) -> f64 {
        self.volume
    }

    /// Toggle fullscreen
    pub fn toggle_fullscreen(&mut self) {
        self.is_fullscreen = !self.is_fullscreen;
    }

    /// Get fullscreen state
    pub fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    /// Add channel to favorites
    pub fn add_favorite(&mut self, channel_id: &str) -> Result<(), String> {
        if let Some(provider_id) = &self.current_provider {
            if let Some(provider) = self.providers.get_mut(provider_id) {
                if let Some(channel) = provider.channels.iter_mut().find(|c| c.id == channel_id) {
                    channel.is_favorite = true;
                    Ok(())
                } else {
                    Err(format!("Channel {} not found", channel_id))
                }
            } else {
                Err("Provider not found".to_string())
            }
        } else {
            Err("No provider selected".to_string())
        }
    }

    /// Remove channel from favorites
    pub fn remove_favorite(&mut self, channel_id: &str) -> Result<(), String> {
        if let Some(provider_id) = &self.current_provider {
            if let Some(provider) = self.providers.get_mut(provider_id) {
                if let Some(channel) = provider.channels.iter_mut().find(|c| c.id == channel_id) {
                    channel.is_favorite = false;
                    Ok(())
                } else {
                    Err(format!("Channel {} not found", channel_id))
                }
            } else {
                Err("Provider not found".to_string())
            }
        } else {
            Err("No provider selected".to_string())
        }
    }

    /// Get favorite channels
    pub fn get_favorites(&self) -> Vec<&Channel> {
        self.providers
            .values()
            .flat_map(|p| p.channels.iter().filter(|c| c.is_favorite))
            .collect()
    }

    /// Create custom channel
    pub fn create_custom_channel(&mut self, name: String, url: String) -> Result<(), String> {
        if !self.config.enable_custom_channels {
            return Err("Custom channels are disabled".to_string());
        }

        let channel = Channel {
            id: format!("custom-{}", name.to_lowercase().replace(' ', "-")),
            name: name.clone(),
            logo: None,
            url,
            categories: vec![],
            is_favorite: false,
            is_custom: true,
        };

        if let Some(provider_id) = &self.current_provider {
            if let Some(provider) = self.providers.get_mut(provider_id) {
                provider.channels.push(channel);
                Ok(())
            } else {
                Err("Provider not found".to_string())
            }
        } else {
            Err("No provider selected".to_string())
        }
    }

    /// Search channels
    pub fn search_channels(&self, query: &str) -> Vec<&Channel> {
        self.providers
            .values()
            .flat_map(|p| {
                p.channels.iter().filter(|c| {
                    c.name.to_lowercase().contains(&query.to_lowercase())
                })
            })
            .collect()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> (usize, usize, usize, usize) {
        let total_providers = self.providers.len();
        let total_channels: usize = self.providers.values().map(|p| p.channels.len()).sum();
        let total_movies: usize = self.providers.values().map(|p| p.movies.len()).sum();
        let total_series: usize = self.providers.values().map(|p| p.series.len()).sum();
        (total_providers, total_channels, total_movies, total_series)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypnotix_player_creation() {
        let player = HypnotixPlayer::with_default_config();
        assert_eq!(player.providers.len(), 0);
        assert_eq!(player.playback_state, PlaybackState::Stopped);
    }

    #[test]
    fn test_config_default() {
        let config = HypnotixConfig::default();
        assert!(config.hardware_acceleration);
        assert!(config.auto_play);
        assert!(config.enable_favorites);
        assert!(config.enable_custom_channels);
    }

    #[test]
    fn test_add_provider() {
        let mut player = HypnotixPlayer::with_default_config();
        let provider = IptvProvider {
            id: "test-provider".to_string(),
            name: "Test Provider".to_string(),
            provider_type: ProviderType::M3UUrl,
            url: Some("http://example.com/playlist.m3u".to_string()),
            username: None,
            password: None,
            channels: vec![],
            movies: vec![],
            series: vec![],
        };

        player.add_provider(provider).unwrap();
        assert_eq!(player.providers.len(), 1);
    }

    #[test]
    fn test_set_provider() {
        let mut player = HypnotixPlayer::with_default_config();
        let provider = IptvProvider {
            id: "test-provider".to_string(),
            name: "Test Provider".to_string(),
            provider_type: ProviderType::M3UUrl,
            url: Some("http://example.com/playlist.m3u".to_string()),
            username: None,
            password: None,
            channels: vec![],
            movies: vec![],
            series: vec![],
        };

        player.add_provider(provider).unwrap();
        player.set_provider("test-provider").unwrap();

        assert_eq!(player.current_provider, Some("test-provider".to_string()));
    }

    #[test]
    fn test_playback_controls() {
        let mut player = HypnotixPlayer::with_default_config();
        player.pause();
        assert_eq!(player.playback_state, PlaybackState::Stopped);
        
        player.resume();
        assert_eq!(player.playback_state, PlaybackState::Stopped);
        
        player.stop();
        assert_eq!(player.playback_state, PlaybackState::Stopped);
    }

    #[test]
    fn test_volume_control() {
        let mut player = HypnotixPlayer::with_default_config();
        player.set_volume(0.75);
        assert_eq!(player.get_volume(), 0.75);
        
        player.set_volume(1.5);
        assert_eq!(player.get_volume(), 1.0);
        
        player.set_volume(-0.5);
        assert_eq!(player.get_volume(), 0.0);
    }

    #[test]
    fn test_fullscreen_toggle() {
        let mut player = HypnotixPlayer::with_default_config();
        player.toggle_fullscreen();
        assert!(player.is_fullscreen());
        
        player.toggle_fullscreen();
        assert!(!player.is_fullscreen());
    }

    #[test]
    fn test_search_channels() {
        let mut player = HypnotixPlayer::with_default_config();
        let provider = IptvProvider {
            id: "test-provider".to_string(),
            name: "Test Provider".to_string(),
            provider_type: ProviderType::M3UUrl,
            url: Some("http://example.com/playlist.m3u".to_string()),
            username: None,
            password: None,
            channels: vec![
                Channel {
                    id: "1".to_string(),
                    name: "Test Channel".to_string(),
                    logo: None,
                    url: "http://example.com/stream.m3u8".to_string(),
                    categories: vec![],
                    is_favorite: false,
                    is_custom: false,
                },
            ],
            movies: vec![],
            series: vec![],
        };

        player.add_provider(provider).unwrap();
        let results = player.search_channels("test");
        
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut player = HypnotixPlayer::with_default_config();
        let provider = IptvProvider {
            id: "test-provider".to_string(),
            name: "Test Provider".to_string(),
            provider_type: ProviderType::M3UUrl,
            url: Some("http://example.com/playlist.m3u".to_string()),
            username: None,
            password: None,
            channels: vec![],
            movies: vec![],
            series: vec![],
        };

        player.add_provider(provider).unwrap();
        let (providers, channels, movies, series) = player.get_statistics();
        
        assert_eq!(providers, 1);
        assert_eq!(channels, 0);
        assert_eq!(movies, 0);
        assert_eq!(series, 0);
    }
}
