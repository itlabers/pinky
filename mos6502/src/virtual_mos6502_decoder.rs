// WARNING: This file was autogenerated; DO NOT EDIT.

#![macro_use]
macro_rules! decoding_logic {
    () => {

        #[derive(Copy, Clone, PartialEq)]
        pub enum Opcode {
            ADC( Location ),
            AND( Location ),
            ASL( Location ),
            BCS( u8 ),
            BCC( u8 ),
            BEQ( u8 ),
            BNE( u8 ),
            BMI( u8 ),
            BPL( u8 ),
            BVC( u8 ),
            BVS( u8 ),
            BIT( Location ),
            BRK,
            CLC,
            CLD,
            CLI,
            CLV,
            CMP( Location ),
            CPX( Location ),
            CPY( Location ),
            DEC( Location ),
            DEX,
            DEY,
            EOR( Location ),
            INC( Location ),
            INX,
            INY,
            JMP( Location ),
            JSR( u16 ),
            LDA( Location ),
            LDX( Location ),
            LDY( Location ),
            LSR( Location ),
            NOP,
            ORA( Location ),
            PHA,
            PHP,
            PLA,
            PLP,
            ROL( Location ),
            ROR( Location ),
            RTI,
            RTS,
            SBC( Location ),
            SEC,
            SED,
            SEI,
            STA( Location ),
            STX( Location ),
            STY( Location ),
            TAX,
            TAY,
            TSX,
            TXA,
            TXS,
            TYA,
            UNK( u8 )
        }

        pub fn decode_instruction( opcode: u8, arg_lo: u8, arg_hi: u8 ) -> Opcode {
            use self::Location::*;
            use self::Opcode::*;
            
            let arg16 = (arg_lo as u16) | ((arg_hi as u16) << 8);
            match opcode {
                0x00 => BRK,
                0x01 => ORA( IndirectIndexed( arg_lo ) ),
                0x05 => ORA( AbsZP( arg_lo ) ),
                0x06 => ASL( AbsZP( arg_lo ) ),
                0x08 => PHP,
                0x09 => ORA( Imm8( arg_lo ) ),
                0x0A => ASL( Reg( A ) ),
                0x0D => ORA( Abs( arg16 ) ),
                0x0E => ASL( Abs( arg16 ) ),
                0x10 => BPL( arg_lo ),
                0x11 => ORA( IndexedIndirect( arg_lo ) ),
                0x15 => ORA( IndexedZP( X, arg_lo ) ),
                0x16 => ASL( IndexedZP( X, arg_lo ) ),
                0x18 => CLC,
                0x19 => ORA( Indexed( Y, arg16 ) ),
                0x1D => ORA( Indexed( X, arg16 ) ),
                0x1E => ASL( Indexed( X, arg16 ) ),
                0x20 => JSR( arg16 ),
                0x21 => AND( IndirectIndexed( arg_lo ) ),
                0x24 => BIT( AbsZP( arg_lo ) ),
                0x25 => AND( AbsZP( arg_lo ) ),
                0x26 => ROL( AbsZP( arg_lo ) ),
                0x28 => PLP,
                0x29 => AND( Imm8( arg_lo ) ),
                0x2A => ROL( Reg( A ) ),
                0x2C => BIT( Abs( arg16 ) ),
                0x2D => AND( Abs( arg16 ) ),
                0x2E => ROL( Abs( arg16 ) ),
                0x30 => BMI( arg_lo ),
                0x31 => AND( IndexedIndirect( arg_lo ) ),
                0x35 => AND( IndexedZP( X, arg_lo ) ),
                0x36 => ROL( IndexedZP( X, arg_lo ) ),
                0x38 => SEC,
                0x39 => AND( Indexed( Y, arg16 ) ),
                0x3D => AND( Indexed( X, arg16 ) ),
                0x3E => ROL( Indexed( X, arg16 ) ),
                0x40 => RTI,
                0x41 => EOR( IndirectIndexed( arg_lo ) ),
                0x45 => EOR( AbsZP( arg_lo ) ),
                0x46 => LSR( AbsZP( arg_lo ) ),
                0x48 => PHA,
                0x49 => EOR( Imm8( arg_lo ) ),
                0x4A => LSR( Reg( A ) ),
                0x4C => JMP( Imm16( arg16 ) ),
                0x4D => EOR( Abs( arg16 ) ),
                0x4E => LSR( Abs( arg16 ) ),
                0x50 => BVC( arg_lo ),
                0x51 => EOR( IndexedIndirect( arg_lo ) ),
                0x55 => EOR( IndexedZP( X, arg_lo ) ),
                0x56 => LSR( IndexedZP( X, arg_lo ) ),
                0x58 => CLI,
                0x59 => EOR( Indexed( Y, arg16 ) ),
                0x5D => EOR( Indexed( X, arg16 ) ),
                0x5E => LSR( Indexed( X, arg16 ) ),
                0x60 => RTS,
                0x61 => ADC( IndirectIndexed( arg_lo ) ),
                0x65 => ADC( AbsZP( arg_lo ) ),
                0x66 => ROR( AbsZP( arg_lo ) ),
                0x68 => PLA,
                0x69 => ADC( Imm8( arg_lo ) ),
                0x6A => ROR( Reg( A ) ),
                0x6C => JMP( Abs( arg16 ) ),
                0x6D => ADC( Abs( arg16 ) ),
                0x6E => ROR( Abs( arg16 ) ),
                0x70 => BVS( arg_lo ),
                0x71 => ADC( IndexedIndirect( arg_lo ) ),
                0x75 => ADC( IndexedZP( X, arg_lo ) ),
                0x76 => ROR( IndexedZP( X, arg_lo ) ),
                0x78 => SEI,
                0x79 => ADC( Indexed( Y, arg16 ) ),
                0x7D => ADC( Indexed( X, arg16 ) ),
                0x7E => ROR( Indexed( X, arg16 ) ),
                0x81 => STA( IndirectIndexed( arg_lo ) ),
                0x84 => STY( AbsZP( arg_lo ) ),
                0x85 => STA( AbsZP( arg_lo ) ),
                0x86 => STX( AbsZP( arg_lo ) ),
                0x88 => DEY,
                0x8A => TXA,
                0x8C => STY( Abs( arg16 ) ),
                0x8D => STA( Abs( arg16 ) ),
                0x8E => STX( Abs( arg16 ) ),
                0x90 => BCC( arg_lo ),
                0x91 => STA( IndexedIndirect( arg_lo ) ),
                0x94 => STY( IndexedZP( X, arg_lo ) ),
                0x95 => STA( IndexedZP( X, arg_lo ) ),
                0x96 => STX( IndexedZP( Y, arg_lo ) ),
                0x98 => TYA,
                0x99 => STA( Indexed( Y, arg16 ) ),
                0x9A => TXS,
                0x9D => STA( Indexed( X, arg16 ) ),
                0xA0 => LDY( Imm8( arg_lo ) ),
                0xA1 => LDA( IndirectIndexed( arg_lo ) ),
                0xA2 => LDX( Imm8( arg_lo ) ),
                0xA4 => LDY( AbsZP( arg_lo ) ),
                0xA5 => LDA( AbsZP( arg_lo ) ),
                0xA6 => LDX( AbsZP( arg_lo ) ),
                0xA8 => TAY,
                0xA9 => LDA( Imm8( arg_lo ) ),
                0xAA => TAX,
                0xAC => LDY( Abs( arg16 ) ),
                0xAD => LDA( Abs( arg16 ) ),
                0xAE => LDX( Abs( arg16 ) ),
                0xB0 => BCS( arg_lo ),
                0xB1 => LDA( IndexedIndirect( arg_lo ) ),
                0xB4 => LDY( IndexedZP( X, arg_lo ) ),
                0xB5 => LDA( IndexedZP( X, arg_lo ) ),
                0xB6 => LDX( IndexedZP( Y, arg_lo ) ),
                0xB8 => CLV,
                0xB9 => LDA( Indexed( Y, arg16 ) ),
                0xBA => TSX,
                0xBC => LDY( Indexed( X, arg16 ) ),
                0xBD => LDA( Indexed( X, arg16 ) ),
                0xBE => LDX( Indexed( Y, arg16 ) ),
                0xC0 => CPY( Imm8( arg_lo ) ),
                0xC1 => CMP( IndirectIndexed( arg_lo ) ),
                0xC4 => CPY( AbsZP( arg_lo ) ),
                0xC5 => CMP( AbsZP( arg_lo ) ),
                0xC6 => DEC( AbsZP( arg_lo ) ),
                0xC8 => INY,
                0xC9 => CMP( Imm8( arg_lo ) ),
                0xCA => DEX,
                0xCC => CPY( Abs( arg16 ) ),
                0xCD => CMP( Abs( arg16 ) ),
                0xCE => DEC( Abs( arg16 ) ),
                0xD0 => BNE( arg_lo ),
                0xD1 => CMP( IndexedIndirect( arg_lo ) ),
                0xD5 => CMP( IndexedZP( X, arg_lo ) ),
                0xD6 => DEC( IndexedZP( X, arg_lo ) ),
                0xD8 => CLD,
                0xD9 => CMP( Indexed( Y, arg16 ) ),
                0xDD => CMP( Indexed( X, arg16 ) ),
                0xDE => DEC( Indexed( X, arg16 ) ),
                0xE0 => CPX( Imm8( arg_lo ) ),
                0xE1 => SBC( IndirectIndexed( arg_lo ) ),
                0xE4 => CPX( AbsZP( arg_lo ) ),
                0xE5 => SBC( AbsZP( arg_lo ) ),
                0xE6 => INC( AbsZP( arg_lo ) ),
                0xE8 => INX,
                0xE9 => SBC( Imm8( arg_lo ) ),
                0xEA => NOP,
                0xEC => CPX( Abs( arg16 ) ),
                0xED => SBC( Abs( arg16 ) ),
                0xEE => INC( Abs( arg16 ) ),
                0xF0 => BEQ( arg_lo ),
                0xF1 => SBC( IndexedIndirect( arg_lo ) ),
                0xF5 => SBC( IndexedZP( X, arg_lo ) ),
                0xF6 => INC( IndexedZP( X, arg_lo ) ),
                0xF8 => SED,
                0xF9 => SBC( Indexed( Y, arg16 ) ),
                0xFD => SBC( Indexed( X, arg16 ) ),
                0xFE => INC( Indexed( X, arg16 ) ),
                _ => UNK( opcode )
            }
        }

        fn exec_unk< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.unk()
        }

        fn exec_adc< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.adc()
        }

        fn exec_and< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.and()
        }

        fn exec_asl< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.asl()
        }

        fn exec_branch_statusflag_carry_true< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.branch( StatusFlag::Carry, true )
        }

        fn exec_branch_statusflag_carry_false< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.branch( StatusFlag::Carry, false )
        }

        fn exec_branch_statusflag_zero_true< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.branch( StatusFlag::Zero, true )
        }

        fn exec_branch_statusflag_zero_false< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.branch( StatusFlag::Zero, false )
        }

        fn exec_branch_statusflag_sign_true< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.branch( StatusFlag::Sign, true )
        }

        fn exec_branch_statusflag_sign_false< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.branch( StatusFlag::Sign, false )
        }

        fn exec_branch_statusflag_overflow_false< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.branch( StatusFlag::Overflow, false )
        }

        fn exec_branch_statusflag_overflow_true< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.branch( StatusFlag::Overflow, true )
        }

        fn exec_bit< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.bit()
        }

        fn exec_brk< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.brk()
        }

        fn exec_clr_statusflag_carry< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.clr( StatusFlag::Carry )
        }

        fn exec_clr_statusflag_bcdmode< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.clr( StatusFlag::BCDMode )
        }

        fn exec_clr_statusflag_irqdisable< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.clr( StatusFlag::IRQDisable )
        }

        fn exec_clr_statusflag_overflow< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.clr( StatusFlag::Overflow )
        }

        fn exec_compare_a< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.compare( A )
        }

        fn exec_compare_x< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.compare( X )
        }

        fn exec_compare_y< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.compare( Y )
        }

        fn exec_incdec_neg_1< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.incdec( -1 )
        }

        fn exec_eor< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.eor()
        }

        fn exec_incdec_1< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.incdec( 1 )
        }

        fn exec_jmp_abs< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.jmp_abs()
        }

        fn exec_jmp_imm< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.jmp_imm()
        }

        fn exec_jsr< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.jsr()
        }

        fn exec_mov2reg_a< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.mov2reg( A )
        }

        fn exec_mov2reg_x< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.mov2reg( X )
        }

        fn exec_mov2reg_y< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.mov2reg( Y )
        }

        fn exec_lsr< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.lsr()
        }

        fn exec_nop< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.nop()
        }

        fn exec_ora< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.ora()
        }

        fn exec_pha< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.pha()
        }

        fn exec_php< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.php()
        }

        fn exec_pla< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.pla()
        }

        fn exec_plp< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.plp()
        }

        fn exec_rol< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.rol()
        }

        fn exec_ror< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.ror()
        }

        fn exec_rti< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.rti()
        }

        fn exec_rts< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.rts()
        }

        fn exec_sbc< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.sbc()
        }

        fn exec_set_statusflag_carry< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.set( StatusFlag::Carry )
        }

        fn exec_set_statusflag_bcdmode< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.set( StatusFlag::BCDMode )
        }

        fn exec_set_statusflag_irqdisable< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.set( StatusFlag::IRQDisable )
        }

        fn exec_reg2mem_a< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.reg2mem( A )
        }

        fn exec_reg2mem_x< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.reg2mem( X )
        }

        fn exec_reg2mem_y< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.reg2mem( Y )
        }

        fn exec_txs< T: Private >( cpu: &mut T ) -> Result< EmulationStatus, EmulationError > {
            cpu.txs()
        }

        fn instruction_for_opcode< T: Private >( opcode: u8 ) -> fn( &mut T ) -> Result< EmulationStatus, EmulationError > {
            match opcode {
                0 => exec_brk,
                1 => exec_ora,
                2 => exec_unk,
                3 => exec_unk,
                4 => exec_unk,
                5 => exec_ora,
                6 => exec_asl,
                7 => exec_unk,
                8 => exec_php,
                9 => exec_ora,
                10 => exec_asl,
                11 => exec_unk,
                12 => exec_unk,
                13 => exec_ora,
                14 => exec_asl,
                15 => exec_unk,
                16 => exec_branch_statusflag_sign_false,
                17 => exec_ora,
                18 => exec_unk,
                19 => exec_unk,
                20 => exec_unk,
                21 => exec_ora,
                22 => exec_asl,
                23 => exec_unk,
                24 => exec_clr_statusflag_carry,
                25 => exec_ora,
                26 => exec_unk,
                27 => exec_unk,
                28 => exec_unk,
                29 => exec_ora,
                30 => exec_asl,
                31 => exec_unk,
                32 => exec_jsr,
                33 => exec_and,
                34 => exec_unk,
                35 => exec_unk,
                36 => exec_bit,
                37 => exec_and,
                38 => exec_rol,
                39 => exec_unk,
                40 => exec_plp,
                41 => exec_and,
                42 => exec_rol,
                43 => exec_unk,
                44 => exec_bit,
                45 => exec_and,
                46 => exec_rol,
                47 => exec_unk,
                48 => exec_branch_statusflag_sign_true,
                49 => exec_and,
                50 => exec_unk,
                51 => exec_unk,
                52 => exec_unk,
                53 => exec_and,
                54 => exec_rol,
                55 => exec_unk,
                56 => exec_set_statusflag_carry,
                57 => exec_and,
                58 => exec_unk,
                59 => exec_unk,
                60 => exec_unk,
                61 => exec_and,
                62 => exec_rol,
                63 => exec_unk,
                64 => exec_rti,
                65 => exec_eor,
                66 => exec_unk,
                67 => exec_unk,
                68 => exec_unk,
                69 => exec_eor,
                70 => exec_lsr,
                71 => exec_unk,
                72 => exec_pha,
                73 => exec_eor,
                74 => exec_lsr,
                75 => exec_unk,
                76 => exec_jmp_imm,
                77 => exec_eor,
                78 => exec_lsr,
                79 => exec_unk,
                80 => exec_branch_statusflag_overflow_false,
                81 => exec_eor,
                82 => exec_unk,
                83 => exec_unk,
                84 => exec_unk,
                85 => exec_eor,
                86 => exec_lsr,
                87 => exec_unk,
                88 => exec_clr_statusflag_irqdisable,
                89 => exec_eor,
                90 => exec_unk,
                91 => exec_unk,
                92 => exec_unk,
                93 => exec_eor,
                94 => exec_lsr,
                95 => exec_unk,
                96 => exec_rts,
                97 => exec_adc,
                98 => exec_unk,
                99 => exec_unk,
                100 => exec_unk,
                101 => exec_adc,
                102 => exec_ror,
                103 => exec_unk,
                104 => exec_pla,
                105 => exec_adc,
                106 => exec_ror,
                107 => exec_unk,
                108 => exec_jmp_abs,
                109 => exec_adc,
                110 => exec_ror,
                111 => exec_unk,
                112 => exec_branch_statusflag_overflow_true,
                113 => exec_adc,
                114 => exec_unk,
                115 => exec_unk,
                116 => exec_unk,
                117 => exec_adc,
                118 => exec_ror,
                119 => exec_unk,
                120 => exec_set_statusflag_irqdisable,
                121 => exec_adc,
                122 => exec_unk,
                123 => exec_unk,
                124 => exec_unk,
                125 => exec_adc,
                126 => exec_ror,
                127 => exec_unk,
                128 => exec_unk,
                129 => exec_reg2mem_a,
                130 => exec_unk,
                131 => exec_unk,
                132 => exec_reg2mem_y,
                133 => exec_reg2mem_a,
                134 => exec_reg2mem_x,
                135 => exec_unk,
                136 => exec_incdec_neg_1,
                137 => exec_unk,
                138 => exec_mov2reg_a,
                139 => exec_unk,
                140 => exec_reg2mem_y,
                141 => exec_reg2mem_a,
                142 => exec_reg2mem_x,
                143 => exec_unk,
                144 => exec_branch_statusflag_carry_false,
                145 => exec_reg2mem_a,
                146 => exec_unk,
                147 => exec_unk,
                148 => exec_reg2mem_y,
                149 => exec_reg2mem_a,
                150 => exec_reg2mem_x,
                151 => exec_unk,
                152 => exec_mov2reg_a,
                153 => exec_reg2mem_a,
                154 => exec_txs,
                155 => exec_unk,
                156 => exec_unk,
                157 => exec_reg2mem_a,
                158 => exec_unk,
                159 => exec_unk,
                160 => exec_mov2reg_y,
                161 => exec_mov2reg_a,
                162 => exec_mov2reg_x,
                163 => exec_unk,
                164 => exec_mov2reg_y,
                165 => exec_mov2reg_a,
                166 => exec_mov2reg_x,
                167 => exec_unk,
                168 => exec_mov2reg_y,
                169 => exec_mov2reg_a,
                170 => exec_mov2reg_x,
                171 => exec_unk,
                172 => exec_mov2reg_y,
                173 => exec_mov2reg_a,
                174 => exec_mov2reg_x,
                175 => exec_unk,
                176 => exec_branch_statusflag_carry_true,
                177 => exec_mov2reg_a,
                178 => exec_unk,
                179 => exec_unk,
                180 => exec_mov2reg_y,
                181 => exec_mov2reg_a,
                182 => exec_mov2reg_x,
                183 => exec_unk,
                184 => exec_clr_statusflag_overflow,
                185 => exec_mov2reg_a,
                186 => exec_mov2reg_x,
                187 => exec_unk,
                188 => exec_mov2reg_y,
                189 => exec_mov2reg_a,
                190 => exec_mov2reg_x,
                191 => exec_unk,
                192 => exec_compare_y,
                193 => exec_compare_a,
                194 => exec_unk,
                195 => exec_unk,
                196 => exec_compare_y,
                197 => exec_compare_a,
                198 => exec_incdec_neg_1,
                199 => exec_unk,
                200 => exec_incdec_1,
                201 => exec_compare_a,
                202 => exec_incdec_neg_1,
                203 => exec_unk,
                204 => exec_compare_y,
                205 => exec_compare_a,
                206 => exec_incdec_neg_1,
                207 => exec_unk,
                208 => exec_branch_statusflag_zero_false,
                209 => exec_compare_a,
                210 => exec_unk,
                211 => exec_unk,
                212 => exec_unk,
                213 => exec_compare_a,
                214 => exec_incdec_neg_1,
                215 => exec_unk,
                216 => exec_clr_statusflag_bcdmode,
                217 => exec_compare_a,
                218 => exec_unk,
                219 => exec_unk,
                220 => exec_unk,
                221 => exec_compare_a,
                222 => exec_incdec_neg_1,
                223 => exec_unk,
                224 => exec_compare_x,
                225 => exec_sbc,
                226 => exec_unk,
                227 => exec_unk,
                228 => exec_compare_x,
                229 => exec_sbc,
                230 => exec_incdec_1,
                231 => exec_unk,
                232 => exec_incdec_1,
                233 => exec_sbc,
                234 => exec_nop,
                235 => exec_unk,
                236 => exec_compare_x,
                237 => exec_sbc,
                238 => exec_incdec_1,
                239 => exec_unk,
                240 => exec_branch_statusflag_zero_true,
                241 => exec_sbc,
                242 => exec_unk,
                243 => exec_unk,
                244 => exec_unk,
                245 => exec_sbc,
                246 => exec_incdec_1,
                247 => exec_unk,
                248 => exec_set_statusflag_bcdmode,
                249 => exec_sbc,
                250 => exec_unk,
                251 => exec_unk,
                252 => exec_unk,
                253 => exec_sbc,
                254 => exec_incdec_1,
                255 => exec_unk,
                _ => unsafe {
                    fast_unreachable!()
                }
            }
        }

        #[derive(Copy, Clone, PartialEq, Eq)]
        struct OpcodeAttributes {
            attr: u8
        }
        
        impl OpcodeAttributes {
            fn empty() -> OpcodeAttributes {
                OpcodeAttributes {
                    attr: 0
                }
            }
            
            fn get( opcode: u8 ) -> OpcodeAttributes {
                OpcodeAttributes {
                    attr: unsafe {
                        *OPCODE_ATTRIBUTE_TABLE.get_unchecked( opcode as usize )
                    }
                }
            }
            
            #[inline(always)]
            fn is_single_byte_imm8( &self ) -> bool {
                self.attr == 0b10000000
            }
            
            #[inline(always)]
            fn is_operand_an_register( &self ) -> bool {
                (self.attr & 0b00000111) == 0b00000111
            }
            
            #[inline(always)]
            fn is_single_byte_instruction( &self ) -> bool {
                (self.attr & 0b10000000) != 0
            }
            
            #[inline(always)]
            fn is_operand_fetched_from_address( &self ) -> bool {
                (self.attr & 0b01000000) != 0
            }
            
            /* Whenever the extra cycle is always forced in indexed and indirect_indexed addressing modes. */
            #[inline(always)]
            fn is_extra_cycle_forced( &self ) -> bool {
                (self.attr & 0b00100000) != 0
            }
            
            #[inline(always)]
            fn get_register( &self ) -> Register8 {
                use std::mem;
                unsafe {
                    mem::transmute( (self.attr & 0b00011000) >> 3 )
                }
            }
            
            #[inline(always)]
            fn get_memop< T: Private >( &self ) -> fn( &mut T ) {
                match self.attr & 0b00000111 {
                    0 => T::memop_imm8,
                    1 => T::memop_abs,
                    2 => T::memop_abs_zp,
                    3 => T::memop_indexed_zp,
                    4 => T::memop_indexed,
                    5 => T::memop_indexed_indirect,
                    6 => T::memop_indirect_indexed,
                    7 => T::memop_reg,
                    _ => unsafe {
                        fast_unreachable!()
                    }
                }
            }
        }

        const OPCODE_ATTRIBUTE_TABLE: &'static [u8] = &[
            0x00, 0x46, 0x80, 0x80, 0x80, 0x42, 0x42, 0x80,
            0x80, 0x00, 0x87, 0x80, 0x80, 0x41, 0x41, 0x80,
            0x00, 0x45, 0x80, 0x80, 0x80, 0x4B, 0x4B, 0x80,
            0x80, 0x54, 0x80, 0x80, 0x80, 0x4C, 0x6C, 0x80,
            0x00, 0x46, 0x80, 0x80, 0x42, 0x42, 0x42, 0x80,
            0x80, 0x00, 0x87, 0x80, 0x41, 0x41, 0x41, 0x80,
            0x00, 0x45, 0x80, 0x80, 0x80, 0x4B, 0x4B, 0x80,
            0x80, 0x54, 0x80, 0x80, 0x80, 0x4C, 0x6C, 0x80,
            0x80, 0x46, 0x80, 0x80, 0x80, 0x42, 0x42, 0x80,
            0x80, 0x00, 0x87, 0x80, 0x00, 0x41, 0x41, 0x80,
            0x00, 0x45, 0x80, 0x80, 0x80, 0x4B, 0x4B, 0x80,
            0x80, 0x54, 0x80, 0x80, 0x80, 0x4C, 0x6C, 0x80,
            0x80, 0x46, 0x80, 0x80, 0x80, 0x42, 0x42, 0x80,
            0x80, 0x00, 0x87, 0x80, 0x00, 0x41, 0x41, 0x80,
            0x00, 0x45, 0x80, 0x80, 0x80, 0x4B, 0x4B, 0x80,
            0x80, 0x54, 0x80, 0x80, 0x80, 0x4C, 0x6C, 0x80,
            0x80, 0x26, 0x80, 0x80, 0x02, 0x02, 0x02, 0x80,
            0x97, 0x80, 0x8F, 0x80, 0x01, 0x01, 0x01, 0x80,
            0x00, 0x25, 0x80, 0x80, 0x0B, 0x0B, 0x13, 0x80,
            0x97, 0x34, 0x80, 0x80, 0x80, 0x2C, 0x80, 0x80,
            0x00, 0x46, 0x00, 0x80, 0x42, 0x42, 0x42, 0x80,
            0x87, 0x00, 0x87, 0x80, 0x41, 0x41, 0x41, 0x80,
            0x00, 0x45, 0x80, 0x80, 0x4B, 0x4B, 0x53, 0x80,
            0x80, 0x54, 0x9F, 0x80, 0x4C, 0x4C, 0x54, 0x80,
            0x00, 0x46, 0x80, 0x80, 0x42, 0x42, 0x42, 0x80,
            0x97, 0x00, 0x8F, 0x80, 0x41, 0x41, 0x41, 0x80,
            0x00, 0x45, 0x80, 0x80, 0x80, 0x4B, 0x4B, 0x80,
            0x80, 0x54, 0x80, 0x80, 0x80, 0x4C, 0x6C, 0x80,
            0x00, 0x46, 0x80, 0x80, 0x42, 0x42, 0x42, 0x80,
            0x8F, 0x00, 0x80, 0x80, 0x41, 0x41, 0x41, 0x80,
            0x00, 0x45, 0x80, 0x80, 0x80, 0x4B, 0x4B, 0x80,
            0x80, 0x54, 0x80, 0x80, 0x80, 0x4C, 0x6C, 0x80,
        ];
    }
}
