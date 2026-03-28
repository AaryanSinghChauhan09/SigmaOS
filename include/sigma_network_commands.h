/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Network Commands Library
 * ================================
 * Comprehensive network management and diagnostics:
 * - Network configuration (ip, ifconfig, route)
 * - DNS tools (dig, nslookup, host, drill)
 * - Connectivity testing (ping, traceroute, mtr)
 * - Network monitoring (netstat, ss, nstat)
 * - Wireless tools (iw, iwconfig, wpa_supplicant)
 * - Firewall (iptables, nftables, ufw, firewalld)
 * - VPN and tunneling (openvpn, wireguard, ssh)
 * - Packet capture (tcpdump, tshark)
 */

#ifndef SIGMA_NETWORK_COMMANDS_H
#define SIGMA_NETWORK_COMMANDS_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

// ==================== NETWORK INTERFACE MANAGEMENT ====================

typedef struct {
    char* name;
    char* type;
    char* state;
    char* mac_address;
    char** ip_addresses;
    uint32_t n_ip_addresses;
    char* broadcast;
    char* netmask;
    uint64_t rx_bytes;
    uint64_t tx_bytes;
    uint32_t rx_packets;
    uint32_t tx_packets;
    uint32_t rx_errors;
    uint32_t tx_errors;
    bool is_up;
    bool is_running;
    uint32_t mtu;
    uint32_t metric;
} SigmaNetworkInterface;

// IP command (modern replacement for ifconfig/route)
int sigma_ip_link_show(void);
int sigma_ip_link_show_interface(const char* interface);
int sigma_ip_link_set_up(const char* interface);
int sigma_ip_link_set_down(const char* interface);
int sigma_ip_link_set_mtu(const char* interface, uint32_t mtu);
int sigma_ip_link_set_name(const char* old_name, const char* new_name);
int sigma_ip_link_set_mac(const char* interface, const char* mac);
int sigma_ip_link_set_promisc(const char* interface, bool on);
int sigma_ip_link_set_arp(const char* interface, bool on);
int sigma_ip_link_set_multicast(const char* interface, bool on);
int sigma_ip_link_set_allmulticast(const char* interface, bool on);
int sigma_ip_link_set_dynamic(const char* interface, bool on);
int sigma_ip_link_set_txqueuelen(const char* interface, uint32_t len);
int sigma_ip_link_add(const char* name, const char* type);
int sigma_ip_link_delete(const char* interface);
int sigma_ip_link_set_master(const char* interface, const char* master);
int sigma_ip_link_set_nomaster(const char* interface);
int sigma_ip_link_show_slave(const char* master);
int sigma_ip_link_show_group(const char* group);
int sigma_ip_link_set_group(const char* interface, const char* group);

// IP address management
int sigma_ip_addr_show(void);
int sigma_ip_addr_show_interface(const char* interface);
int sigma_ip_addr_add(const char* address, const char* interface);
int sigma_ip_addr_del(const char* address, const char* interface);
int sigma_ip_addr_flush(const char* interface);
int sigma_ip_addr_flush_global(void);
int sigma_ip_addr_change(const char* old_addr, const char* new_addr, const char* interface);
int sigma_ip_addr_replace(const char* address, const char* interface);
int sigma_ip_addr_show_to(const char* to);
int sigma_ip_addr_show_label(const char* label);
int sigma_ip_addr_add_label(const char* address, const char* label, const char* interface);
int sigma_ip_addr_del_label(const char* label, const char* interface);
int sigma_ip_addr_show_dynamic(void);
int sigma_ip_addr_show_permanent(void);
int sigma_ip_addr_show_temporary(void);
int sigma_ip_addr_show_deprecated(void);
int sigma_ip_addr_show_primary(void);
int sigma_ip_addr_show_secondary(void);
int sigma_ip_addr_show_tentative(void);
int sigma_ip_addr_show_dadfailed(void);
int sigma_ip_addr_show_dadstate(void);

// IP route management
int sigma_ip_route_show(void);
int sigma_ip_route_show_table(const char* table);
int sigma_ip_route_show_cache(void);
int sigma_ip_route_get(const char* destination);
int sigma_ip_route_add(const char* destination, const char* gateway);
int sigma_ip_route_add_dev(const char* destination, const char* device);
int sigma_ip_route_add_metric(const char* destination, const char* gateway, uint32_t metric);
int sigma_ip_route_add_table(const char* destination, const char* gateway, const char* table);
int sigma_ip_route_add_src(const char* destination, const char* gateway, const char* src);
int sigma_ip_route_add_proto(const char* destination, const char* gateway, const char* proto);
int sigma_ip_route_add_scope(const char* destination, const char* gateway, const char* scope);
int sigma_ip_route_add_type(const char* destination, const char* type);
int sigma_ip_route_add_preference(const char* destination, const char* gateway, const char* pref);
int sigma_ip_route_add_mtu(const char* destination, const char* gateway, uint32_t mtu);
int sigma_ip_route_add_window(const char* destination, const char* gateway, uint32_t window);
int sigma_ip_route_add_rtt(const char* destination, const char* gateway, uint32_t rtt);
int sigma_ip_route_add_rttvar(const char* destination, const char* gateway, uint32_t rttvar);
int sigma_ip_route_add_ssthresh(const char* destination, const char* gateway, uint32_t ssthresh);
int sigma_ip_route_add_cwnd(const char* destination, const char* gateway, uint32_t cwnd);
int sigma_ip_route_add_advmss(const char* destination, const char* gateway, uint32_t advmss);
int sigma_ip_route_add_reordering(const char* destination, const char* gateway, uint32_t reordering);
int sigma_ip_route_add_hoplimit(const char* destination, const char* gateway, uint32_t hoplimit);
int sigma_ip_route_add_initcwnd(const char* destination, const char* gateway, uint32_t initcwnd);
int sigma_ip_route_add_features(const char* destination, const char* gateway, const char* features);
int sigma_ip_route_add_quickack(const char* destination, const char* gateway, bool quickack);
int sigma_ip_route_add_congctl(const char* destination, const char* gateway, const char* congctl);
int sigma_ip_route_del(const char* destination);
int sigma_ip_route_del_all(void);
int sigma_ip_route_flush(const char* table);
int sigma_ip_route_flush_cache(void);
int sigma_ip_route_change(const char* destination, const char* new_gateway);
int sigma_ip_route_replace(const char* destination, const char* gateway);
int sigma_ip_route_append(const char* destination, const char* gateway);
int sigma_ip_route_prepend(const char* destination, const char* gateway);
int sigma_ip_route_save(void);
int sigma_ip_route_restore(void);
int sigma_ip_route_show_root(const char* prefix);
int sigma_ip_route_show_match(const char* prefix);
int sigma_ip_route_show_exact(const char* prefix);
int sigma_ip_route_show_cloned(void);
int sigma_ip_route_show_resolved(void);
int sigma_ip_route_show_from(const char* src);
int sigma_ip_route_show_iif(const char* iif);
int sigma_ip_route_show_oif(const char* oif);
int sigma_ip_route_show_nexthop(const char* gateway);
int sigma_ip_route_show_realms(const char* realms);
int sigma_ip_route_show_stats(void);
int sigma_ip_route_get_vrf(const char* destination, const char* vrf);
int sigma_ip_route_get_uid(const char* destination, uint32_t uid);
int sigma_ip_route_get_mark(const char* destination, uint32_t mark);
int sigma_ip_route_get_fibmatch(const char* destination);

