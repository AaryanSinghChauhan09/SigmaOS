-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: Kyber-1024 Key Encapsulation (Ada/SPARK)
-- Replaces: crypto/SovereignKyber.cpp
-- Formal contracts enforced via SPARK aspects
-- =========================================================================

package Kyber is

   pragma Pure;

   KEY_SIZE      : constant := 32;
   CIPHERTEXT_SIZE : constant := 64;

   type Byte_Array_32  is array (1 .. KEY_SIZE)       of Interfaces.Unsigned_8;
   type Byte_Array_64  is array (1 .. CIPHERTEXT_SIZE) of Interfaces.Unsigned_8;

   -- Tagged Record (Ada OOP base)
   type Kyber_Context is tagged record
      Public_Key  : Byte_Array_32;
      Private_Key : Byte_Array_32;
      Initialized : Boolean := False;
   end record;

   -- Initialize key material (stub: zeroed keys)
   procedure Initialize (Ctx : in out Kyber_Context)
     with Post => Ctx.Initialized = True;

   -- Encapsulate: produce ciphertext + shared secret
   procedure Encapsulate
     (Ctx         : in     Kyber_Context;
      Ciphertext  :    out Byte_Array_64;
      Shared_Secret :  out Byte_Array_32)
     with Pre => Ctx.Initialized;

   -- Decapsulate: recover shared secret from ciphertext
   procedure Decapsulate
     (Ctx          : in     Kyber_Context;
      Ciphertext   : in     Byte_Array_64;
      Shared_Secret :   out Byte_Array_32)
     with Pre => Ctx.Initialized;

end Kyber;
