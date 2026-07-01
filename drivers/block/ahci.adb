-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: AHCI Storage Driver Body (Ada/SPARK)
-- =========================================================================

with Interfaces; use Interfaces;

package body AHCI is

   procedure Initialize (Ctrl : in out AHCI_Controller; Base : Interfaces.Unsigned_64) is
   begin
      Ctrl.Num_Ports  := 0;
      Ctrl.Initialized := True;
      -- Stub: in real impl, probe AHCI BAR registers at Base
      pragma Unreferenced (Base);
   end Initialize;

   procedure Read_Sector
     (Port   : in out AHCI_Port;
      LBA    : in     Interfaces.Unsigned_64;
      Buffer :    out Sector_Buffer) is
      pragma Unreferenced (LBA);
   begin
      Port.State := Active;
      Buffer := (others => 0);
      Port.State := Idle;
   end Read_Sector;

   procedure Write_Sector
     (Port   : in out AHCI_Port;
      LBA    : in     Interfaces.Unsigned_64;
      Buffer : in     Sector_Buffer) is
      pragma Unreferenced (LBA, Buffer);
   begin
      Port.State := Active;
      -- Stub: DMA write to device
      Port.State := Idle;
   end Write_Sector;

   procedure Reset_Port (Port : in out AHCI_Port) is
   begin
      Port.State := Reset;
      Port.State := Idle;
   end Reset_Port;

end AHCI;