// IP neighbor (ARP/NDISC) management
int sigma_ip_neigh_show(void);
int sigma_ip_neigh_show_interface(const char* interface);
int sigma_ip_neigh_show_proxy(void);
int sigma_ip_neigh_show_nud(void);
int sigma_ip_neigh_show_unused(void);
int sigma_ip_neigh_show_incomplete(void);
int sigma_ip_neigh_show_reachable(void);
int sigma_ip_neigh_show_stale(void);
int sigma_ip_neigh_show_failed(void);
int sigma_ip_neigh_add(const char* address, const char* mac, const char* interface);
int sigma_ip_neigh_add_lladdr(const char* address, const char* lladdr, const char* interface);
int sigma_ip_neigh_add_nud_permanent(const char* address, const char* mac, const char* interface);
int sigma_ip_neigh_add_nud_noarp(const char* address, const char* mac, const char* interface);
int sigma_ip_neigh_add_nud_reachable(const char* address, const char* mac, const char* interface);
int sigma_ip_neigh_add_nud_stale(const char* address, const char* mac, const char* interface);
int sigma_ip_neigh_add_proxy(const char* address, const char* interface);
int sigma_ip_neigh_del(const char* address, const char* interface);
int sigma_ip_neigh_del_all(const char* interface);
int sigma_ip_neigh_change(const char* address, const char* new_mac, const char* interface);
int sigma_ip_neigh_replace(const char* address, const char* mac, const char* interface);
int sigma_ip_neigh_flush(const char* interface);
int sigma_ip_neigh_add_router(const char* address, const char* interface);

// IP tunnel management
int sigma_ip_tunnel_add(const char* name, const char* mode, const char* local, const char* remote);
int sigma_ip_tunnel_del(const char* name);
int sigma_ip_tunnel_change(const char* name, const char* option, const char* value);
int sigma_ip_tunnel_show(void);
int sigma_ip_tunnel_show_interface(const char* name);
int sigma_ip_tunnel_6rd_prefix(const char* name, const char* prefix);
int sigma_ip_tunnel_6rd_relay_prefix(const char* name, const char* relay_prefix);
int sigma_ip_tunnel_6rd_reset(const char* name);
int sigma_ip_tunnel_prl(const char* name, const char* prl);
int sigma_ip_tunnel_prl_default(const char* name);
int sigma_ip_tunnel_prl_delete(const char* name, const char* prl);
int sigma_ip_tunnel_isatap_router(const char* name, const char* router);
int sigma_ip_tunnel_isatap_router_delete(const char* name);

// IP tunnel modes
int sigma_ip_tunnel_mode_ipip(const char* name, const char* local, const char* remote);
int sigma_ip_tunnel_mode_gre(const char* name, const char* local, const char* remote);
int sigma_ip_tunnel_mode_sit(const char* name, const char* local, const char* remote);
int sigma_ip_tunnel_mode_isatap(const char* name, const char* local, const char* remote);
int sigma_ip_tunnel_mode_vti(const char* name, const char* local, const char* remote);
int sigma_ip_tunnel_mode_vti6(const char* name, const char* local, const char* remote);
int sigma_ip_tunnel_mode_ip6ip6(const char* name, const char* local, const char* remote);
int sigma_ip_tunnel_mode_ipip6(const char* name, const char* local, const char* remote);
int sigma_ip_tunnel_mode_any6(const char* name, const char* local, const char* remote);
int sigma_ip_tunnel_mode_gre6(const char* name, const char* local, const char* remote);

// IP address family specific
int sigma_ip_4_addr_add(const char* address, const char* interface);
int sigma_ip_4_addr_del(const char* address, const char* interface);
int sigma_ip_4_route_add(const char* destination, const char* gateway);
int sigma_ip_4_route_del(const char* destination);
int sigma_ip_6_addr_add(const char* address, const char* interface);
int sigma_ip_6_addr_del(const char* address, const char* interface);
int sigma_ip_6_route_add(const char* destination, const char* gateway);
int sigma_ip_6_route_del(const char* destination);

// IP netns (network namespace)
int sigma_ip_netns_list(void);
int sigma_ip_netns_add(const char* name);
int sigma_ip_netns_del(const char* name);
int sigma_ip_netns_set(const char* name);
int sigma_ip_netns_exec(const char* name, const char* command);
int sigma_ip_netns_identify(const char* pid);
int sigma_ip_netns_pids(const char* name);
int sigma_ip_netns_monitor(void);
int sigma_ip_netns_list_id(void);

// IP maddress (multicast)
int sigma_ip_maddr_show(void);
int sigma_ip_maddr_add(const char* address, const char* interface);
int sigma_ip_maddr_del(const char* address, const char* interface);

// IP mroute (multicast routing)
int sigma_ip_mroute_show(void);

// IP rule (routing policy database)
int sigma_ip_rule_show(void);
int sigma_ip_rule_add(const char* selector, const char* action);
int sigma_ip_rule_add_from(const char* from, const char* table);
int ip rule add to(const char* to, const char* table);
int sigma_ip_rule_add_fwmark(uint32_t mark, const char* table);
int sigma_ip_rule_add_iif(const char* iif, const char* table);
int sigma_ip_rule_add_oif(const char* oif, const char* table);
int sigma_ip_rule_add_tos(uint8_t tos, const char* table);
int sigma_ip_rule_add_priority(uint32_t priority, const char* selector, const char* table);
int sigma_ip_rule_del(const char* selector);
int sigma_ip_rule_flush(void);
int sigma_ip_rule_save(void);
int sigma_ip_rule_restore(void);

