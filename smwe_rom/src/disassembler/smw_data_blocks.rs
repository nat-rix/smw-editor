use crate::snes_utils::{addr::AddrSnes, rom_slice::SnesSlice};

// -------------------------------------------------------------------------------------------------

pub fn is_in_data_block(addr: AddrSnes) -> bool {
    SMW_BLOCK_SLICES.iter().any(|b| b.contains(addr))
}

pub fn is_in_code_block(addr: AddrSnes) -> bool {
    !is_in_data_block(addr)
}

// -------------------------------------------------------------------------------------------------

const fn make_slice(start: usize, end: usize) -> SnesSlice {
    SnesSlice::new(AddrSnes(start), end - start)
}

/// Table locating all the data blocks in the SMW ROM, to distinguish them from code.
pub static SMW_BLOCK_SLICES: [SnesSlice; 229] = [
    make_slice(0x008475, 0x008494),
    make_slice(0x0084D0, 0x0085D2),
    make_slice(0x008649, 0x008650),
    make_slice(0x008A16, 0x008A4E),
    make_slice(0x008AB4, 0x008ACD),
    make_slice(0x008B57, 0x008CFF),
    make_slice(0x008D90, 0x008DAC),
    make_slice(0x008DE7, 0x008E1A),
    make_slice(0x008FFA, 0x009012),
    make_slice(0x0090D1, 0x00919B),
    make_slice(0x009249, 0x009250),
    make_slice(0x009277, 0x009283),
    make_slice(0x009313, 0x009322),
    make_slice(0x009329, 0x00937D),
    make_slice(0x009389, 0x009391),
    make_slice(0x009451, 0x009468),
    make_slice(0x009875, 0x00987D),
    make_slice(0x009891, 0x0098A9),
    make_slice(0x009A4E, 0x009A52),
    make_slice(0x009AC8, 0x009ACB),
    make_slice(0x009B17, 0x009B1A),
    make_slice(0x009C1B, 0x009C1E),
    make_slice(0x009C1F, 0x009C64),
    make_slice(0x009CCB, 0x009CD1),
    make_slice(0x009E6A, 0x009E82),
    make_slice(0x009EE0, 0x009F06),
    make_slice(0x009F2F, 0x009F37),
    make_slice(0x009F88, 0x009FB8),
    make_slice(0x00A06B, 0x00A087),
    make_slice(0x00A1CE, 0x00A1DA),
    make_slice(0x00A47F, 0x00A488),
    make_slice(0x00A521, 0x00A529),
    make_slice(0x00A586, 0x00A594),
    make_slice(0x00A608, 0x00A60A),
    make_slice(0x00A60D, 0x00A635),
    make_slice(0x00A8C3, 0x00A993),
    make_slice(0x00A9D2, 0x00A9DA),
    make_slice(0x00ABD3, 0x00ABED),
    make_slice(0x00AD1E, 0x00AD25),
    make_slice(0x00AE41, 0x00AE47),
    make_slice(0x00AE65, 0x00AF17),
    make_slice(0x00B091, 0x00B888),
    make_slice(0x00B992, 0x00BA28),
    make_slice(0x00BA4D, 0x00BEB0),
    make_slice(0x00BFC9, 0x00C00D),
    make_slice(0x00C063, 0x00C074),
    make_slice(0x00C0AA, 0x00C0C1),
    make_slice(0x00C29E, 0x00C334),
    make_slice(0x00C453, 0x00C47E),
    make_slice(0x00C599, 0x00C5B5),
    make_slice(0x00C5E1, 0x00C6E7),
    make_slice(0x00C7F9, 0x00C7FD),
    make_slice(0x00C845, 0x00C870),
    make_slice(0x00C9A4, 0x00C9AF),
    make_slice(0x00CB12, 0x00CC14),
    make_slice(0x00CC5C, 0x00CC68),
    make_slice(0x00CDA9, 0x00CDAD),
    make_slice(0x00CE79, 0x00CEB1),
    make_slice(0x00D034, 0x00D044),
    make_slice(0x00D0AE, 0x00D0B6),
    make_slice(0x00D11D, 0x00D129),
    make_slice(0x00D18D, 0x00D197),
    make_slice(0x00D1FF, 0x00D203),
    make_slice(0x00D2BD, 0x00D5F2),
    make_slice(0x00D7A5, 0x00D7E4),
    make_slice(0x00D980, 0x00D988),
    make_slice(0x00DAB7, 0x00DB17),
    make_slice(0x00DC78, 0x00E2BD),
    make_slice(0x00E4B9, 0x00E92B),
    make_slice(0x00EAB9, 0x00EADB),
    make_slice(0x00F05C, 0x00F120),
    make_slice(0x00F3E3, 0x00F3E9),
    make_slice(0x00F69F, 0x00F6DB),
    make_slice(0x00F8DF, 0x00F8F2),
    make_slice(0x00F9F5, 0x00FA10),
    make_slice(0x00FADF, 0x00FB00),
    make_slice(0x00FBA4, 0x00FBAC),
    make_slice(0x00FD24, 0x00FD26),
    make_slice(0x00FD9D, 0x00FDA5),
    make_slice(0x00FE0E, 0x00FE16),
    make_slice(0x00FE94, 0x00FEA8),
    make_slice(0x00FF93, 0x018008),
    make_slice(0x018137, 0x018151),
    make_slice(0x01817D, 0x01830F),
    make_slice(0x018335, 0x018339),
    make_slice(0x01834C, 0x01834E),
    make_slice(0x0183A0, 0x0183A4),
    make_slice(0x0183B3, 0x0183B5),
    make_slice(0x0183EF, 0x0183F2),
    make_slice(0x018466, 0x018468),
    make_slice(0x018526, 0x018528),
    SnesSlice::new(AddrSnes(0x0185CC), 402),
    make_slice(0x018335, 0x018339),
    make_slice(0x0183A0, 0x0183A4),
    make_slice(0x0183B3, 0x0183B5),
    make_slice(0x0183EF, 0x0183F2),
    make_slice(0x018466, 0x018468),
    make_slice(0x018526, 0x018528),
    make_slice(0x0185CC, 0x01875E),
    make_slice(0x0188EC, 0x018904),
    make_slice(0x018CBA, 0x018CBE),
    make_slice(0x018DC7, 0x018DE9),
    make_slice(0x018E6E, 0x018E76),
    make_slice(0x018FC7, 0x018FE7),
    make_slice(0x0190BA, 0x019138),
    make_slice(0x019284, 0x019288),
    make_slice(0x0192C1, 0x0192C9),
    make_slice(0x0195FC, 0x019624),
    make_slice(0x0197AD, 0x0197D5),
    make_slice(0x0198A7, 0x0198A9),
    make_slice(0x019910, 0x019913),
    make_slice(0x019A22, 0x019A2A),
    make_slice(0x019A4E, 0x019A52),
    make_slice(0x019B83, 0x019CF3),
    make_slice(0x019E10, 0x019E28),
    make_slice(0x019F5B, 0x019F71),
    make_slice(0x019F99, 0x019F9B),
    make_slice(0x01A35A, 0x01A365),
    make_slice(0x01A40B, 0x01A40D),
    make_slice(0x01A61E, 0x01A625),
    make_slice(0x01A6D7, 0x01A6D9),
    make_slice(0x01A778, 0x01A77C),
    make_slice(0x01A799, 0x01A79C),
    make_slice(0x01A7C9, 0x01A7DC),
    make_slice(0x01A839, 0x01A83B),
    make_slice(0x01AB2D, 0x01AB31),
    make_slice(0x01AB6A, 0x01AB6F),
    make_slice(0x01AD54, 0x01AD59),
    make_slice(0x01AD68, 0x01AD6E),
    make_slice(0x01AE7F, 0x01AE90),
    make_slice(0x01AEBD, 0x01AEC3),
    make_slice(0x01AF40, 0x01AF54),
    make_slice(0x01B012, 0x01B014),
    make_slice(0x01B01D, 0x01B033),
    make_slice(0x01B1B1, 0x01B1B4),
    make_slice(0x01B212, 0x01B216),
    make_slice(0x01B268, 0x01B26C),
    make_slice(0x01B2C3, 0x01B2D1),
    make_slice(0x01B383, 0x01B395),
    make_slice(0x01B4F9, 0x01B505),
    make_slice(0x01B65A, 0x01B666),
    make_slice(0x01B69F, 0x01B6A5),
    make_slice(0x01B93C, 0x01B93E),
    make_slice(0x01B969, 0x01B97F),
    make_slice(0x01BA95, 0x01BACC),
    make_slice(0x01BC34, 0x01BC38),
    make_slice(0x01BCE0, 0x01BCF0),
    make_slice(0x01BDEA, 0x01BDF2),
    make_slice(0x01BE69, 0x01BE6E),
    make_slice(0x01C0A5, 0x01C0A7),
    make_slice(0x01C1EE, 0x01C1F2),
    make_slice(0x01C313, 0x01C317),
    make_slice(0x01C345, 0x01C349),
    make_slice(0x01C510, 0x01C538),
    make_slice(0x01C554, 0x01C560),
    make_slice(0x01C609, 0x01C61A),
    make_slice(0x01C66D, 0x01C670),
    make_slice(0x01C6E6, 0x01C6ED),
    make_slice(0x01C9B7, 0x01C9BF),
    make_slice(0x01C9D6, 0x01C9DA),
    make_slice(0x01CD1E, 0x01CD2A),
    make_slice(0x01CD92, 0x01CDA7),
    make_slice(0x01CE12, 0x01CE1E),
    make_slice(0x01CE65, 0x01CE6B),
    make_slice(0x01CE65, 0x01CE6B),
    make_slice(0x01CE72, 0x01CE78),
    make_slice(0x01CEAE, 0x01CEB6),
    make_slice(0x01D057, 0x01D059),
    make_slice(0x01D0BE, 0x01D0C0),
    make_slice(0x01D0D4, 0x01D0D7),
    make_slice(0x01D0DE, 0x01D116),
    make_slice(0x01D11D, 0x01D121),
    make_slice(0x01D122, 0x01D146),
    make_slice(0x01D239, 0x01D23F),
    make_slice(0x01D439, 0x01D43E),
    make_slice(0x01D442, 0x01D44E),
    make_slice(0x01D4E7, 0x01D4FB),
    make_slice(0x01D55E, 0x01D5B3),
    make_slice(0x01D717, 0x01D719),
    make_slice(0x01D762, 0x01D768),
    make_slice(0x01D7E1, 0x01D7F4),
    make_slice(0x01DB5A, 0x01DB5C),
    make_slice(0x01DB96, 0x01DBA2),
    make_slice(0x01DC09, 0x01DC0B),
    make_slice(0x01DC3B, 0x01DC54),
    make_slice(0x01DCD1, 0x01DDAC),
    make_slice(0x01DE11, 0x01DE2A),
    make_slice(0x01DEE3, 0x01DF19),
    make_slice(0x01DFC1, 0x01DFD9),
    make_slice(0x01E07B, 0x01E093),
    make_slice(0x01E190, 0x01E198),
    make_slice(0x01E2B0, 0x01E2CF),
    make_slice(0x01E2D8, 0x01E2E0),
    make_slice(0x01E35F, 0x01E363),
    make_slice(0x01E38F, 0x01E393),
    make_slice(0x01E41F, 0x01E42B),
    make_slice(0x01E43C, 0x01E43E),
    make_slice(0x01E611, 0x01E623),
    make_slice(0x01E6FD, 0x01E700),
    make_slice(0x01E76F, 0x01E7A4),
    make_slice(0x01E985, 0x01E98D),
    make_slice(0x01EA17, 0x01EA19),
    make_slice(0x01EBB4, 0x01EBCA),
    make_slice(0x01EC5B, 0x01EC61),
    make_slice(0x01EDE2, 0x01EE61),
    make_slice(0x01F0CB, 0x01F0D3),
    make_slice(0x01F137, 0x01F14B),
    make_slice(0x01F2D9, 0x01F2DF),
    make_slice(0x01F2FF, 0x01F309),
    make_slice(0x01F3D9, 0x01F3DB),
    make_slice(0x01F60A, 0x01F622),
    make_slice(0x01F75C, 0x01F764),
    make_slice(0x01F82D, 0x01F83D),
    make_slice(0x01F873, 0x01F875),
    make_slice(0x01F88C, 0x01F890),
    make_slice(0x01F8CF, 0x01F8D5),
    make_slice(0x01FA37, 0x01FA3D),
    make_slice(0x01FA4C, 0x01FA58),
    make_slice(0x01FAB4, 0x01FAC1),
    make_slice(0x01FAD5, 0x01FAF5),
    make_slice(0x01FD95, 0x01FDA7),
    make_slice(0x01FDF3, 0x01FEBC),
    make_slice(0x01FF53, 0x01FF5B),
    make_slice(0x01FFBF, 0x028008),
    make_slice(0x028072, 0x02808A),
    make_slice(0x0283C8, 0x0283CE),
    make_slice(0x028510, 0x028528),
    make_slice(0x02873A, 0x028752),
    make_slice(0x028789, 0x028792),
];