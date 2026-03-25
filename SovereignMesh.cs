using System;
using System.Collections.Generic;

namespace SigmaOS.Shards
{
    /// <summary>
    /// SIGMA OS: SOVEREIGN MESH CORE (v1.0 - SILICON ZENITH)
    /// ----------------------------------------------------
    /// Base foundational class for all P2P and Mesh communication shards.
    /// Provides low-level routing and state management for distributed nodes.
    /// </summary>
    public abstract class SovereignMeshBase
    {
        public string NodeId { get; private set; }
        protected bool IsConnected = false;

        protected SovereignMeshBase(string nodeId)
        {
            NodeId = nodeId;
            Console.WriteLine($"[MESH_CORE]: Initializing Node Shard: {NodeId}. State: STANDBY.");
        }

        public virtual void Synchronize()
        {
            Console.WriteLine($"[MESH_CORE]: Synchronizing node {NodeId} with Global Sovereign Mesh...");
            IsConnected = true;
        }

        public abstract void RoutePacket(byte[] payload, string targetShard);
    }

    public class ShardRegistry
    {
        private static Dictionary<string, string> _shardRouteMatrix = new Dictionary<string, string>();

        public static void RegisterShard(string shardId, string endpoint)
        {
            _shardRouteMatrix[shardId] = endpoint;
            Console.WriteLine($"[SHARD_REGISTRY]: Shard {shardId} registered at {endpoint}.");
        }

        public static string GetEndpoint(string shardId)
        {
            return _shardRouteMatrix.ContainsKey(shardId) ? _shardRouteMatrix[shardId] : null;
        }
    }
}
