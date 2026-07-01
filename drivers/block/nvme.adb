-- SPDX-License-Identifier: GPL-2.0-or-later
-- NVMe body (Ada/SPARK)

with Interfaces; use Interfaces;

package body NVMe is

   procedure Initialize
     (Ctrl : in out NVMe_Controller;
      BAR0 : in     Interfaces.Unsigned_64) is
   begin
      Ctrl.BAR0        := BAR0;
      Ctrl.Num_Queues  := 1;
      Ctrl.Initialized := True;
   end Initialize;

   procedure Submit_Read
     (Ctrl   : in out NVMe_Controller;
      NSID   : in     Namespace_ID;
      LBA    : in     Interfaces.Unsigned_64;
      Buffer :    out Sector_Buffer;
      Status :    out NVMe_Status) is
      pragma Unreferenced (NSID, LBA);
   begin
      Buffer := (others => 0);
      Status := Success;
   end Submit_Read;

   procedure Submit_Write
     (Ctrl   : in out NVMe_Controller;
      NSID   : in     Namespace_ID;
      LBA    : in     Interfaces.Unsigned_64;
      Buffer : in     Sector_Buffer;
      Status :    out NVMe_Status) is
      pragma Unreferenced (NSID, LBA, Buffer);
   begin
      Status := Success;
   end Submit_Write;

end NVMe;
