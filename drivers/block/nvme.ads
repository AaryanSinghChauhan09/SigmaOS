-- SPDX-License-Identifier: GPL-2.0-or-later
-- =========================================================================
-- SigmaOS: NVMe Storage Driver (Ada/SPARK)
-- Replaces: drivers/block/nvme_shard.cpp
-- =========================================================================

package NVMe is

   pragma Pure;

   QUEUE_DEPTH : constant := 64;
   SECTOR_SIZE : constant := 512;

   type Sector_Buffer is array (1 .. SECTOR_SIZE) of Interfaces.Unsigned_8;
   type Namespace_ID  is range 1 .. 65535;
   type Queue_ID      is range 0 .. QUEUE_DEPTH - 1;

   type NVMe_Status is (Success, Error, Timeout, Aborted);

   -- Tagged Record: NVMe Submission Queue Entry
   type SQ_Entry is tagged record
      Opcode      : Interfaces.Unsigned_8  := 0;
      Namespace   : Namespace_ID           := 1;
      LBA         : Interfaces.Unsigned_64 := 0;
      Block_Count : Interfaces.Unsigned_16 := 0;
   end record;

   -- Tagged Record: NVMe Controller
   type NVMe_Controller is tagged record
      BAR0        : Interfaces.Unsigned_64 := 0;
      Initialized : Boolean                := False;
      Num_Queues  : Natural                := 0;
   end record;

   procedure Initialize
     (Ctrl : in out NVMe_Controller;
      BAR0 : in     Interfaces.Unsigned_64)
     with Post => Ctrl.Initialized = True;

   procedure Submit_Read
     (Ctrl   : in out NVMe_Controller;
      NSID   : in     Namespace_ID;
      LBA    : in     Interfaces.Unsigned_64;
      Buffer :    out Sector_Buffer;
      Status :    out NVMe_Status)
     with Pre => Ctrl.Initialized;

   procedure Submit_Write
     (Ctrl   : in out NVMe_Controller;
      NSID   : in     Namespace_ID;
      LBA    : in     Interfaces.Unsigned_64;
      Buffer : in     Sector_Buffer;
      Status :    out NVMe_Status)
     with Pre => Ctrl.Initialized;

end NVMe;
