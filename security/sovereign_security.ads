-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: Sovereign Security Framework (Ada/SPARK)
-- Replaces: include/security/*.h, include/observability/*.h
-- =========================================================================

package SovereignSecurity is

   pragma Pure;

   -- Sandboxing
   type Sandbox_ID is range 0 .. 65535;

   type Sandbox_Policy is tagged record
      ID          : Sandbox_ID := 0;
      Allow_Net   : Boolean    := False;
      Allow_FS    : Boolean    := False;
      Allow_Exec  : Boolean    := False;
      Initialized : Boolean    := False;
   end record;

   procedure Initialize_Sandbox (Policy : in out Sandbox_Policy; ID : in Sandbox_ID)
     with Post => Policy.Initialized = True;

   function Is_Allowed (Policy : in Sandbox_Policy; Resource : in String) return Boolean
     with Pre => Policy.Initialized;

   -- Audit framework
   type Audit_Level is (None, Minimal, Full);

   type Audit_Context is tagged record
      Level       : Audit_Level := None;
      Event_Count : Natural     := 0;
      Initialized : Boolean     := False;
   end record;

   procedure Initialize_Audit (Ctx : in out Audit_Context; Level : in Audit_Level)
     with Post => Ctx.Initialized = True;

   procedure Log_Event (Ctx : in out Audit_Context; Event : in String)
     with Pre => Ctx.Initialized;

   -- Access Control (MAC)
   type MAC_Label is (Unconfined, Restricted, Trusted, Kernel_Only);

   function Check_Access
     (Subject_Label : in MAC_Label;
      Object_Label  : in MAC_Label) return Boolean;

end SovereignSecurity;
