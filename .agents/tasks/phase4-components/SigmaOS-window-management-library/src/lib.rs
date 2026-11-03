//! SigmaOS window management library — shared window placement, sizing, and focus utilities.

#![allow(dead_code)]

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Rect { pub x: i32, pub y: i32, pub w: u32, pub h: u32 }
impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self { Self { x, y, w, h } }
    pub fn area(&self) -> u64 { self.w as u64 * self.h as u64 }
    pub fn contains(&self, px: i32, py: i32) -> bool { px >= self.x && py >= self.y && px < self.x + self.w as i32 && py < self.y + self.h as i32 }
    pub fn center(&self) -> (i32, i32) { (self.x + self.w as i32 / 2, self.y + self.h as i32 / 2) }
    pub fn split_h(&self, gap: u32) -> (Rect, Rect) {
        let half = (self.w.saturating_sub(gap)) / 2;
        (Rect::new(self.x, self.y, half, self.h), Rect::new(self.x + half as i32 + gap as i32, self.y, self.w - half - gap, self.h))
    }
    pub fn split_v(&self, gap: u32) -> (Rect, Rect) {
        let half = (self.h.saturating_sub(gap)) / 2;
        (Rect::new(self.x, self.y, self.w, half), Rect::new(self.x, self.y + half as i32 + gap as i32, self.w, self.h - half - gap))
    }
}
#[cfg(test)] mod tests { use super::*;
  #[test] fn area() { assert_eq!(Rect::new(0,0,1920,1080).area(), 2073600); }
  #[test] fn contains() { let r = Rect::new(100,100,200,200); assert!(r.contains(150,150)); assert!(!r.contains(50,50)); }
  #[test] fn split_h() { let (a,b) = Rect::new(0,0,1920,1080).split_h(8); assert!(a.w < 1920); assert!(b.x > 0); }
}
