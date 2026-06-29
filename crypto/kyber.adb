-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: Kyber-1024 Body (Ada/SPARK)
-- Implements stub logic — no external libraries, no runtime
-- =========================================================================

with Interfaces; use Interfaces;

package body Kyber is

   procedure Initialize (Ctx : in out Kyber_Context) is
   begin
      -- Zero the key material (sovereign stub)
      Ctx.Public_Key  := (others => 0);
      Ctx.Private_Key := (others => 0);
      Ctx.Initialized := True;
   end Initialize;

   procedure Encapsulate
     (Ctx           : in     Kyber_Context;
      Ciphertext    :    out Byte_Array_64;
      Shared_Secret :    out Byte_Array_32) is
   begin
      -- Stub: produce zeroed ciphertext and shared secret
      Ciphertext    := (others => 0);
      Shared_Secret := (others => 0);
   end Encapsulate;

   procedure Decapsulate
     (Ctx           : in     Kyber_Context;
      Ciphertext    : in     Byte_Array_64;
      Shared_Secret :    out Byte_Array_32) is
      pragma Unreferenced (Ciphertext);
   begin
      -- Stub: produce zeroed shared secret
      Shared_Secret := (others => 0);
   end Decapsulate;

end Kyber;
