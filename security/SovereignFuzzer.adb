--  SigmaOS: SigmaOS Sovereign Fuzzer Shard
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignFuzzer is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignFuzzer_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out SovereignFuzzer_T)
     with Post => Self.Initialized = True;

   procedure injectFault (Self : in out SovereignFuzzer_T)
     with Post => Self.Initialized = True;

   procedure fuzzPQCDilithium (Self : in out SovereignFuzzer_T)
     with Post => Self.Initialized = True;

   procedure audit (Self : in out SovereignFuzzer_T)
     with Post => Self.Initialized = True;

   procedure fuzzer_init (Self : in out SovereignFuzzer_T)
     with Post => Self.Initialized = True;

   procedure fuzzer_inject (Self : in out SovereignFuzzer_T)
     with Post => Self.Initialized = True;

   procedure fuzzer_test_pqc (Self : in out SovereignFuzzer_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure injectFault
     with Export, Convention => C, External_Name => "injectFault";

   procedure fuzzPQCDilithium
     with Export, Convention => C, External_Name => "fuzzPQCDilithium";

   procedure audit
     with Export, Convention => C, External_Name => "audit";

   procedure fuzzer_init
     with Export, Convention => C, External_Name => "fuzzer_init";

   procedure fuzzer_inject
     with Export, Convention => C, External_Name => "fuzzer_inject";

   procedure fuzzer_test_pqc
     with Export, Convention => C, External_Name => "fuzzer_test_pqc";


end Sigma.SovereignFuzzer;
