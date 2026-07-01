--  SigmaOS: SovereignIDS.cpp
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignIDS is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type IDSAction_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out IDSAction_T)
     with Post => Self.Initialized = True;

   procedure addRule (Self : in out IDSAction_T)
     with Post => Self.Initialized = True;

   procedure printAudit (Self : in out IDSAction_T)
     with Post => Self.Initialized = True;

   procedure recordEvent (Self : in out IDSAction_T)
     with Post => Self.Initialized = True;

   procedure ids_init (Self : in out IDSAction_T)
     with Post => Self.Initialized = True;

   procedure ids_inspect (Self : in out IDSAction_T)
     with Post => Self.Initialized = True;

   procedure ids_audit (Self : in out IDSAction_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure printAudit
     with Export, Convention => C, External_Name => "printAudit";

   procedure recordEvent
     with Export, Convention => C, External_Name => "recordEvent";

   procedure ids_init
     with Export, Convention => C, External_Name => "ids_init";

   procedure ids_audit
     with Export, Convention => C, External_Name => "ids_audit";


end Sigma.SovereignIDS;
