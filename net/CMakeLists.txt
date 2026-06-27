cmake_minimum_required(VERSION 3.16)
project(SigmaOS_Net VERSION 1.1.0 LANGUAGES C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Compile flags
set(SIGMA_WARN_FLAGS "-Wall -Wextra -Werror=return-type")
set(CMAKE_C_FLAGS   "${CMAKE_C_FLAGS}   ${SIGMA_WARN_FLAGS}")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${SIGMA_WARN_FLAGS}")

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CMAKE_C_FLAGS   "${CMAKE_C_FLAGS}   -g -O0 -DSIGMA_DEBUG")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -O0 -DSIGMA_DEBUG")
elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(CMAKE_C_FLAGS   "${CMAKE_C_FLAGS}   -O3 -DNDEBUG")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3 -DNDEBUG")
endif()

include_directories(
    ${CMAKE_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/include
    ${CMAKE_SOURCE_DIR}/net
)

# ── Network Stack Library ─────────────────────────────────────────────────
set(NET_SOURCES
    net/tls/sigma_tls.cpp
    net/dns/sigma_dns.cpp
    net/dhcp/sigma_dhcp.cpp
    net/wifi/sigma_wpa3.cpp
    net/sigma_net.cpp
)

add_library(sigma_net STATIC ${NET_SOURCES})
target_include_directories(sigma_net PUBLIC
    ${CMAKE_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/include
)

# ── PQC stub (replace with liboqs in production) ──────────────────────────
# In production link: liboqs (Kyber/Dilithium), OpenSSL (X25519, AES-GCM)
if(EXISTS "${CMAKE_SOURCE_DIR}/crypto/sigma_pqc_stub.c")
    add_library(sigma_pqc_stub STATIC crypto/sigma_pqc_stub.c)
    target_link_libraries(sigma_net PRIVATE sigma_pqc_stub)
endif()

# ── Tests ─────────────────────────────────────────────────────────────────
enable_testing()

add_executable(test_net_stack tests/net/test_net_stack.cpp)
target_link_libraries(test_net_stack PRIVATE sigma_net)
add_test(NAME NetStackTests COMMAND test_net_stack)

# ── Install ───────────────────────────────────────────────────────────────
install(TARGETS sigma_net ARCHIVE DESTINATION lib LIBRARY DESTINATION lib)
install(DIRECTORY net/ DESTINATION include/net FILES_MATCHING PATTERN "*.h")

# ── Summary ───────────────────────────────────────────────────────────────
message(STATUS "")
message(STATUS "SigmaOS Network Stack v${PROJECT_VERSION}")
message(STATUS "  TLS 1.3 + Kyber-1024 hybrid")
message(STATUS "  DNS-over-HTTPS + DNSSEC")
message(STATUS "  DHCP client (RFC 2131/2132)")
message(STATUS "  WPA3/SAE (IEEE 802.11-2020)")
message(STATUS "")
