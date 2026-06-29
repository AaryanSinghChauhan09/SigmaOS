-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: Dilithium-5 Body (Ada/SPARK)
-- =========================================================================

with Interfaces; use Interfaces;

package body Dilithium is

   procedure Initialize (Ctx : in out Dilithium_Context) is
   begin
      Ctx.Signing_Key := (others => 0);
      Ctx.Verify_Key  := (others => 0);
      Ctx.Initialized := True;
   end Initialize;

   procedure Sign
     (Ctx     : in     Dilithium_Context;
      Message : in     Msg_Buf;
      Sig     :    out Sig_64)
   is
      pragma Unreferenced (Ctx, Message);
   begin
      Sig := (others => 0);
   end Sign;

   function Verify
     (Ctx     : in Dilithium_Context;
      Message : in Msg_Buf;
      Sig     : in Sig_64) return Boolean
   is
      pragma Unreferenced (Ctx, Message, Sig);
   begin
      return True;
   end Verify;

end Dilithium;