// IP xfrm (IPsec)
int sigma_ip_xfrm_state_show(void);
int sigma_ip_xfrm_state_add(const char* options);
int sigma_ip_xfrm_state_del(const char* options);
int sigma_ip_xfrm_policy_show(void);
int sigma_ip_xfrm_policy_add(const char* options);
int sigma_ip_xfrm_policy_del(const char* options);
int sigma_ip_xfrm_policy_flush(void);
int sigma_ip_xfrm_policy_count(void);
int sigma_ip_xfrm_monitor(void);

// IP tcp_metrics
int sigma_ip_tcp_metrics_show(void);
int sigma_ip_tcp_metrics_show_address(const char* address);
int sigma_ip_tcp_metrics_delete(const char* address);
int sigma_ip_tcp_metrics_flush(void);

// IP token (IPv6 tokenized interface identifier)
int sigma_ip_token_show(void);
int sigma_ip_token_get(const char* interface);
int sigma_ip_token_set(const char* interface, const char* token);
int sigma_ip_token_delete(const char* interface);

// IP ntable (neighbor table configuration)
int sigma_ip_ntable_show(void);
int sigma_ip_ntable_change(const char* name, const char* option, const char* value);

// IP l2tp (Layer 2 Tunneling Protocol)
int sigma_ip_l2tp_add_tunnel(const char* options);
int sigma_ip_l2tp_del_tunnel(uint32_t tunnel_id);
int sigma_ip_l2tp_show_tunnel(void);
int sigma_ip_l2tp_add_session(const char* options);
int sigma_ip_l2tp_del_session(uint32_t session_id);
int sigma_ip_l2tp_show_session(void);

// IP macsec (MACsec)
int sigma_ip_macsec_add(const char* interface, const char* options);
int sigma_ip_macsec_del(const char* interface);
int sigma_ip_macsec_show(void);
int sigma_ip_macsec_show_interface(const char* interface);

// IP vrf (Virtual Routing and Forwarding)
int sigma_ip_vrf_show(void);
int sigma_ip_vrf_show_table(void);
int sigma_ip_vrf_show_tables(void);
int sigma_ip_vrf_add(const char* name, uint32_t table);
int sigma_ip_vrf_del(const char* name);
int sigma_ip_vrf_exec(const char* name, const char* command);
int sigma_ip_vrf_indentify(const char* pid);
int sigma_ip_vrf_pids(const char* name);

// IP fou (Foo over UDP)
int sigma_ip_fou_add(const char* options);
int sigma_ip_fou_del(const char* port);
int sigma_ip_fou_show(void);

// IP ila (Identifier Locator Addressing)
int sigma_ip_ila_add(const char* options);
int sigma_ip_ila_del(const char* identifier);
int sigma_ip_ila_show(void);

// IP ioam (IOAM)
int sigma_ip_ioam_namespace_show(void);
int sigma_ip_ioam_namespace_add(const char* name, uint32_t data);
int sigma_ip_ioam_namespace_del(const char* name);
int sigma_ip_ioam_sc_show(void);
int sigma_ip_ioam_sc_add(const char* name, const char* options);
int sigma_ip_ioam_sc_del(const char* name);

// Legacy ifconfig
int sigma_ifconfig(void);
int sigma_ifconfig_interface(const char* interface);
int sigma_ifconfig_up(const char* interface);
int sigma_ifconfig_down(const char* interface);
int sigma_ifconfig_mtu(const char* interface, uint32_t mtu);
int sigma_ifconfig_netmask(const char* interface, const char* netmask);
int sigma_ifconfig_broadcast(const char* interface, const char* broadcast);
int sigma_ifconfig_hw(const char* interface, const char* type, const char* address);
int sigma_ifconfig_arp(const char* interface, bool on);
int sigma_ifconfig_promisc(const char* interface, bool on);
int sigma_ifconfig_allmulti(const char* interface, bool on);
int sigma_ifconfig_multicast(const char* interface, bool on);
int sigma_ifconfig_pointopoint(const char* interface, const char* address);
int sigma_ifconfig_media(const char* interface, const char* media);
int sigma_ifconfig_metric(const char* interface, uint32_t metric);
int sigma_ifconfig_txqueuelen(const char* interface, uint32_t len);
int sigma_ifconfig_memstart(const char* interface, uint32_t addr);
int sigma_ifconfig_io_addr(const char* interface, uint32_t addr);
int sigma_ifconfig_irq(const char* interface, uint32_t irq);
int sigma_ifconfig_dynamic(const char* interface);
int sigma_ifconfig_create(const char* interface, const char* type);
int sigma_ifconfig_destroy(const char* interface);
int sigma_ifconfig_add(const char* interface, const char* address);
int sigma_ifconfig_alias(const char* interface, uint32_t alias, const char* address);
int sigma_ifconfig_del(const char* interface, const char* address);
int sigma_ifconfig_unalias(const char* interface, uint32_t alias);

// ==================== DNS TOOLS ====================

// dig
int sigma_dig(const char* domain);
int sigma_dig_a(const char* domain);
int sigma_dig_aaaa(const char* domain);
int sigma_dig_mx(const char* domain);
int sigma_dig_ns(const char* domain);
int sigma_dig_soa(const char* domain);
int sigma_dig_txt(const char* domain);
int sigma_dig_cname(const char* domain);
int sigma_dig_ptr(const char* ip);
int sigma_dig_srv(const char* domain);
int sigma_dig_caa(const char* domain);
int sigma_dig_dnskey(const char* domain);
int sigma_dig_ds(const char* domain);
int sigma_dig_tlsa(const char* domain);
int sigma_dig_any(const char* domain);
int sigma_dig_axfr(const char* domain);
int sigma_dig_ixfr(const char* domain);
int sigma_dig_server(const char* server, const char* domain);
int sigma_dig_port(uint16_t port, const char* domain);
int sigma_dig_reverse(const char* ip);
int sigma_dig_trace(const char* domain);
int sigma_dig_short(const char* domain);
int sigma_dig_plusshort(const char* domain);
int sigma_dig_norecurse(const char* domain);
int sigma_dig_nocmd(const char* domain);
int sigma_dig_nocomments(const char* domain);
int sigma_dig_noquestion(const char* domain);
int sigma_dig_noanswer(const char* domain);
int sigma_dig_noauthority(const char* domain);
int sigma_dig_noadditional(const char* domain);
int sigma_dig_nostats(const char* domain);
int sigma_dig_noclass(const char* domain);
int sigma_dig_notttl(const char* domain);
int sigma_dig_nocrypto(const char* domain);
int sigma_dig_identify(const char* domain);
int sigma_dig_yaml(const char* domain);
int sigma_dig_json(const char* domain);
int sigma_dig_batch(const char* file);
int sigma_dig_key(const char* key_file, const char* domain);
int sigma_dig_sigchase(const char* domain);
int sigma_dig_dnssec(const char* domain);
int sigma_dig_nsid(const char* domain);
int sigma_dig_edns(const uint8_t version, const char* domain);
int sigma_dig_bufsize(uint16_t size, const char* domain);
int sigma_dig_timeout(uint32_t seconds, const char* domain);
int sigma_dig_retries(uint32_t count, const char* domain);
int sigma_dig_tries(uint32_t count, const char* domain);
int sigma_dig_udp(const char* domain);
int sigma_dig_tcp(const char* domain);
int sigma_dig_vc(const char* domain);
int sigma_dig_ignore(const char* domain);
int sigma_dig_fail(const char* domain);
int sigma_dig_besteffort(const char* domain);
int sigma_dig_aaonly(const char* domain);
int sigma_dig_adflag(const char* domain);
int sigma_dig_cdflag(const char* domain);
int sigma_dig_zflag(const char* domain);
int sigma_dig_doflag(const char* domain);
int sigma_dig_mfset(const char* flag, const char* domain);

