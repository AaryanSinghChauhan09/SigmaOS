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
      Record_Out : out Audit_Record
   ) is
      Allowed : constant SigmaBool := Has_Capability (Context, Cap);
   begin
      Record_Out := Audit_Record'(
         Event_ID   => Event_ID,
         Subject_ID => Context.Subject_ID,
         Success    => Allowed,
         Verified   => SigmaBool (True)
      );
   end Verify_Access;

end Sigma.Audit;
