--  SigmaOS: SovereignComplianceAuditor.cpp
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignComplianceAuditor is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type Framework_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out Framework_T)
     with Post => Self.Initialized = True;

   procedure addCheck (Self : in out Framework_T)
     with Post => Self.Initialized = True;

   procedure runFullAudit (Self : in out Framework_T)
     with Post => Self.Initialized = True;

   procedure printReport (Self : in out Framework_T)
     with Post => Self.Initialized = True;

   procedure registerBuiltinChecks (Self : in out Framework_T)
     with Post => Self.Initialized = True;

   procedure compliance_init (Self : in out Framework_T)
     with Post => Self.Initialized = True;

   procedure compliance_audit_full (Self : in out Framework_T)
     with Post => Self.Initialized = True;

   procedure compliance_report (Self : in out Framework_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure runFullAudit
     with Export, Convention => C, External_Name => "runFullAudit";

   procedure printReport
     with Export, Convention => C, External_Name => "printReport";

   procedure registerBuiltinChecks
     with Export, Convention => C, External_Name => "registerBuiltinChecks";

   procedure compliance_init
     with Export, Convention => C, External_Name => "compliance_init";

   procedure compliance_audit_full
     with Export, Convention => C, External_Name => "compliance_audit_full";

   procedure compliance_report
     with Export, Convention => C, External_Name => "compliance_report";


end Sigma.SovereignComplianceAuditor;
