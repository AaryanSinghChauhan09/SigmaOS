//! Custom String Handling for SigmaOS
//! Implements string operations without relying on std
//! Supports UTF-8 encoding and common string operations

const MAX_STRING_LENGTH: usize = 4096;

/// Custom string structure
pub const SigmaString = struct {
    data: [MAX_STRING_LENGTH]u8,
    length: usize,
};

pub fn initString(string: *SigmaString) void {
    string.length = 0;
}

pub fn fromBytes(string: *SigmaString, bytes: []const u8) !void {
    const len = bytes.len;
    if (len > MAX_STRING_LENGTH) {
        return error.StringTooLong;
    }

    @memcpy(string.data[0..len], bytes);
    string.length = len;
}

pub fn fromCStr(string: *SigmaString, ptr: [*]const u8) void {
    var len: usize = 0;
    while (ptr[len] != 0) {
        len += 1;
    }

    if (len > MAX_STRING_LENGTH) {
        len = MAX_STRING_LENGTH;
    }

    @memcpy(string.data[0..len], ptr[0..len]);
    string.length = len;
}

pub fn len(string: *const SigmaString) usize {
    return string.length;
}

pub fn isEmpty(string: *const SigmaString) bool {
    return string.length == 0;
}

pub fn asBytes(string: *const SigmaString) []const u8 {
    return string.data[0..string.length];
}

pub fn pushBytes(string: *SigmaString, bytes: []const u8) !void {
    const new_len = string.length + bytes.len;

    if (new_len > MAX_STRING_LENGTH) {
        return error.StringTooLong;
    }

    @memcpy(string.data[string.length..new_len], bytes);
    string.length = new_len;
}

pub fn pushChar(string: *SigmaString, c: u8) !void {
    if (string.length >= MAX_STRING_LENGTH) {
        return error.StringTooLong;
    }

    string.data[string.length] = c;
    string.length += 1;
}

pub fn pushString(string: *SigmaString, other: *const SigmaString) !void {
    try pushBytes(string, asBytes(other));
}

pub fn clear(string: *SigmaString) void {
    string.length = 0;
}

pub fn truncate(string: *SigmaString, new_len: usize) void {
    if (new_len < string.length) {
        string.length = new_len;
    }
}

pub fn pop(string: *SigmaString) ?u8 {
    if (string.length == 0) {
        return null;
    }

    string.length -= 1;
    return string.data[string.length];
}

pub fn compare(string: *const SigmaString, other: *const SigmaString) i32 {
    const self_bytes = asBytes(string);
    const other_bytes = asBytes(other);
    const min_len = @min(self_bytes.len, other_bytes.len);

    for (0..min_len) |i| {
        if (self_bytes[i] < other_bytes[i]) {
            return -1;
        } else if (self_bytes[i] > other_bytes[i]) {
            return 1;
        }
    }

    if (self_bytes.len < other_bytes.len) {
        return -1;
    } else if (self_bytes.len > other_bytes.len) {
        return 1;
    } else {
        return 0;
    }
}

pub fn equals(string: *const SigmaString, other: *const SigmaString) bool {
    return compare(string, other) == 0;
}

pub fn find(string: *const SigmaString, pattern: *const SigmaString) ?usize {
    const self_bytes = asBytes(string);
    const pattern_bytes = asBytes(pattern);

    if (pattern_bytes.len == 0) {
        return 0;
    }

    if (pattern_bytes.len > self_bytes.len) {
        return null;
    }

    var i: usize = 0;
    while (i <= self_bytes.len - pattern_bytes.len) {
        if (self_bytes[i..i + pattern_bytes.len] == pattern_bytes) {
            return i;
        }
        i += 1;
    }

    return null;
}

pub fn trim(string: *const SigmaString) SigmaString {
    const bytes = asBytes(string);
    var start: usize = 0;
    var end: usize = bytes.len;

    while (start < end and (bytes[start] == ' ' or bytes[start] == '\t' or bytes[start] == '\n' or bytes[start] == '\r')) {
        start += 1;
    }

    while (end > start and (bytes[end - 1] == ' ' or bytes[end - 1] == '\t' or bytes[end - 1] == '\n' or bytes[end - 1] == '\r')) {
        end -= 1;
    }

    var result = SigmaString{ .data = undefined, .length = 0 };
    @memcpy(result.data[0..(end - start)], bytes[start..end]);
    result.length = end - start;
    return result;
}

pub fn toLowercase(string: *const SigmaString) SigmaString {
    const bytes = asBytes(string);
    var result = SigmaString{ .data = undefined, .length = 0 };

    for (bytes) |byte| {
        if (byte >= 'A' and byte <= 'Z') {
            result.data[result.length] = byte + 32;
        } else {
            result.data[result.length] = byte;
        }
        result.length += 1;
    }

    return result;
}

pub fn toUppercase(string: *const SigmaString) SigmaString {
    const bytes = asBytes(string);
    var result = SigmaString{ .data = undefined, .length = 0 };

    for (bytes) |byte| {
        if (byte >= 'a' and byte <= 'z') {
            result.data[result.length] = byte - 32;
        } else {
            result.data[result.length] = byte;
        }
        result.length += 1;
    }

    return result;
}

// C-style string functions
pub fn strlen(ptr: [*]const i8) usize {
    var len: usize = 0;
    while (ptr[len] != 0) {
        len += 1;
    }
    return len;
}

pub fn strcpy(dest: [*]i8, src: [*]const i8) [*]i8 {
    var i: usize = 0;
    while (src[i] != 0) {
        dest[i] = src[i];
        i += 1;
    }
    dest[i] = 0;
    return dest;
}

pub fn strcmp(s1: [*]const i8, s2: [*]const i8) i32 {
    var i: usize = 0;
    while (s1[i] != 0) : (s2[i] != 0) {
        const c1 = @intCast(u8, s1[i]);
        const c2 = @intCast(u8, s2[i]);

        if (c1 < c2) {
            return -1;
        } else if (c1 > c2) {
            return 1;
        }
        i += 1;
    }

    if (s1[i] == 0 and s2[i] == 0) {
        return 0;
    } else if (s1[i] == 0) {
        return -1;
    } else {
        return 1;
    }
}

pub fn strncmp(s1: [*]const i8, s2: [*]const i8, n: usize) i32 {
    for (0..n) |i| {
        const c1 = @intCast(u8, s1[i]);
        const c2 = @intCast(u8, s2[i]);

        if (c1 < c2) {
            return -1;
        } else if (c1 > c2) {
            return 1;
        } else if (c1 == 0) {
            return 0;
        }
    }
    return 0;
}

pub fn strcat(dest: [*]i8, src: [*]const i8) [*]i8 {
    const dest_len = strlen(dest);
    strcpy(dest + dest_len, src);
    return dest;
}

// Error types
pub const Error = error {
    StringTooLong,
};
