pub const boot: &[u8; 0x0100] = &[
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF,
    0x0E, 0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E,
    0xFC, 0xE0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96,
    0x00, 0x13, 0x7B, 0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22,
    0x23, 0x05, 0x20, 0xF9, 0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D,
    0x28, 0x08, 0x32, 0x0D, 0x20, 0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0,
    0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04, 0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20,
    0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2, 0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62,
    0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06, 0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0,
    0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20, 0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F,
    0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17, 0x05, 0x20, 0xF5, 0x22, 0x23,
    0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83,
    0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E,
    0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
    0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C, 0x21,
    0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0,
    0x50
];

pub const game_stub: &[u8] = &[
    0xC3, 0x0C, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC3, 0x0C, 0x02, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x87, 0xE1,
    0x5F, 0x16, 0x00, 0x19, 0x5E, 0x23, 0x56, 0xD5, 0xE1, 0xE9, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC3, 0x7E, 0x01, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xC3, 0xBE, 0x26, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC3, 0xBE, 0x26, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xC3, 0x5B, 0x00, 0xF5, 0xE5, 0xD5, 0xC5, 0xCD, 0x6B, 0x00,
    0x3E, 0x01, 0xE0, 0xCC, 0xC1, 0xD1, 0xE1, 0xF1, 0xD9, 0xF0, 0xCD, 0xEF, 0x78, 0x00,
    0x9F, 0x00, 0xA4, 0x00, 0xBA, 0x00, 0xEA, 0x27, 0xF0, 0xE1, 0xFE, 0x07, 0x28, 0x08,
    0xFE, 0x06, 0xC8, 0x3E, 0x06, 0xE0, 0xE1, 0xC9, 0xF0, 0x01, 0xFE, 0x55, 0x20, 0x08,
    0x3E, 0x29, 0xE0, 0xCB, 0x3E, 0x01, 0x18, 0x08, 0xFE, 0x29, 0xC0, 0x3E, 0x55, 0xE0,
    0xCB, 0xAF, 0xE0, 0x02, 0xC9, 0xF0, 0x01, 0xE0, 0xD0, 0xC9, 0xF0, 0x01, 0xE0, 0xD0,
    0xF0, 0xCB, 0xFE, 0x29, 0xC8, 0xF0, 0xCF, 0xE0, 0x01, 0x3E, 0xFF, 0xE0, 0xCF, 0x3E,
    0x80, 0xE0, 0x02, 0xC9, 0xF0, 0x01, 0xE0, 0xD0, 0xF0, 0xCB, 0xFE, 0x29, 0xC8, 0xF0,
    0xCF, 0xE0, 0x01, 0xFB, 0xCD, 0x98, 0x0A, 0x3E, 0x80, 0xE0, 0x02, 0xC9, 0xF0, 0xCD,
    0xFE, 0x02, 0xC0, 0xAF, 0xE0, 0x0F, 0xFB, 0xC9, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xC3, 0x50, 0x01, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D,
    0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F,
    0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB,
    0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
    0x54, 0x45, 0x54, 0x52, 0x49, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0A, 0x16, 0xBF,
    0xC3, 0x0C, 0x02, 0xCD, 0xE3, 0x29, 0xF0, 0x41, 0xE6, 0x03, 0x20, 0xFA, 0x46, 0xF0,
    0x41, 0xE6, 0x03, 0x20, 0xFA, 0x7E, 0xA0, 0xC9, 0x7B, 0x86, 0x27, 0x22, 0x7A, 0x8E,
    0x27, 0x22, 0x3E, 0x00, 0x8E, 0x27, 0x77, 0x3E, 0x01, 0xE0, 0xE0, 0xD0, 0x3E, 0x99,
    0x32, 0x32, 0x77, 0xC9, 0xF5, 0xC5, 0xD5, 0xE5, 0xF0, 0xCE, 0xA7, 0x28, 0x12, 0xF0,
    0xCB, 0xFE, 0x29, 0x20, 0x0C, 0xAF, 0xE0, 0xCE, 0xF0, 0xCF, 0xE0, 0x01, 0x21, 0x02,
    0xFF, 0x36, 0x81, 0xCD, 0xE0, 0x21, 0xCD, 0xCC, 0x23, 0xCD, 0xB7, 0x23, 0xCD, 0x9E,
    0x23, 0xCD, 0x8C, 0x23, 0xCD, 0x7D, 0x23, 0xCD, 0x6E, 0x23, 0xCD, 0x5F, 0x23, 0xCD,
    0x50, 0x23, 0xCD, 0x41, 0x23, 0xCD, 0x32, 0x23, 0xCD, 0x23, 0x23, 0xCD, 0xF8, 0x22,
    0xCD, 0xE9, 0x22, 0xCD, 0xDA, 0x22, 0xCD, 0xCB, 0x22, 0xCD, 0xBC, 0x22, 0xCD, 0xAD,
    0x22, 0xCD, 0x9E, 0x22, 0xCD, 0xD7, 0x1E, 0xCD, 0xB6, 0xFF, 0xCD, 0xCA, 0x18, 0xFA,
    0xCE, 0xC0, 0xA7, 0x28, 0x1A, 0xF0, 0x98, 0xFE, 0x03, 0x20, 0x14, 0x21, 0x6D, 0x98,
    0xCD, 0x3B, 0x24, 0x3E, 0x01, 0xE0, 0xE0, 0x21, 0x6D, 0x9C, 0xCD, 0x3B, 0x24, 0xAF,
    0xEA, 0xCE, 0xC0, 0x21, 0xE2, 0xFF, 0x34, 0xAF, 0xE0, 0x43, 0xE0, 0x42, 0x3C, 0xE0,
    0x85, 0xE1, 0xD1, 0xC1, 0xF1, 0xD9, 0xAF, 0x21, 0xFF, 0xDF, 0x0E, 0x10, 0x06, 0x00,
    0x32, 0x05, 0x20, 0xFC, 0x0D, 0x20, 0xF9, 0x3E, 0x01, 0xF3, 0xE0, 0x0F, 0xE0, 0xFF,
    0xAF, 0xE0, 0x42, 0xE0, 0x43, 0xE0, 0xA4, 0xE0, 0x41, 0xE0, 0x01, 0xE0, 0x02, 0x3E,
    0x80, 0xE0, 0x40, 0xF0, 0x44, 0xFE, 0x94, 0x20, 0xFA, 0x3E, 0x03, 0xE0, 0x40, 0x3E,
    0xE4, 0xE0, 0x47, 0xE0, 0x48, 0x3E, 0xC4, 0xE0, 0x49, 0x21, 0x26, 0xFF, 0x3E, 0x80,
    0x32, 0x3E, 0xFF, 0x32, 0x36, 0x77, 0x3E, 0x01, 0xEA, 0x00, 0x20, 0x31, 0xFF, 0xCF,
    0xAF, 0x21, 0xFF, 0xDF, 0x06, 0x00, 0x32, 0x05, 0x20, 0xFC, 0x21, 0xFF, 0xCF, 0x0E,
    0x10, 0x06, 0x00, 0x32, 0x05, 0x20, 0xFC, 0x0D, 0x20, 0xF9, 0x21, 0xFF, 0x9F, 0x0E,
    0x20, 0xAF, 0x06, 0x00, 0x32, 0x05, 0x20, 0xFC, 0x0D, 0x20, 0xF9, 0x21, 0xFF, 0xFE,
    0x06, 0x00, 0x32, 0x05, 0x20, 0xFC, 0x21, 0xFE, 0xFF, 0x06, 0x80, 0x32, 0x05, 0x20,
    0xFC, 0x0E, 0xB6, 0x06, 0x0C, 0x21, 0x7F, 0x2A, 0x2A, 0xE2, 0x0C, 0x05, 0x20, 0xFA,
    0xCD, 0x95, 0x27, 0xCD, 0xF3, 0x7F, 0x3E, 0x09, 0xE0, 0xFF, 0x3E, 0x37, 0xE0, 0xC0,
    0x3E, 0x1C, 0xE0, 0xC1, 0x3E, 0x24, 0xE0, 0xE1, 0x3E, 0x80, 0xE0, 0x40, 0xFB, 0xAF,
    0xE0, 0x0F, 0xE0, 0x4A, 0xE0, 0x4B, 0xE0, 0x06, 0xCD, 0xA6, 0x29, 0xCD, 0xF8, 0x02,
    0xCD, 0xF0, 0x7F, 0xF0, 0x80, 0xE6, 0x0F, 0xFE, 0x0F, 0xCA, 0x1B, 0x02, 0x21, 0xA6,
    0xFF, 0x06, 0x02, 0x7E, 0xA7, 0x28, 0x01, 0x35, 0x2C, 0x05, 0x20, 0xF7, 0xF0, 0xC5,
    0xA7, 0x28, 0x04, 0x3E, 0x09, 0xE0, 0xFF, 0xF0, 0x85, 0xA7, 0x28, 0xFB, 0xAF, 0xE0,
    0x85, 0xC3, 0xC4, 0x02, 0xF0, 0xE1, 0xEF, 0xCE, 0x1B, 0xE2, 0x1C, 0x44, 0x12, 0x7B,
    0x12, 0x06, 0x1D, 0x26, 0x1D, 0xAE, 0x03, 0x79, 0x04, 0x44, 0x14, 0x8C, 0x14, 0x07,
    0x1A, 0xC0, 0x1D, 0x16, 0x1F, 0x1F, 0x1F, 0x25, 0x15, 0xB0, 0x14, 0x7B, 0x15, 0xBF,
    0x15, 0x29, 0x16, 0x7A, 0x16, 0xEB, 0x16, 0x13, 0x19, 0x77, 0x06, 0x2C, 0x07, 0x25,
    0x08, 0xE4, 0x08, 0x31, 0x0B, 0xEB, 0x0C, 0xD2, 0x0A, 0x32, 0x0D, 0x23, 0x0E, 0x12,
    0x11, 0x99, 0x0D, 0x8A, 0x0E, 0xCE, 0x1D, 0x41, 0x1E, 0x69, 0x03, 0x93, 0x03, 0x67,
    0x11, 0xE6, 0x11, 0xFC, 0x11, 0x1C, 0x12, 0xC7, 0x05, 0xF7, 0x05, 0xB3, 0x12, 0x05,
    0x13, 0x24, 0x13, 0x51, 0x13, 0x67, 0x13, 0x7E, 0x13, 0xB5, 0x13, 0xE5, 0x13, 0x1B,
    0x13, 0xA0, 0x03, 0xEA, 0x27, 0xCD, 0x20, 0x28, 0xCD, 0xD7, 0x27, 0x11, 0x07, 0x4A,
    0xCD, 0xEB, 0x27, 0xCD, 0x8A, 0x17, 0x21, 0x00, 0xC3, 0x11, 0x50, 0x64, 0x1A, 0x22,
    0x13, 0x7C, 0xFE, 0xC4, 0x20, 0xF8, 0x3E, 0xD3, 0xE0, 0x40, 0x3E, 0xFA, 0xE0, 0xA6,
    0x3E, 0x25, 0xE0, 0xE1, 0xC9, 0xF0, 0xA6, 0xA7, 0xC0, 0x3E, 0xFA, 0xE0, 0xA6, 0x3E,
    0x35, 0xE0, 0xE1, 0xC9, 0xF0, 0x81, 0xA7, 0x20, 0x04, 0xF0, 0xA6, 0xA7, 0xC0, 0x3E,
    0x06, 0xE0, 0xE1, 0xC9, 0xCD, 0x20, 0x28, 0xAF, 0xE0, 0xE9, 0xE0, 0x98, 0xE0, 0x9C,
    0xE0, 0x9B, 0xE0, 0xFB, 0xE0, 0x9F, 0xE0, 0xE3, 0xE0, 0xC7, 0xCD, 0x93, 0x22, 0xCD,
    0x51, 0x26, 0xCD, 0xD7, 0x27, 0x21, 0x00, 0xC8, 0x3E, 0x2F, 0x22, 0x7C, 0xFE, 0xCC,
    0x20, 0xF8, 0x21, 0x01, 0xC8, 0xCD, 0xA9, 0x26, 0x21, 0x0C, 0xC8, 0xCD, 0xA9, 0x26,
    0x21, 0x41, 0xCA, 0x06, 0x0C, 0x3E, 0x8E, 0x22, 0x05, 0x20, 0xFC, 0x11, 0x6F, 0x4B,
    0xCD, 0xEB, 0x27, 0xCD, 0x8A, 0x17, 0x21, 0x00, 0xC0, 0x36, 0x80, 0x2C, 0x36, 0x10,
    0x2C, 0x36,
];