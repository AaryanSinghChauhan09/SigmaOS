--  SigmaOS: =========================================================================
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignWhonixTor is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignWhonixTor_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure forceTorRouting (Self : in out SovereignWhonixTor_T)
     with Post => Self.Initialized = True;

   procedure privacy_enforce_tor (Self : in out SovereignWhonixTor_T)
     with Post => Self.Initialized = True;

   procedure forceTorRouting
     with Export, Convention => C, External_Name => "forceTorRouting";

   procedure privacy_enforce_tor
     with Export, Convention => C, External_Name => "privacy_enforce_tor";


end Sigma.SovereignWhonixTor;
