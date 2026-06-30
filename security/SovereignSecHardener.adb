--  SigmaOS: =========================================================================
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignSecHardener is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   procedure sigma_hardened_strcpy
     with Export, Convention => C, External_Name => "sigma_hardened_strcpy";

   procedure sechardener_init
     with Export, Convention => C, External_Name => "sechardener_init";

   procedure sechardener_apply_to_shard
     with Export, Convention => C, External_Name => "sechardener_apply_to_shard";

   procedure sechardener_validate_buffer
     with Export, Convention => C, External_Name => "sechardener_validate_buffer";

   procedure sechardener_audit_all_shards
     with Export, Convention => C, External_Name => "sechardener_audit_all_shards";


end Sigma.SovereignSecHardener;
