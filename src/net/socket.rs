#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressFamily {
    Unix,
    Inet,
    Inet6,