// nslookup
int sigma_nslookup(const char* domain);
int sigma_nslookup_query(const char* type, const char* domain);
int sigma_nslookup_server(const char* server);
int sigma_nslookup_set_type(const char* type);
int sigma_nslookup_set_debug(void);
int sigma_nslookup_set_d2(void);
int sigma_nslookup_set_norecurse(void);
int sigma_nslookup_set_timeout(uint32_t seconds);
int sigma_nslookup_set_retry(uint32_t count);
int sigma_nslookup_set_vc(void);
int sigma_nslookup_set_novc(void);
int sigma_nslookup_set_ignore(void);
int sigma_nslookup_set_fail(void);
int sigma_nslookup_set_port(uint16_t port);
int sigma_nslookup_set_all(void);
int sigma_nslookup_set_class(const char* class);
int sigma_nslookup_lserver(const char* server);
int sigma_nslookup_root(void);
int sigma_nslookup_finger(const char* user);
int sigma_nslookup_ls(const char* domain);
int sigma_nslookup_ls_a(const char* domain);
int sigma_nslookup_ls_d(const char* domain);
int sigma_nslookup_ls_h(const char* domain);
int sigma_nslookup_ls_l(const char* domain);
int sigma_nslookup_ls_t(const char* type, const char* domain);
int sigma_nslookup_view(const char* file);
int sigma_nslookup_help(void);
int sigma_nslookup_exit(void);

// host
int sigma_host(const char* domain);
int sigma_host_a(const char* domain);
int sigma_host_aaaa(const char* domain);
int sigma_host_cname(const char* domain);
int sigma_host_mx(const char* domain);
int sigma_host_ns(const char* domain);
int sigma_host_ptr(const char* ip);
int sigma_host_soa(const char* domain);
int sigma_host_txt(const char* domain);
int sigma_host_srv(const char* domain);
int sigma_host_any(const char* domain);
int sigma_host_t(const char* type, const char* domain);
int sigma_host_l(const char* domain);
int sigma_host_lv(const char* domain);
int sigma_host_v(const char* domain);
int sigma_host_vv(const char* domain);
int sigma_host_vvv(const char* domain);
int sigma_host_w(const char* domain);
int sigma_host_w_wait(const char* domain, uint32_t seconds);
int sigma_host_r(const char* domain);
int sigma_host_r_retry(const char* domain, uint32_t count);
int sigma_host_s(const char* server, const char* domain);
int sigma_host_p(uint16_t port, const char* domain);
int sigma_host_c(const char* class, const char* domain);
int sigma_host_d(const char* domain);
int sigma_host_d_norecurse(const char* domain);
int sigma_host_i(const char* domain);
int sigma_host_n(const char* domain);
int sigma_host_4(const char* domain);
int sigma_host_6(const char* domain);

// drill
int sigma_drill(const char* domain);
int sigma_drill_a(const char* domain);
int sigma_drill_aaaa(const char* domain);
int sigma_drill_mx(const char* domain);
int sigma_drill_ns(const char* domain);
int sigma_drill_soa(const char* domain);
int sigma_drill_txt(const char* domain);
int sigma_drill_cname(const char* domain);
int sigma_drill_ptr(const char* ip);
int sigma_drill_srv(const char* domain);
int sigma_drill_dnskey(const char* domain);
int sigma_drill_ds(const char* domain);
int sigma_drill_any(const char* domain);
int sigma_drill_axfr(const char* domain);
int sigma_drill_server(const char* server, const char* domain);
int sigma_drill_port(uint16_t port, const char* domain);
int sigma_drill_reverse(const char* ip);
int sigma_drill_trace(const char* domain);
int sigma_drill_sigs(const char* domain);
int sigma_drill_td(const char* domain);
int sigma_drill_s(const char* domain);
int sigma_drill_d(const char* domain);
int sigma_drill_q(const char* domain);
int sigma_drill_a_quiet(const char* domain);
int sigma_drill_v(const char* domain);
int sigma_drill_vv(const char* domain);
int sigma_drill_vvv(const char* domain);
int sigma_drill_w(const char* domain);
int sigma_drill_w_wait(const char* domain, uint32_t seconds);
int sigma_drill_r_retry(const char* domain, uint32_t count);
int sigma_drill_t(const char* type, const char* domain);
int sigma_drill_c(const char* class, const char* domain);
int sigma_drill_k(const char* key_file, const char* domain);
int sigma_drill_o(const char* file, const char* domain);
int sigma_drill_qfile(const char* file, const char* domain);
int sigma_drill_4(const char* domain);
int sigma_drill_6(const char* domain);
int sigma_drill_udp(const char* domain);
int sigma_drill_tcp(const char* domain);

// whois
int sigma_whois(const char* domain);
int sigma_whois_h(const char* host, const char* domain);
int sigma_whois_p(uint16_t port, const char* domain);
int sigma_whois_H(const char* domain);
int sigma_whois_n(const char* domain);
int sigma_whois_a(const char* domain);
int sigma_whois_q(const char* query);
int sigma_whois_v(const char* domain);
int sigma_whois_r(void);
int sigma_whois_R(void);

// ==================== CONNECTIVITY TESTING ====================

