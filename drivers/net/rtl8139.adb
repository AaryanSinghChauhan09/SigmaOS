-- SPDX-License-Identifier: GPL-2.0-or-later
-- Realtek RTL8139 network driver body (Ada/SPARK)

with Interfaces; use Interfaces;

package body RTL8139 is

   procedure Initialize
     (Dev : in out RTL_Device;
      Port : in Interfaces.Unsigned_16) is
   begin
      Dev.IO_Port     := Port;
      Dev.MAC         := (16#52#, 16#54#, 16#00#, 16#12#, 16#34#, 16#56#);
      Dev.Initialized := True;
   end Initialize;

   procedure Transmit
     (Dev    : in out RTL_Device;
      Packet : in     Packet_Buffer;
      Length : in     Interfaces.Unsigned_32) is
      pragma Unreferenced (Packet, Length);
   begin
      -- Stub: output packet size and address to transmit register
      null;
   end Transmit;

end RTL8139;
