--  SigmaOS: SigmaOS Sovereign Privacy Dashboard
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignPrivDash is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   procedure privdash_init
     with Export, Convention => C, External_Name => "privdash_init";

   procedure privdash_list_active_permissions
     with Export, Convention => C, External_Name => "privdash_list_active_permissions";

   procedure privdash_revoke_permission
     with Export, Convention => C, External_Name => "privdash_revoke_permission";


end Sigma.SovereignPrivDash;
