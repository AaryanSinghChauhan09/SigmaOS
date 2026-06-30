--  SigmaOS: SovereignTPM " Measured Boot & TPM Shard
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignTPM is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignTPMEngine_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out SovereignTPMEngine_T)
     with Post => Self.Initialized = True;

   procedure pcr_extend (Self : in out SovereignTPMEngine_T)
     with Post => Self.Initialized = True;

   procedure verifyLatticeIntegrity (Self : in out SovereignTPMEngine_T)
     with Post => Self.Initialized = True;

   procedure sigma_tpm_init (Self : in out SovereignTPMEngine_T)
     with Post => Self.Initialized = True;

   procedure sigma_tpm_verify (Self : in out SovereignTPMEngine_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure pcr_extend
     with Export, Convention => C, External_Name => "pcr_extend";

   procedure sigma_tpm_init
     with Export, Convention => C, External_Name => "sigma_tpm_init";


end Sigma.SovereignTPM;
