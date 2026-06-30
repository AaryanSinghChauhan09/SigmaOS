--  SigmaOS: SovereignVault module
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignVault is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignVault_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure lock (Self : in out SovereignVault_T)
     with Post => Self.Initialized = True;

   procedure unlock (Self : in out SovereignVault_T)
     with Post => Self.Initialized = True;

   procedure lock
     with Export, Convention => C, External_Name => "lock";

   procedure unlock
     with Export, Convention => C, External_Name => "unlock";


end Sigma.SovereignVault;
