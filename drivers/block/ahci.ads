-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: AHCI Storage Driver (Ada/SPARK)
-- Replaces: drivers/block/ahci_shard.cpp
-- =========================================================================

package AHCI is

   pragma Pure;

   MAX_PORTS   : constant := 32;
   SECTOR_SIZE : constant := 512;

   type Sector_Buffer is array (1 .. SECTOR_SIZE) of Interfaces.Unsigned_8;
   type Port_Index    is range 0 .. MAX_PORTS - 1;

   type Port_State is (Idle, Active, Error, Reset);

   -- Tagged Record: AHCI Port (OOP base)
   type AHCI_Port is tagged record
      Index      : Port_Index    := 0;
      State      : Port_State    := Idle;
      Base_Addr  : Interfaces.Unsigned_64 := 0;
      Sectors    : Interfaces.Unsigned_64 := 0;
   end record;

   -- Tagged Record: AHCI Controller
   type AHCI_Controller is tagged record
      Ports      : array (Port_Index) of AHCI_Port;
      Num_Ports  : Natural := 0;
      Initialized : Boolean := False;
   end record;

   procedure Initialize (Ctrl : in out AHCI_Controller; Base : Interfaces.Unsigned_64)
     with Post => Ctrl.Initialized = True;

   procedure Read_Sector
     (Port   : in out AHCI_Port;
      LBA    : in     Interfaces.Unsigned_64;
      Buffer :    out Sector_Buffer)
     with Pre => Port.State = Idle or Port.State = Active;

   procedure Write_Sector
     (Port   : in out AHCI_Port;
      LBA    : in     Interfaces.Unsigned_64;
      Buffer : in     Sector_Buffer)
     with Pre => Port.State = Idle or Port.State = Active;

   procedure Reset_Port (Port : in out AHCI_Port)
     with Post => Port.State = Idle;

end AHCI;