// ping
int sigma_ping(const char* host);
int sigma_ping_4(const char* host);
int sigma_ping_6(const char* host);
int sigma_ping_c(uint32_t count, const char* host);
int sigma_ping_i(double interval, const char* host);
int sigma_ping_I(const char* interface, const char* host);
int sigma_ping_s(uint32_t packetsize, const char* host);
int sigma_ping_S(uint32_t sndbuf, const char* host);
int sigma_ping_t(uint8_t ttl, const char* host);
int sigma_ping_T(uint8_t tos, const char* host);
int sigma_ping_w(uint32_t deadline, const char* host);
int sigma_ping_W(uint32_t timeout, const char* host);
int sigma_ping_l(uint32_t preload, const char* host);
int sigma_ping_p(const char* pattern, const char* host);
int sigma_ping_q(const char* host);
int sigma_ping_R(const char* host);
int sigma_ping_r(const char* host);
int sigma_ping_v(const char* host);
int sigma_ping_vv(const char* host);
int sigma_ping_a(const char* host);
int sigma_ping_A(const char* host);
int sigma_ping_b(const char* host);
int sigma_ping_B(const char* host);
int sigma_ping_D(const char* host);
int sigma_ping_f(const char* host);
int sigma_ping_f_interval(double interval, const char* host);
int sigma_ping_L(const char* host);
int sigma_ping_n(const char* host);
int sigma_ping_m(const char* mark, const char* host);
int sigma_ping_M(const char* host);
int sigma_ping_N(const char* host);
int sigma_ping_O(const char* options, const char* host);
int sigma_ping_p_flow(uint32_t flow, const char* host);
int sigma_ping_Q(uint32_t tos, const char* host);
int sigma_ping_U(const char* host);
int sigma_ping_V(const char* host);
int sigma_ping_z(uint32_t tos, const char* host);
int sigma_ping_errno(const char* host);
int sigma_ping_t_timestamp(const char* host);
int sigma_ping_ttl(uint8_t ttl, const char* host);
int sigma_ping_mtu(uint32_t mtu, const char* host);
int sigma_ping_nodata(const char* host);
int sigma_ping_nopipe(const char* host);
int sigma_ping_ptbsize(uint32_t size, const char* host);

// traceroute
int sigma_traceroute(const char* host);
int sigma_traceroute_4(const char* host);
int sigma_traceroute_6(const char* host);
int sigma_traceroute_I(const char* host); // ICMP
int sigma_traceroute_T(const char* host); // TCP
int sigma_traceroute_p(uint16_t port, const char* host);
int sigma_traceroute_P(const char* protocol, const char* host);
int sigma_traceroute_m(uint8_t max_ttl, const char* host);
int sigma_traceroute_f(uint8_t first_ttl, const char* host);
int sigma_traceroute_g(const char* gateway, const char* host);
int sigma_traceroute_q(uint32_t nqueries, const char* host);
int sigma_traceroute_t(uint8_t tos, const char* host);
int sigma_traceroute_w(double timeout, const char* host);
int sigma_traceroute_s(const char* src_addr, const char* host);
int sigma_traceroute_i(const char* interface, const char* host);
int sigma_traceroute_z(uint32_t pause, const char* host);
int sigma_traceroute_e(const char* host);
int sigma_traceroute_E(const char* host);
int sigma_traceroute_F(const char* host);
int sigma_traceroute_M(const char* host);
int sigma_traceroute_n(const char* host);
int sigma_traceroute_r(const char* host);
int sigma_traceroute_R(const char* host);
int sigma_traceroute_S(const char* host);
int sigma_traceroute_U(const char* host);
int sigma_traceroute_V(const char* host);
int sigma_traceroute_d(uint32_t port, const char* host);

// tracepath
int sigma_tracepath(const char* host);
int sigma_tracepath_4(const char* host);
int sigma_tracepath_6(const char* host);
int sigma_tracepath_n(const char* host);
int sigma_tracepath_b(uint32_t hops, const char* host);
int sigma_tracepath_l(uint32_t length, const char* host);
int sigma_tracepath_m(uint32_t mtu, const char* host);
int sigma_tracepath_p(uint16_t port, const char* host);
int sigma_tracepath_V(const char* host);

// mtr (My Traceroute)
int sigma_mtr(const char* host);
int sigma_mtr_4(const char* host);
int sigma_mtr_6(const char* host);
int sigma_mtr_u(uint8_t max_ttl, const char* host);
int sigma_mtr_l(uint8_t min_ttl, const char* host);
int sigma_mtr_c(uint32_t count, const char* host);
int sigma_mtr_r(const char* host);
int sigma_mtr_t(const char* host);
int sigma_mtr_p(uint16_t port, const char* host);
int sigma_mtr_P(const char* protocol, const char* host);
int sigma_mtr_s(uint32_t packetsize, const char* host);
int sigma_mtr_b(uint32_t bitpattern, const char* host);
int sigma_mtr_G(uint32_t gracetime, const char* host);
int sigma_mtr_i(double interval, const char* host);
int sigma_mtr_a(const char* address, const char* host);
int sigma_mtr_f(const char* filename, const char* host);
int sigma_mtr_n(const char* host);
int sigma_mtr_g(const char* host);
int sigma_mtr_j(const char* host);
int sigma_mtr_x(const char* host);
int sigma_mtr_y(uint8_t tos, const char* host);
int sigma_mtr_z(uint8_t tos, const char* host);
int sigma_mtr_e(const char* host);
int sigma_mtr_h(const char* host);
int sigma_mtr_o(const char* order, const char* host);
int sigma_mtr_C(const char* host);
int sigma_mtr_T(const char* host);
int sigma_mtr_U(const char* host);
int sigma_mtr_no_dns(const char* host);
int sigma_mtr_show_ips(const char* host);
int sigma_mtr_aslookup(const char* host);
int sigma_mtr_report(const char* host);
int sigma_mtr_xml(const char* host);
int sigma_mtr_csv(const char* host);
int sigma_mtr_raw(const char* host);
int sigma_mtr_split(const char* host);
int sigma_mtr_json(const char* host);

// arping
int sigma_arping(const char* host);
int sigma_arping_4(const char* host);
int sigma_arping_6(const char* host);
int sigma_arping_c(uint32_t count, const char* host);
int sigma_arping_w(double timeout, const char* host);
int sigma_arping_I(const char* interface, const char* host);
int sigma_arping_s(const char* source, const char* host);
int sigma_arping_S(const char* source, const char* host);
int sigma_arping_D(const char* host);
int sigma_arping_d(const char* host);
int sigma_arping_f(const char* host);
int sigma_arping_r(const char* host);
int sigma_arping_R(const char* host);
int sigma_arping_Q(const char* host);
int sigma_arping_U(const char* host);
int sigma_arping_A(const char* host);
int sigma_arping_b(const char* host);
int sigma_arping_B(const char* host);
int sigma_arping_p(const char* host);
int sigma_arping_a(const char* host);
int sigma_arping_i(uint32_t interval, const char* host);
int sigma_arping_U_unsolicited(const char* host);
int sigma_arping_A_Advert(const char* host);

