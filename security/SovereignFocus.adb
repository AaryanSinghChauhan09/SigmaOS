--  SigmaOS: SigmaOS Sovereign Focus (S-Focus Shard)
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignFocus is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignFocus_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out SovereignFocus_T)
     with Post => Self.Initialized = True;

   procedure activateFocusLock (Self : in out SovereignFocus_T)
     with Post => Self.Initialized = True;

   procedure deactivateFocusLock (Self : in out SovereignFocus_T)
     with Post => Self.Initialized = True;

   procedure focus_init (Self : in out SovereignFocus_T)
     with Post => Self.Initialized = True;

   procedure focus_activate (Self : in out SovereignFocus_T)
     with Post => Self.Initialized = True;

   procedure focus_deactivate (Self : in out SovereignFocus_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure activateFocusLock
     with Export, Convention => C, External_Name => "activateFocusLock";

   procedure deactivateFocusLock
     with Export, Convention => C, External_Name => "deactivateFocusLock";

   procedure focus_init
     with Export, Convention => C, External_Name => "focus_init";

   procedure focus_activate
     with Export, Convention => C, External_Name => "focus_activate";

   procedure focus_deactivate
     with Export, Convention => C, External_Name => "focus_deactivate";


end Sigma.SovereignFocus;
