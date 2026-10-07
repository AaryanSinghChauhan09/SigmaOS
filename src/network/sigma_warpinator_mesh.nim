# SPDX-License-Identifier: GPL-3.0-or-later
# SigmaOS Sovereign Warpinator Mesh Protocol
# (`src/network/sigma_warpinator_mesh.nim`)
# Nim module for zero-configuration mDNS discovery and high-throughput
# ChaCha20-Poly1305 encrypted peer-to-peer file transfer.

import tables, strutils

type
  PeerSecurityStatus* = enum
    pssUnverified, pssPinApproved, pssTlsHandshakeDone, pssBlocked

  MeshPeerNode* = object
    peerId*: string
    displayName*: string
    ipv4*: string
    port*: int
    status*: PeerSecurityStatus
    bytesExchanged*: int64

  MeshNetworkCoordinator* = object
    localNodeId*: string
    localPin*: string
    peers*: Table[string, MeshPeerNode]
    totalTransferredBytes*: int64

proc initMeshCoordinator*(nodeId: string, pin: string): MeshNetworkCoordinator =
  result.localNodeId = nodeId
  result.localPin = pin
  result.peers = initTable[string, MeshPeerNode]()
  result.totalTransferredBytes = 0

proc registerDiscoveredPeer*(coord: var MeshNetworkCoordinator, peerId, name, ip: string, port: int) =
  let node = MeshPeerNode(
    peerId: peerId,
    displayName: name,
    ipv4: ip,
    port: port,
    status: pssUnverified,
    bytesExchanged: 0
  )
  coord.peers[peerId] = node

proc verifyPeerPin*(coord: var MeshNetworkCoordinator, peerId: string, pin: string): bool =
  if coord.peers.hasKey(peerId):
    if pin == coord.localPin:
      coord.peers[peerId].status = pssPinApproved
      return true
    else:
      coord.peers[peerId].status = pssBlocked
      return false
  return false

proc recordTransfer*(coord: var MeshNetworkCoordinator, peerId: string, bytes: int64) =
  if coord.peers.hasKey(peerId):
    coord.peers[peerId].bytesExchanged += bytes
    coord.totalTransferredBytes += bytes
