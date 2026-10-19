//! SigmaDeck - Deck and Presentation System
//!
//! Provides comprehensive deck and presentation management inspired by Deck Craft.
//! Supports slides, presentations, transitions, and slide organization.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Slide transition type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideTransition {
    None,
    Fade,
    Slide,
    Zoom,
    Flip,
    Rotate,
}

/// Slide element type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideElementType {
    Text,
    Image,
    Shape,
    Chart,
    Video,
    Table,
}

/// Slide element
#[derive(Debug, Clone)]
pub struct SlideElement {
    pub id: String,
    pub element_type: SlideElementType,
    pub content: String,
    pub position_x: f32,
    pub position_y: f32,
    pub width: f32,
    pub height: f32,
}

impl SlideElement {
    pub fn new(id: String, element_type: SlideElementType, content: String) -> Self {
        Self {
            id,
            element_type,
            content,
            position_x: 0.0,
            position_y: 0.0,
            width: 100.0,
            height: 100.0,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position_x = x;
        self.position_y = y;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

/// Slide
#[derive(Debug, Clone)]
pub struct Slide {
    pub id: String,
    pub number: u32,
    pub title: String,
    pub notes: String,
    pub elements: BTreeMap<String, SlideElement>,
    pub transition: SlideTransition,
    pub duration: u32, // in seconds
}

impl Slide {
    pub fn new(id: String, number: u32, title: String) -> Self {
        Self {
            id,
            number,
            title,
            notes: String::new(),
            elements: BTreeMap::new(),
            transition: SlideTransition::None,
            duration: 0,
        }
    }

    pub fn add_element(&mut self, element: SlideElement) {
        self.elements.insert(element.id.clone(), element);
    }

    pub fn remove_element(&mut self, id: &str) -> bool {
        self.elements.remove(id).is_some()
    }

    pub fn get_element(&self, id: &str) -> Option<&SlideElement> {
        self.elements.get(id)
    }

    pub fn set_transition(&mut self, transition: SlideTransition) {
        self.transition = transition;
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
    }

    pub fn element_count(&self) -> usize {
        self.elements.len()
    }
}

/// Presentation deck
#[derive(Debug, Clone)]
pub struct PresentationDeck {
    pub id: String,
    pub name: String,
    pub author: String,
    pub slides: BTreeMap<String, Slide>,
    pub slide_count: u32,
}

impl PresentationDeck {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            author: String::new(),
            slides: BTreeMap::new(),
            slide_count: 0,
        }
    }

    pub fn add_slide(&mut self, slide: Slide) {
        self.slide_count += 1;
        self.slides.insert(slide.id.clone(), slide);
    }

    pub fn remove_slide(&mut self, id: &str) -> bool {
        if self.slides.remove(id).is_some() {
            self.slide_count -= 1;
            true
        } else {
            false
        }
    }

    pub fn get_slide(&self, id: &str) -> Option<&Slide> {
        self.slides.get(id)
    }

    pub fn get_slide_mut(&mut self, id: &str) -> Option<&mut Slide> {
        self.slides.get_mut(id)
    }

    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }
}

/// SigmaDeck - Deck and presentation system
#[derive(Debug)]
pub struct SigmaDeck {
    decks: BTreeMap<String, PresentationDeck>,
    current_deck: Option<String>,
    current_slide: Option<String>,
}

impl SigmaDeck {
    pub fn new() -> Self {
        Self {
            decks: BTreeMap::new(),
            current_deck: None,
            current_slide: None,
        }
    }

    pub fn add_deck(&mut self, deck: PresentationDeck) {
        self.decks.insert(deck.id.clone(), deck);
    }

    pub fn remove_deck(&mut self, id: &str) -> bool {
        if self.current_deck.as_ref() == Some(&id.to_string()) {
            self.current_deck = None;
            self.current_slide = None;
        }
        self.decks.remove(id).is_some()
    }

    pub fn get_deck(&self, id: &str) -> Option<&PresentationDeck> {
        self.decks.get(id)
    }

    pub fn get_deck_mut(&mut self, id: &str) -> Option<&mut PresentationDeck> {
        self.decks.get_mut(id)
    }

    pub fn set_current_deck(&mut self, id: &str) -> bool {
        if self.decks.contains_key(id) {
            self.current_deck = Some(id.to_string());
            self.current_slide = None;
            true
        } else {
            false
        }
    }

    pub fn get_current_deck(&self) -> Option<&PresentationDeck> {
        self.current_deck.as_ref()
            .and_then(|id| self.decks.get(id))
    }

    pub fn set_current_slide(&mut self, slide_id: String) -> bool {
        if let Some(deck_id) = &self.current_deck {
            if let Some(deck) = self.decks.get(deck_id) {
                if deck.slides.contains_key(&slide_id) {
                    self.current_slide = Some(slide_id);
                    return true;
                }
            }
        }
        false
    }

    pub fn get_current_slide(&self) -> Option<&Slide> {
        self.current_deck.as_ref()
            .and_then(|deck_id| self.decks.get(deck_id))
            .and_then(|deck| self.current_slide.as_ref())
            .and_then(|slide_id| deck.slides.get(slide_id))
    }

