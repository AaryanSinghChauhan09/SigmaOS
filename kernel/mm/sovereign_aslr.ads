-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: Sovereign ASLR (Ada/SPARK)
-- Replaces: kernel/mm/sigma_aslr.cpp and kernel/mm/sigma_aslr.h
-- =========================================================================

package Sovereign_Aslr is

   pragma Pure;

   type Address_Space_Layout is record
      Stack_Base   : System.Unsigned_Types.Unsigned_64;
      Heap_Base    : System.Unsigned_Types.Unsigned_64;
      Mmap_Base    : System.Unsigned_Types.Unsigned_64;
      Vdso_Base    : System.Unsigned_Types.Unsigned_64;
      Entropy_Bits : Interfaces.Unsigned_8;
   end record;

   procedure Generate_Layout (Layout : out Address_Space_Layout);

   function Check_WX (Prot_Flags : Interfaces.Unsigned_32) return Integer;

   procedure Init;

end Sovereign_Aslr;
