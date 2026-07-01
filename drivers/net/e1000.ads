-- SPDX-License-Identifier: GPL-2.0-or-later
-- Intel E1000 network driver specification (Ada/SPARK)

package E1000 is

   pragma Pure;

   type MAC_Address is array (1 .. 6) of Interfaces.Unsigned_8;
   type Packet_Buffer is array (1 .. 1518) of Interfaces.Unsigned_8;

   type E1000_Device is tagged record
      IO_Base     : Interfaces.Unsigned_64 := 0;
      MAC         : MAC_Address            := (others => 0);
      Link_Up     : Boolean                := False;
      Initialized : Boolean                := False;
   end record;

   procedure Initialize
     (Dev : in out E1000_Device;
      IO_Base : in Interfaces.Unsigned_64)
     with Post => Dev.Initialized = True;

   procedure Transmit
     (Dev    : in out E1000_Device;
      Packet : in     Packet_Buffer;
      Length : in     Interfaces.Unsigned_32)
     with Pre => Dev.Initialized;

   procedure Receive
     (Dev    : in out E1000_Device;
      Packet :    out Packet_Buffer;
      Length :    out Interfaces.Unsigned_32)
     with Pre => Dev.Initialized;

end E1000;
