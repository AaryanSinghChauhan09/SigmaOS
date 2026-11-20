//! SigmaOS Player Library
//!
//! Playlist and media library management: tracks, albums, playlists,
//! and a queue engine. Backend-agnostic; the media player UI layer
//! drives this model. Upstream: GStreamer playbin, mpv libmpv,
//! Rhythmbox music library model.

use std::collections::HashMap;

/// Metadata for a single media track.
#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub id:         u32,
    pub title:      String,
    pub artist:     String,
    pub album:      String,
    pub duration_s: u32,
    pub uri:        String,
    pub track_num:  u32,
}

impl Track {
    pub fn new(id: u32, title: &str, artist: &str, album: &str,
               duration_s: u32, uri: &str) -> Self {
        Self { id, title: title.into(), artist: artist.into(),
               album: album.into(), duration_s, uri: uri.into(), track_num: 0 }
    }

    pub fn duration_display(&self) -> String {
        let m = self.duration_s / 60;
        let s = self.duration_s % 60;
        format!("{m}:{s:02}")
    }
}

/// A named playlist of track IDs (references into the library).
#[derive(Debug, Clone)]
pub struct Playlist {
    pub name:     String,
    track_ids:    Vec<u32>,
}

impl Playlist {
    pub fn new(name: &str) -> Self { Self { name: name.into(), track_ids: Vec::new() } }
    pub fn add(&mut self, id: u32)    { if !self.track_ids.contains(&id) { self.track_ids.push(id); } }
    pub fn remove(&mut self, id: u32) { self.track_ids.retain(|&t| t != id); }
    pub fn ids(&self) -> &[u32]       { &self.track_ids }
    pub fn len(&self) -> usize        { self.track_ids.len() }
    pub fn is_empty(&self) -> bool    { self.track_ids.is_empty() }
}

/// Repeat mode for the playback queue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeatMode { None, One, All }

/// Playback queue with shuffle and repeat support.
pub struct Queue {
    tracks:     Vec<u32>,   // ordered track IDs
    cursor:     usize,
    repeat:     RepeatMode,
    shuffle:    bool,
}

impl Queue {
    pub fn new() -> Self {
        Self { tracks: Vec::new(), cursor: 0, repeat: RepeatMode::None, shuffle: false }
    }

    pub fn load(&mut self, ids: Vec<u32>) { self.tracks = ids; self.cursor = 0; }
    pub fn append(&mut self, id: u32)     { self.tracks.push(id); }

    pub fn current(&self) -> Option<u32> { self.tracks.get(self.cursor).copied() }

    pub fn advance(&mut self) -> Option<u32> {
        match self.repeat {
            RepeatMode::One => {}   // stay on current
            RepeatMode::All => { self.cursor = (self.cursor + 1) % self.tracks.len().max(1); }
            RepeatMode::None => {
                if self.cursor + 1 < self.tracks.len() { self.cursor += 1; }
                else { return None; }
            }
        }
        self.current()
    }

    pub fn rewind(&mut self) -> Option<u32> {
        if self.cursor > 0 { self.cursor -= 1; }
        self.current()
    }

    pub fn len(&self) -> usize { self.tracks.len() }
    pub fn set_repeat(&mut self, m: RepeatMode) { self.repeat = m; }
    pub fn set_shuffle(&mut self, s: bool)       { self.shuffle = s; }
}

impl Default for Queue { fn default() -> Self { Self::new() } }

/// The central music library: tracks, albums, playlists, and queue.
pub struct Library {
    tracks:    HashMap<u32, Track>,
    playlists: HashMap<String, Playlist>,
    pub queue: Queue,
    next_id:   u32,
}

impl Library {
    pub fn new() -> Self {
        Self {
            tracks:    HashMap::new(),
            playlists: HashMap::new(),
            queue:     Queue::new(),
            next_id:   1,
        }
    }

    pub fn add_track(&mut self, title: &str, artist: &str, album: &str,
                     duration_s: u32, uri: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.tracks.insert(id, Track::new(id, title, artist, album, duration_s, uri));
        id
    }

    pub fn track(&self, id: u32) -> Option<&Track> { self.tracks.get(&id) }
    pub fn track_count(&self) -> usize { self.tracks.len() }

    /// Search tracks by title or artist (case-insensitive).
    pub fn search(&self, query: &str) -> Vec<&Track> {
        let q = query.to_lowercase();
        let mut results: Vec<&Track> = self.tracks.values()
            .filter(|t| t.title.to_lowercase().contains(&q)
                     || t.artist.to_lowercase().contains(&q))
            .collect();
        results.sort_by_key(|t| t.id);
        results
    }

    /// List all tracks in a given album.
    pub fn album_tracks(&self, album: &str) -> Vec<&Track> {
        let mut v: Vec<&Track> = self.tracks.values()
            .filter(|t| t.album.eq_ignore_ascii_case(album))
            .collect();
        v.sort_by_key(|t| t.track_num);
        v
    }

    pub fn create_playlist(&mut self, name: &str) { self.playlists.insert(name.into(), Playlist::new(name)); }
    pub fn playlist_mut(&mut self, name: &str) -> Option<&mut Playlist> { self.playlists.get_mut(name) }
    pub fn playlist(&self, name: &str) -> Option<&Playlist> { self.playlists.get(name) }
    pub fn playlist_names(&self) -> Vec<&str> { self.playlists.keys().map(|s| s.as_str()).collect() }
}

impl Default for Library { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_library() -> Library {
        let mut lib = Library::new();
        lib.add_track("Bohemian Rhapsody", "Queen",    "A Night at the Opera", 355, "file://1.flac");
        lib.add_track("Hotel California",  "Eagles",   "Hotel California",     391, "file://2.flac");
        lib.add_track("Stairway to Heaven","Led Zeppelin","IV",                482, "file://3.flac");
        lib
    }

    #[test]
    fn add_and_retrieve_tracks() {
        let lib = sample_library();
        assert_eq!(lib.track_count(), 3);
        assert_eq!(lib.track(1).unwrap().title, "Bohemian Rhapsody");
    }

    #[test]
    fn search_by_title() {
        let lib = sample_library();
        let r = lib.search("hotel");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].artist, "Eagles");
    }

    #[test]
    fn search_by_artist() {
        let lib = sample_library();
        let r = lib.search("queen");
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn playlist_add_remove() {
        let mut lib = sample_library();
        lib.create_playlist("Favorites");
        lib.playlist_mut("Favorites").unwrap().add(1);
        lib.playlist_mut("Favorites").unwrap().add(2);
        assert_eq!(lib.playlist("Favorites").unwrap().len(), 2);
        lib.playlist_mut("Favorites").unwrap().remove(1);
        assert_eq!(lib.playlist("Favorites").unwrap().len(), 1);
    }

    #[test]
    fn queue_advance_repeat_all() {
        let mut q = Queue::new();
        q.load(vec![1, 2, 3]);
        q.set_repeat(RepeatMode::All);
        assert_eq!(q.current(), Some(1));
        q.advance(); assert_eq!(q.current(), Some(2));
        q.advance(); assert_eq!(q.current(), Some(3));
        q.advance(); assert_eq!(q.current(), Some(1)); // wraps
    }

    #[test]
    fn track_duration_display() {
        let t = Track::new(1, "T", "A", "B", 245, "file://t.mp3");
        assert_eq!(t.duration_display(), "4:05");
    }
}
