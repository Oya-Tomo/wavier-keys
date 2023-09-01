/*
 * https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2
 */

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum KeyCodes {
    Reserved = 0x00,       // No key pressed
    ErrOvf = 0x01, // Keyboard Error Roll Over - used for all slots if too many keys are pressed ("Phantom key")
    PostFail = 0x02, // Keyboard POST Fail
    ErrorUndefined = 0x03, // Keyboard Error Undefined

    KeyA = 0x04, // Keyboard a and A
    KeyB = 0x05, // Keyboard b and B
    KeyC = 0x06, // Keyboard c and C
    KeyD = 0x07, // Keyboard d and D
    KeyE = 0x08, // Keyboard e and E
    KeyF = 0x09, // Keyboard f and F
    KeyG = 0x0a, // Keyboard g and G
    KeyH = 0x0b, // Keyboard h and H
    KeyI = 0x0c, // Keyboard i and I
    KeyJ = 0x0d, // Keyboard j and J
    KeyK = 0x0e, // Keyboard k and K
    KeyL = 0x0f, // Keyboard l and L
    KeyM = 0x10, // Keyboard m and M
    KeyN = 0x11, // Keyboard n and N
    KeyO = 0x12, // Keyboard o and O
    KeyP = 0x13, // Keyboard p and P
    KeyQ = 0x14, // Keyboard q and Q
    KeyR = 0x15, // Keyboard r and R
    KeyS = 0x16, // Keyboard s and S
    KeyT = 0x17, // Keyboard t and T
    KeyU = 0x18, // Keyboard u and U
    KeyV = 0x19, // Keyboard v and V
    KeyW = 0x1a, // Keyboard w and W
    KeyX = 0x1b, // Keyboard x and X
    KeyY = 0x1c, // Keyboard y and Y
    KeyZ = 0x1d, // Keyboard z and Z

    Key1 = 0x1e, // Keyboard 1 and !
    Key2 = 0x1f, // Keyboard 2 and @
    Key3 = 0x20, // Keyboard 3 and #
    Key4 = 0x21, // Keyboard 4 and $
    Key5 = 0x22, // Keyboard 5 and %
    Key6 = 0x23, // Keyboard 6 and ^
    Key7 = 0x24, // Keyboard 7 and &
    Key8 = 0x25, // Keyboard 8 and *
    Key9 = 0x26, // Keyboard 9 and (
    Key0 = 0x27, // Keyboard 0 and )

    Enter = 0x28,      // Keyboard Return (ENTER)
    Escape = 0x29,     // Keyboard ESCAPE
    BackSpace = 0x2a,  // Keyboard DELETE (Backspace)
    Tab = 0x2b,        // Keyboard Tab
    Space = 0x2c,      // Keyboard Spacebar
    Minus = 0x2d,      // Keyboard - and _
    Equal = 0x2e,      // Keyboard = and +
    LeftBrace = 0x2f,  // Keyboard [ and {
    RightBrace = 0x30, // Keyboard ] and }
    BackSlash = 0x31,  // Keyboard \ and |
    HashTilde = 0x32,  // Keyboard Non-US # and ~
    SemiColon = 0x33,  // Keyboard ; and :
    Apostrophe = 0x34, // Keyboard ' and "
    Grave = 0x35,      // Keyboard ` and ~
    Comma = 0x36,      // Keyboard , and <
    Dot = 0x37,        // Keyboard . and >
    Slash = 0x38,      // Keyboard / and ?
    CapsLock = 0x39,   // Keyboard Caps Lock

    F1 = 0x3a,  // Keyboard F1
    F2 = 0x3b,  // Keyboard F2
    F3 = 0x3c,  // Keyboard F3
    F4 = 0x3d,  // Keyboard F4
    F5 = 0x3e,  // Keyboard F5
    F6 = 0x3f,  // Keyboard F6
    F7 = 0x40,  // Keyboard F7
    F8 = 0x41,  // Keyboard F8
    F9 = 0x42,  // Keyboard F9
    F10 = 0x43, // Keyboard F10
    F11 = 0x44, // Keyboard F11
    F12 = 0x45, // Keyboard F12

    PrintScreen = 0x46, // Keyboard Print Screen
    ScrollLock = 0x47,  // Keyboard Scroll Lock
    Pause = 0x48,       // Keyboard Pause
    Insert = 0x49,      // Keyboard Insert
    Home = 0x4a,        // Keyboard Home
    PageUp = 0x4b,      // Keyboard Page Up
    Delete = 0x4c,      // Keyboard Delete Forward
    End = 0x4d,         // Keyboard End
    PageDown = 0x4e,    // Keyboard Page Down
    Right = 0x4f,       // Keyboard Right Arrow
    Left = 0x50,        // Keyboard Left Arrow
    Down = 0x51,        // Keyboard Down Arrow
    Up = 0x52,          // Keyboard Up Arrow

    NumLock = 0x53,    // Keyboard Num Lock and Clear
    KPSlash = 0x54,    // Keypad /
    KPAsterisk = 0x55, // Keypad *
    KPMinum = 0x56,    // Keypad -
    KPPlus = 0x57,     // Keypad +
    KPEnter = 0x58,    // Keypad ENTER
    KP1 = 0x59,        // Keypad 1 and End
    KP2 = 0x5a,        // Keypad 2 and Down Arrow
    KP3 = 0x5b,        // Keypad 3 and PageDn
    KP4 = 0x5c,        // Keypad 4 and Left Arrow
    KP5 = 0x5d,        // Keypad 5
    KP6 = 0x5e,        // Keypad 6 and Right Arrow
    KP7 = 0x5f,        // Keypad 7 and Home
    KP8 = 0x60,        // Keypad 8 and Up Arrow
    KP9 = 0x61,        // Keypad 9 and Page Up
    KP0 = 0x62,        // Keypad 0 and Insert
    KPDot = 0x63,      // Keypad . and Delete

    K102ND = 0x64,  // Keyboard Non-US \ and |
    Compose = 0x65, // Keyboard Application
    Power = 0x66,   // Keyboard Power
    KPEqual = 0x67, // Keypad =

    F13 = 0x68, // Keyboard F13
    F14 = 0x69, // Keyboard F14
    F15 = 0x6a, // Keyboard F15
    F16 = 0x6b, // Keyboard F16
    F17 = 0x6c, // Keyboard F17
    F18 = 0x6d, // Keyboard F18
    F19 = 0x6e, // Keyboard F19
    F20 = 0x6f, // Keyboard F20
    F21 = 0x70, // Keyboard F21
    F22 = 0x71, // Keyboard F22
    F23 = 0x72, // Keyboard F23
    F24 = 0x73, // Keyboard F24

    Open = 0x74,       // Keyboard Execute
    Help = 0x75,       // Keyboard Help
    Props = 0x76,      // Keyboard Menu
    Front = 0x77,      // Keyboard Select
    Stop = 0x78,       // Keyboard Stop
    Again = 0x79,      // Keyboard Again
    Undo = 0x7a,       // Keyboard Undo
    Cut = 0x7b,        // Keyboard Cut
    Copy = 0x7c,       // Keyboard Copy
    Paste = 0x7d,      // Keyboard Paste
    Find = 0x7e,       // Keyboard Find
    Mute = 0x7f,       // Keyboard Mute
    VolumeUp = 0x80,   // Keyboard Volume Up
    VolumeDown = 0x81, // Keyboard Volume Down

    LockingCapsLock = 0x82,   // Keyboard Locking Caps Lock
    LockingNumLock = 0x83,    // Keyboard Locking Num Lock
    LockingScrollLock = 0x84, // Keyboard Locking Scroll Lock
    KPComma = 0x85,           // Keypad Comma
    KPEqualSign = 0x86,       // Keypad Equal Sign

    RO = 0x87,               // Keyboard International1
    KATAKANAHIRAGANA = 0x88, // Keyboard International2
    YEN = 0x89,              // Keyboard International3
    HENKAN = 0x8a,           // Keyboard International4
    MUHENKAN = 0x8b,         // Keyboard International5
    KPJPCOMMA = 0x8c,        // Keyboard International6
    INTR7 = 0x8d,            // Keyboard International7
    INTR8 = 0x8e,            // Keyboard International8
    INTR9 = 0x8f,            // Keyboard International9

    HANGEUL = 0x90,         // Keyboard LANG1
    HANJA = 0x91,           // Keyboard LANG2
    KATAKANA = 0x92,        // Keyboard LANG3
    HIRAGANA = 0x93,        // Keyboard LANG4
    ZENKAKUHANKAKU = 0x94,  // Keyboard LANG5
    LANG6 = 0x95,           // Keyboard LANG6
    LANG7 = 0x96,           // Keyboard LANG7
    LANG8 = 0x97,           // Keyboard LANG8
    LANG9 = 0x98,           // Keyboard LANG9
    AlternateErase = 0x99,  // Keyboard Alternate Erase
    SysReqAttention = 0x9a, // Keyboard SysReq/Attention

    Cancel = 0x9b,     // Keyboard Cancel
    Clear = 0x9c,      // Keyboard Clear
    Prior = 0x9d,      // Keyboard Prior
    Return = 0x9e,     // Keyboard Return
    Separator = 0x9f,  // Keyboard Separator
    Out = 0xa0,        // Keyboard Out
    Oper = 0xa1,       // Keyboard Oper
    ClearAgain = 0xa2, // Keyboard Clear/Again
    CrSelProps = 0xa3, // Keyboard CrSel/Props
    ExSel = 0xa4,      // Keyboard ExSel

    K00 = 0xb0,  // Keypad 00
    K000 = 0xb1, // Keypad 000

    ThousandsSeparator = 0xb2, // Thousands Separator
    DecimalSeparator = 0xb3,   // Decimal Separator
    CurrencyUnit = 0xb4,       // Currency Unit
    CurrencySubunit = 0xb5,    //  Currency Sub-unit
    KPLeftParen = 0xb6,        // Keypad (
    KPRightParen = 0xb7,       // Keypad )

    /*
    0xb8  Keypad {
    0xb9  Keypad }
    0xba  Keypad Tab
    0xbb  Keypad Backspace
    0xbc  Keypad A
    0xbd  Keypad B
    0xbe  Keypad C
    0xbf  Keypad D
    0xc0  Keypad E
    0xc1  Keypad F
    0xc2  Keypad XOR
    0xc3  Keypad ^
    0xc4  Keypad %
    0xc5  Keypad <
    0xc6  Keypad >
    0xc7  Keypad &
    0xc8  Keypad &&
    0xc9  Keypad |
    0xca  Keypad ||
    0xcb  Keypad :
    0xcc  Keypad #
    0xcd  Keypad Space
    0xce  Keypad @
    0xcf  Keypad !
    0xd0  Keypad Memory Store
    0xd1  Keypad Memory Recall
    0xd2  Keypad Memory Clear
    0xd3  Keypad Memory Add
    0xd4  Keypad Memory Subtract
    0xd5  Keypad Memory Multiply
    0xd6  Keypad Memory Divide
    0xd7  Keypad +/-
    0xd8  Keypad Clear
    0xd9  Keypad Clear Entry
    0xda  Keypad Binary
    0xdb  Keypad Octal
    0xdc  Keypad Decimal
    0xdd  Keypad Hexadecimal
    */
    LeftCtrl = 0xe0,   // Keyboard Left Control
    LeftShift = 0xe1,  // Keyboard Left Shift
    LeftAlt = 0xe2,    // Keyboard Left Alt
    LeftMeta = 0xe3,   // Keyboard Left GUI
    RightCtrl = 0xe4,  // Keyboard Right Control
    RightShift = 0xe5, // Keyboard Right Shift
    RightAlt = 0xe6,   // Keyboard Right Alt
    RightMeta = 0xe7,  // Keyboard Right GUI

    MediaPlayPause = 0xe8,
    MediaStopCD = 0xe9,
    MediaPreviousSong = 0xea,
    MediaNextSong = 0xeb,
    MediaEjectCD = 0xec,
    MediaVolumeUp = 0xed,
    MediaVolumeDown = 0xee,
    MediaMute = 0xef,
    MediaWWW = 0xf0,
    MediaBack = 0xf1,
    MediaForward = 0xf2,
    MediaStop = 0xf3,
    MediaFing = 0xf4,
    MediaScrollUp = 0xf5,
    MediaScrollDown = 0xf6,
    MediaEdit = 0xf7,
    MediaSleep = 0xf8,
    MediaCoffee = 0xf9,
    MediaRefresh = 0xfa,
    MediaCalc = 0xfb,
}

#[allow(unused)]
pub enum ModifierMasks {
    LeftCtrl = 1 << 0,
    LeftShift = 1 << 1,
    LeftAlt = 1 << 2,
    LeftGui = 1 << 3,
    RightCtrl = 1 << 4,
    RightShift = 1 << 5,
    RightAlt = 1 << 6,
    RightGui = 1 << 7,
}
