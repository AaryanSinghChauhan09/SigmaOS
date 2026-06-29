# Sigma CommNet — Community-Owned Internet

sigma-commnet turns a single ISP connection into a village or colony-owned internet infrastructure. One SigmaOS machine with two network cards becomes a gateway node, sharing bandwidth fairly across every household and institution in the community — at exactly cost, with no profit and no middleman.

---

## The Problem It Solves

Internet in rural India costs ₹500–1500/month per household. For a village with 20 families, each paying separately, that's ₹10,000–30,000/month total. But a single 100 Mbps Jio/BSNL fiber connection costs ₹800–1200/month and serves all 20 households simultaneously.

sigma-commnet is the OS-level infrastructure to do this legally and fairly.

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│              BSNL / Jio / Starlink                  │
│                    (upstream)                        │
└─────────────────────┬───────────────────────────────┘
                      │  One shared connection
┌─────────────────────▼───────────────────────────────┐
│           sigma-commnet Gateway Node                │
│        (SigmaOS machine with 2 NICs)                │
│   eth0 = upstream ← → wlan0/eth1 = community mesh  │
└──────┬──────────┬──────────┬──────────┬─────────────┘
       │          │          │          │
    Node 1     Node 2     Node 3     Node 4
   (House 1)  (House 2)  (House 3)  (School)
   SigmaOS    SigmaOS    SigmaOS    SigmaOS
```

The gateway node runs `sigma-commetd` and manages:
- Upstream bandwidth (via `tc HTB` fair-share QoS)
- DID-based access control (only enrolled members connect)
- Local content cache (government sites, NCERT, eNAM served from local storage)
- Billing (calculates exact cost per household, collects via UPI)
- Access logs (mandatory DoT compliance)

---

## Features

### Automatic Fair-Share Bandwidth (QoS)

sigma-commnet uses Linux `tc HTB` (Hierarchical Token Bucket) to share bandwidth fairly:

```
Total upstream: 100 Mbps
Active households: 10 of 20 enrolled

Each active household gets: 10 Mbps guaranteed
                            20 Mbps burst (if others idle)

Priority members (school, clinic): 20 Mbps guaranteed always
```

If a household is idle, its share is automatically redistributed. Nobody gets locked out, nobody monopolises the connection.

### Local Content Caching

Government websites, educational content, and farm data are cached locally on the gateway node. This means:
- NCERT textbooks load instantly even on 2G-equivalent local Wi-Fi
- eNAM mandi prices work even when upstream ISP is slow
- Aadhaar-linked services serve cached responses for common queries

**Pre-seeded domains:**

| Domain | Content | Category | 
| --- | --- | --- | 
| `ncert.nic.in` | School textbooks | Education | 
| `enam.gov.in` | Mandi prices | Agriculture | 
| `pmkisan.gov.in` | PM-KISAN status | Government | 
| `nhp.gov.in` | Health portal | Health | 
| `digilocker.gov.in` | Document access | Government | 
| `epfindia.gov.in` | EPF balance | Financial | 
| `india.gov.in` | Govt services portal | Government | 

Add any domain to the cache:
```bash
sigma-commnet cache add --url "ncert.nic.in" --category education
sigma-commnet cache sync --all  # Sync all cached domains now
```

### Community Dashboard

A web-based dashboard (accessible on the local mesh, no internet needed) shows:
- Who is using how much bandwidth right now
- Monthly usage per household
- Cache hit ratio (bandwidth saved)
- Upstream status (up/down)
- Cost per household this month

### Billing — Flat Cost Share, No Profit

```
Monthly ISP cost: ₹1200
Enrolled households: 20
Cost per household: ₹60/month

