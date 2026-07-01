--  SigmaOS: SigmaOS Sovereign Threat Hunter Shard
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignThreatHunter is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignThreatHunter_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out SovereignThreatHunter_T)
     with Post => Self.Initialized = True;

   procedure executeAudit (Self : in out SovereignThreatHunter_T)
     with Post => Self.Initialized = True;

   procedure audit (Self : in out SovereignThreatHunter_T)
     with Post => Self.Initialized = True;

   procedure threat_hunter_init (Self : in out SovereignThreatHunter_T)
     with Post => Self.Initialized = True;

   procedure threat_hunter_audit (Self : in out SovereignThreatHunter_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure executeAudit
     with Export, Convention => C, External_Name => "executeAudit";

   procedure audit
     with Export, Convention => C, External_Name => "audit";

   procedure threat_hunter_init
     with Export, Convention => C, External_Name => "threat_hunter_init";

   procedure threat_hunter_audit
     with Export, Convention => C, External_Name => "threat_hunter_audit";


end Sigma.SovereignThreatHunter;
