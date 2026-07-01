-- SPDX-License-Identifier: GPL-2.0-or-later
-- Intel E1000 network driver body (Ada/SPARK)

with Interfaces; use Interfaces;

package body E1000 is

   procedure Initialize
     (Dev : in out E1000_Device;
      IO_Base : in Interfaces.Unsigned_64) is
   begin
      Dev.IO_Base     := IO_Base;
      Dev.MAC         := (16#00#, 16#1B#, 16#21#, 16#3C#, 16#4D#, 16#5E#);
      Dev.Link_Up     := True;
      Dev.Initialized := True;
   end Initialize;

   procedure Transmit
     (Dev    : in out E1000_Device;
      Packet : in     Packet_Buffer;
      Length : in     Interfaces.Unsigned_32) is
      pragma Unreferenced (Packet, Length);
   begin
      -- Stub: write packet info to transmit ring descriptor
      null;
   end Transmit;

   procedure Receive
     (Dev    : in out E1000_Device;
      Packet :    out Packet_Buffer;
      Length :    out Interfaces.Unsigned_32) is
      pragma Unreferenced (Dev);
   begin
      Packet := (others => 0);
      Length := 0;
   end Receive;

end E1000;
