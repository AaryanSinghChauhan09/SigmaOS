--  SigmaOS: SigmaOS Sovereign DID (Decentralized Identifier)
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.SovereignDID is

   type SigmaU8  is mod 2**8;
   type SigmaU16 is mod 2**16;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaI32 is range -(2**31) .. (2**31 - 1);
   type SigmaI64 is range -(2**63) .. (2**63 - 1);
   type SigmaBool is new Boolean;

   type SovereignDIDManager_T is tagged record
      Initialized : SigmaBool := False;
   end record;

   procedure init (Self : in out SovereignDIDManager_T)
     with Post => Self.Initialized = True;

   procedure createDID (Self : in out SovereignDIDManager_T)
     with Post => Self.Initialized = True;

   procedure verifyDID (Self : in out SovereignDIDManager_T)
     with Post => Self.Initialized = True;

   procedure did_init (Self : in out SovereignDIDManager_T)
     with Post => Self.Initialized = True;

   procedure did_create (Self : in out SovereignDIDManager_T)
     with Post => Self.Initialized = True;

   procedure did_verify (Self : in out SovereignDIDManager_T)
     with Post => Self.Initialized = True;

   procedure init
     with Export, Convention => C, External_Name => "init";

   procedure createDID
     with Export, Convention => C, External_Name => "createDID";

   procedure did_init
     with Export, Convention => C, External_Name => "did_init";

   procedure did_create
     with Export, Convention => C, External_Name => "did_create";


end Sigma.SovereignDID;
