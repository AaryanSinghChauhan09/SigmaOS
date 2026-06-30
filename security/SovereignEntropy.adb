--  SigmaOS: SigmaOS Sovereign Entropy (Neural Entropy Source)
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignEntropy is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignEntropySource_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out SovereignEntropySource_T)
     with Post => Self.Initialized = True;

   procedure harvestEntropy (Self : in out SovereignEntropySource_T)
     with Post => Self.Initialized = True;

   procedure drainPool (Self : in out SovereignEntropySource_T)
     with Post => Self.Initialized = True;

   procedure entropy_init (Self : in out SovereignEntropySource_T)
     with Post => Self.Initialized = True;

   procedure entropy_get (Self : in out SovereignEntropySource_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure drainPool
     with Export, Convention => C, External_Name => "drainPool";

   procedure entropy_init
     with Export, Convention => C, External_Name => "entropy_init";


end Sigma.SovereignEntropy;
