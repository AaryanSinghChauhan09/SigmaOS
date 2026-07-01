-- SPDX-License-Identifier: GPL-2.0-or-later
-- Realtek RTL8139 network driver specification (Ada/SPARK)

package RTL8139 is

   pragma Pure;

   type MAC_Address is array (1 .. 6) of Interfaces.Unsigned_8;
   type Packet_Buffer is array (1 .. 1518) of Interfaces.Unsigned_8;

   type RTL_Device is tagged record
      IO_Port     : Interfaces.Unsigned_16 := 0;
      MAC         : MAC_Address            := (others => 0);
      Initialized : Boolean                := False;
   end record;

   procedure Initialize
     (Dev : in out RTL_Device;
      Port : in Interfaces.Unsigned_16)
     with Post => Dev.Initialized = True;

   procedure Transmit
     (Dev    : in out RTL_Device;
      Packet : in     Packet_Buffer;
      Length : in     Interfaces.Unsigned_32)
     with Pre => Dev.Initialized;

end RTL8139;
