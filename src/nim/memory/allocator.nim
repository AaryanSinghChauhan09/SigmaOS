# Custom Memory Allocator for SigmaOS
# Implements memory allocation without relying on std
# Uses buddy system algorithm for efficient memory management

const BLOCK_SIZE = 4096
const MAX_ORDER = 10

type
  MemoryBlock = object
    size: int
    used: bool
    next: ptr MemoryBlock
    prev: ptr MemoryBlock

  BuddyAllocator = object
    freeLists: array[MAX_ORDER + 1, ptr MemoryBlock]
    totalMemory: int
    usedMemory: int
    heapStart: pointer
    heapSize: int

proc initAllocator(allocator: ptr BuddyAllocator, heapStart: pointer, heapSize: int) =
  allocator.freeLists = cast[array[MAX_ORDER + 1, ptr MemoryBlock]]([nil, nil, nil, nil, nil, nil, nil, nil, nil, nil, nil])
  allocator.totalMemory = heapSize
  allocator.usedMemory = 0
  allocator.heapStart = heapStart
  allocator.heapSize = heapSize
  
  var remaining = heapSize
  var current = heapStart
  
  while remaining >= BLOCK_SIZE:
    var order = calculateOrder(remaining)
    var blockSize = BLOCK_SIZE * (1 shl order)
    
    var block = cast[ptr MemoryBlock](current)
    block.size = blockSize
    block.used = false
    block.next = nil
    block.prev = nil
    
    addToFreeList(allocator, block, order)
    
    current = cast[pointer](cast[int](current) + blockSize)
    remaining -= blockSize

proc calculateOrder(size: int): int =
  var order = 0
  var blockSize = BLOCK_SIZE
  while blockSize < size and order < MAX_ORDER:
    blockSize = blockSize * 2
    order += 1
  return order

proc addToFreeList(allocator: ptr BuddyAllocator, block: ptr MemoryBlock, order: int) =
  block.next = allocator.freeLists[order]
  if not isNil(allocator.freeLists[order]):
    allocator.freeLists[order].prev = block
  allocator.freeLists[order] = block

proc removeFromFreeList(allocator: ptr BuddyAllocator, block: ptr MemoryBlock, order: int) =
  let prev = block.prev
  let next = block.next
  
  if not isNil(prev):
    prev.next = next
  else:
    allocator.freeLists[order] = next
  
  if not isNil(next):
    next.prev = prev
  
  block.prev = nil
  block.next = nil

proc allocate(allocator: ptr BuddyAllocator, size: int): pointer =
  let alignedSize = (size + sizeof(MemoryBlock) + 15) and (not 15)
  let order = calculateOrder(alignedSize)
  
  var currentOrder = order
  var block: ptr MemoryBlock = nil
  
  while currentOrder <= MAX_ORDER:
    if not isNil(allocator.freeLists[currentOrder]):
      block = allocator.freeLists[currentOrder]
      break
    currentOrder += 1
  
  if isNil(block):
    return nil
  
  removeFromFreeList(allocator, block, currentOrder)
  
  while currentOrder > order:
    currentOrder -= 1
    let splitSize = BLOCK_SIZE * (1 shl currentOrder)
    let buddy = cast[ptr MemoryBlock](cast[int](block) + splitSize)
    
    buddy.size = splitSize
    buddy.used = false
    buddy.next = nil
    buddy.prev = nil
    
    addToFreeList(allocator, buddy, currentOrder)
    
    block.size = splitSize
  
  block.used = true
  allocator.usedMemory += block.size
  
  return cast[pointer](cast[int](block) + sizeof(MemoryBlock))

proc deallocate(allocator: ptr BuddyAllocator, ptr: pointer) =
  let block = cast[ptr MemoryBlock](cast[int](ptr) - sizeof(MemoryBlock))
  
  if not block.used:
    return
  
  block.used = false
  allocator.usedMemory -= block.size
  
  let size = block.size
  let order = calculateOrder(size)
  
  var currentBlock = block
  var currentOrder = order
  
  while currentOrder < MAX_ORDER:
    let buddy = getBuddy(currentBlock, currentOrder)
    
    if buddy.used or buddy.size != currentBlock.size:
      break
    
    removeFromFreeList(allocator, buddy, currentOrder)
    
    if cast[int](currentBlock) < cast[int](buddy):
      currentBlock.size = currentBlock.size * 2
    else:
      buddy.size = buddy.size * 2
      currentBlock = buddy
    
    currentOrder += 1
  
  addToFreeList(allocator, currentBlock, currentOrder)

proc getBuddy(block: ptr MemoryBlock, order: int): ptr MemoryBlock =
  let blockAddr = cast[int](block)
  let blockSize = BLOCK_SIZE * (1 shl order)
  let buddyAddr = blockAddr xor blockSize
  return cast[ptr MemoryBlock](buddyAddr)

proc getStats(allocator: ptr BuddyAllocator): tuple[total: int, used: int, free: int] =
  return (allocator.totalMemory, allocator.usedMemory, allocator.totalMemory - allocator.usedMemory)

# External allocator functions (would be provided by kernel)
proc kernelAlloc(size: int): pointer {.importc, header: "<kernel.h>".}
proc kernelFree(ptr: pointer) {.importc, header: "<kernel.h>".}