// nmap (network scanner)
int sigma_nmap(const char* target);
int sigma_nmap_sS(const char* target);
int sigma_nmap_sT(const char* target);
int sigma_nmap_sU(const char* target);
int sigma_nmap_sP(const char* target);
int sigma_nmap_sN(const char* target);
int sigma_nmap_sF(const char* target);
int sigma_nmap_sX(const char* target);
int sigma_nmap_sA(const char* target);
int sigma_nmap_sW(const char* target);
int sigma_nmap_sM(const char* target);
int sigma_nmap_sI(const char* zombie, const char* target);
int sigma_nmap_sO(const char* target);
int sigma_nmap_sV(const char* target);
int sigma_nmap_sC(const char* target);
int sigma_nmap_sL(const char* target);
int sigma_nmap_sn(const char* target);
int sigma_nmap_sY(const char* target);
int sigma_nmap_sZ(const char* target);
int sigma_nmap_sO_os(const char* target);
int sigma_nmap_sO_version(const char* target);
int sigma_nmap_p(uint16_t port, const char* target);
int sigma_nmap_p_range(const char* range, const char* target);
int sigma_nmap_F(const char* target);
int sigma_nmap_r(const char* target);
int sigma_nmap_6(const char* target);
int sigma_nmap_A(const char* target);
int sigma_nmap_O(const char* target);
int sigma_nmap_v(const char* target);
int sigma_nmap_vv(const char* target);
int sigma_nmap_d(const char* target);
int sigma_nmap_dd(const char* target);
int sigma_nmap_reason(const char* target);
int sigma_nmap_open(const char* target);
int sigma_nmap_packet_trace(const char* target);
int sigma_nmap_iflist(const char* target);
int sigma_nmap_append_output(const char* target);
int sigma_nmap_resume(const char* file);
int sigma_nmap_stylesheet(const char* file, const char* target);
int sigma_nmap_no_stylesheet(const char* target);
int sigma_nmap_xml(const char* target);
int sigma_nmap_grepable(const char* target);
int sigma_nmap_oA(const char* basename, const char* target);
int sigma_nmap_oN(const char* file, const char* target);
int sigma_nmap_oX(const char* file, const char* target);
int sigma_nmap_oS(const char* file, const char* target);
int sigma_nmap_oG(const char* file, const char* target);
int sigma_nmap_iL(const char* file);
int sigma_nmap_excludefile(const char* file);
int sigma_nmap_exclude(const char* target);
int sigma_nmap_randomize_hosts(const char* target);
int sigma_nmap_scan_delay(const char* target, uint32_t time);
int sigma_nmap_max_scan_delay(const char* target, uint32_t time);
int sigma_nmap_min_hostgroup(uint32_t size, const char* target);
int sigma_nmap_max_hostgroup(uint32_t size, const char* target);
int sigma_nmap_min_parallelism(uint32_t num, const char* target);
int sigma_nmap_max_parallelism(uint32_t num, const char* target);
int sigma_nmap_min_rtt_timeout(uint32_t time, const char* target);
int sigma_nmap_max_rtt_timeout(uint32_t time, const char* target);
int sigma_nmap_initial_rtt_timeout(uint32_t time, const char* target);
int sigma_nmap_max_retries(uint32_t num, const char* target);
int sigma_nmap_host_timeout(uint32_t time, const char* target);
int sigma_nmap_scan_delay_t(uint32_t time, const char* target);
int sigma_nmap_max_scan_delay_t(uint32_t time, const char* target);
int sigma_nmap_defeat_rst_ratelimit(const char* target);
int sigma_nmap_defeat_icmp_ratelimit(const char* target);
int sigma_nmap_nsock_engine(const char* engine, const char* target);
int sigma_nmap_T(const char* timing, const char* target);
int sigma_nmap_T0(const char* target);
int sigma_nmap_T1(const char* target);
int sigma_nmap_T2(const char* target);
int sigma_nmap_T3(const char* target);
int sigma_nmap_T4(const char* target);
int sigma_nmap_T5(const char* target);

// ==================== NETWORK MONITORING ====================

// netstat
int sigma_netstat(void);
int sigma_netstat_a(void);
int sigma_netstat_t(void);
int sigma_netstat_u(void);
int sigma_netstat_l(void);
int sigma_netstat_n(void);
int sigma_netstat_r(void);
int sigma_netstat_s(void);
int sigma_netstat_i(void);
int sigma_netstat_g(void);
int sigma_netstat_e(void);
int sigma_netstat_m(void);
int sigma_netstat_p(void);
int sigma_netstat_c(uint32_t seconds);
int sigma_netstat_C(void);
int sigma_netstat_Z(void);
int sigma_netstat_M(void);
int sigma_netstat_program(const char* program);
int sigma_netstat_numeric_hosts(void);
int sigma_netstat_numeric_ports(void);
int sigma_netstat_numeric_users(void);
int sigma_netstat_symbolic(void);
int sigma_netstat_extend(void);
int sigma_netstat_timer(void);
int sigma_netstat_f(const char* family);
int sigma_netstat_4(void);
int sigma_netstat_6(void);
int sigma_netstat_route(void);
int sigma_netstat_interfaces(void);
int sigma_netstat_groups(void);
int sigma_netstat Masquerade(void);
int sigma_netstat_statistics(void);

