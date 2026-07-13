# Custom String Handling for SigmaOS
# Implements string operations without relying on std
# Supports UTF-8 encoding and common string operations

const MAX_STRING_LENGTH = 4096

type
  SigmaString = object
    data: array[MAX_STRING_LENGTH, char]
    length: int

proc initString(string: var SigmaString) =
  string.length = 0

proc fromBytes(string: var SigmaString, bytes: openArray[char]) =
  let len = bytes.len
  if len > MAX_STRING_LENGTH:
    return
  
  for i in 0..<len:
    string.data[i] = bytes[i]
  string.length = len

proc fromCStr(string: var SigmaString, ptr: cstring) =
  var len = 0
  while ptr[len] != '\0':
    len += 1
  
  if len > MAX_STRING_LENGTH:
    len = MAX_STRING_LENGTH
  
  for i in 0..<len:
    string.data[i] = ptr[i]
  string.length = len

proc len(string: SigmaString): int =
  return string.length

proc isEmpty(string: SigmaString): bool =
  return string.length == 0

proc asBytes(string: SigmaString): openArray[char] =
  return string.data[0..<string.length]

proc pushBytes(string: var SigmaString, bytes: openArray[char]) =
  let newLen = string.length + bytes.len
  
  if newLen > MAX_STRING_LENGTH:
    return
  
  for i in 0..<bytes.len:
    string.data[string.length + i] = bytes[i]
  string.length = newLen

proc pushChar(string: var SigmaString, c: char) =
  if string.length >= MAX_STRING_LENGTH:
    return
  
  string.data[string.length] = c
  string.length += 1

proc pushString(string: var SigmaString, other: SigmaString) =
  pushBytes(string, asBytes(other))

proc clear(string: var SigmaString) =
  string.length = 0

proc truncate(string: var SigmaString, newLen: int) =
  if newLen < string.length:
    string.length = newLen

proc pop(string: var SigmaString): char =
  if string.length == 0:
    return '\0'
  
  string.length -= 1
  return string.data[string.length]

proc compare(string: SigmaString, other: SigmaString): int =
  let selfBytes = asBytes(string)
  let otherBytes = asBytes(other)
  let minLen = min(selfBytes.len, otherBytes.len)
  
  for i in 0..<minLen:
    if selfBytes[i] < otherBytes[i]:
      return -1
    elif selfBytes[i] > otherBytes[i]:
      return 1
  
  if selfBytes.len < otherBytes.len:
    return -1
  elif selfBytes.len > otherBytes.len:
    return 1
  else:
    return 0

proc equals(string: SigmaString, other: SigmaString): bool =
  return compare(string, other) == 0

proc find(string: SigmaString, pattern: SigmaString): int =
  let selfBytes = asBytes(string)
  let patternBytes = asBytes(pattern)
  
  if patternBytes.len == 0:
    return 0
  
  if patternBytes.len > selfBytes.len:
    return -1
  
  var i = 0
  while i <= selfBytes.len - patternBytes.len:
    var found = true
    for j in 0..<patternBytes.len:
      if selfBytes[i + j] != patternBytes[j]:
        found = false
        break
    
    if found:
      return i
    
    i += 1
  
  return -1

proc trim(string: SigmaString): SigmaString =
  let bytes = asBytes(string)
  var start = 0
  var endPos = bytes.len
  
  while start < endPos and (bytes[start] == ' ' or bytes[start] == '\t' or bytes[start] == '\n' or bytes[start] == '\r'):
    start += 1
  
  while endPos > start and (bytes[endPos - 1] == ' ' or bytes[endPos - 1] == '\t' or bytes[endPos - 1] == '\n' or bytes[endPos - 1] == '\r'):
    endPos -= 1
  
  var result: SigmaString
  initString(result)
  
  for i in start..<endPos:
    result.data[result.length] = bytes[i]
    result.length += 1
  
  return result

proc toLowercase(string: SigmaString): SigmaString =
  let bytes = asBytes(string)
  var result: SigmaString
  initString(result)
  
  for byte in bytes:
    if byte >= 'A' and byte <= 'Z':
      result.data[result.length] = chr(ord(byte) + 32)
    else:
      result.data[result.length] = byte
    result.length += 1
  
  return result

proc toUppercase(string: SigmaString): SigmaString =
  let bytes = asBytes(string)
  var result: SigmaString
  initString(result)
  
  for byte in bytes:
    if byte >= 'a' and byte <= 'z':
      result.data[result.length] = chr(ord(byte) - 32)
    else:
      result.data[result.length] = byte
    result.length += 1
  
  return result

# C-style string functions
proc strlen(ptr: cstring): int =
  var len = 0
  while ptr[len] != '\0':
    len += 1
  return len

proc strcpy(dest: cstring, src: cstring): cstring =
  var i = 0
  while src[i] != '\0':
    dest[i] = src[i]
    i += 1
  dest[i] = '\0'
  return dest

proc strcmp(s1: cstring, s2: cstring): int =
  var i = 0
  while s1[i] != '\0' and s2[i] != '\0':
    let c1 = ord(s1[i])
    let c2 = ord(s2[i])
    
    if c1 < c2:
      return -1
    elif c1 > c2:
      return 1
    i += 1
  
  if s1[i] == '\0' and s2[i] == '\0':
    return 0
  elif s1[i] == '\0':
    return -1
  else:
    return 1

proc strncmp(s1: cstring, s2: cstring, n: int): int =
  for i in 0..<n:
    let c1 = ord(s1[i])
    let c2 = ord(s2[i])
    
    if c1 < c2:
      return -1
    elif c1 > c2:
      return 1
    elif c1 == 0:
      return 0
  
  return 0

proc strcat(dest: cstring, src: cstring): cstring =
  let destLen = strlen(dest)
  strcpy(dest[destLen], src)
  return dest
