# Zero-Trust Networking Architecture

## Principles

1. **Never trust, always verify** — no implicit trust from network location
2. **Least privilege** — grant only needed access
3. **Micro-segmentation** — divide network into small isolated zones
4. **Assume breach** — design as if attacker is already inside
5. **Continuous monitoring** — verify every request

## Implementation

### mTLS (Mutual TLS) for All Services

```
Service A                     Service B
   │─── ClientHello ──────────→│
   │←── ServerHello ───────────│
   │←── Server Certificate ────│
   │─── Client Certificate ───→│
   │─── CertificateVerify ────→│
   │←── Finished ──────────────│
   │   (Encrypted + Authenticated)
   │─── Request ──────────────→│
   │←── Response ──────────────│
```

### Identity-Based Access

Every workload has a cryptographic identity:
- **SPIFFE ID**: `spiffe://sigmaos.local/ns/default/sa/nginx`
- **Certificate**: X.509 + Dilithium-5 (post-quantum)
- **Short-lived**: Rotated every 24 hours

### Network Policy

```yaml
# Only allow frontend → backend on port 8080
apiVersion: networking.sigma/v1
kind: NetworkPolicy
metadata:
  name: allow-frontend-to-backend
spec:
  podSelector:
    app: backend
  ingress:
  - from:
    - podSelector:
        app: frontend
    ports:
    - protocol: TCP
      port: 8080
```

### eBPF Enforcement

Network policies enforced at eBPF layer:

```c
SEC("tc/ingress")
int enforce_policy(struct __sk_buff *skb) {
    struct iphdr *ip = ...;
    struct tcphdr *tcp = ...;

    // Look up source identity in eBPF map
    struct identity *id = bpf_map_lookup_elem(&identity_map, &ip->saddr);
    if (!id) return TC_ACT_SHOT;  // No identity = deny

    // Check policy
    struct policy_key key = {.src_id = id->id, .dst_port = tcp->dest};
    if (!bpf_map_lookup_elem(&policy_map, &key))
        return TC_ACT_SHOT;  // No matching policy = deny

    return TC_ACT_OK;
}
```

## Components

| Component | Function |
|-----------|---------|
| ZenithNet Agent | Per-node identity + policy enforcement |
| Certificate Authority | Issues short-lived mTLS certs |
| Policy Engine | Evaluates access control rules |
| Flow Monitor | Logs and audits all connections |
| Anomaly Detector | ML-based threat detection |
