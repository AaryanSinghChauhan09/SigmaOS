-- SPDX-License-Identifier: MIT
-- Copyright (c) 2024-2026 SigmaOS Project
-- kernel/memory/sigma_hugepages.ads — Huge Page Manager (SPARK/Ada spec)
-- 2MB and 1GB huge pages with formal contracts

with Interfaces; use Interfaces;

package Sigma.Hugepages
  with SPARK_Mode => On
is
   PAGE_2MB : constant := 2 * 1024 * 1024;
   PAGE_1GB : constant := 1024 * 1024 * 1024;

   type Page_Size is (Size_4KB, Size_2MB, Size_1GB);
   type Phys_Addr is new Unsigned_64;
   type Virt_Addr is new Unsigned_64;

   subtype Huge2MB_Count is Natural range 0 .. 512;
   subtype Huge1GB_Count is Natural range 0 .. 4;

   -- Pool of huge pages
   type Huge2MB_Pool is private;
   type Huge1GB_Pool is private;

   -- Initialise pools from available physical memory
   procedure Init_2MB(Pool     : out Huge2MB_Pool;
                      Base     : in  Phys_Addr;
                      Count    : in  Huge2MB_Count)
   with
     SPARK_Mode => On,
     Pre  => Count > 0 and Count <= 512,
     Post => Available_2MB(Pool) = Count;

   procedure Init_1GB(Pool     : out Huge1GB_Pool;
                      Base     : in  Phys_Addr;
                      Count    : in  Huge1GB_Count)
   with
     SPARK_Mode => On,
     Pre  => Count > 0 and Count <= 4,
     Post => Available_1GB(Pool) = Count;

   -- Allocate one huge page; returns 0 on failure
   function Alloc_2MB(Pool : in out Huge2MB_Pool) return Phys_Addr
   with
     SPARK_Mode => On,
     Post => (if Alloc_2MB'Result /= 0 then
                Available_2MB(Pool) = Available_2MB(Pool)'Old - 1);

   function Alloc_1GB(Pool : in out Huge1GB_Pool) return Phys_Addr
   with
     SPARK_Mode => On,
     Post => (if Alloc_1GB'Result /= 0 then
                Available_1GB(Pool) = Available_1GB(Pool)'Old - 1);

   -- Free a huge page
   procedure Free_2MB(Pool : in out Huge2MB_Pool; Addr : in Phys_Addr)
   with
     SPARK_Mode => On,
     Pre  => Addr /= 0 and (Unsigned_64(Addr) mod Unsigned_64(PAGE_2MB)) = 0,
     Post => Available_2MB(Pool) <= Available_2MB(Pool)'Old + 1;

   procedure Free_1GB(Pool : in out Huge1GB_Pool; Addr : in Phys_Addr)
   with
     SPARK_Mode => On,
     Pre  => Addr /= 0 and (Unsigned_64(Addr) mod Unsigned_64(PAGE_1GB)) = 0,
     Post => Available_1GB(Pool) <= Available_1GB(Pool)'Old + 1;

   function Available_2MB(Pool : Huge2MB_Pool) return Huge2MB_Count;
   function Available_1GB(Pool : Huge1GB_Pool) return Huge1GB_Count;

   -- Map a virtual address to a 2MB huge page
   procedure Map_2MB(VA   : in Virt_Addr;
                     PA   : in Phys_Addr;
                     Exec : in Boolean)
   with
     SPARK_Mode => On,
     Pre  => PA /= 0
         and (Unsigned_64(VA) mod Unsigned_64(PAGE_2MB)) = 0
         and (Unsigned_64(PA) mod Unsigned_64(PAGE_2MB)) = 0;

private
   type Phys_Array_2MB is array (Huge2MB_Count range <>) of Phys_Addr;
   type Huge2MB_Pool is record
      Pages  : Phys_Array_2MB (1 .. 512);
      Count  : Huge2MB_Count := 0;
      Free   : Huge2MB_Count := 0;
   end record;

   type Phys_Array_1GB is array (Huge1GB_Count range <>) of Phys_Addr;
   type Huge1GB_Pool is record
      Pages  : Phys_Array_1GB (1 .. 4);
      Count  : Huge1GB_Count := 0;
      Free   : Huge1GB_Count := 0;
   end record;

end Sigma.Hugepages;
