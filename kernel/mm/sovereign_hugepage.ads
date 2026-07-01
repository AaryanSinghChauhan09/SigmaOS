-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: Sovereign Hugepage Management (Ada/SPARK Spec)
-- Replaces: kernel/mm/sigma_hugepage.h
-- =========================================================================

package Sovereign_Hugepage is

   pragma Pure;

   type Hugepage_Status is (Success, Error_OOM, Error_Alignment);

   procedure Allocate_Hugepage
     (Phys_Addr : in     System.Unsigned_Types.Unsigned_64;
      Virt_Addr : in     System.Unsigned_Types.Unsigned_64;
      Status    :    out Hugepage_Status)
     with Pre => Phys_Addr mod 2097152 = 0 and Virt_Addr mod 2097152 = 0; -- 2MB alignment requirement

end Sovereign_Hugepage;
