#[rustfmt::skip]

// Bitrate Lookup Table
// |----|--------|--------|--------|-------------|
// |bits| V1, L1 | V1, L2 | V1, L3 | V2, L2 & L3 |
// |----|--------|--------|--------|-------------|
// |0000|free    |free    |free    |free         |
// |0001|32      |32      |32      |8            |
// |0010|64      |48      |40      |16           |
// |0011|96      |56      |48      |24           |
// |0100|128     |64      |56      |32           |
// |0101|160     |80      |64      |40           |
// |0110|192     |96      |80      |48           |
// |0111|224     |112     |96      |56           |
// |1000|256     |128     |112     |64           |
// |1001|288     |160     |128     |80           |
// |1010|320     |192     |160     |96           |
// |1011|352     |224     |192     |112          |
// |1100|384     |256     |224     |128          |
// |1101|416     |320     |256     |144          |
// |1110|448     |384     |320     |160          |
// |1111|bad     |bad     |bad     |bad          |
// |----|--------|--------|--------|-------------|
pub static BITRATE_INDEX: [[u16; 15]; 5] = [
    [0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448,], // Version 1, Layer 1
    [0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384,], // Version 1, Layer 2
    [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320,], // Version 1, Layer 3
    [0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256,], // Version 2, Layer 1
    [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160], // Version 2, Layer 2 & Layer 3
];

// Sampling Rate Frequency Index (Hz)
// |------|-------|-------|---------|
// | bits | MPEG1 | MPEG2 | MPEG2.5 |
// |------|-------|-------|---------|
// |  00  | 44100 | 22050 | 11025   |
// |  01  | 48000 | 24000 | 12000   |
// |  10  | 32000 | 16000 | 8000    |
// |  11  |reserv.|reserv.| reserv. |
// |------|-------|-------|---------|
pub static SAMPLING_RATE: [[u16; 3]; 3] = [
    [44100, 48000, 32000],
    [22050, 24000, 16000],
    [11025, 12000, 8000],
];

// |----------------|-------|-------|
// | scale compress | slen1 | slen2 |
// |----------------|-------|-------|
// | 0              | 0     | 0     |
// | 1              | 0     | 1     |
// | 2              | 0     | 2     |
// | 3              | 0     | 3     |
// | 4              | 3     | 0     |
// | 5              | 1     | 1     |
// | 6              | 1     | 2     |
// | 7              | 1     | 3     |
// | 8              | 2     | 1     |
// | 9              | 2     | 2     |
// | 10             | 2     | 3     |
// | 11             | 3     | 1     |
// | 12             | 3     | 2     |
// | 13             | 3     | 3     |
// | 14             | 4     | 2     |
// | 15             | 4     | 3     |
// |----------------|-------|-------|
pub static SCALE_COMPRESS: [(u8, u8); 16] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (3, 0),
    (1, 1),
    (1, 2),
    (1, 3),
    (2, 1),
    (2, 2),
    (2, 3),
    (3, 1),
    (3, 2),
    (3, 3),
    (4, 2),
    (4, 3),
];
