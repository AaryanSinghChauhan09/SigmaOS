-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: Dilithium-5 Digital Signatures (Ada/SPARK)
-- Replaces: crypto/SovereignDilithium5.cpp
-- =========================================================================

package Dilithium is

   pragma Pure;

   SIGNING_KEY_SIZE   : constant := 64;
   SIGNATURE_SIZE     : constant := 64;
   MESSAGE_MAX        : constant := 256;

   type Byte_64  is array (1 .. SIGNING_KEY_SIZE) of Interfaces.Unsigned_8;
   type Sig_64   is array (1 .. SIGNATURE_SIZE)   of Interfaces.Unsigned_8;
   type Msg_Buf  is array (1 .. MESSAGE_MAX)       of Interfaces.Unsigned_8;

   -- Tagged Record (Ada OOP base)
   type Dilithium_Context is tagged record
      Signing_Key : Byte_64;
      Verify_Key  : Byte_64;
      Initialized : Boolean := False;
   end record;

   procedure Initialize (Ctx : in out Dilithium_Context)
     with Post => Ctx.Initialized = True;

   procedure Sign
     (Ctx     : in     Dilithium_Context;
      Message : in     Msg_Buf;
      Sig     :    out Sig_64)
     with Pre => Ctx.Initialized;

   function Verify
     (Ctx     : in Dilithium_Context;
      Message : in Msg_Buf;
      Sig     : in Sig_64) return Boolean
     with Pre => Ctx.Initialized;

end Dilithium;