sigma-commnet bill generate --month 2026-07
→ Generates UPI payment requests for ₹60 to each enrolled member
```

The billing system:
- Calculates exact ISP cost ÷ enrolled members
- Optionally weights by usage (members who use more pay proportionally more)
- Generates UPI payment link for each member
- Records payment via community UPI VPA

**This is not reselling.** Cost-sharing among a defined community is permitted under TRAI guidelines.

### Offline Mode

When the upstream ISP fails, sigma-commnet switches to offline mode automatically:
- All cached content (NCERT, government portals, health info) continues to work
- sigma-gram panchayat records remain accessible
- sigma-health local patient records remain accessible
- Community dashboard shows "Offline since [time]"

### DID-Based Access Control

Only enrolled community members can connect. Enrollment is simple:
1. Head of household submits their SigmaOS DID to the gateway admin
2. Gateway admin approves
3. All devices with registered MAC addresses get access
4. Non-enrolled devices see a DID enrollment page

Access logs are maintained for 6 months as required by DoT rules.

---

## TRAI Compliance

sigma-commnet is designed to comply with TRAI's community Wi-Fi regulations:

| Requirement | sigma-commnet | 
| --- | --- | 
| Not reselling (cost-sharing only) | ✅ Billing = exact ISP cost ÷ members | 
| Maximum 20 users per hotspot | ✅ `max_members` enforced at gateway | 
| Access logs maintained | ✅ 6-month logs, tamper-evident | 
| ISP T&C compliant | ✅ Gateway has single ISP account | 
| Hotspot license | ✅ Community setup guide includes license checklist | 

---

## Setup

### Quick Setup

```bash
# Setup gateway: upstream on eth0, community mesh on wlan0, up to 20 members
sigma-commnet setup --gateway eth0 --mesh wlan0 --members 20

# Add a member
sigma-commnet member add \
  --name "Ramesh Kumar" \
  --did "did:sigma:abc123..." \
  --mac "AA:BB:CC:DD:EE:FF"

# Check status
sigma-commnet status
```

### Full Setup Walkthrough

```bash
# 1. Configure upstream ISP connection
sigma-commnet upstream set --iface eth0 --bandwidth 100 --isp "Jio Fiber" --cost 1200

# 2. Configure mesh network
sigma-commnet mesh set --iface wlan0 --ssid "GramNet-Rampur" --password <wpa3-password>

# 3. Set billing UPI
sigma-commnet billing set --upi "gramnet.rampur@upi" --split equal

# 4. Enable caching
sigma-commnet cache add --url "ncert.nic.in" --category education
sigma-commnet cache add --url "enam.gov.in" --category agriculture

# 5. Start service
sigma-commnet start

# 6. Generate monthly bill
sigma-commnet bill generate --month 2026-07
```

---

## Commands Reference

```bash
sigma-commnet setup --gateway <iface> --mesh <iface> --members <n>
sigma-commnet status
sigma-commnet member list
sigma-commnet member add --name <> --did <> --mac <>
sigma-commnet member suspend --id <> --reason <>
sigma-commnet bandwidth report --week last
sigma-commnet bandwidth report --month 2026-06
sigma-commnet cache add --url <domain> --category <cat>
sigma-commnet cache sync --all
sigma-commnet cache stats
sigma-commnet bill generate --month 2026-07
sigma-commnet bill paid --member <id> --month 2026-07 --upi-ref <>
sigma-commnet qos show
sigma-commnet access-log --from 2026-06-01 --to 2026-06-30
```

---

## Hardware Requirements

**Gateway node (minimum):**
- Any SigmaOS machine with 2 network interfaces (NICs)
- 1 GB RAM, 10 GB storage (for cache)
- Example: ₹3,000 Raspberry Pi 4 + USB ethernet adapter

**Community mesh:**
- Any 802.11n/ac Wi-Fi router or access point (acts as dumb AP)
- Or: cat5e ethernet run to each house (PoE switches)

**Upstream:**
- Any single ISP connection: Jio Fiber, BSNL, ACT, Starlink
- 4G LTE router as backup upstream

---

*See also: [System Daemons](System-Daemons) · [Sigma Self-Heal](Sigma-Self-Heal) · [SigmaOS Vision for India](SigmaOS-Vision-India) · [India Business Strategy](India-Business-Strategy)*
