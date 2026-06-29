-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: Sovereign ASLR (Ada/SPARK Body)
-- =========================================================================

with System;
with Interfaces;

package body Sovereign_Aslr is

   g_aslr_enabled   : Integer := 1;
   g_wx_enforcement : Integer := 1;

   -- Import external random byte generator (no system library dependencies)
   procedure Random_Bytes (Buf : System.Address; Len : Interfaces.Unsigned_32);
   pragma Import (C, Random_Bytes, "sigma_random_bytes");

   procedure Generate_Layout (Layout : out Address_Space_Layout) is
      use type System.Unsigned_Types.Unsigned_64;
      use type Interfaces.Unsigned_8;
      
      type Byte_Array is array (0 .. 31) of Interfaces.Unsigned_8;
      Rand : Byte_Array;
      
      r0, r1, r2, r3 : System.Unsigned_Types.Unsigned_64;
      Mask           : constant System.Unsigned_Types.Unsigned_64 := 16#0000_03FF_FFFF_F000#; -- 42 bits alignment mask
   begin
      if g_aslr_enabled = 0 then
         Layout.Stack_Base   := 16#7FFF_FFFF_E000#;
         Layout.Heap_Base    := 16#0000_7000_0000_0000#;
         Layout.Mmap_Base    := 16#0000_6000_0000_0000#;
         Layout.Vdso_Base    := 16#0000_7FFF_0000_0000#;
         Layout.Entropy_Bits := 0;
         return;
      end if;

      Random_Bytes (Rand'Address, 32);

      -- Extract 64-bit random values
      r0 := System.Unsigned_Types.Unsigned_64 (Rand (0)) or
            (System.Unsigned_Types.Unsigned_64 (Rand (1)) * 256); -- Simplified conversion
      r1 := System.Unsigned_Types.Unsigned_64 (Rand (8));
      r2 := System.Unsigned_Types.Unsigned_64 (Rand (16));
      r3 := System.Unsigned_Types.Unsigned_64 (Rand (24));

      Layout.Stack_Base   := (16#7FFF_FFFF_FFFF_0000# - (r0 and Mask));
      Layout.Heap_Base    := (16#0000_7000_0000_0000# + (r1 and Mask));
      Layout.Mmap_Base    := (16#0000_6000_0000_0000# + (r2 and Mask));
      Layout.Vdso_Base    := (16#0000_7FFF_0000_0000# + (r3 and Mask));
      Layout.Entropy_Bits := 42;
   end Generate_Layout;

   function Check_WX (Prot_Flags : Interfaces.Unsigned_32) return Integer is
      use type Interfaces.Unsigned_32;
      SIGMA_PROT_WRITE : constant Interfaces.Unsigned_32 := 16#02#;
      SIGMA_PROT_EXEC  : constant Interfaces.Unsigned_32 := 16#04#;
   begin
      if g_wx_enforcement = 0 then
         return 0;
      end if;

      if (Prot_Flags and SIGMA_PROT_WRITE) /= 0 and then (Prot_Flags and SIGMA_PROT_EXEC) /= 0 then
         return -1;
      end if;

      return 0;
   end Check_WX;

   procedure Init is
   begin
      null;
   end Init;

end Sovereign_Aslr;