// ss (socket statistics)
int sigma_ss(void);
int sigma_ss_a(void);
int sigma_ss_l(void);
int sigma_ss_t(void);
int sigma_ss_u(void);
int sigma_ss_4(void);
int sigma_ss_6(void);
int sigma_ss_n(void);
int sigma_ss_r(void);
int sigma_ss_o(void);
int sigma_ss_e(void);
int sigma_ss_m(void);
int sigma_ss_i(void);
int sigma_ss_s(void);
int sigma_ss_Z(void);
int sigma_ss_p(void);
int sigma_ss_f(const char* filter);
int sigma_ss_d(const char* dccp_states);
int sigma_ss_dccp(const char* states);
int sigma_ss_raw(const char* states);
int sigma_ss_unix(const char* states);
int sigma_ss_udp(const char* states);
int sigma_ss_tcp(const char* states);
int sigma_ss_sctp(const char* states);
int sigma_ss_vsock(const char* states);
int sigma_ss_xdp(const char* states);
int sigma_ss_packet(const char* states);
int sigma_ss_netlink(const char* states);
int sigma_ss_diag(const char* protocol, const char* diag);
int sigma_ss_query(const char* query);
int sigma_ss_summary(void);
int sigma_ss_memory(void);
int sigma_ss_processes(void);
int sigma_ss_timer(void);
int sigma_ss_info(void);
int sigma_ss_ipv4(void);
int sigma_ss_ipv6(void);
int sigma_ss_hide(void);
int sigma_ss_show(void);
int sigma_ss_resolve(void);
int sigma_ss_service(void);
int sigma_ss_port(uint16_t port);
int sigma_ss_src(const char* source);
int sigma_ss_dst(const char* dest);
int sigma_ss_not_src(const char* source);
int sigma_ss_not_dst(const char* dest);
int sigma_ss_state(const char* state);
int sigma_ss_exclude(const char* state);
int sigma_ss_local_process(const char* process);
int sigma_ss_local_pinfo(void);
int sigma_ss_kill(const char* target);
int sigma_ss_netns(const char* namespace);

// nstat (network statistics)
int sigma_nstat(void);
int sigma_nstat_a(void);
int sigma_nstat_r(void);
int sigma_nstat_r_reset(void);
int sigma_nstat_d(void);
int sigma_nstat_s(void);
int sigma_nstat_t(void);
int sigma_nstat_z(void);
int sigma_nstat_n(void);
int sigma_nstat_j(void);
int sigma_nstat_l(uint32_t lines);
int sigma_nstat_filter(const char* pattern);

// rtacct (network route accounting)
int sigma_rtacct(void);
int sigma_rtacct_c(uint32_t cache);
int sigma_rtacct_r(void);

// ==================== WIRELESS TOOLS ====================

// iw
int sigma_iw_dev(void);
int sigma_iw_dev_info(void);
int sigma_iw_dev_interface_add(const char* name, const char* type);
int sigma_iw_dev_interface_del(const char* name);
int sigma_iw_dev_interface_set_type(const char* name, const char* type);
int sigma_iw_dev_interface_set_4addr(const char* name, bool on);
int sigma_iw_dev_interface_set_monitor(const char* name, const char* flags);
int sigma_iw_dev_interface_set_meshid(const char* name, const char* meshid);
int sigma_iw_dev_interface_set_noack_map(const char* name, uint16_t map);
int sigma_iw_dev_interface_set_peer(const char* name, const char* peer);
int sigma_iw_dev_interface_set_channel(const char* name, uint8_t channel);
int sigma_iw_dev_interface_set_freq(const char* name, uint32_t freq);
int sigma_iw_dev_interface_set_bitrates(const char* name, const char* rates);
int sigma_iw_dev_interface_set_txpower(const char* name, const char* power);
int sigma_iw_dev_interface_set_power_save(const char* name, bool on);
int sigma_iw_dev_interface_set_power_save_timeout(const char* name, uint32_t timeout);
int sigma_iw_dev_interface_set_mac(const char* name, const char* mac);
int sigma_iw_dev_interface_set_mcast_rate(const char* name, uint32_t rate);
int sigma_iw_dev_interface_set_mesh_param(const char* name, const char* param, const char* value);
int sigma_iw_dev_interface_set_wiphy_netns(const char* name, const char* namespace);
int sigma_iw_dev_interface_set_wiphy_freq(const char* name, uint32_t freq);
int sigma_iw_dev_interface_set_wiphy_channel(const char* name, uint8_t channel);
int sigma_iw_dev_interface_set_wiphy_rate(const char* name, const char* rate);
int sigma_iw_dev_interface_set_wiphy_retry(const char* name, uint8_t short_retry, uint8_t long_retry);
int sigma_iw_dev_interface_set_wiphy_rts(const char* name, uint32_t rts);
int sigma_iw_dev_interface_set_wiphy_frag(const char* name, uint32_t frag);
int sigma_iw_dev_interface_set_wiphy_txpower(const char* name, const char* power);
int sigma_iw_dev_interface_set_wiphy_antenna(const char* name, uint32_t tx, uint32_t rx);
int sigma_iw_dev_interface_set_wiphy_netns_pid(const char* name, uint32_t pid);
int sigma_iw_dev_interface_set_wiphy_netns_fd(const char* name, int fd);
int sigma_iw_dev_interface_set_wiphy_coalesce(const char* name, const char* coalesce);
int sigma_iw_scan(void);
int sigma_iw_scan_trigger(void);
int sigma_iw_scan_dump(void);
int sigma_iw_scan_freq(uint32_t freq);
int sigma_iw_scan_ap_force(void);
int sigma_iw_scan_passive(void);
int sigma_iw_scan_randomise(const char* addr);
int sigma_iw_scan_no_cck(void);
int sigma_iw_scan_width(const char* width);
int sigma_iw_reg_get(void);
int sigma_iw_reg_set(const char* country);
int sigma_iw_reg_set_with_dfs(const char* country);
int sigma_iw_reg_reload(void);
int sigma_iw_reg_notify(uint32_t initiator, const char* alpha2);
int sigma_iw_event(void);
int sigma_iw_event_t(const char* type);
int sigma_iw_event_f(const char* file);
int sigma_iw_phy(void);
int sigma_iw_phy_list(void);
int sigma_iw_phy_info(void);
int sigma_iw_phy_reg_dump(const char* phy);
int sigma_iw_phy_reg_get(const char* phy, uint32_t address);
int sigma_iw_phy_reg_set(const char* phy, uint32_t address, uint32_t value);
int sigma_iw_wdev(void);
int sigma_iw_wdev_add(const char* name, const char* type);
int sigma_iw_wdev_del(const char* name);
int sigma_iw_connect(const char* ssid);
int sigma_iw_connect_freq(uint32_t freq);
int sigma_iw_connect_key(const char* key);
int sigma_iw_connect_key_index(uint8_t index);
int sigma_iw_connect_bssid(const char* bssid);
int sigma_iw_connect_type(const char* type);
int sigma_iw_connect_mcast_rate(uint32_t rate);
int sigma_iw_connect_4addr(void);
int sigma_iw_disconnect(void);
int sigma_iw_link(void);
int sigma_iw_link_freq(void);
int sigma_iw_link_channel(void);
int sigma_iw_link_bss(void);
int sigma_iw_link_peer(void);
int sigma_iw_link_sta(void);
int sigma_iw_station_dump(void);
int sigma_iw_station_set(const char* station, const char* param, const char* value);
int sigma_iw_station_del(const char* station);
int sigma_iw_survey_dump(void);
int sigma_iw_mesh_join(const char* meshid);
int sigma_iw_mesh_leave(void);
int sigma_iw_mpath_dump(void);
int sigma_iw_mpath_set(const char* destination, const char* next_hop);
int sigma_iw_mpath_del(const char* destination);
int sigma_iw_mpp_dump(void);
int sigma_iw_mpp_set(const char* mac, const char* proxy);
int sigma_iw_mpp_del(const char* mac);
int sigma_iw_o_cb_join(void);
int sigma_iw_o_cb_leave(void);
int sigma_iw_cqm(const char* rssi_thold, const char* rssi_hyst);
int sigma_iw_ftm(const char* peer);
int sigma_iw_p2p_find(void);
int sigma_iw_p2p_stop_find(void);
int sigma_iw_p2p_connect(const char* peer, const char* pin);
int sigma_iw_p2p_listen(void);
int sigma_iw_p2p_group_add(const char* freq);
int sigma_iw_p2p_group_remove(const char* ifname);
int sigma_iw_p2p_peers(void);
int sigma_iw_p2p_set(const char* param, const char* value);
int sigma_iw_p2p_get(const char* param);
int sigma_iw_p2p_flush(void);
int sigma_iw_p2p_cancel(void);
int sigma_iw_p2p_invite(const char* peer);
int sigma_iw_p2p_reject(const char* peer);
int sigma_iw_p2p_serv_disc_external(uint8_t id);
int sigma_iw_p2p_serv_disc_req(const char* peer, const char* query);
int sigma_iw_p2p_serv_disc_cancel_req(uint64_t id);
int sigma_iw_p2p_service_add(const char* type, const char* query, const char* response);
int sigma_iw_p2p_service_del(const char* type, const char* query);
int sigma_iw_p2p_service_flush(void);
int sigma_iw_p2p_ext_listen(uint32_t period, uint32_t interval);
int sigma_iw_p2p_remove_client(const char* peer);
int sigma_iw_coalesce_enable(void);
int sigma_iw_coalesce_disable(void);

