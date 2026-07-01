pragma SPARK_Mode (On);

package body Sigma.Audit is

   function Has_Capability (Context : Security_Context; Cap : Capability_T) return SigmaBool is
   begin
      return Context.Privileges (Cap);
   end Has_Capability;

   procedure Verify_Access (
      Context   : in  Security_Context;
      Cap       : in  Capability_T;
      Event_ID  : in  SigmaU64;
      Severity  : in  Severity_T;
      Record_Out : out Audit_Record
   ) is
      Allowed : constant SigmaBool := Has_Capability (Context, Cap);
   begin
      Record_Out := Audit_Record'(
         Event_ID   => Event_ID,
         Subject_ID => Context.Subject_ID,
         Severity   => Severity,
         Success    => Allowed,
         Verified   => SigmaBool (True)
      );
   end Verify_Access;

   procedure Log_Audit_Event (
      Logger    : in out Audit_Logger;
      Item      : in     Audit_Record
   ) is
   begin
      Logger.Buffer (Logger.Head) := Item;
      if Logger.Head = Buffer_Size then
         Logger.Head := 1;
      else
         Logger.Head := Logger.Head + 1;
      end if;

      if Logger.Count < Buffer_Size then
         Logger.Count := Logger.Count + 1;
      end if;
   end Log_Audit_Event;

end Sigma.Audit;
