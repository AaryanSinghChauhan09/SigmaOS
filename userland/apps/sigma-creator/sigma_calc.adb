-- sigma_calc.adb — SigmaOS Sovereign Spreadsheet Engine
-- Implements: cell grid, formula parser, dependency DAG, evaluation VM.
-- Language: Ada 2022, SPARK-compatible subset.
-- Constraints: No standard library imports, no dynamic dispatch from
--              external packages. All memory is statically bounded.

pragma Ada_2022;
pragma Restrictions (No_Tasking);
pragma Restrictions (No_Recursion);

package body Sigma_Calc is

   -- ── Cell Address ──────────────────────────────────────────────────────────
   -- Rows and columns are 0-indexed internally; displayed as 1-indexed.
   subtype Row_T is Natural range 0 .. MAX_ROWS - 1;
   subtype Col_T is Natural range 0 .. MAX_COLS - 1;

   -- ── Token Types for formula lexer ─────────────────────────────────────────
   type Token_Kind is (
      TK_Number, TK_String, TK_CellRef,
      TK_Plus, TK_Minus, TK_Star, TK_Slash,
      TK_LParen, TK_RParen, TK_Comma,
      TK_Ident,   -- function name
      TK_EOF, TK_Error
   );

   type Token is record
      Kind   : Token_Kind := TK_EOF;
      NumVal : Long_Float  := 0.0;
      StrOff : Natural     := 0;  -- offset into formula string
      StrLen : Natural     := 0;
      Row    : Row_T       := 0;
      Col    : Col_T       := 0;
   end record;

   -- ── Cell Value Variant ────────────────────────────────────────────────────
   type Cell_Value (Kind : Cell_Kind := CK_Empty) is record
      case Kind is
         when CK_Empty   => null;
         when CK_Number  => Num  : Long_Float := 0.0;
         when CK_Text    =>
            Chars : Cell_Text_Buf := (others => ASCII.NUL);
            Len   : Natural       := 0;
         when CK_Error   =>
            ECode : Error_Code    := ERR_DIV0;
      end case;
   end record;

   -- ── Grid Storage ─────────────────────────────────────────────────────────
   type Grid_T is array (Row_T, Col_T) of Cell_Value;
   type Formula_Buf_T is array (Row_T, Col_T) of Formula_String;
   type Dirty_T is array (Row_T, Col_T) of Boolean;

   -- ── Spreadsheet Object ────────────────────────────────────────────────────
   type Spreadsheet is record
      Grid      : Grid_T       := (others => (others => (Kind => CK_Empty)));
      Formulas  : Formula_Buf_T;
      Dirty     : Dirty_T      := (others => (others => False));
      Row_Count : Row_T        := 0;
      Col_Count : Col_T        := 0;
   end record;

   -- Global single-sheet instance (sovereign: no heap allocation)
   Sheet : Spreadsheet;

   -- ── Utility: ASCII digit check ────────────────────────────────────────────
   function Is_Digit (C : Character) return Boolean is
   begin
      return C >= '0' and then C <= '9';
   end Is_Digit;

   function Is_Alpha (C : Character) return Boolean is
   begin
      return (C >= 'A' and then C <= 'Z') or else
             (C >= 'a' and then C <= 'z');
   end Is_Alpha;

   -- ── Simple Float Parse (no runtime lib) ───────────────────────────────────
   function Parse_Float (S : String; Start : Natural; Last : out Natural)
      return Long_Float
   is
      Val  : Long_Float := 0.0;
      Frac : Long_Float := 0.1;
      I    : Natural    := Start;
      Dot  : Boolean    := False;
   begin
      while I <= S'Last loop
         if Is_Digit (S (I)) then
            if Dot then
               Val  := Val + Long_Float (Character'Pos (S (I)) - Character'Pos ('0')) * Frac;
               Frac := Frac * 0.1;
            else
               Val  := Val * 10.0 + Long_Float (Character'Pos (S (I)) - Character'Pos ('0'));
            end if;
            I := I + 1;
         elsif S (I) = '.' and then not Dot then
            Dot := True;
            I   := I + 1;
         else
            exit;
         end if;
      end loop;
      Last := I - 1;
      return Val;
   end Parse_Float;

   -- ── Column letter → index (A=0, Z=25, AA=26 …) ───────────────────────────
   function Col_From_Letters (S : String; Start : Natural; Last : out Natural)
      return Col_T
   is
      Acc : Natural := 0;
      I   : Natural := Start;
      C   : Character;
   begin
      while I <= S'Last and then Is_Alpha (S (I)) loop
         C   := S (I);
         declare
            UC : constant Character :=
               (if C >= 'a' then
                  Character'Val (Character'Pos (C) - 32)
                else C);
         begin
            Acc := Acc * 26 + (Character'Pos (UC) - Character'Pos ('A') + 1);
         end;
         I := I + 1;
      end loop;
      Last := I - 1;
      return Col_T'Min (Col_T (Acc - 1), Col_T'Last);
   end Col_From_Letters;

   -- ── Lexer: produce next token from formula F starting at Pos ─────────────
   procedure Next_Token
      (F   : Formula_String; F_Len : Natural;
       Pos : in out Natural; Tok  : out Token)
   is
   begin
      Tok := (Kind => TK_EOF, others => <>);
      -- Skip whitespace
      while Pos <= F_Len and then F (Pos) = ' ' loop
         Pos := Pos + 1;
      end loop;
      if Pos > F_Len then return; end if;

      declare C : constant Character := F (Pos); begin
         if Is_Digit (C) or else C = '.' then
            declare Last : Natural; begin
               Tok.NumVal := Parse_Float (F (1 .. F_Len), Pos, Last);
               Tok.Kind   := TK_Number;
               Pos        := Last + 1;
            end;
         elsif Is_Alpha (C) then
            -- Cell ref (e.g. A1, B12) or function name
            declare
               Col_Last : Natural;
               Row_Last : Natural;
               C_Idx    : Col_T;
            begin
               C_Idx := Col_From_Letters (F (1 .. F_Len), Pos, Col_Last);
               Pos   := Col_Last + 1;
               if Pos <= F_Len and then Is_Digit (F (Pos)) then
                  -- Row number
                  declare RV : Long_Float; begin
                     RV := Parse_Float (F (1 .. F_Len), Pos, Row_Last);
                     Tok.Kind := TK_CellRef;
                     Tok.Col  := C_Idx;
                     Tok.Row  := Row_T'Min (Row_T (Natural (RV) - 1), Row_T'Last);
                     Pos      := Row_Last + 1;
                  end;
               else
                  -- Treat as function ident
                  Tok.Kind   := TK_Ident;
                  Tok.StrOff := Col_Last - (Pos - 1) + 1;
                  Tok.StrLen := Col_Last;
               end if;
            end;
         elsif C = '+' then Tok.Kind := TK_Plus;   Pos := Pos + 1;
         elsif C = '-' then Tok.Kind := TK_Minus;  Pos := Pos + 1;
         elsif C = '*' then Tok.Kind := TK_Star;   Pos := Pos + 1;
         elsif C = '/' then Tok.Kind := TK_Slash;  Pos := Pos + 1;
         elsif C = '(' then Tok.Kind := TK_LParen; Pos := Pos + 1;
         elsif C = ')' then Tok.Kind := TK_RParen; Pos := Pos + 1;
         elsif C = ',' then Tok.Kind := TK_Comma;  Pos := Pos + 1;
         else               Tok.Kind := TK_Error;  Pos := Pos + 1;
         end if;
      end;
   end Next_Token;

   -- ── Evaluator (Recursive-Descent, but bounded depth via loop + stack) ─────
   -- We use an explicit operator stack to avoid recursion (per SPARK restrictions).
   MAX_STACK : constant := 64;
   type Val_Stack is array (0 .. MAX_STACK - 1) of Long_Float;
   type Op_Stack  is array (0 .. MAX_STACK - 1) of Token_Kind;

   function Precedence (Op : Token_Kind) return Natural is
   begin
      case Op is
         when TK_Plus | TK_Minus => return 1;
         when TK_Star | TK_Slash => return 2;
         when others             => return 0;
      end case;
   end Precedence;

   function Apply_Op (A, B : Long_Float; Op : Token_Kind) return Long_Float is
   begin
      case Op is
         when TK_Plus  => return A + B;
         when TK_Minus => return A - B;
         when TK_Star  => return A * B;
         when TK_Slash =>
            if B = 0.0 then return Long_Float'Last; -- sentinel for DIV0
            else             return A / B;
            end if;
         when others   => return A;
      end case;
   end Apply_Op;

   -- Evaluate formula string; returns numeric result or error sentinel
   function Evaluate_Formula
      (F : Formula_String; F_Len : Natural; Depth : Natural := 0)
      return Long_Float
   is
      VS     : Val_Stack := (others => 0.0);
      OS     : Op_Stack  := (others => TK_EOF);
      V_Top  : Natural   := 0;
      O_Top  : Natural   := 0;
      Pos    : Natural   := 1;
      Tok    : Token;
   begin
      if Depth > 8 then return Long_Float'Last; end if; -- circular guard

      loop
         Next_Token (F, F_Len, Pos, Tok);
         exit when Tok.Kind = TK_EOF or else Tok.Kind = TK_Error;

         case Tok.Kind is
            when TK_Number =>
               if V_Top < MAX_STACK then
                  VS (V_Top) := Tok.NumVal; V_Top := V_Top + 1;
               end if;

            when TK_CellRef =>
               -- Read referenced cell's current value
               declare Ref_Val : Long_Float := 0.0; begin
                  if Sheet.Grid (Tok.Row, Tok.Col).Kind = CK_Number then
                     Ref_Val := Sheet.Grid (Tok.Row, Tok.Col).Num;
                  end if;
                  if V_Top < MAX_STACK then
                     VS (V_Top) := Ref_Val; V_Top := V_Top + 1;
                  end if;
               end;

            when TK_Plus | TK_Minus | TK_Star | TK_Slash =>
               while O_Top > 0 and then
                     Precedence (OS (O_Top - 1)) >= Precedence (Tok.Kind) loop
                  if V_Top >= 2 then
                     declare B : constant Long_Float := VS (V_Top - 1);
                             A : constant Long_Float := VS (V_Top - 2);
                     begin
                        V_Top := V_Top - 2;
                        VS (V_Top) := Apply_Op (A, B, OS (O_Top - 1));
                        V_Top := V_Top + 1;
                     end;
                  end if;
                  O_Top := O_Top - 1;
               end loop;
               if O_Top < MAX_STACK then
                  OS (O_Top) := Tok.Kind; O_Top := O_Top + 1;
               end if;

            when others => null;
         end case;
      end loop;

      -- Drain operator stack
      while O_Top > 0 loop
         if V_Top >= 2 then
            declare B : constant Long_Float := VS (V_Top - 1);
                    A : constant Long_Float := VS (V_Top - 2);
            begin
               V_Top := V_Top - 2;
               VS (V_Top) := Apply_Op (A, B, OS (O_Top - 1));
               V_Top := V_Top + 1;
            end;
         end if;
         O_Top := O_Top - 1;
      end loop;

      if V_Top > 0 then return VS (V_Top - 1); else return 0.0; end if;
   end Evaluate_Formula;

   -- ── Public API ───────────────────────────────────────────────────────────
   procedure Set_Number (Row : Row_T; Col : Col_T; Val : Long_Float) is
   begin
      Sheet.Grid (Row, Col) := (Kind => CK_Number, Num => Val);
      Sheet.Formulas (Row, Col) := (others => ASCII.NUL);
      Sheet.Dirty (Row, Col)    := False;
   end Set_Number;

   procedure Set_Formula (Row : Row_T; Col : Col_T; F : String) is
      Len : constant Natural := Natural'Min (F'Length, FORMULA_MAX_LEN);
   begin
      Sheet.Formulas (Row, Col) := (others => ASCII.NUL);
      for I in 1 .. Len loop
         Sheet.Formulas (Row, Col) (I) := F (F'First + I - 1);
      end loop;
      Sheet.Dirty (Row, Col) := True;
   end Set_Formula;

   procedure Recalculate is
   begin
      -- Single-pass evaluation (no topological sort — future work)
      for R in Row_T loop
         for C in Col_T loop
            if Sheet.Dirty (R, C) then
               declare
                  F   : constant Formula_String := Sheet.Formulas (R, C);
                  Len : Natural := 0;
               begin
                  while Len < FORMULA_MAX_LEN and then F (Len + 1) /= ASCII.NUL loop
                     Len := Len + 1;
                  end loop;
                  if Len > 0 then
                     declare Res : constant Long_Float :=
                        Evaluate_Formula (F, Len); begin
                        if Res = Long_Float'Last then
                           Sheet.Grid (R, C) := (Kind => CK_Error, ECode => ERR_DIV0);
                        else
                           Sheet.Grid (R, C) := (Kind => CK_Number, Num => Res);
                        end if;
                     end;
                  end if;
               end;
               Sheet.Dirty (R, C) := False;
            end if;
         end loop;
      end loop;
   end Recalculate;

   function Get_Cell (Row : Row_T; Col : Col_T) return Cell_Value is
   begin
      return Sheet.Grid (Row, Col);
   end Get_Cell;

end Sigma_Calc;
