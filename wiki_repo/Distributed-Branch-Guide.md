# 🌐 SigmaOS Distributed Branch — `release/distributed`

> **Sovereign Consensus: No single point of failure, no external dependency.**

The `release/distributed` branch enables SigmaOS to operate as a **node in a distributed cluster**, absorbing:
- **Raft consensus** (etcd, TiKV)
- **Byzantine fault tolerance** (PBFT, HotStuff)
- **Distributed file systems** (Ceph RADOS, GlusterFS)
- **Container orchestration** (Kubernetes node agent)
- **P2P networking** (libp2p, BitTorrent DHT)

---

## 🏗 Cluster Architecture

```
┌────────────────────────────────────────────────┐
│               SigmaOS Cluster                  │
│                                                │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │  Node 0  │  │  Node 1  │  │  Node 2  │     │
│  │ (Leader) │  │(Follower)│  │(Follower)│     │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘     │
│       └─────────────┼─────────────┘            │
│              sigma_raft.cpp                     │
│          (Raft consensus engine)               │
└────────────────────────────────────────────────┘
```

---

## 🗳 Raft Consensus Implementation (`sigma_raft.cpp`)

Absorbed from **etcd's raft library** and **Raft paper (Ongaro & Ousterhout 2014)**:

### Node States
```cpp
enum RaftState { FOLLOWER, CANDIDATE, LEADER };

struct RaftNode {
    u32      node_id;
    RaftState state;
    u32      current_term;   /* Monotonically increasing */
    u32      voted_for;      /* CandidateID we voted for (or 0) */
    u32      commit_index;   /* Highest committed log index */
    u32      last_applied;   /* Last entry applied to state machine */
    u64      election_timeout_tsc; /* TSC deadline for election */
};
```

### Log Entry
```cpp
struct RaftLogEntry {
    u32 term;        /* Term when entry was received by leader */
    u32 index;       /* Position in the log */
    u8  command[512]; /* State machine command payload */
    u32 cmd_len;
};
```

### Election Algorithm
1. Follower becomes **Candidate** after election timeout (150–300ms random)
2. Increments `current_term`, votes for self
3. Sends `RequestVote` RPCs to all nodes
4. Becomes **Leader** on majority vote
5. Immediately sends heartbeat `AppendEntries` RPCs

---

## 🗄 Distributed Storage (`sigma_drbd.cpp`)

Absorbed from **DRBD** (Distributed Replicated Block Device) and **Ceph RADOS**:

| Feature | Implementation |
|---------|---------------|
| Block replication | Synchronous mirror to N nodes |
| Split-brain detection | Fencing via `sigma_stonith.cpp` |
| Quorum | Majority of nodes must agree |
| Recovery | Bitmap-based dirty region sync |

---

## 📡 Distributed RPC Protocol

Custom binary RPC protocol (no gRPC, no Protobuf):

```
┌─────────┬──────────┬────────┬─────────────────┐
│ Magic   │ Msg Type │ Length │    Payload       │
│ 0xSIGMA │ (u8)     │ (u32)  │ (variable)       │
└─────────┴──────────┴────────┴─────────────────┘
```

Message Types:
- `0x01` — `RequestVote`
- `0x02` — `RequestVoteResponse`
- `0x03` — `AppendEntries` (heartbeat or log replication)
- `0x04` — `AppendEntriesResponse`
- `0x10` — `ClientRequest` (forwarded to leader)
- `0x11` — `ClientResponse`

---

## 🔍 Service Discovery (`sigma_gossip.cpp`)

Absorbed from **Serf** (HashiCorp), **Cassandra gossip**, **Kubernetes kube-proxy**:

- SWIM protocol (Scalable Weakly-consistent Infection-style Membership)
- UDP multicast for initial peer discovery
- Exponential backoff on probe failures

---

*Branch: `release/distributed` | Cluster minimum: 3 nodes (quorum = 2)*
