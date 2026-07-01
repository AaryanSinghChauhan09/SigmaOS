--  SigmaOS: SigmaOS Sovereign RBAC (Role-Based Access Control)
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignRBAC is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignRBAC_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out SovereignRBAC_T)
     with Post => Self.Initialized = True;

   procedure checkPermission (Self : in out SovereignRBAC_T)
     with Post => Self.Initialized = True;

   procedure rbac_init (Self : in out SovereignRBAC_T)
     with Post => Self.Initialized = True;

   procedure rbac_check (Self : in out SovereignRBAC_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure rbac_init
     with Export, Convention => C, External_Name => "rbac_init";


end Sigma.SovereignRBAC;
