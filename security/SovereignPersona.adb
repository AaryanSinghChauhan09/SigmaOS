--  SigmaOS: SigmaOS Sovereign Persona Engine
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignPersona is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   procedure persona_init
     with Export, Convention => C, External_Name => "persona_init";

   procedure persona_set_mode
     with Export, Convention => C, External_Name => "persona_set_mode";

   procedure persona_automate_workflow
     with Export, Convention => C, External_Name => "persona_automate_workflow";


end Sigma.SovereignPersona;
