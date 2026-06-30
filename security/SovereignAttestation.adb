--  SigmaOS: Sigma Sovereign Hardware Attestation Shard
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignAttestation is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type AttestationState_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure initialize (Self : in out AttestationState_T)
     with Post => Self.Initialized = True;

   procedure verifyIntegrity (Self : in out AttestationState_T)
     with Post => Self.Initialized = True;

   procedure probeHardware (Self : in out AttestationState_T)
     with Post => Self.Initialized = True;

   procedure sigma_attestation_init (Self : in out AttestationState_T)
     with Post => Self.Initialized = True;

   procedure sigma_attestation_verify (Self : in out AttestationState_T)
     with Post => Self.Initialized = True;

   procedure initialize
     with Export, Convention => C, External_Name => "initialize";

   procedure probeHardware
     with Export, Convention => C, External_Name => "probeHardware";

   procedure sigma_attestation_init
     with Export, Convention => C, External_Name => "sigma_attestation_init";


end Sigma.SovereignAttestation;
