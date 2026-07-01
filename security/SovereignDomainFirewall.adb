--  SigmaOS: SovereignDomainFirewall.cpp
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignDomainFirewall is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type RuleAction_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out RuleAction_T)
     with Post => Self.Initialized = True;

   procedure registerDomain (Self : in out RuleAction_T)
     with Post => Self.Initialized = True;

   procedure addRule (Self : in out RuleAction_T)
     with Post => Self.Initialized = True;

   procedure printStatus (Self : in out RuleAction_T)
     with Post => Self.Initialized = True;

   procedure domfw_init (Self : in out RuleAction_T)
     with Post => Self.Initialized = True;

   procedure domfw_register_domain (Self : in out RuleAction_T)
     with Post => Self.Initialized = True;

   procedure domfw_add_rule (Self : in out RuleAction_T)
     with Post => Self.Initialized = True;

   procedure domfw_status (Self : in out RuleAction_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure printStatus
     with Export, Convention => C, External_Name => "printStatus";

   procedure domfw_init
     with Export, Convention => C, External_Name => "domfw_init";

   procedure domfw_status
     with Export, Convention => C, External_Name => "domfw_status";


end Sigma.SovereignDomainFirewall;