    pub fn next_slide(&mut self) -> bool {
        if let Some(deck_id) = &self.current_deck {
            if let Some(deck) = self.decks.get(deck_id) {
                if let Some(current_id) = &self.current_slide {
                    let mut found = false;
                    for (id, _) in &deck.slides {
                        if found {
                            self.current_slide = Some(id.clone());
                            return true;
                        }
                        if id == current_id {
                            found = true;
                        }
                    }
                    self.current_slide = None;
                    false
                } else {
                    // First slide
                    if let Some((first_id, _)) = deck.slides.iter().next() {
                        self.current_slide = Some(first_id.clone());
                        true
                    } else {
                        false
                    }
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn previous_slide(&mut self) -> bool {
        if let Some(deck_id) = &self.current_deck {
            if let Some(deck) = self.decks.get(deck_id) {
                if let Some(current_id) = &self.current_slide {
                    let mut prev_id: Option<String> = None;
                    for (id, _) in &deck.slides {
                        if id == current_id {
                            if let Some(prev) = prev_id {
                                self.current_slide = Some(prev);
                                return true;
                            }
                            return false;
                        }
                        prev_id = Some(id.clone());
                    }
                    false
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn list_decks(&self) -> Vec<&PresentationDeck> {
        self.decks.values().collect()
    }

    pub fn deck_count(&self) -> usize {
        self.decks.len()
    }
}

impl Default for SigmaDeck {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaDeck instance
static mut SIGMADECK_INSTANCE: Option<SigmaDeck> = None;

/// Initialize the global SigmaDeck instance
pub fn init_sigmadeck() {
    unsafe {
        SIGMADECK_INSTANCE = Some(SigmaDeck::new());
    }
}

/// Get the global SigmaDeck instance
pub fn get_sigmadeck() -> Option<&'static SigmaDeck> {
    unsafe {
        SIGMADECK_INSTANCE.as_ref()
    }
}

/// Get the global SigmaDeck instance (mutable)
pub fn get_sigmadeck_mut() -> Option<&'static mut SigmaDeck> {
    unsafe {
        SIGMADECK_INSTANCE.as_mut()
    }
}

/// Convenience function to add a deck
pub fn add_deck(deck: PresentationDeck) -> bool {
    if let Some(deck_sys) = get_sigmadeck_mut() {
        deck_sys.add_deck(deck);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_element_creation() {
        let element = SlideElement::new("elem1".to_string(), SlideElementType::Text, "Hello".to_string());
        assert_eq!(element.id, "elem1");
        assert_eq!(element.element_type, SlideElementType::Text);
    }

    #[test]
    fn test_slide_element_modification() {
        let mut element = SlideElement::new("elem1".to_string(), SlideElementType::Text, "Test".to_string());
        element.set_position(50.0, 100.0);
        element.set_size(200.0, 150.0);
        assert_eq!(element.position_x, 50.0);
        assert_eq!(element.width, 200.0);
    }

    #[test]
    fn test_slide_creation() {
        let slide = Slide::new("slide1".to_string(), 1, "Introduction".to_string());
        assert_eq!(slide.id, "slide1");
        assert_eq!(slide.number, 1);
        assert_eq!(slide.transition, SlideTransition::None);
    }

    #[test]
    fn test_slide_modification() {
        let mut slide = Slide::new("slide1".to_string(), 1, "Test".to_string());
        slide.set_transition(SlideTransition::Fade);
        slide.set_duration(5);
        slide.set_notes("Speaker notes".to_string());
        assert_eq!(slide.transition, SlideTransition::Fade);
        assert_eq!(slide.duration, 5);
    }

    #[test]
    fn test_presentation_deck_creation() {
        let deck = PresentationDeck::new("deck1".to_string(), "Presentation".to_string());
        assert_eq!(deck.id, "deck1");
        assert_eq!(deck.slide_count, 0);
    }

    #[test]
    fn test_presentation_deck_management() {
        let mut deck = PresentationDeck::new("deck1".to_string(), "Test".to_string());
        let slide = Slide::new("slide1".to_string(), 1, "Intro".to_string());
        deck.add_slide(slide);
        assert_eq!(deck.slide_count, 1);
    }

    #[test]
    fn test_sigmadeck_creation() {
        let deck_sys = SigmaDeck::new();
        assert_eq!(deck_sys.deck_count(), 0);
    }

    #[test]
    fn test_sigmadeck_management() {
        let mut deck_sys = SigmaDeck::new();
        let deck = PresentationDeck::new("deck1".to_string(), "Test".to_string());
        deck_sys.add_deck(deck);
        assert_eq!(deck_sys.deck_count(), 1);
    }

    #[test]
    fn test_slide_navigation() {
        let mut deck_sys = SigmaDeck::new();
        let mut deck = PresentationDeck::new("deck1".to_string(), "Test".to_string());
        deck.add_slide(Slide::new("slide1".to_string(), 1, "Slide 1".to_string()));
        deck.add_slide(Slide::new("slide2".to_string(), 2, "Slide 2".to_string()));
        deck_sys.add_deck(deck);
        deck_sys.set_current_deck("deck1");
        assert!(deck_sys.next_slide());
        assert!(deck_sys.next_slide());
        assert!(!deck_sys.next_slide());
    }
}
