-- SPDX-License-Identifier: MIT
-- Copyright (c) 2024-2026 SigmaOS Project
-- kernel/memory/sigma_hugepages.adb — Huge Page Manager body (SPARK/Ada)

package body Sigma.Hugepages
  with SPARK_Mode => On
is

   procedure Init_2MB(Pool  : out Huge2MB_Pool;
                      Base  : in  Phys_Addr;
                      Count : in  Huge2MB_Count) is
   begin
      Pool.Count := Count;
      Pool.Free  := Count;
      for I in 1 .. Count loop
         Pool.Pages(I) :=
           Phys_Addr(Unsigned_64(Base) + Unsigned_64(I - 1) * Unsigned_64(PAGE_2MB));
      end loop;
   end Init_2MB;

   procedure Init_1GB(Pool  : out Huge1GB_Pool;
                      Base  : in  Phys_Addr;
                      Count : in  Huge1GB_Count) is
   begin
      Pool.Count := Count;
      Pool.Free  := Count;
      for I in 1 .. Count loop
         Pool.Pages(I) :=
           Phys_Addr(Unsigned_64(Base) + Unsigned_64(I - 1) * Unsigned_64(PAGE_1GB));
      end loop;
   end Init_1GB;

   function Alloc_2MB(Pool : in out Huge2MB_Pool) return Phys_Addr is
   begin
      if Pool.Free = 0 then return 0; end if;
      declare
         Addr : constant Phys_Addr := Pool.Pages(Pool.Free);
      begin
         Pool.Pages(Pool.Free) := 0;
         Pool.Free := Pool.Free - 1;
         return Addr;
      end;
   end Alloc_2MB;

   function Alloc_1GB(Pool : in out Huge1GB_Pool) return Phys_Addr is
   begin
      if Pool.Free = 0 then return 0; end if;
      declare
         Addr : constant Phys_Addr := Pool.Pages(Pool.Free);
      begin
         Pool.Pages(Pool.Free) := 0;
         Pool.Free := Pool.Free - 1;
         return Addr;
      end;
   end Alloc_1GB;

   procedure Free_2MB(Pool : in out Huge2MB_Pool; Addr : in Phys_Addr) is
   begin
      if Pool.Free < Pool.Count then
         Pool.Free := Pool.Free + 1;
         Pool.Pages(Pool.Free) := Addr;
      end if;
   end Free_2MB;

   procedure Free_1GB(Pool : in out Huge1GB_Pool; Addr : in Phys_Addr) is
   begin
      if Pool.Free < Pool.Count then
         Pool.Free := Pool.Free + 1;
         Pool.Pages(Pool.Free) := Addr;
      end if;
   end Free_1GB;

   function Available_2MB(Pool : Huge2MB_Pool) return Huge2MB_Count is (Pool.Free);
   function Available_1GB(Pool : Huge1GB_Pool) return Huge1GB_Count is (Pool.Free);

   procedure Map_2MB(VA : in Virt_Addr; PA : in Phys_Addr; Exec : in Boolean) is
      -- x86-64: PML4[va>>39] → PDPT[va>>30] → PD[va>>21] = huge page entry
      -- bit 7 = PS (page size = 2MB), bit 63 = NX if not executable
      PS_BIT  : constant Unsigned_64 := 16#80#;
      NX_BIT  : constant Unsigned_64 := Shift_Left(1, 63);
      P_RW_US : constant Unsigned_64 := 16#07#; -- present + r/w + user
      Flags   : Unsigned_64 := Unsigned_64(PA) or P_RW_US or PS_BIT;
      PD_Idx  : constant Natural :=
        Natural(Shift_Right(Unsigned_64(VA), 21) and 16#1FF#);
   begin
      if not Exec then Flags := Flags or NX_BIT; end if;
      -- NOTE: actual page-directory update requires CR3 walk
      -- This is the logical spec; physical write handled by sigma_vmm.zig
      null; -- TODO: call into Zig VMM via extern linkage
   end Map_2MB;

end Sigma.Hugepages;