// iwconfig (legacy wireless tools)
int sigma_iwconfig(void);
int sigma_iwconfig_interface(const char* interface);
int sigma_iwconfig_essid(const char* interface, const char* essid);
int sigma_iwconfig_mode(const char* interface, const char* mode);
int sigma_iwconfig_freq(const char* interface, double freq);
int sigma_iwconfig_channel(const char* interface, uint8_t channel);
int sigma_iwconfig_bitrate(const char* interface, const char* bitrate);
int sigma_iwconfig_rate(const char* interface, const char* rate);
int sigma_iwconfig_txpower(const char* interface, const char* power);
int sigma_iwconfig_sens(const char* interface, int sens);
int sigma_iwconfig_retry(const char* interface, const char* retry);
int sigma_iwconfig_rts(const char* interface, const char* rts);
int sigma_iwconfig_frag(const char* interface, const char* frag);
int sigma_iwconfig_key(const char* interface, const char* key);
int sigma_iwconfig_enc(const char* interface, const char* enc);
int sigma_iwconfig_power(const char* interface, const char* power);
int sigma_iwconfig_nick(const char* interface, const char* nick);
int sigma_iwconfig_nwid(const char* interface, const char* nwid);
int sigma_iwconfig_ap(const char* interface, const char* ap);
int sigma_iwconfig_commit(const char* interface);

// iwlist
int sigma_iwlist_scanning(const char* interface);
int sigma_iwlist_frequency(const char* interface);
int sigma_iwlist_rate(const char* interface);
int sigma_iwlist_keys(const char* interface);
int sigma_iwlist_power(const char* interface);
int sigma_iwlist_txpower(const char* interface);
int sigma_iwlist_retry(const char* interface);
int sigma_iwlist_ap(const char* interface);
int sigma_iwlist_peers(const char* interface);
int sigma_iwlist_event(const char* interface);
int sigma_iwlist_auth(const char* interface);
int sigma_iwlist_wpakeys(const char* interface);
int sigma_iwlist_genie(const char* interface);
int sigma_iwlist_modulation(const char* interface);
int sigma_iwlist_bitrate(const char* interface);
int sigma_iwlist_encryption(const char* interface);
int sigma_iwlist_channels(const char* interface);
int sigma_iwlist_accesspoints(const char* interface);
int sigma_iwlist_freq(const char* interface);
int sigma_iwlist_sens(const char* interface);
int sigma_iwlist_range(const char* interface);
int sigma_iwlist_wpa(const char* interface);

// iwpriv
int sigma_iwpriv(const char* interface);
int sigma_iwpriv_all(const char* interface);
int sigma_iwpriv_set(const char* interface, const char* param, const char* value);
int sigma_iwpriv_get(const char* interface, const char* param);

// wpa_supplicant
int sigma_wpa_supplicant(const char* interface, const char* config);
int sigma_wpa_supplicant_b(const char* bridge);
int sigma_wpa_supplicant_B(const char* daemon_file);
int sigma_wpa_supplicant_c(const char* config);
int sigma_wpa_supplicant_d(const char* driver);
int sigma_wpa_supplicant_D(const char* driver);
int sigma_wpa_supplicant_e(const char* entropy_file);
int sigma_wpa_supplicant_f(const char* config_string);
int sigma_wpa_supplicant_g(const char* global);
int sigma_wpa_supplicant_G(const char* group);
int sigma_wpa_supplicant_h(void);
int sigma_wpa_supplicant_H(const char* ctrl_interface);
int sigma_wpa_supplicant_i(const char* interface);
int sigma_wpa_supplicant_I(const char* interface);
int sigma_wpa_supplicant_j(const char* interface);
int sigma_wpa_supplicant_K(void);
int sigma_wpa_supplicant_L(const char* license_file);
int sigma_wpa_supplicant_m(const char* model_name);
int sigma_wpa_supplicant_n(const char* no_interface);
int sigma_wpa_supplicant_N(const char* no_interface);
int sigma_wpa_supplicant_o(const char* override_driver);
int sigma_wpa_supplicant_O(const char* override_ctrl);
int sigma_wpa_supplicant_p(const char* pid_file);
int sigma_wpa_supplicant_P(const char* P2P_device);
int sigma_wpa_supplicant_q(const char* increase);
int sigma_wpa_supplicant_r(const char* var, const char* value);
int sigma_wpa_supplicant_R(const char* reference_file);
int sigma_wpa_supplicant_s(const char* set)
;

