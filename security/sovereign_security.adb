-- SPDX-License-Identifier: GPL-2.0-or-later
-- SovereignSecurity body (Ada/SPARK)

with Interfaces; use Interfaces;

package body SovereignSecurity is

   procedure Initialize_Sandbox (Policy : in out Sandbox_Policy; ID : in Sandbox_ID) is
   begin
      Policy.ID          := ID;
      Policy.Allow_Net   := False;
      Policy.Allow_FS    := False;
      Policy.Allow_Exec  := False;
      Policy.Initialized := True;
   end Initialize_Sandbox;

   function Is_Allowed (Policy : in Sandbox_Policy; Resource : in String) return Boolean is
   begin
      if Resource = "net" then return Policy.Allow_Net; end if;
      if Resource = "fs"  then return Policy.Allow_FS;  end if;
      if Resource = "exec" then return Policy.Allow_Exec; end if;
      return False;
   end Is_Allowed;

   procedure Initialize_Audit (Ctx : in out Audit_Context; Level : in Audit_Level) is
   begin
      Ctx.Level       := Level;
      Ctx.Event_Count := 0;
      Ctx.Initialized := True;
   end Initialize_Audit;

   procedure Log_Event (Ctx : in out Audit_Context; Event : in String) is
      pragma Unreferenced (Event);
   begin
      Ctx.Event_Count := Ctx.Event_Count + 1;
   end Log_Event;

   function Check_Access
     (Subject_Label : in MAC_Label;
      Object_Label  : in MAC_Label) return Boolean is
   begin
      if Subject_Label = Kernel_Only then return True; end if;
      if Object_Label = Kernel_Only then return Subject_Label = Trusted; end if;
      return Subject_Label >= Object_Label;
   end Check_Access;

end SovereignSecurity;
