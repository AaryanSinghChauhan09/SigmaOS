# OSS Absorption: Prometheus Alerting & Alertmanager

> **Status**: 📋 Planned | **Source Project**: Prometheus Alertmanager | **Target Shard**: `SigmaOS Incident Response & Alerts`

---

## 1. Executive Summary

Prometheus Alertmanager handles alerts sent by client applications such as Prometheus. It takes care of deduplicating, grouping, and routing them to the correct receiver integration, supporting silences and alert inhibition.

SigmaOS absorbs the **alert routing trees, inhibition rules, and silencing** model of Alertmanager, integrating it directly into `sigma-alerts` to handle system error routing.

---

## 2. Key Features Absorbed

### 2.1 System Alert Routing Tree

When hardware faults or resource over-utilization thresholds are crossed, `sigma-alerts` routes the event down a nested tree.

```yaml
# /etc/sigma/alertmanager.yaml
route:
  group_by: ['alertname', 'service']
  group_wait: 30s
  receiver: 'local-tui'
  routes:
    - match:
        severity: critical
      receiver: 'admin-page'
```

---

## 3. References & Standards

- Prometheus Alertmanager — `github.com/prometheus/alertmanager` (Apache-2.0 License)
