--  SigmaOS: Sovereign Security and Auditing Module
--  Migrated from C/C++ to Ada/SPARK — no runtime, no external packages.
--  All types hand-defined. OOP via tagged types + dispatching.
--  SPARK_Mode: On — formal verification enabled.

pragma SPARK_Mode (On);

package Sigma.Audit is

   type SigmaU8 is mod 2**8;
   type SigmaU32 is mod 2**32;
   type SigmaU64 is mod 2**64;
   type SigmaBool is new Boolean;

   type Capability_T is (
      CAP_SYS_ADMIN,
      CAP_NET_ADMIN,
      CAP_FS_WRITE,
      CAP_AUDIT_LOG
   );

   type Privilege_Set is array (Capability_T) of SigmaBool;

   type Security_Context is record
      Subject_ID  : SigmaU32;
      Privileges  : Privilege_Set;
      Hardened    : SigmaBool;
   end record;

   type Audit_Record is record
      Event_ID   : SigmaU64;
      Subject_ID : SigmaU32;
      Success    : SigmaBool;
      Verified   : SigmaBool;
   end record;

   --  Verifies if a subject context possesses a given capability.
   --  Formal Contract: Returns True only if the capability is enabled in the subject's privileges.
   function Has_Capability (Context : Security_Context; Cap : Capability_T) return SigmaBool
     with Post => (if Has_Capability'Result = SigmaBool (True) then Context.Privileges (Cap) = SigmaBool (True));

   --  Performs an access control check and generates an audited log record.
   --  Formal Contract: audit record success matches capability status.
   procedure Verify_Access (
      Context   : in  Security_Context;
      Cap       : in  Capability_T;
      Event_ID  : in  SigmaU64;
      Record_Out : out Audit_Record
   )
     with Post => Record_Out.Success = Has_Capability (Context, Cap) and
                  Record_Out.Verified = SigmaBool (True) and
                  Record_Out.Subject_ID = Context.Subject_ID;

end Sigma.Audit;
