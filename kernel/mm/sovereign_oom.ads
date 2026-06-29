-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: Out of Memory (OOM) Protection Layer (Ada/SPARK Spec)
-- Replaces: kernel/mm/sigma_oom.h
-- =========================================================================

package Sovereign_Oom is

   pragma Pure;

   type System_Resources is record
      Total_Memory_Bytes : System.Unsigned_Types.Unsigned_64;
      Free_Memory_Bytes  : System.Unsigned_Types.Unsigned_64;
   end record;

   procedure Check_Memory_Level
     (Resources  : in     System_Resources;
      Kill_Target_Pid :    out Integer;
      Oom_Triggered   :    out Boolean)
     with Post => (if Oom_Triggered then Kill_Target_Pid /= 0);

end Sovereign_Oom;
