--  SigmaOS: SigmaOS Sovereign SEL (Security Enforcement Lattice)
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignSEL is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignSEL_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out SovereignSEL_T)
     with Post => Self.Initialized = True;

   procedure verifyShardTrust (Self : in out SovereignSEL_T)
     with Post => Self.Initialized = True;

   procedure spawnSandbox (Self : in out SovereignSEL_T)
     with Post => Self.Initialized = True;

   procedure enforcePolicy (Self : in out SovereignSEL_T)
     with Post => Self.Initialized = True;

   procedure audit (Self : in out SovereignSEL_T)
     with Post => Self.Initialized = True;

   procedure sel_init_shard (Self : in out SovereignSEL_T)
     with Post => Self.Initialized = True;

   procedure sel_spawn_sandbox (Self : in out SovereignSEL_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure spawnSandbox
     with Export, Convention => C, External_Name => "spawnSandbox";

   procedure enforcePolicy
     with Export, Convention => C, External_Name => "enforcePolicy";

   procedure audit
     with Export, Convention => C, External_Name => "audit";

   procedure sel_init_shard
     with Export, Convention => C, External_Name => "sel_init_shard";

   procedure sel_spawn_sandbox
     with Export, Convention => C, External_Name => "sel_spawn_sandbox";


end Sigma.SovereignSEL;
