// SigmaOS — sigma-netmesh-routing: Mesh Routing Protocols
// Modularised from: SovereignNetMesh.c
// USP: OOP encapsulation of dynamic routing protocols tailored for decentralized meshes.

#ifndef SIGMA_NETMESH_ROUTING_HPP
#define SIGMA_NETMESH_ROUTING_HPP

namespace sigma {
namespace netmesh {

struct RouteEntry {
    unsigned int destination_ip;
    unsigned int gateway_ip;
    unsigned int metric;
    unsigned int next_hop_mac[6];
};

class IMeshRouter {
public:
    virtual ~IMeshRouter() = default;
    virtual void update_routing_table() = 0;
    virtual bool resolve_route(unsigned int dest_ip, RouteEntry* out_route) = 0;
};

// Specialized implementation: Optimized Link State Routing
class OLSRRouter : public IMeshRouter {
private:
    RouteEntry table[128];
    unsigned int entry_count;

public:
    OLSRRouter() : entry_count(0) {}

    void update_routing_table() override {
        // Broadcast topology control messages to peers
        // Recalculate shortest paths using Dijkstra's algorithm
    }

    bool resolve_route(unsigned int dest_ip, RouteEntry* out_route) override {
        for (unsigned int i = 0; i < entry_count; ++i) {
            if (table[i].destination_ip == dest_ip) {
                *out_route = table[i];
                return true;
            }
        }
        return false;
    }
};

} // namespace netmesh
} // namespace sigma

#endif /* SIGMA_NETMESH_ROUTING_HPP */
