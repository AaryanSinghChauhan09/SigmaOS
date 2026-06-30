--  SigmaOS: SigmaOS Sovereign Trust Fabric
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignTrustFabric is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignTrustFabric_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out SovereignTrustFabric_T)
     with Post => Self.Initialized = True;

   procedure verifyShardTrust (Self : in out SovereignTrustFabric_T)
     with Post => Self.Initialized = True;

   procedure addTrustedNode (Self : in out SovereignTrustFabric_T)
     with Post => Self.Initialized = True;

   procedure trust_init (Self : in out SovereignTrustFabric_T)
     with Post => Self.Initialized = True;

   procedure trust_verify (Self : in out SovereignTrustFabric_T)
     with Post => Self.Initialized = True;

   procedure trust_add_node (Self : in out SovereignTrustFabric_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure addTrustedNode
     with Export, Convention => C, External_Name => "addTrustedNode";

   procedure trust_init
     with Export, Convention => C, External_Name => "trust_init";

   procedure trust_add_node
     with Export, Convention => C, External_Name => "trust_add_node";


end Sigma.SovereignTrustFabric;
