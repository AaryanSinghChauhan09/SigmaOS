-- sigma_calc.ads — SigmaOS Sovereign Spreadsheet Specification
-- Ada package spec for sigma_calc.adb
pragma Ada_2022;
pragma Restrictions (No_Tasking);

package Sigma_Calc
   with SPARK_Mode => On
is
   -- Grid dimensions (compile-time constants)
   MAX_ROWS        : constant := 1024;
   MAX_COLS        : constant := 256;
   FORMULA_MAX_LEN : constant := 512;

   subtype Row_T is Natural range 0 .. MAX_ROWS - 1;
   subtype Col_T is Natural range 0 .. MAX_COLS - 1;

   subtype Formula_String is String (1 .. FORMULA_MAX_LEN);
   subtype Cell_Text_Buf  is String (1 .. 128);

   type Cell_Kind is (CK_Empty, CK_Number, CK_Text, CK_Error);
   type Error_Code is (ERR_DIV0, ERR_REF, ERR_VALUE, ERR_NAME);

   type Cell_Value (Kind : Cell_Kind := CK_Empty) is record
      case Kind is
         when CK_Empty  => null;
         when CK_Number => Num   : Long_Float := 0.0;
         when CK_Text   =>
            Chars : Cell_Text_Buf := (others => ASCII.NUL);
            Len   : Natural       := 0;
         when CK_Error  => ECode : Error_Code := ERR_DIV0;
      end case;
   end record;

   -- ── API ──────────────────────────────────────────────────────────────────
   procedure Set_Number  (Row : Row_T; Col : Col_T; Val : Long_Float)
      with Global => null;

   procedure Set_Formula (Row : Row_T; Col : Col_T; F : String)
      with Global => null,
           Pre    => F'Length <= FORMULA_MAX_LEN;

   procedure Recalculate
      with Global => null;

   function Get_Cell (Row : Row_T; Col : Col_T) return Cell_Value
      with Global => null;

end Sigma_Calc;
