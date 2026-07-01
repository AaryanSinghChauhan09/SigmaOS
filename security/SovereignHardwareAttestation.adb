--  SigmaOS: SovereignHardwareAttestation.cpp
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignHardwareAttestation is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type AttestStage_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure fnv1a_hash (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure init (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure recordMeasurement (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure verifyBootChain (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure generateQuote (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure printStatus (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure attest_init (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure attest_record (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure attest_verify_boot (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure attest_status (Self : in out AttestStage_T)
     with Post => Self.Initialized = True;

   procedure fnv1a_hash
     with Export, Convention => C, External_Name => "fnv1a_hash";

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure printStatus
     with Export, Convention => C, External_Name => "printStatus";

   procedure attest_init
     with Export, Convention => C, External_Name => "attest_init";

   procedure attest_status
     with Export, Convention => C, External_Name => "attest_status";


end Sigma.SovereignHardwareAttestation;
