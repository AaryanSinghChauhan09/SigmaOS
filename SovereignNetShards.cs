// -----------------------------------------------------------------------------
// SigmaOS Enterprise Network Daemon (v3.0)
// High-performance, latency-optimized UDP routing for Voice over IP.
// -----------------------------------------------------------------------------

using System;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Security.Cryptography;

namespace SigmaOS.Shards
{
    public class EncryptedUDPMeshStream
    {
        private UdpClient _udpMeshNode;
        private IPEndPoint _remoteShard;
        private byte[] _EnterpriseKey = Encoding.UTF8.GetBytes("AbsoluteEnterprisety-v75.0_______");

        public EncryptedUDPMeshStream(int localPort, string remoteIP, int remotePort)
        {
            _udpMeshNode = new UdpClient(localPort);
            _remoteShard = new IPEndPoint(IPAddress.Parse(remoteIP), remotePort);
            Console.WriteLine($"[NET_DAEMON]: End-to-End Encrypted UDP Mesh-Stream bound to Port {localPort}. Target node: {remoteIP}:{remotePort}");
            Console.WriteLine($"[NET_DAEMON]: VoIP P2P Orchestration Active. Direct peer routing initiated.");
        }

        // Handles the transmission of compressed voice packets over a P2P connection
        public void TransmitAudioShard(byte[] rawAudioFragment)
        {
            byte[] encryptedFragment = EncryptFragment(rawAudioFragment);
            _udpMeshNode.Send(encryptedFragment, encryptedFragment.Length, _remoteShard);
            Console.WriteLine($"[NET_DAEMON_AUDIO]: Transmitted {encryptedFragment.Length} bytes via encrypted UDP tunnel.");
        }

        private byte[] EncryptFragment(byte[] payload)
        {
            using (Aes aesAlg = Aes.Create())
            {
                aesAlg.Key = _EnterpriseKey;
                aesAlg.Mode = CipherMode.CBC;
                aesAlg.Padding = PaddingMode.PKCS7;
                aesAlg.GenerateIV();
                
                ICryptoTransform encryptor = aesAlg.CreateEncryptor(aesAlg.Key, aesAlg.IV);
                
                using (var msEncrypt = new System.IO.MemoryStream())
                {
                    // Prepend the IV to the stream for the receiver to use
                    msEncrypt.Write(aesAlg.IV, 0, aesAlg.IV.Length);
                    
                    using (var csEncrypt = new CryptoStream(msEncrypt, encryptor, CryptoStreamMode.Write))
                    {
                        csEncrypt.Write(payload, 0, payload.Length);
                        csEncrypt.FlushFinalBlock();
                    }
                    
                    byte[] encrypted = msEncrypt.ToArray();
                    Console.WriteLine($"[NET_DAEMON_AUDIO]: Packet payload encapsulated with AES-256. IV Length: {aesAlg.IV.Length}. Final Size: {encrypted.Length} bytes.");
                    return encrypted;
                }
            }
        }
    }

    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Initiating Enterprise Communications Matrix...");
            var meshNode = new EncryptedUDPMeshStream(5000, "127.0.0.1", 5001);
            byte[] mockAudioData = { 0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA };
            meshNode.TransmitAudioShard(mockAudioData);
        }
    }
}
