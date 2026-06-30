--  SigmaOS: SovereignAdaptivePrivacy.cpp
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignAdaptivePrivacy is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type RoutingMode_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out RoutingMode_T)
     with Post => Self.Initialized = True;

   procedure registerProfile (Self : in out RoutingMode_T)
     with Post => Self.Initialized = True;

   procedure bindDomain (Self : in out RoutingMode_T)
     with Post => Self.Initialized = True;

   procedure setThreatLevel (Self : in out RoutingMode_T)
     with Post => Self.Initialized = True;

   procedure printStatus (Self : in out RoutingMode_T)
     with Post => Self.Initialized = True;

   procedure privacy_init (Self : in out RoutingMode_T)
     with Post => Self.Initialized = True;

   procedure privacy_set_threat (Self : in out RoutingMode_T)
     with Post => Self.Initialized = True;

   procedure privacy_resolve (Self : in out RoutingMode_T)
     with Post => Self.Initialized = True;

   procedure privacy_status (Self : in out RoutingMode_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure setThreatLevel
     with Export, Convention => C, External_Name => "setThreatLevel";

   procedure printStatus
     with Export, Convention => C, External_Name => "printStatus";

   procedure privacy_init
     with Export, Convention => C, External_Name => "privacy_init";

   procedure privacy_set_threat
     with Export, Convention => C, External_Name => "privacy_set_threat";

   procedure privacy_status
     with Export, Convention => C, External_Name => "privacy_status";


end Sigma.SovereignAdaptivePrivacy;
